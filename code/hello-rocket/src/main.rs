#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate askama;
extern crate rocket_contrib;

mod homepage;
mod impressum;

fn main() {
    rocket::ignite()
        .mount("/", routes![homepage::index_action])
        .mount("/", routes![impressum::index_action])
        .launch();
}
