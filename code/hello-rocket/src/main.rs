#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate askama;

use askama::Template;

#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate<'a> {
    name: &'a str,
}

#[get("/")]
fn hello() -> HomepageTemplate<'static> {
    HomepageTemplate { name: "world" }
}
fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .launch();
}
