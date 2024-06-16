pub mod using_lifetimes_to_reduce_ambiguity {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    fn get_oldest<'a>(person1: &'a Person, person2: &'a Person) -> &'a Person {
        if person1.age > person2.age {
            person1
        } else {
            person2
        }
    }

    pub fn lifetimes_in_functions() {
        let person_1 = Person { name: "John".to_string(), age: 20 };
        let person_2 = Person { name: "Jane".to_string(), age: 30 };
        let person_3 = get_oldest(&person_1, &person_2);

        println!("person_1: {:?}", person_1);
        println!("person_2: {:?}", person_2);

        println!("The oldest person is: {:?}", person_3)
    }
}

pub mod using_lifetimes_to_reduce_ambiguity_v2 {
    #[derive(Debug)]
    struct Person<'a> {
        name: String,
        age: u8,
        favorite_fruit: &'a str,
    }

    pub fn lifetimes_in_structs() {
        let winter_fruits = [
            "Apples".to_string(),
            "Clementines".to_string(),
            "Grapefruit".to_string(),
            "Kiwis".to_string()
        ];

        let person_1 = Person {
            name: "John".to_string(),
            age: 20,
            favorite_fruit: &winter_fruits[0],
        };
        println!("person_1: {:?}", person_1);

        let person_2 = Person {
            name: "Jane".to_string(),
            age: 30,
            favorite_fruit: &winter_fruits[1],
        };
        println!("person_2: {:?}", person_2);
    }

    // Lifetime Elision Rules
    // 1. The compiler assigns a lifetime parameter to each parameter that's a reference.
    // 2. If there is exactly one input lifetime parameter, that lifetime is assigned to
    //    all output lifetime parameters.
    // 3. If there are multiple input lifetime parameters, but one of them is &self or
    //    &mut self, the lifetime of self is assigned to all output lifetime parameters.
    fn first_word_in_string(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        s
    }

    pub fn lifetime_elision_rules() {
        let array_of_sentences = ["Hello World!", "How are you?", "I am fine"];

        for sentence in array_of_sentences.iter() {
            println!("The first word in the sentence `{}` is: {}",
                     sentence, first_word_in_string(sentence));
        }
    }

    // static lifetimes lasts for the whole duration of the program,
    // i.e. they are not bound to their enclosing scope
    const GLOBAL_GREETING: &'static str = "Hello World as a constant";

    pub fn static_lifetimes() {
        let global_greeting: &'static str = "Hello World as a String Literal";

        println!("Constant: {}", GLOBAL_GREETING);
        println!("String Literal: {}", global_greeting);
    }
}