use std::f64::consts::PI;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

pub fn setupEnumPractice() {
    let shape = Shape::Triangle(5.0, 7.4, 10.3);
    let area = calcArea(&shape);
    println!("area of {shape:?} is {area}");
    ifLetPractice(&shape);
}

fn calcArea(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => { return PI*(radius*radius); }, 
        Shape::Rectangle(height, width) => { return height*width; },
        // Heron's Formula
        Shape::Triangle(side1, side2, side3) => 
        { 
            let s = (side1 + side2 + side3) / 2.0;
            return (s*(s - side1)*(s - side2)*(s - side3)).sqrt();
        }, 
    }
}

fn ifLetPractice(shape: &Shape) {
    if let Shape::Circle(radius) = shape {
        println!("This is a circle with radius {radius}");
    }
    else if let Shape::Rectangle(height, width) = shape {
        println!("This is a rectangle with height {height} and width {width}");
    }
    else if let Shape::Triangle(side1, side2, side3) = shape {
        println!("This is a triangle with sides {side1}, {side2}, {side3}");
    }
    else {
        println!("No idea what this shape is man");
    }
}