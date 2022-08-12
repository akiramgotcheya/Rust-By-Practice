// Question 10
// Fill the blank to make the code work

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    
    assert_eq!([x,y], [3,2]);

    println!("Success!");
} 

// Answer is [3,2] since x is the first & y si the last value of the given tuples. 
