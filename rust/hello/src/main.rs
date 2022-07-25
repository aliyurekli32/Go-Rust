fn main() {
    println!("Hello, world!");

}

pub fn hello( name: String ) -> String {
    "Hello, ".to_string() + &name.to_string()
}

//write test code for main above with rust

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello(String::from("Chris"));
        assert_eq!(result, "Hello, Chris");
    }
}


