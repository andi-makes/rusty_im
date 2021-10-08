use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    let db = rusty_im::get_database().await.unwrap();

    println!("{:?}", req);
    req.uri().query().unwrap().split('&').for_each(|f| println!("{}", f));

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/list/batch") => {
            *response.body_mut() = Body::from(
                format!("{:?}", rusty_im::list_batch(&db).await.unwrap())
            );
        },
        (&Method::GET, "/list/item") => {
            *response.body_mut() = Body::from(
                format!("{:?}", rusty_im::list_item(&db).await.unwrap())
            );
        },
        (&Method::GET, "/list/model") => {
            *response.body_mut() = Body::from(
                format!("{:?}", rusty_im::list_model(&db).await.unwrap())
            );
        },
        (&Method::GET, "/list/property") => {
            *response.body_mut() = Body::from(
                format!("{:?}", rusty_im::list_property(&db).await.unwrap())
            );
        },
        _ => {
            *response.body_mut() = Body::from("404 - Not Found");
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });
    let server = Server::bind(&addr).serve(make_service);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("SERVER ERROR: {}", e);
    }
}