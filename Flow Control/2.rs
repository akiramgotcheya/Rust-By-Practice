//Question 2
// Fix the errors

fn main() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!("{}, and is a small number, increase ten-fold",n);

            10 * n
        } else {
            println!("{}, and is a big number, halve the number",n);

            n / 2
        };

    println!("{} -> {}", n, big_n);
} 

// Here there are 2 ways to solve this. Either by converting all the numbers to f:32 or converting 2.0 in ln 15 to i:32.
// Also removed semicolon from ln 15 and placed it in ln 16.
// Both Println was missing formatting specifier & arguments.
