use askama::Template;

#[derive(Template)]
#[template(path = "homepage/index.html")]
pub struct HomepageTemplate<'a> {
    page_title: &'a str,
}

#[get("/")]
pub fn index_action() -> HomepageTemplate<'static> {
    HomepageTemplate { page_title: "Null Development" }
}
