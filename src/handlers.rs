use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::server::Handler;

pub struct Default404Handler;

impl Handler for Default404Handler {
    fn handle(&self,  _: Request, mut response: Response) {
        {*response.status_mut() = StatusCode::NotFound}
        response.send(b"page not found").ok();
    }
}


pub fn default_404_handler(_: Request, mut response: Response) {
    {*response.status_mut() = StatusCode::NotFound}
    response.send(b"page not found").ok();
}

pub fn method_not_supported_handler(_: Request, mut response: Response) {
    {*response.status_mut() = StatusCode::MethodNotAllowed}
    response.send(b"method not supported").ok();
}

pub fn internal_server_error_handler(_: Request, mut response: Response) {
    {*response.status_mut() = StatusCode::InternalServerError}
    response.send(b"internal server error").ok();
}

pub fn not_implemented_handler(_: Request, mut response: Response) {
    {*response.status_mut() = StatusCode::NotImplemented}
    response.send(b"not implemented").ok();
}
