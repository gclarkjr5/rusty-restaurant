use actix_web::{web, App, HttpServer, dev::Server};

use crate::handlers::{index, ping, read_menu, add_food, read_food, delete_food, update_food};

use crate::models::MenuState;

pub fn run(port: u16) -> std::io::Result<Server> {

    let menu = web::Data::new(
        MenuState::default()
    );

    let server = HttpServer::new(move || {
        App::new()
            .app_data(menu.clone())
            .service(index)
            .service(ping)
            .service(read_menu)
            .service(read_food)
            .service(add_food)
            .service(delete_food)
            .service(update_food)
    })
    .bind(("127.0.0.1", port))?
    .run();

    Ok(server)
}