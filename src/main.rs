//#[macro_use]
extern crate log;
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
//extern crate futures_cpupool;
extern crate tokio_core;
//#[macro_use]
//extern crate lazy_static;

use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::{Core, Handle};
//use hyper_tls::NativeTlsClient;
//use hyper::header::Headers;
//use hyper::client::Response;
use futures::{Future, Stream};
//use futures::Future;
//use futures_cpupool::CpuPool;
//use std::str::FromStr;

//lazy_static! {
//    static ref REQUEST_CPU_POOL: CpuPool = {
//        CpuPool::new_num_cpus()
//    };
//}

//pub fn get_ <Error: 'static>(url: Uri, headers: Headers) -> Box<Future<Item=Response, Error=Error> + std::marker::Send>
//    where Error: From<hyper::Error> + std::marker::Send
//{
//    REQUEST_CPU_POOL
//        .spawn_fn(|| {
//            let mut core = Core::new().unwrap();
//            let handle = core.handle();
//            let client = Client::configure()
//                .connector(HttpsConnector::new(4, &handle).unwrap())
//                .build(&handle);
//
//            let mut req = Request::new(Method::Get, url);
////            req.headers_mut().set(headers);
//
//            let get = client.request(req).and_then(|res| {
////                println!("POST: {}", res.status());
//
//                res.body().concat2()
//            });
//
//
////            let res = core.run(client.get("https://hyper.rs".parse().unwrap())).unwrap();
//
//            info!("calling GET {:?}", url);
//            Ok(core.run(get).unwrap())
//        })
//        .boxed()
//}

fn get_https_body(handle: Handle, url: Uri) -> Box<Future<Item=String, Error=()>> {
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let f = client.get(url)
        .map_err(|_err| ())
        .and_then(|resp| {
            resp.body().concat2().map_err(|_err| ()).map(|chunk| {
                let v = chunk.to_vec();
                String::from_utf8_lossy(&v).to_string()
            })
        });
    Box::new(f)
}

fn get_http_body(handle: Handle, url: Uri) -> Box<Future<Item=String, Error=()>> {
    let client = Client::new(&handle);
    let f = client.get(url)
        .map_err(|_err| ())
        .and_then(|resp| {
            resp.body().concat2().map_err(|_err| ()).map(|chunk| {
                let v = chunk.to_vec();
                String::from_utf8_lossy(&v).to_string()
            })
        });
    Box::new(f)
}

fn get_body(handle: Handle, url: Uri) -> Box<Future<Item=String, Error=()>> {
    match url.scheme() {
        Some("https") => get_https_body(handle, url),
        _ => get_http_body(handle, url),
    }
}

//pub fn post_async<Error: 'static>(url: Url,
//                                  headers: Headers,
//                                  body: String)
//                                  -> Box<Future<Item=Response, Error=Error> + std::marker::Send>
//    where Error: From<hyper::Error> + std::marker::Send
//{
//    REQUEST_CPU_POOL
//        .spawn_fn(move || {
//            let ssl = NativeTlsClient::new().unwrap();
//            let connector = HttpsConnector::new(ssl);
//            let client = Client::with_connector(connector);
//            info!("calling POST {:?}", url);
//            Ok(try!(client.post(url).headers(headers).body(&body).send()))
//        })
//        .boxed()
//}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let f = get_body(handle, "https://hyper.rs".parse().unwrap()).map(|s| {
        println!("resp: {}", s);
    });
    core.run(f).unwrap();
}