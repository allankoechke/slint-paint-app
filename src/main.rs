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
            // CanvasShapeType::Select => num_points >= 2,
            _ => false,
        }
    });

    main_window.on_bounding_box_min_pos(move |shape: CanvasShape| {
        match shape.r#type {
            CanvasShapeType::Dot => {
                let x = shape.r#segments.row_data(0).unwrap().x-shape.r#stroke_width/2.0;
                let y = shape.r#segments.row_data(0).unwrap().y-shape.r#stroke_width/2.0;
                Point{x,y}
            },
            CanvasShapeType::Line => {
                let mut starting_pt = Point{x: 0.0, y: 0.0};

                // Get min x
                if shape.r#segments.row_data(0).unwrap().x > shape.r#segments.row_data(1).unwrap().x {
                    starting_pt.x = shape.r#segments.row_data(1).unwrap().x;
                } else {
                    starting_pt.x = shape.r#segments.row_data(0).unwrap().x;
                }

                // Get min y
                if shape.r#segments.row_data(0).unwrap().y > shape.r#segments.row_data(1).unwrap().y {
                    starting_pt.y = shape.r#segments.row_data(1).unwrap().y;
                } else {
                    starting_pt.y = shape.r#segments.row_data(0).unwrap().y;
                }

                starting_pt
            },
            _ => Point{x: 0.0, y: 0.0},
        }
    });

    main_window.on_bounding_box_max_pos(move |shape: CanvasShape| {
        match shape.r#type {
            CanvasShapeType::Dot => {
                Point{
                    x: shape.r#segments.row_data(0).unwrap().x+shape.r#stroke_width/2.0,
                    y: shape.r#segments.row_data(0).unwrap().y+shape.r#stroke_width/2.0,
                }
            },
            CanvasShapeType::Line => {
                let mut ending_pt: Point = Point{x: 0.0, y: 0.0};

                // Get min x
                if shape.r#segments.row_data(0).unwrap().x > shape.r#segments.row_data(1).unwrap().x {
                    ending_pt.x = shape.r#segments.row_data(0).unwrap().x;
                } else {
                    ending_pt.x = shape.r#segments.row_data(1).unwrap().x;
                }

                // Get min y
                if shape.r#segments.row_data(0).unwrap().y > shape.r#segments.row_data(1).unwrap().y {
                    ending_pt.y = shape.r#segments.row_data(0).unwrap().y;
                } else {
                    ending_pt.y = shape.r#segments.row_data(1).unwrap().y;
                }

                ending_pt
            },
            _ => Point{x: 0.0, y: 0.0},
        }
    });

    main_window.run().unwrap();
}