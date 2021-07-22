use crate::utils::Severity;
use crate::{CONFIG, MAILER};

use lettre::{Message, Transport};
use sproot::models::{Alerts, Incidents};

pub fn send_alert(alert: &Alerts, incident: &Incidents) {
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
        .subject(format!("Speculare: new Incident for {}", alert.name))
        .body(format!(
            "Incident n°{} - {}\n\nTable: {}\nLookup: {}\nResult: {}\nUpdated At: {}",
            incident.id,
            Severity::from(incident.severity).to_string(),
            alert.table,
            alert.lookup,
            incident.result,
            incident.updated_at.to_string()
        ))
        .unwrap();

    // Send the email
    match MAILER.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
