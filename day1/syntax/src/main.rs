fn main() {
    //variable bindings
    // let x = 5;

    //patterns
    //let (x, y) = (1, 2);
    //println!("x is {} and y is {}.", x, y);

    //statically typed language
    //“x is a binding with the type i32 and the value five.”
    //let x: i32 = 5;

    //mutability
    //By default bindings are immutable

    //This will not compiled
    //let x: i32;
    //println!("The value of x is: {}", x);

    //block scope example
    //let x: i32 = 17;
    /*{
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); */

    //shadowing of variable binding
/*     let x: i32 = 8;
    {
        println!("{}", x); // Prints "8"
        let x = 12;
        println!("{}", x); // Prints "12"
    }
    println!("{}", x); // Prints "8"
    let x = 42;
    println!("{}", x); // Prints "42"*/
    print_sum(5,10);
    let z = add_one(1);
    println!("{}", z);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
    }