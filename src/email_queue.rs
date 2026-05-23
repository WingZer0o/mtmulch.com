use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use crate::helpers::email::send_otp_email;

pub struct EmailJob {
    pub to: String,
    pub code: String,
}

// Arc makes Sender<T> Sync so it can live in web::Data
pub type EmailQueue = Arc<mpsc::Sender<EmailJob>>;

pub fn start_worker(api_key: String) -> EmailQueue {
    let (tx, rx) = mpsc::channel::<EmailJob>();

    thread::spawn(move || {
        for job in rx {
            if let Err(e) = send_otp_email(&api_key, &job.to, &job.code) {
                eprintln!("[email-worker] failed to send to {}: {}", job.to, e);
            }
            // 500 ms between sends = max 2 emails/second
            thread::sleep(Duration::from_millis(500));
        }
    });

    Arc::new(tx)
}
