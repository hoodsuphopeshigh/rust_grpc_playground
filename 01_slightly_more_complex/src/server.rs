use tonic::{transport::Server, Request, Response, Status};

use slightly_more_complex::user_service_server::{UserServiceServer, UserService};
use slightly_more_complex::{UserRequest, UserResponse};

pub mod slightly_more_complex {
    tonic::include_proto!("slightlymorecomplex");
}

#[derive(Debug, Default)]
pub struct MyUser {}

#[tonic::async_trait]
impl UserService for MyUser {
    async fn create_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        println!("Got a request to create a User: {:?}", request);

        match request.into_inner().data {
            Some(user_payload) => {
                let response = slightly_more_complex::UserResponse {
                    message: format!("User {} has been created", user_payload.name).into(),
                    created: 1,
                };

                Ok(Response::new(response))
            }
            None => {
                Err(Status::unknown("Request has failed due to an unknown reason"))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = MyUser::default();

    println!("Server running on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
