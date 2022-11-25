fn main() {
    // Owner = name of variable
    // mut = mutable
    let hi: Vec<i32> = (0..10).collect();

    // Use ampersand to borrow object
    fn print_length(val: &Vec<i32>) {
        println!("{}", val.len());
    }

    print_length(&hi);
    println!("Hello, world!");
}
