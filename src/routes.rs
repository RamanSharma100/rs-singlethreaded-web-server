use std::collections::HashMap;
 use std::boxed::Box;


use crate::request::{HTTPRequestMethod, Request};
use crate::response::Response;


pub struct Routes {
    pub routes: HashMap<HTTPRequestMethod, HashMap<String, Box<dyn FnMut(Request, Response) + 'static>>>,
}

impl Routes{
    pub fn new() -> Routes {
        Routes {
            routes: HashMap::new(),
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: FnMut(Request , Response) + 'static
    {
        let route_handlers = self.routes.entry(HTTPRequestMethod::GET).or_insert_with(HashMap::new);
        route_handlers.insert(path.to_string(), Box::new(handler));
    }


    pub fn resolve(&mut self, method: HTTPRequestMethod, path: &str) -> Option<&mut Box<dyn FnMut(Request , Response) + 'static>> {
        self.routes.get_mut(&method)?.get_mut(path)
    }

}