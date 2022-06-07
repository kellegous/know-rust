use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::env;
use std::sync::Arc;

fn serve_hello(name: &str) -> Response<Body> {
    let body = format!(
        r###"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Hello</title>
            <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
            <link href="https://fonts.googleapis.com/css?family=Abril+Fatface" rel="stylesheet">
            <link href='/css' rel='stylesheet' type='text/css'>
        </head>
        <body>
            <div>Hello, {}</div>
        </body>
        </html>
    "###,
        name
    );
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html;charset=utf-8")
        .body(Body::from(body))
        .unwrap()
}

fn serve_css() -> Response<Body> {
    let body = r###"
        body {
            position: absolute;
            top: 0; left: 0; right: 0; bottom: 0;
            font-family: 'Abril Fatface', cursive;
            font-size: 72pt;
            text-align: center;
            display: flex;
            justify-content: center;
            align-items: center;
            background-color: #ef4723;
            color: #fff;
        }
    "###;

    Response::builder()
        .header(header::CONTENT_TYPE, "text/css")
        .body(Body::from(body))
        .unwrap()
}

fn serve_content(req: Request<Body>, name: &str) -> Response<Body> {
    match req.uri().path() {
        "/" => serve_hello(name),
        "/css" => serve_css(),
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap(),
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let name = Arc::new(env::args().nth(1).unwrap_or("world".into()));

    let service = make_service_fn(move |_| {
        let name = name.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let name = name.clone();
                async move { Ok::<_, Infallible>(serve_content(req, &name)) }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
