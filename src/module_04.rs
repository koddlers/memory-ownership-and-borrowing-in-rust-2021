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

    fn update_string(s: &mut String) {
        s.push_str(" Another World!");
    }

    pub fn mutable_references() {
        let mut x = String::from("Hello");
        let y = &mut x;

        y.push_str(" World!");
        // both of the following calls to `update_string` are equivalent
        update_string(y);
        // update_string(&mut x);

        println!("The value of x is: {}", x);

        let mut a = 32;
        let b = &mut a;

        *b += 1;

        // printing `a` here is not ok, `ERROR: immutable borrow occurs here`
        // println!("The value of a is {}", a);
        println!("The value of b is {}", b);
        // but it's ok here
        println!("The value of a is {}", a);
    }
}