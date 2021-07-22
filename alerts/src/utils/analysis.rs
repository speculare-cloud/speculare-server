use super::{query::execute_query, IncidentStatus, QueryType, Severity};

use chrono::prelude::Utc;
use evalexpr::*;
use sproot::{
    models::{Alerts, Incidents, IncidentsDTO, IncidentsDTOUpdate},
    ConnType,
};

/// This function is the core of the monitoring, this is where we:
/// - Execute the query and get the result
/// - Evaluate if we need to trigger an incidents or not
pub fn execute_analysis(query: &str, alert: &Alerts, qtype: &QueryType, conn: &ConnType) {
    // Execute the query passed as arguement (this query was build previously)
    let result = execute_query(query, &alert.host_uuid, qtype, conn);
    trace!("{}", &result);

    // Determine if we are in a Warn or Crit level of incidents
    let should_warn = eval_boolean(&alert.warn.replace("$this", &result)).unwrap_or_else(|e| {
        panic!(
            "Failed to parse the String to an expression (warn: {}): {}",
            alert.warn, e
        )
    });
    let should_crit = eval_boolean(&alert.crit.replace("$this", &result)).unwrap_or_else(|e| {
        panic!(
            "Failed to parse the String to an expression (crit: {}): {}",
            alert.crit, e
        )
    });
    trace!("{:?}, {:?}", should_warn, should_crit);

    // Check if an active incident already exist for this alarm.
    let prev_incident = Incidents::exist(conn, alert.id);
    let prev_incident: Option<Incidents> = match prev_incident {
        Ok(res) => Some(res),
        Err(err) => {
            // If the error if not NofFound, this mean we have something else to care about
            if err != diesel::result::Error::NotFound {
                panic!("prev_incident returned an error that is not NotFound");
            }
            None
        }
    };
    trace!("Previous incident is some: {}", prev_incident.is_some());

    // Assert that we do not create an incident for nothing
    if !(should_warn || should_crit) {
        trace!("We don't need to create an incident");
        // Check if an incident was active
        if let Some(prev_incident) = prev_incident {
            trace!("We need to resolve the previous incident however");
            let incident_id = prev_incident.id;
            let incident_dto = IncidentsDTOUpdate {
                status: Some(IncidentStatus::Resolved as i32),
                ..Default::default()
            };
            Incidents::update(conn, &incident_dto, incident_id)
                .expect("Failed to update (resolve) the incidents");
        }
        return;
    }

    // Determine the incident severity
    let severity = match (should_warn, should_crit) {
        (true, false) => Severity::Warning,
        (false, true) => Severity::Critical,
        (true, true) => Severity::Critical,
        (false, false) => {
            panic!("should_warn && should_crit are both false, this should never happens.")
        }
    };
    trace!("The severity of this one is: {}", severity.to_string());

    // If it exist we create an update in the cases where:
    // - We need to update the severity of the incidents
    // - The result changed
    // In all cases we need to update the updated_at field.
    match prev_incident {
        Some(incident) => {
            trace!("Update the previous incident using the new values");
            let incident_id = incident.id;
            // Update the previous incident
            let incident_dto = IncidentsDTOUpdate {
                result: Some(result),
                updated_at: Some(Utc::now().naive_local()),
                severity: Some(severity as i32),
                ..Default::default()
            };
            Incidents::update(conn, &incident_dto, incident_id)
                .expect("Failed to update the incidents");
        }
        None => {
            trace!("Create a new incident based on the current values");
            // Clone the alert to allow us to own it in the IncidentsDTO
            let calert = alert.clone();
            let incident = IncidentsDTO {
                result,
                updated_at: Utc::now().naive_local(),
                host_uuid: calert.host_uuid,
                status: IncidentStatus::Active as i32,
                severity: severity as i32,
                alerts_id: calert.id,
                alerts_name: calert.name,
                alerts_table: calert.table,
                alerts_lookup: calert.lookup,
                alerts_warn: calert.warn,
                alerts_crit: calert.crit,
                alerts_info: calert.info,
                alerts_where_clause: calert.where_clause,
            };
            let incident =
                Incidents::ginsert(conn, &[incident]).expect("Failed to insert a new incident");
            super::mail::send_alert(alert, &incident);
        }
    }
}
