// Question 2

fn main() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean
    {
        true => 1,
        false => 0
    };
    assert_eq!(binary, 1);

    println!("Success!");
}

// Converting boolean to binary and using match expression from Ln10
