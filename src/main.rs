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
    router.get("login/:system", login_handler, "loginEndpoint");

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

fn login_handler(req: &mut Request) -> IronResult<Response> {
    let play_system = String::from(req.extensions.get::<Router>().unwrap().find("system").unwrap());
    let user = String::from(req.extensions.get::<Router>().unwrap().find("user").unwrap());
    let pass = String::from(req.extensions.get::<Router>().unwrap().find("password").unwrap());


    match play_system {
        "xbox" => xbox_login(user, pass),
        "ps"   => ps_login(user, pass),
        "bnet" => bnet_login(user, pass),
        _      => fail(user, pass)
    }

    fn xbox_login(cred_name: String, cred_pass: String) {

    }

    fn ps_login(cred_name: String, cred_pass: String) {

    }

    fn pc_login(cred_name: String, cred_pass: String) {

    }

    let test_var = Responder { message: string_from_req_param};
    let payload = json::encode(&test_var).unwrap();
    let my_response = Response::with((status::Ok, payload));

    Ok(my_response)
}
