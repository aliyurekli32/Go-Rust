fn main() {
    println!("Hello, world!");

}

pub fn hello() -> String {
    "Hello, world".to_string()
}

//write test code for main above with rust

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello();
        assert_eq!(result, "Hello, world");
    }
}


