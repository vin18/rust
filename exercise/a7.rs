enum Color {
    Red,
    Yellow,
    Blue
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("Red color"),
        Color::Yellow => println!("Yellow color"),
        Color::Blue => println!("Blue color"),
    }
}

fn main() {
    print_color(Color::Blue);
}