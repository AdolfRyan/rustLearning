pub mod factory;
pub mod animals;
pub mod basic;
pub mod pro;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn use_cat() {
        // animals::Cat::hello();
        assert_eq!(animals::Cat::is_cat(), true);
    }

    #[test]
    fn use_dog() {
        // animals::Dog::hello();
        assert_eq!(animals::Dog::is_dog(), true);
    }
}
