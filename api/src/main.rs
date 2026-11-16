use poem::{
    EndpointExt, Route, Server, get, handler, listener::TcpListener, middleware::Tracing, web::Path,
};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn createWebsite()->String{

}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let app = Route::new().at("/hello/:name", get(hello));
    .at("/website",post(createWebsite) )
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
