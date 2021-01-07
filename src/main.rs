use azure_core::errors::{AzureError, UnexpectedHTTPResult};
use azure_core::*;
use azure_storage::blob::prelude::*;
use azure_storage::client;
use azure_storage::key_client::KeyClient;
use prelude::Range;

use tokio_stream::{Stream, StreamExt};

#[tokio::main]
async fn main() {
    let client = client::with_azure_sas("foo", "notatoken");

    let mut stream = get_blob_stream(client);

    while let Some(res) = stream.next().await {
        println!("{:?}", res.unwrap());
    }
}

fn get_blob_stream(client: KeyClient) -> impl Stream<Item = Result<Vec<u8>, AzureError>> {
    let stream = client
        .stream_blob()
        .with_container_name("foo")
        .with_blob_name("foo")
        .with_range(&Range::new(0, 2000))
        .finalize();
    stream
}
