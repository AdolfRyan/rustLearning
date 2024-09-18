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

  pub mod owner_ship {
      fn takes_ownership(s: String) {
          println!("s = {}", s);
      }

      fn makes_copy(i: i32) {
          println!("i = {}", i);
      }

      pub fn run() {
          //堆 和 栈 编译时数据的类型大小是固定的，就是分配在栈上
          {
              let x = 5;
              let y = x;
              println!("x = {}, y = {}", x, y);
          }
          //println!("y = {}", y);
          let s1 = String::from("hello");
          println!("s1 = {}", s1); //String 类型离开作用域会调用drop方法释放内存
          let s2 = s1;
          // println!("s1 = {}", s1); //s1已经被移动到s2
          println!("s2 = {}", s2);

          let i1 = 5;
          makes_copy(i1);
          println!("i1 = {}", i1);
          let s = String::from("hello");
          takes_ownership(s);
          // println!("s = {}", s); //s已经被移动到takes_ownership
      }
  }

  pub mod reference {
      fn gives_ownership() -> String {
          let s = String::from("hello");
          s
      }

      fn takes_and_gives_back(s: String) -> String {
          s
      }

      fn calculate_length(s: &String) -> usize {
          s.len()
      }

      fn modify_s(s: &mut String) {
          s.push_str(" world");
      }

      //引用不拥有这个值，s离开作用域被回收，返回的引用指向的是一个无效的值
      // fn dangle()->&String{
      //     let s = String::from("hello");
      //     &s
      // }

      pub fn run() {
          let s1 = gives_ownership();
          println!("s1 = {}", s1);
          let s2 = String::from("hello");
          let s3 = takes_and_gives_back(s2);
          // println!("s2 = {}", s2);//s2已经被移动到takes_and_gives_back
          println!("s3 = {}", s3);
          let s4 = String::from("hello");
          let len = calculate_length(&s4);
          println!("s4 = {}, len = {}", s4, len);

          let mut s5 = String::from("hello");
          let ms = &mut s5;
          modify_s(ms);
          // println!("s5 = {}", s5);
          println!("ms = {}", ms);
      }
  }

  pub mod slice {
      //slice就是String中一部分值的引用
      pub fn run() {
          let s = String::from("hello world");
          let sl = &s[0..5]; //等价于&s[..5] = &s[0..=4]
          println!("sl = {}", sl);
          let sl = &s[6..11]; //等价于&s[6..]
          println!("sl = {}", sl);
          let a = [1, 2, 3, 4, 5];
          let sl = &a[1..3];
          println!("sl = {:?}", sl);
      }
  }

  pub mod struct_mod {
      #[derive(Debug)]
      struct User {
          name: String,
          acount: String,
          nonce: u32,
          active: bool,
      }

      struct Point(i32, i32);

      pub fn run() {
          let mut user = User {
              name: String::from("zhangsan"),
              acount: String::from("11111"),
              nonce: 10000,
              active: true,
          };
          let user1 = User { ..user };
          user.acount = String::from("22222");
          // println!(
          //     "user = {} {} {} {}",
          //     user.name, user.acount, user.nonce, user.active
          // );
          println!("user = {:#?}", user1);

          let point = Point(1, 2);
          println!("point = {} {}", point.0, point.1);
      }
  }

  pub mod funct {
      #[derive(Debug)]
      struct Dog {
          name: String,
          weight: f32,
          height: f32,
      }

      impl Dog {
          fn get_name(&self) -> &str {
              &self.name
          }

          fn get_weight(&self) -> f32 {
              self.weight
          }

          fn get_height(&self) -> f32 {
              self.height
          }
      }
      pub fn run() {
          let dog = Dog {
              name: String::from("dog"),
              weight: 10.0,
              height: 20.0,
          };

          println!("dog = {:#?}", dog);
          println!(
              "name = {} {} {}",
              dog.get_name(),
              dog.get_weight(),
              dog.get_height()
          );
      }
  }

  pub mod enumeration {
      enum Message {
          Quit,
          Move(i32, i32),
          Write(String),
          Change(i32, i32, i32),
      }

      impl Message {
          fn print(&self) {
              match *self {
                  Message::Quit => println!("Quit"),
                  Message::Move(x, y) => println!("Move x = {}, y = {}", x, y),
                  Message::Change(x, y, z) => println!("Change x = {}, y = {}, z = {}", x, y, z),
                  _ => println!("Write"),
              }
          }
      }

      pub fn run() {
          let m = Message::Move(1, 2);
          m.print();
          let q = Message::Quit;
          q.print();
          let w = Message::Write(String::from("hello"));
          w.print();
          let c = Message::Change(1, 2, 3);
          c.print();
      }
  }

  pub mod option {
      fn plus_one(x: Option<i32>) -> Option<i32> {
          match x {
              None => None,
              Some(i) => Some(i + 1),
          }
      }
      pub fn run() {
          let x: i32 = 5;
          let y: Option<i32> = Some(5);
          let i = match y {
              Some(i) => {
                  println!("i = {}", i);
                  i
              }
              None => {
                  println!("None");
                  0
              }
          };
          println!("y + x = {}", x + i);

          let five = Some(5);
          if let Some(result) = plus_one(five) {
              println!("result = {}", result);
          } else {
              println!("None");
          }
      }
  }

  pub mod vector {
      enum ConText {
          Text(String),
          Float(f32),
          Int(i32),
      }
      pub fn run() {
          let c: Vec<ConText> = vec![
              ConText::Text(String::from("hello")),
              ConText::Float(1.0),
              ConText::Int(1),
          ];
          for i in &c {
              match i {
                  ConText::Text(s) => println!("Text = {}", s),
                  ConText::Float(f) => println!("Float = {}", f),
                  ConText::Int(i) => println!("Int = {}", i),
              }
          }
          let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
          v.push(6);

          let first = &v[0];
          let second = v.get(1);
          match second {
              Some(i) => println!("second = {}", i),
              None => println!("None"),
          }
          println!("first = {}", *first);
          for i in &v {
              println!("{}", i);
          }

          for i in &mut v {
              *i += 50;
          }

          for i in v.iter() {
              println!("{}", i);
          }
      }
  }

  pub mod string {
      pub fn run() {
          let s = String::from("hello");
          let mut s2 = String::from("world");
          s2.push_str(" world");
          let s3 = s + &s2;
          println!("s3 = {}", s3);

          let s1 = String::from("tic");
          let s2 = String::from("tac");
          let s3 = String::from("toe");
          let s = format!("{}-{}-{}", s1, s2, s3);
          println!("s = {}", s);

          let hello = "你好";
          println!("你好 length = {}", hello.len());
          for c in hello.chars() {
              println!("{}", c);
          }
          for c in hello.bytes() {
              println!("{}", c);
          }
      }
  }

  pub mod hashmap {
      use std::{collections::HashMap, hash::Hash};
      pub fn run() {
          let mut map: HashMap<String, i32> = HashMap::new();
          map.insert(String::from("Blue"), 1);
          map.insert(String::from("Yellow"), 2);

          let keys = vec![String::from("Blueeee"), String::from("Yellowwww")];
          let values = vec![1, 2];
          let scroes: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

          if let Some(score) = scroes.get(&String::from("Blueeee")) {
              println!("score = {}", score);
          } else {
              println!("None");
          }

          for (key, value) in &scroes {
              println!("{}: {}", key, value);
          }

          let mut ss = HashMap::new();
          ss.insert(String::from("Blue"), 10);
          ss.entry(String::from("Yellow")).or_insert(50);
          ss.entry(String::from("Blue")).or_insert(50);
          println!("{:?}", ss);

          let text = "hello world wonderful world";
          let mut map = HashMap::new();
          for word in text.split_whitespace() {
              let count = map.entry(word).or_insert(0);
              *count += 1;
          }
      }
  }

  pub mod error_mod {
      //示例 代码原型测试用panic!\unwrap!\expect!
      //实际项目中使用Result<T,E>类型
      use std::fs::File;
      use std::io;
      use std::io::Read;

      // fn read_username_from_file() -> Result<String, io::Error> {
      //     let f = File::open("hello.txt");
      //     let mut f = match f {
      //         Ok(file) => file,
      //         Err(error) => return Err(error),
      //     };

      //     let mut s = String::new();
      //     match f.read_to_string(&mut s) {
      //         Ok(_) => Ok(s),
      //         Err(error) => Err(error),
      //     }
      // }

      // fn read_username_from_file() -> Result<String, io::Error> {
      //     let mut f = File::open("hello.txt")?;

      //     let mut s = String::new();
      //     f.read_to_string(&mut s)?;
      //     Ok(s)
      // }

      fn read_username_from_file() -> Result<String, io::Error> {
          let mut s = String::new();
          File::open("hello.txt")?.read_to_string(&mut s)?;
          Ok(s)
      }

      pub fn run() {
          // let f = File::open("hello.txt").unwrap();
          // let f = File::open("hello.txt").expect("Failed to open hello.txt");
          let r = read_username_from_file();
          match r {
              Ok(s) => println!("s = {}", s),
              Err(e) => println!("err = {:?}", e),
          }
      }
  }

  pub mod test_mod {
      
  }
}
