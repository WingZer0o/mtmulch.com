use azure_identity::DefaultAzureCredential;
use azure_storage::{CloudLocation, ConnectionString, StorageCredentials};
use azure_storage_blobs::prelude::{ClientBuilder, ContainerClient};
use std::sync::Arc;

pub fn create_client() -> ContainerClient {
    let container = std::env::var("AZURE_STORAGE_CONTAINER")
        .unwrap_or_else(|_| "recordings".to_string());

    match std::env::var("AZURE_STORAGE_CONNECTION_STRING") {
        Ok(conn_str) => create_client_from_connection_string(&conn_str, container),
        Err(_) => create_client_from_identity(container),
    }
}

fn create_client_from_connection_string(conn_str: &str, container: String) -> ContainerClient {
    let cs = ConnectionString::new(conn_str)
        .expect("invalid AZURE_STORAGE_CONNECTION_STRING");

    let builder = match (cs.blob_endpoint, cs.account_name, cs.account_key) {
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

fn create_client_from_identity(container: String) -> ContainerClient {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT")
        .expect("AZURE_STORAGE_ACCOUNT must be set when AZURE_STORAGE_CONNECTION_STRING is not set");

    let credential = Arc::new(DefaultAzureCredential::create(Default::default()).expect("failed to create DefaultAzureCredential"));
    ClientBuilder::new(account, StorageCredentials::token_credential(credential))
        .container_client(container)
}
