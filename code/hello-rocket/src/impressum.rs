use askama::Template;

#[derive(Template)]
#[template(path = "impressum/index.html")]
pub struct ImpressumTemplate<'a> {
    page_title: &'a str,
}

#[get("/impressum")]
pub fn index_action() -> ImpressumTemplate<'static> {
    ImpressumTemplate { page_title: "Null Development - impressum" }
}
