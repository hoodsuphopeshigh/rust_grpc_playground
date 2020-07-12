use slightly_more_complex::user_service_client::UserServiceClient;
use slightly_more_complex::{User, UserRequest};

pub mod slightly_more_complex {
    tonic::include_proto!("slightlymorecomplex");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let user = User {
        name: "Dave".into(),
        email: "dave@dave.com".into(),
        age: 50.into(),
    };

    let request = tonic::Request::new(UserRequest {
        data: Some(user)
    });

    let response = client.create_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
