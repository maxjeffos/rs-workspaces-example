
use rand;

pub fn get_msg() -> String {
    String::from("hello")
}

pub fn get_random_u32() -> u32 {
    rand::random()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
