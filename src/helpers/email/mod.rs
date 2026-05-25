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

    resend_post(api_key, &body)
}

pub fn send_verification_email(api_key: &str, to_email: &str, link: &str) -> Result<(), String> {
    let body = json!({
        "from": "noreply@mtmulch.com",
        "to": [to_email],
        "subject": "Verify your email address",
        "html": format!(
            "<p>Thanks for signing up! Please verify your email address by clicking the link below.</p>\
             <p><a href=\"{link}\">Verify my email</a></p>\
             <p>This link expires in 24 hours. If you did not create an account, you can ignore this email.</p>"
        ),
    });

    resend_post(api_key, &body)
}

pub fn send_account_locked_email(api_key: &str, to_email: &str, link: &str) -> Result<(), String> {
    let body = serde_json::json!({
        "from": "noreply@mtmulch.com",
        "to": [to_email],
        "subject": "Your account has been locked",
        "html": format!(
            "<p>Your account has been locked after too many failed login attempts.</p>\
             <p>Click the link below to verify your email address and unlock your account.</p>\
             <p><a href=\"{link}\">Unlock my account</a></p>\
             <p>This link expires in 1 hour. If you did not attempt to log in, please contact support immediately.</p>"
        ),
    });

    resend_post(api_key, &body)
}

pub fn send_new_blog_post_email(api_key: &str, to_email: &str, title: &str, link: &str) -> Result<(), String> {
    let body = json!({
        "from": "noreply@mtmulch.com",
        "to": [to_email],
        "subject": format!("New post: {}", title),
        "html": format!(
            "<p>A new blog post has been published: <strong>{title}</strong></p>\
             <p><a href=\"{link}\">Read it here</a></p>\
             <p>You're receiving this because you subscribed to blog updates.</p>"
        ),
    });

    resend_post(api_key, &body)
}

pub fn send_blog_verification_email(api_key: &str, to_email: &str, link: &str) -> Result<(), String> {
    let body = json!({
        "from": "noreply@mtmulch.com",
        "to": [to_email],
        "subject": "Confirm your blog subscription",
        "html": format!(
            "<p>Thanks for subscribing! Please confirm your email address by clicking the link below.</p>\
             <p><a href=\"{link}\">Confirm subscription</a></p>\
             <p>This link expires in 1 hour. If you did not request this, you can ignore this email.</p>"
        ),
    });

    resend_post(api_key, &body)
}

pub fn send_blog_unsubscribe_email(api_key: &str, to_email: &str, link: &str) -> Result<(), String> {
    let body = json!({
        "from": "noreply@mtmulch.com",
        "to": [to_email],
        "subject": "Confirm your blog unsubscription",
        "html": format!(
            "<p>We received a request to unsubscribe this address from the blog.</p>\
             <p><a href=\"{link}\">Confirm unsubscribe</a></p>\
             <p>This link expires in 1 hour. If you did not request this, you can ignore this email.</p>"
        ),
    });

    resend_post(api_key, &body)
}

fn resend_post(api_key: &str, body: &serde_json::Value) -> Result<(), String> {
    match ureq::post("https://api.resend.com/emails")
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(body)
    {
        Ok(_) => Ok(()),
        Err(ureq::Error::Status(code, _)) => Err(format!("Resend returned status {}", code)),
        Err(e) => Err(format!("Resend request failed: {}", e)),
    }
}
