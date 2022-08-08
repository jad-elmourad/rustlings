// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(u8),
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(1));
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
