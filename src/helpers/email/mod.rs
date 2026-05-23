use serde_json::json;

pub fn send_otp_email(api_key: &str, to_email: &str, code: &str) -> Result<(), String> {
    let body = json!({
        "from": "noreply@mtmulch.com",
        "to": [to_email],
        "subject": "Your verification code",
        "html": format!(
            "<p>Your verification code is: <strong>{}</strong></p>\
             <p>This code expires in 10 minutes.</p>",
            code
        ),
    });

    match ureq::post("https://api.resend.com/emails")
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(&body)
    {
        Ok(_) => Ok(()),
        Err(ureq::Error::Status(code, _)) => Err(format!("Resend returned status {}", code)),
        Err(e) => Err(format!("Resend request failed: {}", e)),
    }
}
