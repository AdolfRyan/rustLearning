use std::fs::File;
use std::io;
use std::io::ErrorKind;

pub mod chap_9 {
    use super::*;

    pub fn demo9_5() {
        let f = File::open("hello.txt").map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Tried to create file but there is a problem: {:?}", error);
                })
            } else {
                panic!("There was a problem opening the file: {:?}", error);
            }
        });
    }

    pub fn demo9_6() {
      
    }
}
