fn main() {
    // immutability();
    // shadowing();

    // Functions
    // let x = five();
    // println!("The value of x is {x}");

    // Control flow
    control_flow();
}

fn immutability() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 5;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}")
    }

    println!("The value of x is {x}");
}

// Functions
fn five() -> i32 {
    5
}

// Control flow
fn control_flow() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {number}");

    //loop {
        // println!("again!");
    //}
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result of the loop is {result}");

}
