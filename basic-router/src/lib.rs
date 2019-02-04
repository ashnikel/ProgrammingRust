#![allow(dead_code)]

use std::boxed::Box;
use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    header: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    header: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    /// Create an empty router.
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    /// Add a route to the router.
    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}

fn not_found_response() -> Response {
    Response {
        code: 404,
        header: HashMap::new(),
        body: b"<h1>Page not found</h1>".to_vec(),
    }
}

fn get_form_response() -> Response {
    Response {
        code: 200,
        header: HashMap::new(),
        body: b"<form>".to_vec(),
    }
}

fn get_gcd_response(_req: &Request) -> Response {
    Response {
        code: 500,
        header: HashMap::new(),
        body: b"<h1>Internal server error</h1>".to_vec(),
    }
}

fn req(url: &str) -> Request {
    Request {
        method: "GET".to_string(),
        url: url.to_string(),
        header: HashMap::new(),
        body: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_router() {
        let mut router = BasicRouter::new();
        router.add_route("/", |_| get_form_response());
        router.add_route("/gcd", |req| get_gcd_response(req));

        assert_eq!(router.handle_request(&req("/asdf")).code, 404);
        assert_eq!(router.handle_request(&req("/")).code, 200);
        assert_eq!(router.handle_request(&req("/gcd")).code, 500);
    }
}
