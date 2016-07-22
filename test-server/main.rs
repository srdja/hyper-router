extern crate hyper;
extern crate hyper_router;

use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper_router::{Route, RouterBuilder};

fn request_handler(_: Request, res: Response) {
    res.send(b"Hello World").unwrap();
}


struct StructHandler;

impl Handler for StructHandler {
    fn handle(&self, req: Request, res: Response) {
    }
}


fn main() {
    let handler = StructHandler {};

    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(request_handler))
        .add(Route::from(Method::Patch, "/asd").using(handler))
        .build();

    Server::http("0.0.0.0:8080").unwrap()
        .handle(router).unwrap();


//        .handle(move |req: Request, res: Response| {
//            router.handle(req, res);
//        }).unwrap();

    /*
    Server::http("0.0.0.0:8080").unwrap()
        .handle(move |request: Request, response: Response| {
            match router.find_handler(&request) {
                Ok(handler) => handler(request, response),
                Err(StatusCode::NotFound) => response.send(b"not found").unwrap(),
                Err(_) => response.send(b"some error").unwrap(),
            }
        }).unwrap();
*/
}
