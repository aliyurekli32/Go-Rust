fn main() {
    println!("Hello, world!");

}
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";
const SPANISH_HELLO_PREFIX: &str = "Hola, ";
const FRENCH_HELLO_PREFIX: &str = "Bonjour, ";
const SPANISH: &str = "spanish";
const FRENCH: &str = "french";

pub fn hello( name: String, language: String ) -> String {
    if name == "" {
        return "Hello, World".to_string();
    }
    let prefix = match language.as_str() {
        SPANISH =>  &SPANISH_HELLO_PREFIX,
        FRENCH =>  &FRENCH_HELLO_PREFIX,
        _ => ENGLISH_HELLO_PREFIX,
    };
    return prefix.to_string() + &name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello(String::from("Chris"),String::from(""));
        assert_eq!(result, "Hello, Chris");
    }

    #[test]
    fn it_works_default() {
        let result = hello(String::from(""),String::from(""));
        assert_eq!(result, "Hello, World");
    }

    #[test]
    fn it_works_spanish() {
        let result = hello(String::from("Elodie"),String::from("spanish"));
        assert_eq!(result, "Hola, Elodie");
    }

    #[test]
    fn it_works_french() {
        let result = hello(String::from("Elodie"),String::from("french"));
        assert_eq!(result, "Bonjour, Elodie");
    }

}


