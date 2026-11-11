fn show_label(label: String) {
    println!("{}", label);
}

fn main() {
    let label = String::from("sensor");
    show_label(label);
    println!("{}", label); // error: borrowed after move
}
