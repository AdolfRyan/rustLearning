pub mod basic {
    use super::*;

    pub mod variables {
        pub fn run() {
            //不可用
            let mut b: i32 = 2;
            println!("b = {}", b);
            b = 3;
            println!("b = {}", b);
            //隐藏性
            let b: f32 = 1.2;
            println!("b = {}", b);
            //常量
            const MAX_POINTS: u32 = 100000;
            println!("MAX_POINTS = {}", MAX_POINTS);
        }
    }

    pub mod types {
        fn show(arr: [u32; 6]) {
            for i in &arr {
                println!("{}", i);
            }
        }
        pub fn run() {
            println!("-----------------");
            println!("This it mod types");
            //bool
            let is_true: bool = true;
            let is_false: bool = false;
            println!("is_true = {}", is_true);
            println!("is_false = {}", is_false);
            //char 4字节32位 c中1字节8位
            let c: char = 'a';
            println!("char = {}", c);
            // i8 i16 i32 i64 isize u8 u16 u32 u64 f32 f64
            let c: i8 = 111;
            println!("int = {}", c);
            //自适应类型 isize, usize
            println!("max = {}", std::u64::MAX);
            //数组
            let arr: [u32; 6] = [1, 2, 3, 4, 5, 6];
            show(arr);
            //元组
            let tuple: (i8, i64, f32) = (1, 2, 3.0);
            let (a, b, c) = tuple;
            println!("tuple = {} {} {}", a, b, c);
        }
    }

    pub mod function {
        pub fn other_function() {
            println!("This is other function");
        }
        pub fn other_function1(a: i32, b: i32) {
            println!("a = {}, b = {}", a, b);
        }
        pub fn other_function2(a: i32, b: i32) -> i32 {
            a + b
        }
    }

    pub mod control_flow {
        pub fn run() {
            //if else
            let y = 1;
            if y == 1 {
                println!("y = 1");
            } else {
                println!("y != 1");
            }
            //if let
            let condition = true;
            let x = if condition { 5 } else { 6 };
            println!("x = {}", x);
            //loop 
            let mut counter = 0;
            counter = loop {
                println!("counter = {}", counter);
                counter += 1;
                if counter == 10 {
                    break counter * 2;
                }
            };
            println!("counter = {}", counter);
            //while
            //for
            let arr = [1, 2, 3, 4, 5];
            for i in arr.iter() {
                println!("{}", i);
            }
        }
    }

    pub mod ownership {
        pub fn run(){

        }
    }


    
}
