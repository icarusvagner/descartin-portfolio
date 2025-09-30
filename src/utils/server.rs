use leptos::prelude::*;

#[server]
pub async fn api_send_email(fullname: String, email_address: String, subject: String, message: String) -> Result<String, ServerFnError> {
    use crate::utils::services::email;
    use log::error;

    let email_sent = email::send_email(email_address, fullname, message, subject);

    match email_sent.await {
        Ok(res) => Ok(res),
        Err(err) => {
            error!("{err:?}");
            Err(ServerFnError::new("Failed to send email".to_string()))
        }
    }
}
