fn main() {
    println!("Hello, world!");
    add_string();
}

fn add_string() {
    let s = String::new();
    let x = s + "1231" + &String::from("value");
    println!("x:{x}");
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
