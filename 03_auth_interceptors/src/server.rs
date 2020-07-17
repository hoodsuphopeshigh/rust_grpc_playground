use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

pub mod auth {
    tonic::include_proto!("auth");
}

use auth::auth_server::{Auth, AuthServer};
use auth::{AuthRequest, AuthResponse};

#[derive(Debug, Default)]
pub struct MyAuth {}

#[tonic::async_trait]
impl Auth for MyAuth {
    async fn secret_thing(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = AuthResponse {
            payload: format!("Did secret thing: {}!", request.into_inner().payload).into(),
        };

        Ok(Response::new(reply))
    }
}

fn auth_intercept(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) => {
            let token = t.to_str().unwrap().split(" ").last().unwrap();
            match Uuid::parse_str(&token) {
                Ok(_) => Ok(req),
                _ => Err(Status::unauthenticated("Invalid auth token")),
            }
        }
        _ => Err(Status::unauthenticated("Invalid auth token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let auth = MyAuth::default();
    let svc = AuthServer::with_interceptor(auth, auth_intercept);

    println!("Server running on {}", addr);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
