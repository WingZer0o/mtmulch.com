use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use crate::helpers::email::{send_otp_email, send_verification_email};

pub enum EmailJob {
    Otp { to: String, code: String },
    Verification { to: String, link: String },
}

// Arc makes Sender<T> Sync so it can live in web::Data
pub type EmailQueue = Arc<mpsc::Sender<EmailJob>>;

pub fn start_worker(api_key: String) -> EmailQueue {
    let (tx, rx) = mpsc::channel::<EmailJob>();

    thread::spawn(move || {
        for job in rx {
            let result = match &job {
                EmailJob::Otp { to, code } => send_otp_email(&api_key, to, code),
                EmailJob::Verification { to, link } => send_verification_email(&api_key, to, link),
            };
            if let Err(e) = result {
                let to = match &job {
                    EmailJob::Otp { to, .. } | EmailJob::Verification { to, .. } => to,
                };
                eprintln!("[email-worker] failed to send to {}: {}", to, e);
            }
            // 500 ms between sends = max 2 emails/second
            thread::sleep(Duration::from_millis(500));
        }
    });

    Arc::new(tx)
}
