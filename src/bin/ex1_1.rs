use std::fmt;
pub const PI: f64 = 3.1416;

const INPUT_SHAPES: &[Shape] = &[ 
        Shape::Circle(0, 0, 1), Shape::Circle(50, 50, 15), 
        Shape::Rectangle(40, 40, 20, 20), Shape::Rectangle(10, 40, 15, 10),
        Shape::Triangle(40, 35, 15, 20, 10, 10), Shape::Triangle(10, 40, 15, 10, 20, 10)
    ]; 
 

fn main() {
    let input_list = INPUT_SHAPES; 
    let shape_list = input_list.clone(); 

    let omap = shape_list.iter().map( 
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) ); 
    let output: Vec<_> = omap.collect();
    println!("{output:?}")
}

#[derive(Debug)]
enum Shape {
    Circle(i32, i32, i32),
    Rectangle(i32, i32, i32, i32),
    Triangle(i32,i32,i32,i32,i32,i32)
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Circle(x, y, r) => write!(f, "{}", format!("<Circle: {x}, {y}, {r}>")),
            Shape::Rectangle(x, y, w, h) => write!(f, "{}", format!("<Rectangle: {x}, {y}, {w}, {h}>")),
            Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3 ) => write!(f, "{}", format!("Triangle: {x1}, {y1}, {x2}, {y2}, {x3}, {y3}>"))
        }
    }
}

impl Shape {
    fn rep_string(&self) -> String {
        return self.to_string()
    }

    fn area(&self) -> f64{
        let result = match &self {
            Shape::Circle(x,y,r) => PI * *r as f64 * *r as f64,
            Shape::Rectangle(x,y,w,h) => *w as f64 * *h as f64 ,
            Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3 ) => 0.5 * ((x1 - x3)*(y2 - y1) - (x1 - x2)*(y3 - y1)) as f64
        };
        return result
    }
}

 

