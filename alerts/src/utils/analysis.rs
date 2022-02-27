use super::{query::execute_query, IncidentStatus, QueryType, Severity};

use chrono::prelude::Utc;
use evalexpr::*;
use sproot::{
    errors::AppErrorType,
    models::{Alerts, Incidents, IncidentsDTO, IncidentsDTOUpdate},
    ConnType,
};

/// This function is the core of the monitoring, this is where we:
/// - Execute the query and get the result
/// - Evaluate if we need to trigger an incidents or not
pub fn execute_analysis(query: &str, alert: &Alerts, qtype: &QueryType, conn: &ConnType) {
    trace!(
        "Executing {} analysis for {:.6}",
        alert.name,
        alert.host_uuid
    );
    // Execute the query passed as arguement (this query was build previously)
    let result = match execute_query(query, &alert.host_uuid, qtype, conn) {
        Ok(result) => result,
        Err(e) => {
            if e.error_type == AppErrorType::NotFound {
                return;
            } else {
                error!(
                    "Analysis: alert {} for host_uuid {:.6} execute_query failed: {}",
                    alert.name, alert.host_uuid, e
                );
                std::process::exit(1);
            }
        }
    };
    trace!("> Result of the query is {}", &result);

    // Determine if we are in a Warn or Crit level of incidents
    let should_warn = eval_boolean(&alert.warn.replace("$this", &result)).unwrap_or_else(|e| {
        error!(
            "alert {} for host_uuid {:.6} failed to parse the String to an expression (warn: {}): {}",
            alert.name, alert.host_uuid, alert.warn, e
        );
        std::process::exit(1);
    });
    let should_crit = eval_boolean(&alert.crit.replace("$this", &result)).unwrap_or_else(|e| {
        error!(
            "alert {} for host_uuid {:.6} failed to parse the String to an expression (crit: {}): {}",
            alert.name, alert.host_uuid, alert.warn, e
        );
        std::process::exit(1);
    });
    trace!("> Should warn/crit {:?}, {:?}", should_warn, should_crit);

    // Check if an active incident already exist for this alarm.
    let prev_incident: Option<Incidents> =
        match Incidents::exist_name(conn, &alert.host_uuid, &alert.name) {
            Ok(res) => Some(res),
            Err(e) => {
                if e != diesel::result::Error::NotFound {
                    error!(
                        "alert {} for host_uuid {:.6} checking previous exists failed: {:?}",
                        alert.name, alert.host_uuid, e
                    );
                }
                None
            }
        };
    trace!("> Previous incident is some: {}", prev_incident.is_some());

    // Assert that we do not create an incident for nothing
    if !(should_warn || should_crit) {
        trace!("> We don't need to create an incident");
        // Check if an incident was active
        if let Some(prev_incident) = prev_incident {
            info!("> We need to resolve the previous incident however");
            let incident_id = prev_incident.id;
            let incident_dto = IncidentsDTOUpdate {
                status: Some(IncidentStatus::Resolved as i32),
                updated_at: Some(Utc::now().naive_local()),
                resolved_at: Some(Utc::now().naive_local()),
                ..Default::default()
            };
            let incident = Incidents::gupdate(conn, &incident_dto, incident_id)
                .expect("Failed to update (resolve) the incidents");
            super::mail::send_alert(&incident);
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
    info!("> The severity of this one is: {}", severity.to_string());

    // If it exist we create an update in the cases where:
    // - We need to update the severity of the incidents
    // - The result changed
    // In all cases we need to update the updated_at field.
    match prev_incident {
        Some(prev_incident) => {
            trace!("> Update the previous incident using the new values");
            let incident_id = prev_incident.id;
            // Determine the severity of this incident.
            // We won't downgrade an incident because it's been a "critical" incident in the past.
            // And reporting it as a simple warning thanks to the "fix" or smthg is not relevant. (for me, I guess)
            let curr_severity = severity as i32;
            // Check if we should update the severity and thus sending an escalation alert
            let mut should_alert = false;
            let mut incident_severity = None;
            if prev_incident.severity < curr_severity {
                info!("> This incident have to be escalated");
                should_alert = true;
                incident_severity = Some(curr_severity);
            };
            // Update the previous incident
            let incident_dto = IncidentsDTOUpdate {
                result: Some(result),
                updated_at: Some(Utc::now().naive_local()),
                severity: incident_severity,
                ..Default::default()
            };
            let incident = Incidents::gupdate(conn, &incident_dto, incident_id)
                .expect("Failed to update the incidents");
            if should_alert {
                super::mail::send_escalate(&incident);
            }
        }
        None => {
            info!("> Create a new incident based on the current values");
            // Clone the alert to allow us to own it in the IncidentsDTO
            let calert: Alerts = alert.clone();
            let incident = IncidentsDTO {
                result,
                started_at: Utc::now().naive_local(),
                updated_at: Utc::now().naive_local(),
                resolved_at: None,
                host_uuid: calert.host_uuid,
                hostname: calert.hostname,
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
            super::mail::send_alert(&incident);
        }
    }
}
