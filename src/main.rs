#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Json;


#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
        'status':'success',
        'message':'Hello API'
    }")
}
fn main() {
    rocket::ignite().mount("/api", routes![hello]).launch();

}

