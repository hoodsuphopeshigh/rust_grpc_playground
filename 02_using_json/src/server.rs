use tonic::{transport::Server, Request, Response, Status};

use using_json::json_service_server::{JsonService, JsonServiceServer};
use using_json::{User, UserRequest, UserRequests, UserResponse, UserResponses};

pub mod using_json {
    tonic::include_proto!("usingjson");
}

#[derive(Debug, Default)]
pub struct MyUser {}

#[tonic::async_trait]
impl JsonService for MyUser {
    async fn create_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        println!("Got a request to create a User: {:?}", request);

        match request.into_inner().data {
            Some(user_payload) => {
                let response = using_json::UserResponse {
                    message: format!("User {} has been created", user_payload.name).into(),
                    created: 1,
                };

                Ok(Response::new(response))
            }
            None => Err(Status::unknown(
                "Request has failed due to an unknown reason",
            )),
        }
    }

    async fn create_multiple_users(
        &self,
        request: Request<UserRequests>,
    ) -> Result<Response<UserResponses>, Status> {
        println!("Got a request to create multiple Users: {:?}", request);
        let users: Vec<User> = request.into_inner().users;
        let response = users
            .iter()
            .map(|user| UserResponse {
                message: format!("User {} has been created", user.name).into(),
                created: 1,
            })
            .collect();

        Ok(Response::new(UserResponses { response }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = MyUser::default();

    println!("Server running on {}", addr);

    Server::builder()
        .add_service(JsonServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
