use tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    let expr1 = " 1 + 2 * 3 - (4 / 2) + 6 - 3  ";

    let tokenizer = Tokenizer::new(expr1);

    for x in tokenizer {
        println!("{:?}", x);
    }

    println!("--------------------------------");

    let expr2 = "100 - (30 + 5) / 5 - 2 * 10";
    let tokenizer2 = Tokenizer::new(expr2);

    tokenizer2.into_iter().for_each( |e| {
        println!("{:?}", e);
    });
}
