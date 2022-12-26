//use dotenv::dotenv;
use rocket::fs::{relative, FileServer}; // <--
use rocket_dyn_templates::{context, Template};
use std::env;

#[macro_use]
extern crate rocket;
mod lightning;
mod routes;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/new_invoice")]
pub fn invoice() -> Template {
    Template::render("invoice", context! {})
}

#[get("/new_payment")]
pub fn payment() -> Template {
    Template::render("payment", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("static"))) // <-- Seteamos un directorio para contenido estático
        .mount(
            "/",
            routes![
                index,
                invoice,
                payment,
                routes::hello,
                routes::create_invoice,
                routes::lookup_invoices,
                routes::lookup_invoice,
                routes::get_wallet_balance,
                routes::payment
            ],
        )
        .attach(Template::fairing()) // <--
}
