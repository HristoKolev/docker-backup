use super::prelude::*;

use lettre_email::Email;
use lettre::{SmtpClient, ClientSecurity, Transport};
use lettre::smtp::authentication::{Credentials, Mechanism};

pub struct EmailClient {
    username: String,
    password: String,
    smtp_host: String,
    smtp_port: u16,
}

impl EmailClient {

    pub fn new(username: &str,
               password: &str,
               smtp_host: &str,
               smtp_port: u16,
    ) -> EmailClient
    {
        EmailClient {
            username: username.to_string(),
            password: password.to_string(),
            smtp_host: smtp_host.to_string(),
            smtp_port,
        }
    }

    pub fn send(self, message: &EmailMessage) -> Result<()> {

        let address = (&*self.smtp_host, self.smtp_port);

        let credentials = Credentials::new(
            self.username.clone(),
            self.password.clone()
        );

        let mut smtp_client = SmtpClient::new(address,ClientSecurity::None)?
            .credentials(credentials)
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .transport();

        let mut email_builder = Email::builder()
            .from(&*self.username)
            .subject(&*message.subject)
            .html(&*message.content);

        for address in &message.to_addresses {
            email_builder = email_builder.bcc(&address[..]);
        }

        let email= email_builder.build()?.into();

        smtp_client.send(email)?;

        Ok(())
    }
}

pub struct EmailMessage {
    to_addresses: Vec<String>,
    content: String,
    subject: String,
}

impl EmailMessage {

    pub fn new(to_address: Vec<String>,
               subject: &str,
               content: &str,
               ) -> EmailMessage {
        EmailMessage {
            to_addresses: to_address,
            content: content.to_string(),
            subject: subject.to_string(),
        }
    }
}
