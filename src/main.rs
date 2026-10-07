use slint::*;

slint::include_modules!();

fn main() {
    // Create main window instance
    let main_window = MainWindow::new().unwrap();

    main_window.on_build_commands(move |shape: CanvasShape| {
        let mut path_string = SharedString::new();

        match shape.r#type {
            CanvasShapeType::Dot => {
                
            },
            CanvasShapeType::Line => {
                if shape.r#segments.row_count() >= 2 {
                    if let (Some(start), Some(end)) = (shape.r#segments.row_data(0), shape.r#segments.row_data(1)) {
                        let line_command = std::format!("M {} {} L {} {}", start.x, start.y, end.x, end.y);
                        path_string.push_str(&line_command);
                    }
                }

                else {
                    // show error?
                    println!("Error: Not enough points to build a line. Expected 2, got {}", shape.r#segments.row_count());
                }
            },
            CanvasShapeType::Rectangle => {},
            CanvasShapeType::Select => {},
            _ => todo!(),
        }

        // Implementation for building path commands based on shape
        println!("Built path string: '{}'", path_string);
        path_string
    });

    main_window.on_should_end_drawing(move |shape_type: CanvasShapeType, num_points: i32| {
        match shape_type {
            CanvasShapeType::Dot => num_points >= 1,
            CanvasShapeType::Line => num_points >= 2,
            CanvasShapeType::Rectangle => num_points >= 2,
            CanvasShapeType::Select => num_points >= 2,
            _ => false,
        }
    });

    // Run the event loop
    main_window.run().unwrap();
}
