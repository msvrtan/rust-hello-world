use askama::Template;

#[derive(Template)]
#[template(path = "impressum/index.html")]
pub struct ImpressumTemplate<'a> {
    name: &'a str,
}

#[get("/impressum")]
pub fn index_action() -> ImpressumTemplate<'static> {
    ImpressumTemplate { name: "world" }
}
