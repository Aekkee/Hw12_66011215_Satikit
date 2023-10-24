use std::f64::consts::PI;

struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

struct Rectangle {
    x: i32,
    y: i32,
    w: i32, 
    h: i32, 
}

struct Triangle {
    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32)
}

impl Circle {
    fn new(x: i32, y: i32, r:i32) -> Box<dyn Shape>{
        Box::new( Circle{x , y, r})
    }
}

impl Rectangle {
    fn new(x: i32, y: i32, w:i32, h:i32) -> Box< dyn Shape>{
        Box::new( Rectangle{x , y, w, h})
    }
}

impl Triangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Box< dyn Shape>{
        Box::new( Triangle{p1:(x1,y1), p2:(x2, y2), p3:(x3,y3)})
    }
}

trait Shape {
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl Shape for Circle {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.r)
    }

    fn area(&self) -> f64 {
        PI * (self.r * self.r) as f64
    }

    fn clone_box(&self) -> Box<dyn Shape> {
        Circle::new(self.x, self.y, self.r)
    }
}

impl Shape for Rectangle {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.w, self.h)
    }

    fn area(&self) -> f64 {
        (self.w * self.h) as f64
    }

    fn clone_box(&self) -> Box<dyn Shape> {
        Rectangle::new(self.x, self.y, self.w, self.h)
    }
}

impl Shape for Triangle {
    fn rep_string(&self) -> String {
        format!("<Triangle: {}, {}, {}, {}, {},{}>", self.p1.0, self.p1.1, self.p2.0, self.p2.1, self.p3.0, self.p3.1)
    }

    fn area(&self) -> f64 {
        0.5 * ((self.p1.0 - self.p3.0)*(self.p2.1 - self.p1.0) - (self.p1.0 - self.p2.0)*(self.p3.1 - self.p1.1)) as f64
    }

    fn clone_box(&self) -> Box<dyn Shape> {
        Triangle::new(self.p1.0, self.p1.1, self.p2.0, self.p2.1, self.p3.0, self.p3.1)
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

fn main() {
    println!("Hello, world!");
}

fn input_shape_list() -> Vec<Box<dyn Shape>> { 
    vec![ 
        Circle::new(0, 0, 1), Circle::new(50, 50, 15), 
        Rectangle::new(40, 40, 20, 20), Rectangle::new(10, 40, 15, 10) 
    ] 
} 

const EXPECTED_001: &[&str] = &[ 
    "<Circle: 0, 0, 1>", "<Circle: 50, 50, 15>", 
    "<Rectangle: 40, 40, 20, 20>", "<Rectangle: 10, 40, 15, 10>" 
]; 

const EXPECTED_002: &[&str] = &[ 
    "<Circle: 0, 0, 1>, area: 3.14", 
    "<Circle: 50, 50, 15>, area: 706.86", 
    "<Rectangle: 40, 40, 20, 20>, area: 400.00", 
    "<Rectangle: 10, 40, 15, 10>, area: 150.00" 
];

#[test] 
    fn test_shapes_001() { 
        let shape_list = input_shape_list(); 
 
        let omap = shape_list.iter().map(|s| s.rep_string()); 
        let output: Vec<_> = omap.collect(); 
        assert_eq!(output, EXPECTED_001); 
    }


#[test] 
fn test_shapes_002() { 
    let shape_list = input_shape_list(); 

    let omap = shape_list.iter().map( 
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area())    ); 
    let output: Vec<_> = omap.collect(); 
    assert_eq!(output, EXPECTED_002); 
}

#[test] 
    fn test_shapes_003() { 
        let input_list = input_shape_list(); 
        let shape_list = input_list.clone(); 
 
        let omap = shape_list.iter().map( 
            |s| format!("{}, area: {:.2}", s.rep_string(), s.area())    ); 
        let output: Vec<_> = omap.collect(); 
        assert_eq!(output, EXPECTED_002); 
    }
