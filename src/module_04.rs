pub mod borrowing_values_by_reference {
    fn print_length(s: &String) {
        println!("The length of `{}` is {}.", s, s.len());
    }

    pub fn immutable_references() {
        let x = String::from("Hello World");
        let y = &x;
        let z = &x;

        println!("x is {}", x);
        println!("y is {}", y);
        println!("*y is {}", *y);
        println!("z is {}", z);

        print_length(&x);
        print_length(y);
        print_length(&y);
    }
}