fn main() {
    let mut i = 5;
    while i > 1 {
        if i % 2 == 0 {
            println!("{}", i);
        }
        i -= 1;
    }
}
