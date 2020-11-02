// fn main() {
//     println!("Hello, world!");
// }


use actix_web::{ web, App, HttpServer, Responder };


async fn index() -> impl Responder {
    "Hello world from the app example!"
}


#[actix_web::main]
async fn main() -> std::io::Result< () > {
    HttpServer::new( || {
        App::new().service(
            // -- prefixes all resources nd routes attached to it...
            web::scope( "/app" )
                // -- ...so this handles requests for `GET /app/index.html`
                .route( "/index.html", web::get().to(index) ),
        )
    } )
    .bind( "127.0.0.1:8081" )?
    .run()
    .await  // now accessible via `http://127.0.0.1:8081/app/index.html`
}
