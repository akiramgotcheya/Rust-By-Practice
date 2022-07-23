// Question 8
// Fill in the blanks

fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;
       }
       
       break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

// We are skipping iteration by using continue in Ln9 since the value we are looking for is 66.
// We end the for loop with break statement. 
