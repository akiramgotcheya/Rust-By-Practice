//Question 3

fn main() {
    for n in 1..=99 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 

// There is no error in the code but since our goal is to print Success, Replacing 100 in Ln4 with number < 100 will do the trick
// We can modify Ln5 as well but since the question specifically asks us to modify Ln4, this will be the solution.
