match my_number {
    0       => io::println("zero"),
    1 | 2   => io::println("one or two"),
    3..10   => io::println("three to ten"),
    _       => io::println("something else")
}
