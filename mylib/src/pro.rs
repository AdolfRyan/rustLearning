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
            let add_one3 = |x| x + 1;

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

        impl<T: Fn(u32) -> u32> Cacher<T> {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
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

// trait Iterator {
//     type Item;
//     fn next(mut self) -> Option<Self::Item>;//定义trait的关联类型；
// }
//next 是Iterator被要求实现的唯一的一个方法

pub mod iterator {
    pub mod lecture_1 {
        use std::iter;

        pub fn run() {
            let v1 = vec![1, 2, 3];
            let mut v1_iter = v1.iter();
            // for val in v1_iter {
            //     println!("val = {}", val)
            // }
            if let Some(v) = v1_iter.next() {
                println!("val = {}", v)
            }
            if let Some(v) = v1_iter.next() {
                println!("val = {}", v)
            }
            if let Some(v) = v1_iter.next() {
                println!("val = {}", v)
            }
            if let Some(v) = v1_iter.next() {
                println!("val = {}", v)
            } else {
                println!("emd!!!!!!")
            }

            let mut v2 = vec![1, 2, 3];
            let mut v2_iter = v2.iter_mut();
            if let Some(v) = v2_iter.next() {
                *v = 3;
            }
            println!("v2 = {:?}", v2);

            //---消费适配器----
            let v1 = vec![1, 2, 3];
            let v1_iter = v1.iter();
            let total: i32 = v1_iter.sum();
            println!("total is : {}", total);

            //-----迭代适配器-------
            let v1 = vec![1, 2, 3];
            let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
            println!("v2 is {:?}", v2);

            let v1 = vec![1, 2, 3, 10];
            let v2: Vec<_> = v1.into_iter().filter(|x| *x > 5).collect();
            println!("v2 is {:?}", v2);
        }
    }

    pub mod lecture_2 {
        use crate::factory::produce_washing_machine::B::C;

        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        pub fn run() {
            let mut counter = Counter::new();
            for i in 0..6 {
                if let Some(v) = counter.next() {
                    println!("i = {}, v = {}", i, v);
                } else {
                    println!("i = {} at end", i);
                    break;
                }
            }
        }
    }
}

pub mod cargo_mod {
    pub mod lecture_1 {
        pub fn run() {
            //cargo build --release
            //cargo run --release

            // [profile.dev]
            // opt-level = 0
            // [profile.release]
            // opt-level = 3
            println!("hello world");
        }
    }

    pub mod lecture_2 {
        use crypto::digest::Digest;
        use crypto::sha3::Sha3;
        pub fn run() {
            let mut haser = Sha3::sha3_256();
            haser.input_str("hello word");
            let res = haser.result_str();
            println!("{}", res)
        }
    }

    pub mod lecture_3 {
        pub fn run() {
            //! My Crate
            //! 'my_crate' is a collectionr of utilties to make perfecting certain culculations more convenient
            ///给这个数字加一  代表文档注释
            /// #Example
            /// ```
            /// let five = 5;
            /// ```
            pub fn add_one(x: i32) -> i32 {
                x + 1
            }
        }
    }
}

pub mod box_mod {
    //编译时大小位置，需要确切大小的上下文
    //不被拷贝下转移所有权
    pub mod lecture_1 {
        pub fn run() {
            //b存储在栈上，5存储在堆上
            let b = Box::new(5);
            println!("b = {}", b);
        }
    }

    pub mod lecture_2 {
        // enum List {
        //     Cons(i32, List),
        //     Nil,
        // }
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        pub fn run() {
            use List::Cons;
            use List::Nil;
            // let list = Cons(1, Cons(2, Cons(3, Nil)));
            let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        }
    }

    pub mod lecture_3 {
        pub fn run() {
            let x = 5;
            let y = &x;
            assert_eq!(5, x);
            assert_eq!(5, *y);

            let z = Box::new(x);
            assert_eq!(5, *z);
        }
    }

    pub mod lecture_4 {
        use std::ops::Deref;
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &T {
                &self.0
            }
        }

        fn hello(name: &str) {
            println!("hello, {}", name);
        }

        pub fn run() {
            // let x = 5;
            // let y = MyBox::new(x);
            // assert_eq!(5, x);
            // assert_eq!(5, *y);
            let m = MyBox::new(String::from("RRRRRust"));
            hello(&m);
        }
    }

    pub mod lecture_5 {
        pub fn run() {}
    }
}

pub mod drop_mod {
    //类似析构函数，当值离开作用域的时候执行drop函数
    pub mod lecture_1 {
        struct Dog {
            name: String,
        }

        impl Drop for Dog {
            fn drop(&mut self) {
                println!("{} leave", self.name);
            }
        }
        pub fn run() {
            let a = Dog {
                name: String::from("wangcai"),
            };
            {
                let v = Dog {
                    name: String::from("dahuang"),
                };
                println!("-------------------1111111");
            }
            println!("-------------0000")
        }
    }

    pub mod lecture_2 {
        struct Dog {
            name: String,
        }

        impl Drop for Dog {
            fn drop(&mut self) {
                println!("{} leaves", self.name);
            }
        }

        pub fn run() {
            let a = Dog {
                name: String::from("wangcai"),
            };
            let b = Dog {
                name: String::from("dahuang"),
            };
            drop(b);
        }
    }
}

pub mod rc_pointer_mod {
    pub mod lecture_1 {
        use std::rc::Rc;
        // enum List {
        //     Cons(i32, Box<List>),
        //     Nil,
        // }
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        pub fn run() {
            use List::{Cons, Nil};
            // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
            // let b = Cons(3, Box::new(a));
            // let c = Cons(4, Box::new(a));
            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            let b = Cons(3, a.clone());
        }
    }

    pub mod lecture_2 {
        use std::rc::Rc;

        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        pub fn run() {
            use List::{Cons, Nil};
            let a = Rc::new(Cons(10, Rc::new(Cons(5, Rc::new(Nil)))));
            println!("count agter a a = {}", Rc::strong_count(&a));

            let b = Cons(3, a.clone());
            println!("count afer b a = {}", Rc::strong_count(&a));

            {
                let c = Cons(4, a.clone());
                println!("count after c a = {}", Rc::strong_count(&a));
            }

            println!("count a = {}", Rc::strong_count(&a));
        }
    }
}
