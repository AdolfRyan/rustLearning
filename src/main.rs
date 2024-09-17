use world_hello::basic;
use mylib::factory as test;
use mylib::factory::produce_refrigerator::A;
use mylib::factory::produce_washing_machine::B::C;

extern crate crypto;
use crypto::digest::Digest;
use crypto::sha3::Sha3;



fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello world");
    println!("hash: {}", hasher.result_str());


    let a = A::new();
    a.print();
    test::produce_refrigerator::produce_re();
    C::print_c();


    basic::error_mod::run();

    
}
