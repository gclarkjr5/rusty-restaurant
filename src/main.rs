use rusty_restaurant::router;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8888;
    router::run(port)?.await
}