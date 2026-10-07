use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: SecretString,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: SecretString,
    ) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };
        self.http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence, Word};
    use secrecy::SecretString;
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange

        // Start a wiremock HTTP server in a background thread
        let mock_server = MockServer::start().await;
        // Setup email client with random email and URI from wiremock server
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let token: String = Word().fake();
        let email_client =
            EmailClient::new(mock_server.uri(), sender, SecretString::new(token.into()));

        // Attach a Mock to the wiremock server that matches any request and returns a 200 response without a body
        // Expect to only receive exactly one request
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Data for send_email()
        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // Assert

        // No explicit code here, wiremock will assert the mocks itself when it goes out of scope
    }
}
