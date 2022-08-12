// Question 6
// Remove a line in the code to make it compile

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

// We can solve it in 2 different ways - either remove Ln5 containing let x = x or 6 containing x += 3.
