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

    #[derive(Debug)]
    struct Person {
        id: u8,
        age: u8,
        name: String,
    }

    impl Clone for Person {
        fn clone(&self) -> Self {
            println!("Copied from this old value: {:?}", self);

            Self {
                id: self.id,
                age: self.age,
                name: self.name.clone(),
            }
        }
    }

    pub fn copy_and_clone_traits() {
        // Data types that implement the `Copy` trait
        /// Scalar Types
        // * Integer (i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize)
        // * Float (f32, f64)
        // * Boolean (bool)
        // * Character (char)
        /// Compound Types
        // * Array
        // * Tuple

        let x = 42;
        let mut y = x;

        println!("Before, x is {}", x);
        println!("Before, y is {}", y);

        y = 500;

        println!("After, x is {}", x);
        println!("After, y is {}", y);

        let person_1 = Person {
            id: 1,
            age: 20,
            name: "Shaun".to_string(),
        };

        // `clone()` returns a deep copy of the original, which is essentially
        // a new instance with the same `field:values` combination as the original
        let mut person_2 = person_1.clone();
        println!("person_2 (after clone): {:?}", person_2);

        person_2.id = 2;
        person_2.age = 30;
        person_2.name = "John".to_string();

        println!("person_1: {:?}", person_1);
        println!("person_2 (after modification): {:?}", person_2);
    }
}