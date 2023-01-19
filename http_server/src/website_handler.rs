use super::server::Handler;
use super::http::Response;
use super::http::Request;
use super::http::StatusCode;
use super::http::Method;

pub struct WebSiteHandler;

impl Handler for WebSiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		match request.method() {
			Method::GET =>
				Response::new(
					StatusCode::Ok,
					Some("<h1>Test web site handler</h1>".to_string())),
			_ => Response::new(StatusCode::NotFound, None)
		}
	}
}
