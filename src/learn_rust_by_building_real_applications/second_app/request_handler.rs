use super::server::Handler;
use crate::second_app::http::{Method, ParseError, Request, Response, StatusCode};

pub struct RequestHandler;

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Main Page</h1>".to_string())),
                "/hello" => {
                    Response::new(StatusCode::Ok, Some("<h1>Hello Client</h1>".to_string()))
                }
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
        // Response::new(StatusCode::Ok, Some("<h1>IT Works!!!!</h1>".to_string()))
    }
}
