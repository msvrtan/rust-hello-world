use askama::Template;

#[derive(Template)]
#[template(path = "homepage/index.html")]
pub struct HomepageTemplate<'a> {
    name: &'a str,
}

#[get("/")]
pub fn index_action() -> HomepageTemplate<'static> {
    HomepageTemplate { name: "world" }
}
