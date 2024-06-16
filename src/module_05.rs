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
}