use nannou::{draw::mesh::Colors, prelude::*};

fn main() {
    use std::collections::HashMap;

    let mut my_colors: HashMap<&str, Vec<LinSrgb>> = HashMap::new();

    my_colors.insert(
        "peach",
        vec![
            rgb(0.976, 0.631, 0.737), // Base (#F9A1BC)
            rgb(0.992, 0.827, 0.878), // Variant (#FDD3E0)
        ],
    );

    my_colors.insert(
        "mint",
        vec![
            rgb(0.663, 0.875, 0.847), // Base (#A9DFD8)
            rgb(0.847, 0.953, 0.945), // Variant (#D8F3F1)
        ],
    );

    my_colors.insert(
        "lilac",
        vec![
            rgb(0.780, 0.725, 0.957), // Base (#C7B9F4)
            rgb(0.894, 0.875, 0.984), // Variant (#E4DFFB)
        ],
    );
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse with default size and position.
    draw.ellipse().color(STEELBLUE);
    let start_point = pt2(-30.0, -20.0);

    let end_point = pt2(400.0, -20.0);

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(my_colors["peach"][0])
        .end_cap(nannou::lyon::tessellation::LineCap::Round);
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
