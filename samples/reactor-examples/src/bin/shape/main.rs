#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Shapes", app)
}

fn app(_: &mut RenderCx) -> Element {
    vstack((
        text_block("Rounded rectangle"),
        Shape::rectangle()
            .fill_rgb(40, 120, 200)
            .corner_radius(8.0)
            .width(160.0)
            .height(48.0),
        text_block("Ellipse"),
        Shape::ellipse()
            .fill_rgb(220, 80, 120)
            .width(80.0)
            .height(80.0),
        text_block("Line"),
        Shape::line(0.0, 0.0, 200.0, 0.0)
            .stroke(Color::rgb(80, 80, 80))
            .stroke_thickness(3.0)
            .width(200.0)
            .height(4.0),
        Shape::rectangle()
            .stroke(Color::rgb(40, 120, 200))
            .stroke_thickness(2.0)
            .width(160.0)
            .height(48.0),
    ))
    .spacing(8.0)
    .padding(16.0)
    .into()
}
