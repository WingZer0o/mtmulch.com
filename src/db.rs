use diesel::PgConnection;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::r2d2;
use serde::Deserialize;

pub type DbPool = r2d2::Pool<PgManager>;

pub struct PgManager {
    url: String,
}

impl PgManager {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Deserialize)]
struct ImdsToken {
    access_token: String,
}

fn fetch_managed_identity_token() -> Result<String, String> {
    // Container Apps injects IDENTITY_ENDPOINT + IDENTITY_HEADER; fall back to
    // the standard IMDS address used by VMs.
    let (url, header_name, header_value) =
        match (std::env::var("IDENTITY_ENDPOINT"), std::env::var("IDENTITY_HEADER")) {
            (Ok(ep), Ok(hdr)) => (
                format!(
                    "{}?resource=https://ossrdbms-aad.database.windows.net&api-version=2019-08-01",
                    ep
                ),
                "X-IDENTITY-HEADER",
                hdr,
            ),
            _ => (
                "http://169.254.169.254/metadata/identity/oauth2/token\
                 ?api-version=2018-02-01\
                 &resource=https://ossrdbms-aad.database.windows.net"
                    .to_string(),
                "Metadata",
                "true".to_string(),
            ),
        };

    let token: ImdsToken = ureq::get(&url)
        .set(header_name, &header_value)
        .call()
        .map_err(|e| format!("token request failed: {}", e))?
        .into_json()
        .map_err(|e| format!("token response parse failed: {}", e))?;

    // Quick sanity log — JWTs start with "eyJ" and are ~1500 chars.
    let preview: String = token.access_token.chars().take(20).collect();
    eprintln!(
        "[db] fetched MI token (len={}, prefix={}...)",
        token.access_token.len(),
        preview
    );

    Ok(token.access_token)
}

fn build_url(base_url: &str, token: &str) -> Result<String, String> {
    let mut parsed = url::Url::parse(base_url).map_err(|e| e.to_string())?;
    parsed
        .set_password(Some(token))
        .map_err(|_| "failed to set password in URL".to_string())?;
    Ok(parsed.to_string())
}

impl r2d2::ManageConnection for PgManager {
    type Connection = PgConnection;
    type Error = diesel::ConnectionError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let url = if std::env::var("USE_MANAGED_IDENTITY").is_ok() {
            let token = fetch_managed_identity_token()
                .map_err(|e| diesel::ConnectionError::BadConnection(e))?;
            build_url(&self.url, &token)
                .map_err(|e| diesel::ConnectionError::BadConnection(e))?
        } else {
            self.url.clone()
        };

        PgConnection::establish(&url)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.batch_execute("SELECT 1")
            .map_err(|e| diesel::ConnectionError::BadConnection(e.to_string()))
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

pub fn create_pool(database_url: &str) -> DbPool {
    r2d2::Pool::builder()
        .build(PgManager::new(database_url.to_string()))
        .expect("Failed to create DB connection pool")
}