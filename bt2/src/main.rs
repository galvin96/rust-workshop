use std::io;

fn main() {
    let org_string = "adbcdaDd";
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input.pop();
    let count = org_string.matches(&input).count();
    println!("count: {count}")
}
