use crate::utils::Severity;
use crate::{CONFIG, MAILER};

use lettre::{Message, Transport};
use sproot::models::Incidents;

pub fn send_alert(incident: &Incidents) {
    let sender = CONFIG
        .get_str("SMTP_EMAIL_SENDER")
        .expect("Missing SMTP_EMAIL_SENDER in the config.");
    let receiver = CONFIG
        .get_str("SMTP_EMAIL_RECEIVER")
        .expect("Missing SMTP_EMAIL_RECEIVER in the config.");

    // Build the email with all params
    let email = Message::builder()
        .from(sender.parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(format!("Speculare: new Incident for {}", incident.alerts_name))
        .body(format!(
            "Incident n°{} - {}\n\nTable: {}\nLookup: {}\nResult: {}\nWarn: {}\nCrit: {}\n\nUpdated At: {}",
            incident.id,
            Severity::from(incident.severity).to_string(),
            incident.alerts_table,
            incident.alerts_lookup,
            incident.result,
            incident.alerts_warn,
            incident.alerts_crit,
            incident.updated_at.format("%Y-%m-%d %H:%M:%S").to_string()
        ))
        .unwrap();

    // Send the email
    match MAILER.send(&email) {
        Ok(_) => debug!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
