use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use crate::helpers::email::{
    send_account_locked_email, send_blog_unsubscribe_email, send_blog_verification_email,
    send_new_blog_post_email, send_otp_email, send_verification_email,
};

pub enum EmailJob {
    Otp { to: String, code: String },
    Verification { to: String, link: String },
    AccountLocked { to: String, link: String },
    BlogVerification { to: String, link: String },
    BlogUnsubscribe { to: String, link: String },
    NewBlogPost { to: String, title: String, link: String },
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
                EmailJob::AccountLocked { to, link } => send_account_locked_email(&api_key, to, link),
                EmailJob::BlogVerification { to, link } => send_blog_verification_email(&api_key, to, link),
                EmailJob::BlogUnsubscribe { to, link } => send_blog_unsubscribe_email(&api_key, to, link),
                EmailJob::NewBlogPost { to, title, link } => send_new_blog_post_email(&api_key, to, title, link),
            };
            if let Err(e) = result {
                let to = match &job {
                    EmailJob::Otp { to, .. }
                    | EmailJob::Verification { to, .. }
                    | EmailJob::AccountLocked { to, .. }
                    | EmailJob::BlogVerification { to, .. }
                    | EmailJob::BlogUnsubscribe { to, .. }
                    | EmailJob::NewBlogPost { to, .. } => to,
                };
                eprintln!("[email-worker] failed to send to {}: {}", to, e);
            }
            // 500 ms between sends = max 2 emails/second
            thread::sleep(Duration::from_millis(500));
        }
    });

    Arc::new(tx)
}
