use azure_storage::{CloudLocation, ConnectionString, StorageCredentials};
use azure_storage_blobs::prelude::{ClientBuilder, ContainerClient};

/// Builds a `ContainerClient` from environment variables.
///
/// Switch between local Azurite and production Azure Blob Storage by changing
/// `AZURE_STORAGE_CONNECTION_STRING`:
///
/// - **Azurite** (has `BlobEndpoint=http://…`): uses `CloudLocation::Emulator`
///   with the well-known dev credentials.
/// - **Production** (has `AccountName` + `AccountKey`, no custom endpoint):
///   uses `CloudLocation::Public` with your real account key.
pub fn create_client() -> ContainerClient {
    let conn_str = std::env::var("AZURE_STORAGE_CONNECTION_STRING")
        .expect("AZURE_STORAGE_CONNECTION_STRING must be set");
    let container = std::env::var("AZURE_STORAGE_CONTAINER")
        .unwrap_or_else(|_| "recordings".to_string());

    let cs = ConnectionString::new(&conn_str)
        .expect("invalid AZURE_STORAGE_CONNECTION_STRING");

    let builder = match (cs.blob_endpoint, cs.account_name, cs.account_key) {
        // Custom BlobEndpoint → Azurite or other emulator.
        // Extract host/port from the endpoint URL; credentials come from the
        // well-known emulator key rather than whatever is in the connection string.
        (Some(blob_endpoint), _, _) => {
            let url = url::Url::parse(blob_endpoint)
                .expect("invalid BlobEndpoint URL in AZURE_STORAGE_CONNECTION_STRING");
            let address = url.host_str().unwrap_or("127.0.0.1").to_owned();
            let port = url.port().unwrap_or(10000);
            ClientBuilder::with_location(
                CloudLocation::Emulator { address, port },
                StorageCredentials::emulator(),
            )
        }
        // No custom endpoint + explicit account/key → production Azure.
        (None, Some(account), Some(key)) => ClientBuilder::new(
            account.to_owned(),
            StorageCredentials::access_key(account, key.to_owned()),
        ),
        _ => panic!(
            "AZURE_STORAGE_CONNECTION_STRING must contain either \
             BlobEndpoint (emulator) or AccountName + AccountKey (production)"
        ),
    };

    builder.container_client(container)
}
