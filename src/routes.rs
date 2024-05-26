use std::collections::HashMap;
use crate::request::{Request, HTTPRequestMethod};
use crate::response::Response;

pub type Handler = Box<dyn FnMut(Request, &mut Response) + Send + 'static>;

pub struct Routes {
    pub routes: HashMap<HTTPRequestMethod, HashMap<String, Handler>>,
    pub parameterized_routes: HashMap<HTTPRequestMethod, Vec<(Vec<String>, Handler)>>,
}

impl Routes {
    pub fn new() -> Routes {
        Routes {
            routes: HashMap::new(),
            parameterized_routes: HashMap::new(),
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: FnMut(Request, &mut Response) + Send + 'static
    {
        if path.contains(":") {
            let route_handlers = self.parameterized_routes.entry(HTTPRequestMethod::GET).or_insert_with(Vec::new);
            let parts = path.split("/").map(|part| part.to_string()).collect();
            route_handlers.push((parts, Box::new(handler)));
        } else {
            let route_handlers = self.routes.entry(HTTPRequestMethod::GET).or_insert_with(HashMap::new);
            route_handlers.insert(path.to_string(), Box::new(handler));
        }
    }

    pub fn resolve(&mut self, method: HTTPRequestMethod, path: &str) -> Option<(&mut Handler, HashMap<String, String>)> {
        if let Some(route_handlers) = self.routes.get_mut(&method) {
            if let Some(handler) = route_handlers.get_mut(path) {
                return Some((handler, HashMap::new()));
            }
        }

        if let Some(handlers) = self.parameterized_routes.get_mut(&method) {
            for (parts, handler) in handlers.iter_mut() {
                if let Some(params) = Routes::match_parameterized_route(path, parts) {
                    return Some((handler, params));
                }
            }
        }

        None
    }

    fn match_parameterized_route(path: &str, parts: &[String]) -> Option<HashMap<String, String>> {
        let path_parts: Vec<&str> = path.split("/").collect();
        if path_parts.len() != parts.len() {
            return None;
        }
        let mut params: HashMap<String, String> = HashMap::new();
        for (path_part, part) in path_parts.iter().zip(parts.iter()) {
            if part.starts_with(":") {
                let part_name = &part[1..];
                params.insert(part_name.to_string(), path_part.to_string());
            } else if part != path_part {
                return None;
            }
        }
        Some(params)
    }
}
