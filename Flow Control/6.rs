//Question 6
// Fill in the blanks to make the last println! work !

fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        n+=1;
    }

    println!("n reached {}, so loop is over",n);
}

// Ln9 was while condition was missing and added iteration to counter variable in Ln21 for loop to work
// Also the question is vague. Asks us to make the last println! work. You can do this by adding break in Ln21 which makes program to print last Ln.
