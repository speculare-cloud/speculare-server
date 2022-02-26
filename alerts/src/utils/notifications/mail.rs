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
    hostname: &'a str,
    status: &'a str,
    severity: &'a str,
    started_at: &'a str,
    updated_at: &'a str,
    lookup: &'a str,
    result: &'a str,
    warn: &'a str,
    crit: &'a str,
}

fn send_mail(incident: &Incidents, template: String) {
    // Retreive the sender and receiver from the config.
    let sender = CONFIG
        .get_string("SMTP_EMAIL_SENDER")
        .expect("Missing SMTP_EMAIL_SENDER in the config.");
    let receiver = CONFIG
        .get_string("SMTP_EMAIL_RECEIVER")
        .expect("Missing SMTP_EMAIL_RECEIVER in the config.");

    // Build the email with all params
    let email = Message::builder()
        // Sender is the email of the sender, which is used by the SMTP
        // if the sender is not equals to the smtp server account, the mail will ends in the spam.
        .from(sender.parse().unwrap())
        // Receiver is the person who should get the email
        .to(receiver.parse().unwrap())
        // Subject will looks like: "Hostname [alert_name] - 23 Jul 2021 at 17:51"
        .subject(format!("{} [{}] - {}", incident.hostname, incident.alerts_name, incident.started_at.format("%d %b %Y at %H:%M")))
        .multipart(
                // Use multipart to have a fallback
            MultiPart::alternative()
                    // This singlepart is the fallback for the html code
                    // ==> Very basic.
                    .singlepart(
                        SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("There's a new error being reported by Speculare.\nAllow this mail to be displayed as HTML and go to your dashboard."))
                    )
                    // This singlepart is the html design with all fields replaced
                    // ==> Prettier, ...
                    .singlepart(
                        SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(template)
                    )
        ).unwrap();

    // Send the email
    match MAILER.send(&email) {
        Ok(_) => debug!("Email sent successfully!"),
        Err(e) => error!("Could not send email: {:?}", e),
    }
}

/// Send an email alerting that a new incident was created.
pub fn send_alert(incident: &Incidents) {
    // Convert the status, severity, started_at & updated_at to string
    let incident_status = IncidentStatus::from(incident.status).to_string();
    let incident_severity = Severity::from(incident.severity).to_string();
    let started_at = incident.started_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let updated_at = incident.updated_at.format("%Y-%m-%d %H:%M:%S").to_string();

    // Build the IncidentTemplate (html code)
    // The IncidentTemplate struct is used to hold all the information
    // about the template, which values are needed, ...
    let incident_template = IncidentTemplate {
        incident_id: incident.id,
        alert_name: &incident.alerts_name,
        hostname: &incident.hostname,
        status: &incident_status,
        severity: &incident_severity,
        started_at: &started_at,
        updated_at: &updated_at,
        lookup: &incident.alerts_lookup,
        result: &incident.result,
        warn: &incident.alerts_warn,
        crit: &incident.alerts_crit,
    }
    .render()
    .unwrap();

    send_mail(incident, incident_template);
}

/// Structure representing the escalation template html sent by mail
#[derive(Template)]
#[template(path = "escalate.html")]
struct EscalateTemplate<'a> {
    incident_id: i32,
    alert_name: &'a str,
    hostname: &'a str,
    status: &'a str,
    severity: &'a str,
    started_at: &'a str,
    updated_at: &'a str,
    lookup: &'a str,
    result: &'a str,
    warn: &'a str,
    crit: &'a str,
}

pub fn send_escalate(incident: &Incidents) {
    // Convert the status, severity, started_at & updated_at to string
    let incident_status = IncidentStatus::from(incident.status).to_string();
    let incident_severity = Severity::from(incident.severity).to_string();
    let started_at = incident.started_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let updated_at = incident.updated_at.format("%Y-%m-%d %H:%M:%S").to_string();

    // Build the EscalateTemplate (html code)
    // The EscalateTemplate struct is used to hold all the information
    // about the template, which values are needed, ...
    let escalate_template = EscalateTemplate {
        incident_id: incident.id,
        alert_name: &incident.alerts_name,
        hostname: &incident.hostname,
        status: &incident_status,
        severity: &incident_severity,
        started_at: &started_at,
        updated_at: &updated_at,
        lookup: &incident.alerts_lookup,
        result: &incident.result,
        warn: &incident.alerts_warn,
        crit: &incident.alerts_crit,
    }
    .render()
    .unwrap();

    send_mail(incident, escalate_template);
}
