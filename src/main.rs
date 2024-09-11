mod colors;
mod generate_svg;
use rocket::response::content::RawXml;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    format!("try the /label route")
}

#[get("/label")]
fn label_route() -> String {
    format!("also add a name, a prompt and a color for the label. eg: /build/passing/green")
}

#[get("/label/<name>/<prompt>/<color>")]
fn get_label(name: &str, prompt: &str, color: &str) -> RawXml<String> {
    let label_color = colors::get_valid_color(color);

    let build_up_svg = generate_svg::generate(name, prompt, label_color, colors::BG);

    RawXml(build_up_svg)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, label_route, get_label])
}
