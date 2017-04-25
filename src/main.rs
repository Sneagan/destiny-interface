extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Responder {
    message: String
}

fn main() {
    let mut router = Router::new();
    router.get("/", index_handler, "index");
    router.get("/:query", query_handler, "queryEndpoint");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn index_handler(req: &mut Request) -> IronResult<Response> {
    let response = "this is a test";
    Ok(Response::with((status::Ok, response)))
}

fn query_handler(req: &mut Request) -> IronResult<Response> {
    let string_from_req_param = String::from(req.extensions.get::<Router>().unwrap().find("query").unwrap());
    let test_var = Responder { message: string_from_req_param};
    let payload = json::encode(&test_var).unwrap();
    let my_response = Response::with((status::Ok, payload));

    Ok(my_response)
}
