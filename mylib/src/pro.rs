pub mod template {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T> Point<T> {
        fn get_x(&self) -> &T {
            &self.x
        }
        fn get_y(&self) -> &T {
            &self.y
        }
    }

    impl<T, U> Point2<T, U> {
        fn create_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut larger = list[0];
        for &item in list.iter() {
            if item > larger {
                larger = item;
            }
        }
        larger
    }

    pub fn run() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
        let char_list = vec!['a', 'b', 'c', 'd', 'e'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);

        let a = Point { x: 5, y: 10 };
        println!("{:?}", a.get_x());
        println!("{:?}", a.get_y());
        println!("{:?}", a);

        let c = Point2 { x: 5, y: 'a' };
        println!("{:?}", c);
        let d = Point2 { x: '*', y: 10 };
        let p = c.create_point(d);
        println!("p is {:?}", p);
    }
}

pub mod trait_mod {
    pub trait GetInformation {
        fn get_name(&self) -> &str;
        fn get_age(&self) -> u8;
    }

    trait SchoolName {
        fn get_school_name(&self) -> String {
            String::from("School")
        }
    }

    pub struct Student {
        name: String,
        age: u8,
    }

    impl SchoolName for Student {}
    
    pub struct Teacher {
        name: String,
        age: u8,
        subject: String,
    }

    impl SchoolName for Teacher {
        fn get_school_name(&self) -> String {
            format!("School of {}", self.subject)
        }
    }

    impl GetInformation for Student {
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_age(&self) -> u8 {
            self.age
        }
    }

    impl GetInformation for Teacher {
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_age(&self) -> u8 {
            self.age
        }
    }

    fn print_information<T: GetInformation>(item: T) {
        println!("Name: {}", item.get_name());
        println!("Age: {}", item.get_age());
    } 

    pub fn run() {
        let student = Student {
            name: String::from("Alice"),
            age: 20,
        };
        let teacher = Teacher {
            name: String::from("Bob"),
            age: 30,
            subject: String::from("Math"),
        };
        println!("Student name: {}", student.get_name());
        println!("Student age: {}", student.get_age());
        println!("Teacher name: {}", teacher.get_name());
        println!("Teacher age: {}", teacher.get_age());

        let s = student.get_school_name();
        let t = teacher.get_school_name();
        println!("Student school name: {}", s);
        println!("Teacher school name: {}", t);

        print_information(student);
    }
}
