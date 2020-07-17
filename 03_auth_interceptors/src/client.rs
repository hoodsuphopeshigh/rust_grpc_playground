pub mod auth {
    tonic::include_proto!("auth");
}

use auth::auth_client::AuthClient;
use auth::AuthRequest;

use tonic::{metadata::MetadataValue, transport::Channel, Request};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token_value = format!("Bearer {}", Uuid::new_v4());
    let auth_token = MetadataValue::from_str(&token_value)?;

    let conn = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = AuthClient::with_interceptor(conn, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert("authorization", auth_token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(AuthRequest {
        payload: "World".into(),
    });

    let response = client.secret_thing(request).await?;

    println!("RESPONSE={:?}", response);

    let mut failing_client = AuthClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(AuthRequest {
        payload: "World".into(),
    });

    let failing_response = failing_client.secret_thing(request).await?;

    println!("FAILED RESPONSE={:?}", failing_response);

    Ok(())
}
