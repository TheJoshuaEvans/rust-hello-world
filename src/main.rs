use std::fmt;

// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html

// A struct with two fields
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle{top_left, bottom_right} = rect;
    
    // Create a new rectangle, shifted to have the top left point at (0, 0)
    let shifted_rect = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { 
            x: bottom_right.x - top_left.x,
            y: bottom_right.y - top_left.y,
        },
    };

    // Now, the absolute value of the bottom right point will give us the length of the sides
    let x_side_length = shifted_rect.bottom_right.x.abs();
    let y_side_length = shifted_rect.bottom_right.y.abs();

    // The areas is the side lengths multiplied
    return x_side_length * y_side_length;
}

fn test_rect_area() {
    let test_rect_50 = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 10.0, y: -5.0 },
    };

    let test_rect_25 = Rectangle {
        top_left: Point { x: 5.0, y: 0.0 },
        bottom_right: Point { x: 10.0, y: -5.0 },
    };

    let test_rect_100 = Rectangle {
        top_left: Point { x: -5.0, y: 5.0 },
        bottom_right: Point { x: 5.0, y: -5.0 },
    };

    let test_rect_20 = Rectangle {
        top_left: Point { x: -1_000_000.0, y: -1_000_000.0 },
        bottom_right: Point { x: -9_999_95.0, y: -1_000_004.0 }
    };

    println!("Expect Area 50 vs {}", rect_area(test_rect_50));
    println!("Expect Area 25 vs {}", rect_area(test_rect_25));
    println!("Expect Area 100 vs {}", rect_area(test_rect_100));
    println!("Expect Area 20 vs {}", rect_area(test_rect_20));
}

fn square(top_left: Point, side_length: f32) -> Rectangle {
    let Point{x, y} = top_left;
    Rectangle {
        top_left,
        bottom_right: Point { 
            x: x + side_length,
            y: y - side_length,
        }
    }
}

fn test_square() {
    struct TestSquareData {
        top_left: Point,
        side_length: f32,
        expected_bottom_right: Point,
    }
    let tests: [TestSquareData; 3] = [
        TestSquareData {
            top_left: Point {x: 0.0, y:0.0},
            side_length: 10.0,
            expected_bottom_right: Point {x: 10.0, y: -10.0},
        },
        TestSquareData {
            top_left: Point {x: -10.0, y:10.0},
            side_length: 20.0,
            expected_bottom_right: Point {x: 10.0, y: -10.0},
        },
        TestSquareData {
            top_left: Point { x: -1_000_000.0, y: 1_000_000.0 },
            side_length: 50.0,
            expected_bottom_right: Point { x: -999_950.0, y: 999_950.0 },
        }
    ];
    
    for test in tests.iter() {
        println!(
            "Expected bottom right: {} vs {}",
            test.expected_bottom_right,
            square(test.top_left, test.side_length).bottom_right,
        );
    }
}

fn main() {
    test_rect_area();
    test_square();
}
