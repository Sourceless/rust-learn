fn main() {
    let hi = "hi";
    let mut count = 0;

    while count < 10 {
        io::println(fmt!("count: %?", count));
        count += 1;
    }
}
