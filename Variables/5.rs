// Question 5
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)

fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x =  42;
    println!("{}", x); // Prints "42"
}

// Out of scopes, interchanged assert_eq! - x value in Ln8 & 11.
