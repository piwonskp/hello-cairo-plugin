

mod hello_module {
    #[hello]
    fn add(a: u8,b: u8) -> u8 {
        a + b
    }

}


fn multiply(a: u8, b: u8) -> u8 {
    a * b
}


fn main() {
    hello_module::add(1, 5);
    multiply(4, 7);
}