fn main() {
    let readings = vec![3, 8, 12, 5, 7, 10];
    let total: i32 = readings.iter().filter(|&&n| n % 2 == 0).sum();
    println!("{}", total);
}
