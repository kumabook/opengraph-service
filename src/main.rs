extern crate iron;
extern crate router;
extern crate hyper;
extern crate hyper_native_tls;
extern crate urlencoded;
extern crate opengraph;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod error;

use std::io::Read;
use std::str::FromStr;

use std::env;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use router::{Router};
use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use urlencoded::UrlEncodedQuery;
use opengraph::scraper::extract;

use error::Error;

fn main() {
    let mut router = Router::new();
    router.get("/opengraph", opengraph, "opengraph");

    let port_str = match env::var("PORT") {
        Ok(n)  => n,
        Err(_) => "8080".to_string()
    };
    let port: u16 = match port_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Faild to parse port");
            return;
        }
    };
    println!("PORT {}", port_str);
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    Iron::new(router).http(SocketAddrV4::new(ip, port)).unwrap();
}

fn application_json() -> Mime {
    Mime::from_str("application/json").ok().unwrap()
}

fn opengraph(req: &mut Request) -> IronResult<Response> {
    fn opengraph2(req: &mut Request) -> Result<Response, Error> {
        let ref params = try!(req.get_ref::<UrlEncodedQuery>());
        let url        = try!(params.get("url").ok_or(Error::BadRequest));
        let tls        = NativeTlsClient::new().unwrap();
        let connector  = HttpsConnector::new(tls);
        let client     = Client::with_connector(connector);

        let mut res = client.get(&url[0])
            .header(Connection(vec![ConnectionOption::Close]))
            .send()?;

        if res.status.is_success() {
            let mut body = String::from("");
            let _ = res.read_to_string(&mut body);
            if let Some(object) = extract(body) {
                let json_str = serde_json::to_string(&object).unwrap();
                println!("handle {}", url[0]);
                return Ok(Response::with((status::Ok, application_json(), json_str)));
            }
        }
        println!("unhandle {}", url[0]);
        Ok(Response::with((status::Ok, application_json(), "{}")))
    }
    opengraph2(req).map_err(|err| IronError::from(err))
}
