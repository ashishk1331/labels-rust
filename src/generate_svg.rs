use svg::node::element::{Group, Rectangle, Text};
use svg::node::Text as TextNode;
use svg::Document;

pub fn generate(name: &str, prompt: &str, label_color: &str, background_color: &str) -> String {
    let mut document = Document::new()
        .set("xmlns", "http://www.w3.org/2000/svg")
        .set("width", 88.6)
        .set("height", 20)
        .set("viewBox", (0, 0, 886, 200))
        .set("role", "img");

    // Title element
    let title = svg::node::element::Title::new().add(TextNode::new("build: passing"));

    // Group for rectangles
    let rect_group = Group::new()
        .add(
            Rectangle::new()
                .set("fill", background_color)
                .set("width", 368)
                .set("height", 200),
        )
        .add(
            Rectangle::new()
                .set("fill", label_color)
                .set("x", 368)
                .set("width", 518)
                .set("height", 200),
        );

    // Group for texts with the shadow effect and text itself
    let text_group = Group::new()
        .set("aria-hidden", "true")
        .set("fill", "#fff")
        .set("text-anchor", "start")
        .set("font-family", "Verdana,DejaVu Sans,sans-serif")
        .set("font-size", 110)
        .add(
            Text::new()
                .set("x", 50)
                .set("y", 138)
                .set("textLength", 268)
                .add(TextNode::new(name)),
        )
        .add(
            Text::new()
                .set("x", 418)
                .set("y", 138)
                .set("textLength", 418)
                .add(TextNode::new(prompt)),
        );

    // Add title, rectangles and text to the document
    document = document.add(title).add(rect_group).add(text_group);

    document.to_string()
}
