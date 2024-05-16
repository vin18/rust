let a = 1;
let b = 2;
let c = 3;

let a = 99;
if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}

let a = 99;
if a > 200 {
    println!("Huge number");
} else if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}