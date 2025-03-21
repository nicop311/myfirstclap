#![deny(warnings)]

use std::convert::Infallible;
use log::{info, debug, warn, trace, error};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

//use pretty_env_logger;

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World from serve hello.rs!")))
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    info!("such information");
    debug!("This is a debug message");
    trace!("This trace will be hard to wash");
    warn!("o_O");
    error!("much error");

    server.await?;

    Ok(())
}