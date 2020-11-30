// -- Note: everything below is from the previous section on application-state, I now need to explore _shared_ application-state.

// use actix_web::{ get, web, App, HttpServer };


// // -- This struct represents state
// struct AppState {
//     app_name: String,
// }


// #[get("/")]
// async fn index( data: web::Data<AppState> ) -> String {
//     let app_name = &data.app_name;  // <- get app_name

//     format!( "Hello {}!", app_name ) // <- response with app_name
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new( || {
//         App::new()
//             .data( AppState {
//                 app_name: String::from( "Actix-web" ),
//             } )
//             .service( index )
//     } )
//     .bind( "127.0.0.1:8080" )?
//     .run()
//     .await
// }
