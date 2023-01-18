use super::server::Handler;
use super::http::response::Response;
use super::http::request::Request;
use super::http::status_code::StatusCode;

pub struct WebSiteHandler;

impl Handler for WebSiteHandler {
	fn handle_request(&mut self, _request: &Request) -> Response {
		Response::new(
			StatusCode::Ok,
			Some("<h1>Test web site handler</h1>".to_string()))
	}
}