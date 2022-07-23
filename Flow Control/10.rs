// Question 10
// Fill in the blank

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter*2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

// For program to print successs the result=20. We are achieving this by multiplying counter value with 2 and return it using break.  
