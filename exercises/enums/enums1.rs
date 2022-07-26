// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    Quit,
    Echo (i32, String),
    Move {x: i64},
    ChangeColor (fn()->i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(1, String::from("ok")));
    println!("{:?}", Message::Move{x:1});
    println!("{:?}", Message::ChangeColor(||1));
}
