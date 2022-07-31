// Question 3
// Fill in the blanks

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
} 

fn show_message(msg: Message) {
    match msg {
        Message::Move{x:a, y:b}=> { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
            println!("Move in the x direction {} and in the y direction {}", a,b);
        },
        Message::ChangeColor(r, g, b) => {
            assert_eq!(r, 255);
            assert_eq!(g, 255);
            assert_eq!(b, 0);
            println!("Change the color to red {}, green {}, and blue {}", r, g, b );
        }
         Message::Write(_) => {
         
         println!("no data in these variants")
             
         }
        
        Message::Quit => {
        println!("The Quit variant has no data to destructure.");
        } 
    }
}

// added required message crate in Ln27,38,44 to match 
// and print macros in Ln30,36,40 to print value
