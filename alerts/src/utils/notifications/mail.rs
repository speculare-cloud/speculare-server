use crate::utils::{IncidentStatus, Severity};
use crate::{CONFIG, MAILER};

use askama::Template;
use lettre::{
    message::{header, MultiPart, SinglePart},
    Message, Transport,
};
use sproot::models::Incidents;

/// Structure representing the incident template html sent by mail
#[derive(Template)]
#[template(path = "incident.html")]
struct IncidentTemplate<'a> {
    incident_id: i32,
    alert_name: &'a str,
    host_uuid: &'a str,
    status: &'a str,
    severity: &'a str,
    started_at: &'a str,
    updated_at: &'a str,
    lookup: &'a str,
    result: &'a str,
    warn: &'a str,
    crit: &'a str,
}

/// Send an email alerting that a new incident was created.
///
/// TODO - Pass in async
pub fn send_alert(incident: &Incidents) {
    // Retreive the sender and receiver from the config.
    let sender = CONFIG
        .get_str("SMTP_EMAIL_SENDER")
        .expect("Missing SMTP_EMAIL_SENDER in the config.");
    let receiver = CONFIG
        .get_str("SMTP_EMAIL_RECEIVER")
        .expect("Missing SMTP_EMAIL_RECEIVER in the config.");

    // Convert the status, severity & updated_at to string
    let incident_status = IncidentStatus::from(incident.status).to_string();
    let incident_severity = Severity::from(incident.severity).to_string();
    let started_at = incident.started_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let updated_at = incident.updated_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let started_at_subject = incident.started_at.format("%d %b %Y at %H:%M").to_string();

    // Build the IncidentTemplate (html code)
    let incident_template = IncidentTemplate {
        incident_id: incident.id,
        alert_name: &incident.alerts_name,
        host_uuid: &incident.host_uuid,
        status: &incident_status,
        severity: &&incident_severity,
        started_at: &started_at,
        updated_at: &updated_at,
        lookup: &incident.alerts_lookup,
        result: &incident.result,
        warn: &incident.alerts_warn,
        crit: &incident.alerts_crit,
    }
    .render()
    .unwrap();

    // Build the email with all params
    let email = Message::builder()
        .from(sender.parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(format!("{} - {}", incident.alerts_name, started_at_subject))
        .multipart(
                // Use multipart to have a fallback
            MultiPart::alternative()
                    // This singlepart is the fallback for the html code
                    // ==> Very basic.
                    .singlepart(
                        SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(format!(
                            "Host: {}\nStatus: {}\n\nSeverity level: {}\nTable: {}\nLookup: {}\nResult: {}\nWarn: {}\nCrit: {}\n\nUpdated At: {}",
                            incident.host_uuid,
                            incident_status,
                            incident_severity,
                            incident.alerts_table,
                            incident.alerts_lookup,
                            incident.result,
                            incident.alerts_warn,
                            incident.alerts_crit,
                            updated_at
                        ))
                    )
                    // This singlepart is the html design with all fields replaced
                    // ==> Prettier, ...
                    .singlepart(
                        SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(incident_template)
                    )
        ).unwrap();

    // Send the email
    match MAILER.send(&email) {
        Ok(_) => debug!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
