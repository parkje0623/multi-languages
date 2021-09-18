// any_shape_zero_area - time:   [36.515 us 37.206 us 38.047 us] Found 5 outliers among 100 measurements (5.00%) 1 (1.00%) high mild 4 (4.00%) high severe
// any_circle_zero_area - time:   [20.012 us 20.282 us 20.621 us] Found 18 outliers among 100 measurements (18.00%) 4 (4.00%) high mild 14 (14.00%) high severe
// any_rectangle_zero_area - time:   [20.878 us 21.213 us 21.653 us] Found 19 outliers among 100 measurements (19.00%) 9 (9.00%) high mild 10 (10.00%) high severe
use rand::Rng;

pub trait Shape {
    fn area(&self) -> f64;
    fn description(&self) -> &str; // used to inspect types during testing
}

#[derive(Debug, Clone)]
pub struct Circle {
    radius: f64,
}
impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    pub fn random() -> Circle {
        Circle {
            radius: rand::random::<f64>() + 1.0,
        }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    fn description(&self) -> &str {
        "circle"
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    pub fn random() -> Rectangle {
        Rectangle {
            width: rand::random::<f64>() + 1.0,
            height: rand::random::<f64>() + 1.0,
        }
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn description(&self) -> &str {
        "rectangle"
    }
}

pub fn any_circle_zero_area(shapes: &Vec<Box<Circle>>) -> bool {
    for i in shapes.iter() {
        if i.area() == 0.0 {
            return true;
        }
    }
    return false;
}

pub fn any_rectangle_zero_area(shapes: &Vec<Box<Rectangle>>) -> bool {
    for i in shapes.iter() {
        if i.area() == 0.0 {
            return true;
        }
    }
    return false;
}

pub fn any_shape_zero_area(shapes: &Vec<Box<dyn Shape>>) -> bool {
    for i in shapes.iter() {
        if i.area() == 0.0 {
            return true;
        }
    }
    return false;
}

// generate 2*n Circles
pub fn make_circle_vec(n: usize) -> Vec<Box<Circle>> {
    let mut circle: Vec<Box<Circle>> = Vec::new();
    for i in 0..2*n {
        circle.insert(i, Box::new(Circle::random()));
    }
    return circle;
}

// generate 2*n Rectangles
pub fn make_rectangle_vec(n: usize) -> Vec<Box<Rectangle>> {
    let mut rect: Vec<Box<Rectangle>> = Vec::new();
    for i in 0..2*n {
        rect.insert(i, Box::new(Rectangle::random()));
    }
    return rect;
}

//generate n Circles and n Rectangles
pub fn make_mixed_vec(n: usize) -> Vec<Box<dyn Shape>> {
    let mut mixed: Vec<Box<dyn Shape>> = Vec::new();
    for i in 0..n {
        mixed.insert(i, Box::new(Circle::random()));
    }
    for i in n..2*n {
        mixed.insert(i, Box::new(Rectangle::random()));
    }
    return mixed;
}