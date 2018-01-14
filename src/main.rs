#![feature(plugin)]
#![plugin(clippy)]

//#[macro_use]
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate hyper_tls;
extern crate log;
extern crate tokio_core;
// #[macro_use]
// extern crate lazy_static;

use hyper::{Client, Post, Request, Uri};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::{Core, Handle};
use futures::{Future, Stream};
//use futures::Future;
// use futures_cpupool::CpuPool;

// pub struct scraper {
//     pool: CpuPool
// }

// impl scraper {
//     pub new(num: usize) -> Self {
// pub fn init_pool(num: usize) -> CpuPool {
//     CpuPool::new(num)
// }
//     }
// }

// lazy_static! {
//    static ref REQUEST_CPU_POOL: CpuPool = {
//        CpuPool::new_num_cpus()
//    };
// }

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

fn get_body(handle: &Handle, url: Uri) -> Box<Future<Item = String, Error = ()>> {
    match url.scheme() {
        Some("https") => get_https_body(handle, url),
        _ => get_http_body(handle, url),
    }
}

fn get_http_body(handle: &Handle, url: Uri) -> Box<Future<Item = String, Error = ()>> {
    let client = Client::new(handle);
    let f = client.get(url).map_err(|_err| ()).and_then(|resp| {
        resp.body().concat2().map_err(|_err| ()).map(|chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
        })
    });
    Box::new(f)
}

fn get_https_body(handle: &Handle, url: Uri) -> Box<Future<Item = String, Error = ()>> {
    let client = Client::configure()
        .connector(HttpsConnector::new(4, handle).unwrap())
        .build(handle);
    let f = client.get(url).map_err(|_err| ()).and_then(|resp| {
        resp.body().concat2().map_err(|_err| ()).map(|chunk| {
            let v = chunk.to_vec();
            String::from_utf8_lossy(&v).to_string()
        })
    });
    Box::new(f)
}

fn post_body(handle: &Handle, url: Uri, body: String) -> Box<Future<Item = String, Error = ()>> {
    match url.scheme() {
        Some("https") => post_https_body(handle, url, body),
        _ => post_http_body(handle, url, body),
    }
}

pub fn post_http_body(
    handle: &Handle,
    url: Uri,
    body: String,
) -> Box<Future<Item = String, Error = ()>> {
    //  headers: Headers,
    let mut req = Request::new(Post, url);
    // req.headers_mut().set();
    req.set_body(body);
    let client = Client::new(handle);
    let f = client
        .request(req)
        .and_then(|res| {
            res.body().fold(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok::<_, hyper::Error>(acc)
            })
        })
        .map_err(|_err| ())
        .map(|vec| String::from_utf8(vec).unwrap());
    Box::new(f)
}

pub fn post_https_body(
    handle: &Handle,
    url: Uri,
    body: String,
) -> Box<Future<Item = String, Error = ()>> {
    //  headers: Headers,
    let mut req = Request::new(Post, url);
    // req.headers_mut().set();
    req.set_body(body);
    let client = Client::configure()
        .connector(HttpsConnector::new(4, handle).unwrap())
        .build(handle);
    let f = client
        .request(req)
        .and_then(|res| {
            res.body().fold(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok::<_, hyper::Error>(acc)
            })
        })
        .map_err(|_err| ())
        .map(|vec| String::from_utf8(vec).unwrap());
    Box::new(f)
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let f = get_body(&handle, "https://hyper.rs".parse().unwrap()).map(|s| {
        println!("resp: {}", s);
    });
    core.run(f).unwrap();
    // core.run(f).unwrap();
}
