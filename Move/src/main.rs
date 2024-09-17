// moving basically means moving the ownership from one variable to another variable
fn main() {
      let a = String::from("kedar");
      let b = a;

    //   println!("{}" , a);
    println!("{}" , b);
    // this abouve statement throws an error because we moved the ownership from a to b 
    // now a does not store any addres init so its gives an error 

    //some more example
    // 1
    let a1 = String::from("new String ");
    {
        let a2 = a1;
        println!("{}" , a2);
    }
    // println!("{}" , a1);
    // the above line also gives error


    // 2
    let s1 = String::from("s1 string");
    print_statement(s1);

    // println!("{}" , s1);
    // this above line also gives the error because the Ownership is moved from s1 to s2


    // overcome the problem in 2 
    let mut string1 = String::from(" mutable String");
    string1 = print_statement_and_return(string1);
    println!("{}" , string1);

    // ownership came back to the string1 because the function return a string and that string give the ownership to again to string1

}

fn print_statement(s2 : String){
    println!("{}" , s2);
}

fn print_statement_and_return(s2 : String) -> String{
    println!("{} ", s2);

    return s2;
}
