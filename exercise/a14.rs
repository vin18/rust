struct Person {
    name: String,
    fav_color: String,
    age: i32
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::From("green"),
            age: 7
        },
        Person {
            name: String::from("Anna"),
            fav_color: String::From("purple"),
            age: 9
        },
        Person {
            name: String::from("Katty"),
            fav_color: String::From("blue"),
            age: 14
        }
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name)
            print(&person.fav_color)
        }
    }
}