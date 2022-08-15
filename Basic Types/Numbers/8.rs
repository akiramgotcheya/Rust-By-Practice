//Question 8
// Make it work in two distinct ways

fn main() {
    assert!(0.1_f32 +0.2_f32 ==0.3_f32);
    println!("Success!");
}

// or


fn main() {
    let v = 0.3_f32;
    assert!(0.1_f32+0.2_f32==v);

    println!("Success!");
}


