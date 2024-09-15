// we should use enum types any time you need to represent a fixed set of constants. 
enum Direction{
    North , 
    South ,
    East ,
    West
}

enum Shape{
    Rectangle(f64 , f64) , // accpets two arguments lenght and breasdth of flote 64 bits each
    Circle (f64), //accepts 1 arguments radis 
    Square (f64)

}

fn main() {
     let my_direction = Direction::North;
    //  println!("{}" , my_direction);
    move_around(my_direction);


    // for shapes
    let rect = Shape::Rectangle(1.0, 10.0);
    let circle = Shape::Circle(1.2);
    let square = Shape::Square(4.0);

    println!("{}" , calculate_area(rect));
    println!("{}" , calculate_area(circle));
    println!("{}" , calculate_area(square));
}

// this can only have 4 values which is easy and precise 
// we can also use a strinfs in the place of enums but then movearound function sjould have the string type which will eventually become difficult 

fn move_around(direction : Direction){
    println!("hi there ")
}

//calculate the area by using match for getting all the shapes area
// this is known as pattern matching  
fn calculate_area(shape : Shape) ->f64 {
     let area = match shape {
         Shape::Circle(r)=> 3.14 * r * r,
         Shape::Rectangle(a , b ) => a*b,
         Shape::Square(l)=> l*l,
     };

     return area;
}
