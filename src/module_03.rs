pub mod managing_memory_with_ownership {
    pub fn copyable_data_types() {
        let x = 5;
        let y = x;      // this is fine

        println!("x: {}", x);
        println!("y: {}", y);

        let a = String::from("Hello World");
        // moving the value pointed to by `a` to `b`
        let b = a;

        // this line will produce the error: `value borrowed here after move`
        // println!("a: {}", a);
        // while this line is fine
        println!("b: {}", b);
    }
}