use Route;
use hyper::server::Handler;


pub struct RouteBuilder {
    route: Route
}


impl RouteBuilder {
    pub fn new(route: Route) -> RouteBuilder {
        RouteBuilder {
            route: route
        }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    pub fn using<T: 'static + Handler>(mut self, handler: T) -> Route {
        self.route.handler = Box::new(handler);
        self.route
    }
}
