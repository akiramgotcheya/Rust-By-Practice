// Question 5
// Fix errors and panics to make it work

fn main() {
   let v1 = 247_u8 + 8;
   let v2 = i8::checked_add(119, 8).unwrap();
   println!("{},{}",v1,v2);
}


//sum of u8 should be <=255 and i8 should be <=127
