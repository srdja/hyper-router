use super::Route;
use super::Router;
use handlers;

use hyper::server::Handler;

/// Builder for a router
///
/// Example usage:
///
//#[derive(Debug)]
pub struct RouterBuilder {
    routes: Vec<Route>,
    handler_404: Box<Handler>,
    handler_405: Box<Handler>,
    handler_500: Box<Handler>,
    handler_501: Box<Handler>,
}


impl RouterBuilder {
    pub fn new() -> RouterBuilder {
        RouterBuilder {
            routes: vec![],
            handler_404: Box::new(handlers::default_404_handler),
            handler_405: Box::new(handlers::method_not_supported_handler),
            handler_500: Box::new(handlers::internal_server_error_handler),
            handler_501: Box::new(handlers::not_implemented_handler),
        }
    }

    /// Adds new `Route` for `Router` that is being built.
    ///
    /// Example:
    ///
    /// ```ignore
    /// use hyper::server::{Request, Response};
    /// use hyper_router::{Route, RouterBuilder};
    ///
    /// fn some_handler(_: Request, _: Response) {
    ///   // do something
    /// }
    ///
    /// RouterBuilder::new().add(Route::get(r"/person/\d+").using(some_handler));
    /// ```
    pub fn add(mut self, route: Route) -> RouterBuilder {
        self.routes.push(route);
        self
    }


    pub fn set_handler_404<T: 'static + Handler>(mut self, handler: T) -> RouterBuilder {
        self.handler_404 = Box::new(handler);
        self
    }


    pub fn set_handler_405<T: 'static + Handler>(mut self, handler: T) -> RouterBuilder {
        self.handler_405 = Box::new(handler);
        self
    }


    pub fn set_handler_500<T: 'static + Handler>(mut self, handler: T) -> RouterBuilder {
        self.handler_500 = Box::new(handler);
        self
    }


    pub fn set_handler_501<T: 'static + Handler>(mut self, handler: T) -> RouterBuilder {
        self.handler_501 = Box::new(handler);
        self
    }


    pub fn build(self) -> Router {
        Router {
            routes: self.routes,
            handler_404: self.handler_404,
            handler_405: self.handler_405,
            handler_500: self.handler_500,
            handler_501: self.handler_501,
        }
    }
}
