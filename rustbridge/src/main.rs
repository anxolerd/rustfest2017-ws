fn say_hello(name: &str) -> String {
    let message = format!("hello, {}!", name);
    message
}


fn main() {
    println!("{}", say_hello("Cthulhu"));
}
