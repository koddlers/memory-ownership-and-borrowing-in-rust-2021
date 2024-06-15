pub mod managing_memory_with_ownership {
    pub fn copyable_data_types() {
        let x = 5;
        let y = x;      // this is fine

        println!("x: {}", x);
        println!("y: {}", y);

        let a = String::from("Hello World");
        // moving the value pointed to by `a` to `b`
        let b = a;

        // the following line will produce the error: `value borrowed here after move`
        // println!("a: {}", a);
        // while this line is fine
        println!("b: {}", b);
    }

    fn print_string(a: &String) {
        println!("This is the value of my string: {:?}", a);
    }

    pub fn non_copyable_data_types() {
        let x = String::from("42");
        let y = x;

        // the following line will produce the error: `value borrowed here after move`
        // println!("x: {}", x);
        println!("y: {}", y);

        let x = String::from("42");

        // print_string(x);
        // the following line will produce the error: `value used here after move`
        // print_string(x);

        // there are two ways to solve this
        // solution 1: to send a copy of the data using the `clone()`
        // which is a deep copy and expensive
        // print_string(x.clone());
        // print_string(x);

        // alternatively, the better way to do this is by sending an immutable reference
        // thus the function `print_string` borrows the value of the variable being sent
        print_string(&x);
        print_string(&x);
    }
}