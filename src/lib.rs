#![doc(html_root_url = "https://marad.github.io/hyper-router/doc/hyper_router")]

//! # Hyper Router
//!
//! This cargo is a small extension to the great Hyper HTTP library. It basically is
//! adds the ability to define routes to request handlers and then query for the handlers
//! by request path.
//!
//! ## Usage
//!
//! To use the library just add:
//!
//! ```text
//! hyper-router = "*"
//! ```
//!
//! to your dependencies.
//!
//! ```no_run
//! extern crate hyper;
//! extern crate hyper_router;
//!
//! use hyper::server::{Server, Request, Response};
//! use hyper::status::StatusCode;
//! use hyper_router::{Route, RouterBuilder};
//!
//! fn basic_handler(_: Request, res: Response) {
//!   res.send(b"Hello World!").unwrap();
//! }
//!
//! fn main() {
//!   let router = RouterBuilder::new()
//!     .add(Route::get("/greet").using(basic_handler))
//!     .build();
//!
//!   Server::http("0.0.0.0:8080").unwrap()
//!     .handle(move |request: Request, response: Response| {
//!       match router.find_handler(&request) {
//!         Ok(handler) => handler(request, response),
//!         Err(StatusCode::NotFound) => response.send(b"not found").unwrap(),
//!         Err(_) => response.send(b"some error").unwrap()
//!       }
//!     }).unwrap();
//! }
//! ```
//!
//! This code will start Hyper server and add use router to find handlers for request.
//! We create the `Route` so that when we visit path `/greet` the `basic_handler` handler
//! will be called.
//!
//! ## Things to note
//!
//! * `Path::new` method accepts regular expressions so you can match every path you please.
//! * If you have request matching multiple paths the one that was first `add`ed will be chosen.
//! * This library is in an early stage of development so there may be breaking changes comming
//! (but I'll try as hard as I can not to break backwards compatibility or break it just a little -
//! I promise I'll try!).
//!
//! # Waiting for your feedback
//!
//! I've created this little tool to help myself learn Rust and to avoid using big frameworks
//! like Iron or rustful. I just want to keep things simple.
//!
//! Obviously I could make some errors or bad design choices so I'm waiting for your feedback!
//! You may create an issue at [project's bug tracker](https://github.com/marad/hyper-router/issues).

extern crate hyper;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;

use std::ops::Deref;

mod path;
pub mod route;
mod builder;
pub mod handlers;

pub use self::path::Path;
pub use self::route::Route;
pub use self::route::RouteBuilder;
pub use self::builder::RouterBuilder;

use hyper::server::Handler;

pub type HttpResult<T> = Result<T,StatusCode>;

/// This is the one. The router.
// #[derive(Debug)]
pub struct Router {
    routes: Vec<Route>,
    handler_404: Box<Handler>,
    handler_405: Box<Handler>,
    handler_500: Box<Handler>,
    handler_501: Box<Handler>,
}


impl Handler for Router {
    fn handle<'a, 'b>(&'a self, req: Request<'a, 'b>, res: Response<'a>) {
        if let AbsolutePath(req_path) = req.uri.clone() {
            match self.find_route(&req_path, &req.method) {
                Ok(route) => {
                    route.handler.deref().handle(req, res);
                },
                Err(e) => {
                    match e {
                        StatusCode::MethodNotAllowed => {
                            self.handler_405.handle(req, res);
                        }
                        StatusCode::NotFound => {
                            self.handler_404.handle(req, res);
                        }
                        _ => {
                            self.handler_500.handle(req, res);
                        }
                    }
                }
            }
        } else {
            self.handler_501.handle(req, res);
        }
    }
}


impl Router {
    pub fn find_route(&self, path: &str, method: &Method) -> Result<&Route, StatusCode> {
        let routes: Vec<&Route> = self.routes.iter()
            .filter(|route| {
                route.path.matcher.is_match(&path)
            }).collect();
        if !routes.is_empty() {
            match routes.iter().find(|r| r.method == *method) {
                Some(route) => Ok(route),
                None => Err(StatusCode::MethodNotAllowed)
            }
        } else {
            Err(StatusCode::NotFound)
        }
    }
}

    /*
    /// Finds handler for given Hyper request.
    ///
    /// This method uses default error handlers.
    /// If the request does not match any route than default 404 handler is returned.
    /// If the request match some routes but http method does not match (used GET but routes are
    /// defined for POST) than default method not supported handler is returned.
    pub fn find_handler_with_defaults(&self, request: &Request) -> Box<Handler> {
        if let AbsolutePath(request_path) = request.uri.clone() {
            let matching_routes = self.find_matching_routes(&request_path);
            match matching_routes.len() {
                x if x <= 0 => Box::new(handlers::Default404Handler{}),
                _ => {
                    self.find_for_method(&matching_routes, &request.method)
                        .unwrap_or(Box::new(handlers::method_not_supported_handler))
                }
            }
        } else {
            Box::new(handlers::not_implemented_handler)
        }
    }
*/


/*
    /// Finds handler for given Hyper request.
    ///
    /// It returns handler if it's found or `StatusCode` for error.
    /// This method may return `NotFound`, `MethodNotAllowed` or `NotImplemented`
    /// status codes.
    pub fn find_handler(&self, request: &Request) -> HttpResult<Box<Handler>> {
        if let AbsolutePath(request_path) = request.uri.clone() {
            let matching_routes = self.find_matching_routes(&request_path);
            match matching_routes.len() {
                x if x <= 0 => Err(StatusCode::NotFound),
                _ => {
                    self.find_for_method(&matching_routes, &request.method)
                        .map(|handler| Ok(handler))
                        .unwrap_or(Err(StatusCode::MethodNotAllowed))
                }
            }
        } else {
            Err(StatusCode::NotImplemented)
        }
    }
*/



/*
    /// Returns vector of `Route`s that match to given path.
    pub fn find_matching_routes(&self, request_path: &str) -> Vec<&Route> {
        self.routes.iter()
            .filter(|route| {
                route.path.matcher.is_match(&request_path)
            })
            .collect()
    }
*/
/*
    fn find_for_method<'a>(&'a self, routes: &Vec<&'a Route>, method: &Method) -> Option<Box<Handler>> {
        let method = method.clone();
//        routes.iter()
//            .find(|route| route.method == method)
        //            .map(|route| route.handler)

        Some(Box::new(handlers::Default404Handler{}))
    }

} */
