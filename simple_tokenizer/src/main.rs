use tokenizer::{Token, Tokenizer};

mod tokenizer;

fn main() {
    let expr1 = " 1 + 2 * 3 - (4 / 2) + 6 - 3  ";

    let tokenizer = Tokenizer::new(expr1);

    for x in tokenizer {
        println!("{:?}", x);
    }
}
