pub mod produce_refrigerator {
    pub struct A {
        pub a: i32,
        name: String,
    }

    impl A {
        pub fn new() -> A {
            A {
                a: 1,
                name: String::from("A"),
            }
        }
        pub fn print(&self) {
            println!("a: {}, name: {}", self.a, self.name);
        }
    }

    pub fn produce_re() {
        println!("Produce refrigerator");
    }
}

pub mod produce_washing_machine {
    pub fn produce_wa() {
        println!("Produce washing machine");
    }

    pub mod B {
      pub fn print_b() {
        println!("B");
      }

      pub mod C {
        pub fn print_c() {
          println!("C");
          super::print_b();
        }
      }
    }
}
