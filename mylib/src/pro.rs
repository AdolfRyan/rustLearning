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
    pub mod lecture_2 {

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

    pub mod lecture_3 {

        trait getName {
            fn get_name(&self) -> &String;
        }

        trait getAge {
            fn get_age(&self) -> u8;
        }

        fn print_information<T: getName + getAge>(item: T) {
            // where T: getName + getAge
            println!("Name: {}", item.get_name());
            println!("Age: {}", item.get_age());
        }

        #[derive(Debug)]
        pub struct Student {
            name: String,
            age: u8,
        }

        impl getName for Student {
            fn get_name(&self) -> &String {
                &self.name
            }
        }

        impl getAge for Student {
            fn get_age(&self) -> u8 {
                self.age
            }
        }

        fn produce_item_with_age() -> impl getAge {
            Student {
                name: String::from("Alice"),
                age: 20,
            }
        }

        pub fn run() {
            let student = Student {
                name: String::from("Alice"),
                age: 20,
            };
            print_information(student);

            let s = produce_item_with_age();
        }
    }

    pub mod lecture_6 {
        trait getName {
            fn get_name(&self) -> &String;
        }
        trait getAge {
            fn get_age(&self) -> u8;
        }

        struct peopleMatchInformation<T, U> {
            master: T,
            student: U,
        }

        impl<T: getName + getAge, U: getName + getAge> peopleMatchInformation<T, U> {
            fn print_all_information(&self) {
                println!("Master name: {}", self.master.get_name());
                println!("Master age: {}", self.master.get_age());
                println!("Student name: {}", self.student.get_name());
                println!("Student age: {}", self.student.get_age());
            }
        }

        struct Teacher {
            name: String,
            age: u8,
        }

        impl getName for Teacher {
            fn get_name(&self) -> &String {
                &self.name
            }
        }

        impl getAge for Teacher {
            fn get_age(&self) -> u8 {
                self.age
            }
        }

        struct Student {
            name: String,
            age: u8,
        }

        impl getName for Student {
            fn get_name(&self) -> &String {
                &self.name
            }
        }

        impl getAge for Student {
            fn get_age(&self) -> u8 {
                self.age
            }
        }

        pub fn run() {
            let teacher = Teacher {
                name: String::from("Bob"),
                age: 30,
            };
            let student = Student {
                name: String::from("Alice"),
                age: 20,
            };
            let people = peopleMatchInformation {
                master: teacher,
                student,
            };
            people.print_all_information();
        }
    }

    pub mod lecture_7 {
        trait getName {
            fn get_name(&self) -> &String;
        }
        trait printName {
            fn print_name(&self);
        }

        impl<T: getName> printName for T {
            fn print_name(&self) {
                println!("Name: {}", self.get_name());
            }
        }

        struct Student {
            name: String,
            age: u8,
        }

        impl getName for Student {
            fn get_name(&self) -> &String {
                &self.name
            }
        }
        pub fn run() {
            let s = Student {
                name: String::from("Alice"),
                age: 20,
            };
            s.print_name();
        }
    }
}

pub mod life_time {
    pub mod lecture_1 {
        pub fn run() {
            // let r;
            // {
            //     let x = 5;
            //     r = &x;
            // }
            // println!("r: {}", r);
        }
    }

    pub mod lecture_2 {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }

        // fn a_str<'a>(x: &str, y: &str) -> &'a str {
        //     let r = String::from("ssss");
        //     r.as_str()
        // }

        pub fn run() {
            let s1 = String::from("long string is long");
            let s2 = String::from("xyz");
            let result = longest(s1.as_str(), s2.as_str());
            println!("The longest string is {}", result);
        }
    }

    pub mod lecture_3 {
        struct A<'a> {
            name: &'a str,
        }

        pub fn run() {
            let a = A { name: "Alice" };
            println!("Name: {}", a.name);
        }
    }

    pub mod lecture_5 {
        struct stuA<'a> {
            name: &'a str,
        }

        impl<'a> stuA<'a> {
            //带有self的默认返回的生命周期是self的生命周期
            fn get_name(&self) -> &'a str {
                self.name
            }
        }

        pub fn run() {
            let name = String::from("Alice");
            let a = stuA { name: &name };
            println!("Name: {}", a.get_name());
        }
    }

    pub mod lecture_6 {
        //static生命周期存活于整个程序期间
        use std::fmt::Display;

        fn function<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
            println!("{}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        pub fn run() {
            let x = String::from("long string is long");
            let y = String::from("xyz");
            let ann = String::from("annotation");
            let result = function(x.as_str(), y.as_str(), ann);
            println!("The longest string is {}", result);
        }
    }
}

pub mod closure {
    pub mod lecture_1 {
        //格式
        fn add_one(x: u32) -> u32 {
            println!("{}", x + 1);
            x + 1
        }
        pub fn run() {
            let add_one1 = |x: u32| -> u32 { x + 1 };
            let add_one2 = |x| x + 1;
            let add_one3 = |x| {x + 1};

            let a = add_one(5);
            let b = add_one1(5);
            let c = add_one2(5);
            let d = add_one3(5);
            println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

            let i = 1;
            let exe = |x| x + i;
            let r = exe(5);
            println!("r: {}", r);
            
        }
    }

    pub mod lecture_2 {
        struct Cacher<T: Fn(u32) -> u32> {
            calculation: T,
            value: Option<u32>,
        }

        impl<T: Fn(u32) -> u32> Cacher<T>
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value:None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        pub fn run() {
            let mut c = Cacher::new(|x| x + 1);
            let v1 = c.value(1);
            let v2 = c.value(20);
            println!("v1: {}, v2: {}", v1, v2);
        }
    }

    pub mod lecture_3 {
        pub fn run() {
            // let x = 4;
            // let equal_to_x = |z| z == x;
            // let y = 4;
            // println!("{}", equal_to_x(y));
            let x = vec![1, 2, 3];
            let equal_to_x = move |z| z == x;
            // println!("can't use x here: {:?}", x);
            let y = vec![1, 2, 3];
            println!("{}", equal_to_x(y));
        }
    }

}
