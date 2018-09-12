use sendgrid::{Mail,Destination};
use sendgrid::SGClient;
use failure::Error;

// Urgh, I guess we're rewriting the sendgrid bindings too. So much allocating :<

pub struct SendgridMailer {
    mailer: SGClient,
    to: String,
    from: String,
    subject: String, // TODO(richo) should this be a closure or something?
}

impl SendgridMailer {
    pub fn new(token: String, from: String, to: String, subject: String) -> SendgridMailer {
        SendgridMailer {
            mailer: SGClient::new(token),
            to,
            from,
            subject
        }
    }
}

pub trait MailReport {
    fn send_report(&self, report: &str) -> Result<String, Error>;
}

impl MailReport for SendgridMailer {
    fn send_report(&self, report: &str) -> Result<String, Error> {
        let msg = Mail::new()
            .add_to(Destination {
                address: &self.to,
                name: "archiver recipient",
            })
            .add_from(&self.from)
            .add_subject(&self.subject)
            .add_text(report);
        self.mailer.send(msg)
            .map_err(|e| format_err!("Sendgrid failed: {:?}", e))
    }
}

impl MailReport for Option<SendgridMailer> {
    fn send_report(&self, report: &str) -> Result<String, Error> {
        match self {
            Some(mailer) => mailer.send_report(report),
            None => Ok("".into()),
        }
    }
}
