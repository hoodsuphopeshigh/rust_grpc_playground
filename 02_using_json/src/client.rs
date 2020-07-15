use serde::{Deserialize, Serialize};
use using_json::json_service_client::JsonServiceClient;
use using_json::{User, UserRequest, UserRequests};

pub mod using_json {
    tonic::include_proto!("usingjson");
}

#[derive(Debug, Serialize, Deserialize)]
struct UserParams<'a> {
    name: &'a str,
    email: &'a str,
    age: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = JsonServiceClient::connect("http://[::1]:50051").await?;
    let json_string = r#"{"name":"Dave","email": "dave@dave.com", "age": 50}"#;
    let user_params: UserParams = serde_json::from_str(json_string)?;
    let message = User {
        name: user_params.name.into(),
        email: user_params.email.into(),
        age: user_params.age,
    };

    let request = tonic::Request::new(UserRequest {
        data: Some(message),
    });

    let response = client.create_user(request).await?;

    println!("RESPONSE={:?}", response);

    let json_collection = r#"[{"name":"Dave","email": "dave@dave.com", "age": 50},{"name":"Frank","email": "frank@frank.com", "age": 20}]"#;
    let user_collection: Vec<UserParams> = serde_json::from_str(json_collection)?;
    let multi_users: Vec<User> = user_collection
        .iter()
        .map(|user| User {
            name: user.name.into(),
            email: user.email.into(),
            age: user.age,
        })
        .collect();
    let multi_request = tonic::Request::new(UserRequests { users: multi_users });
    let multi_response = client.create_multiple_users(multi_request).await?;

    println!("MULTI RESPONSE={:?}", multi_response);
    Ok(())
}
