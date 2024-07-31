use logos::Logos;
use serde_vdf::lexer::{parse, Token};

fn main() {
    const VDF: &str = include_str!("D:/Steam/steamapps/libraryfolders.vdf");
    let mut lexer = Token::lexer(VDF);

    match parse(&mut lexer) {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
