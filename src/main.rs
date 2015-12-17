#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;
extern crate rustc_serialize;
extern crate iron;
extern crate params;
use std::error::Error;
use std::collections::HashMap;
use iron::prelude::*;
use iron::mime::Mime;
use iron::{Handler,status};
use params::Params;
use rustc_serialize::json::{ToJson, Json};


fn handle(req: &mut Request) -> IronResult<Response> {
    println!("{:?}", req.get_ref::<Params>());
    let pm = req.get_ref::<Params>().unwrap();
    for (key, val) in pm.iter() {
        println!("key:{}", key);
        match *val {
            params::Value::Null => println!("{}", "null"),
            params::Value::Boolean(value) => println!("bool:{:?}", value),
            params::Value::I64(value)  => println!("i64:{:?}", value),
            params::Value::U64(value)  => println!("u64:{:?}", value),
            params::Value::F64(value)  => println!("f64:{:?}", value),
            params::Value::String(ref value) => println!("String:{:?}", value),
            params::Value::File(ref value) => println!("File:{:?}", value),
            params::Value::Array(ref value) => println!("Array:{:?}", value),
            params::Value::Map(ref value) => println!("Map:{:?}", value),
        }
    }
    let content_type = "text/html".parse::<Mime>().unwrap();
    /*let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, "{\"!\"}")))*/
    let mut resp = Response::new();
    let name = "Rust";
    let mut buffer = String::new();
    html!(buffer, {
        p { "Hi, " $name "!" }
    }).unwrap();
    resp.set_mut(content_type).set_mut(buffer).set_mut(status::Ok);
    Ok(resp)
}

struct Team {
    name: String,
    pts: u16
}

impl ToJson for Team {
    fn to_json(&self) -> Json {
        let mut m: HashMap<String, Json> = HashMap::new();
        m.insert("name".to_string(), self.name.to_json());
        m.insert("pts".to_string(), self.pts.to_json());
        m.to_json()
    }
}

fn make_datam<'a> () -> HashMap<String, Json> {
    let mut data = HashMap::new();

    data.insert("year".to_string(), "2015".to_json());

    let teams = vec![ Team { name: "Jiangsu Sainty".to_string(),
                             pts: 43u16 },
                      Team { name: "Beijing Guoan".to_string(),
                             pts: 27u16 },
                      Team { name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16 },
                      Team { name: "Shandong Luneng".to_string(),
                             pts: 12u16 } ];

    data.insert("teams".to_string(), teams.to_json());
    data
}

fn make_data () -> HashMap<String, Json> {
    let mut data = HashMap::new();

    data.insert("year".to_string(), "2015".to_json());

    let teams = vec![ Team { name: "Jiangsu Sainty".to_string(),
                             pts: 43u16 },
                      Team { name: "Beijing Guoan".to_string(),
                             pts: 27u16 },
                      Team { name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16 },
                      Team { name: "Shandong Luneng".to_string(),
                             pts: 12u16 } ];

    data.insert("teams".to_string(), teams.to_json());
    data
}

struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.add_route("".to_string(), handle);

    router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with("Hello world !"))
    });

    router.add_route("hello/again".to_string(), |_: &mut Request| {
       Ok(Response::with("Hello again !"))
    });

    router.add_route("error".to_string(), |_: &mut Request| {
       Ok(Response::with(status::BadRequest))
    });

    let mut chain = Chain::new(router);

    // chain.link_after(hbs);

    Iron::new(chain).http("wram:8080").unwrap();
}
