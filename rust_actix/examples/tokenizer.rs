use tokenizers::tokenizer::Tokenizer;
use std::path::PathBuf;

fn newtok(model_dir: &str) -> () {
    let mut path = PathBuf::from(model_dir);
    path.push("tokenizer.json");
    let tokenizer = Tokenizer::from_file(path).unwrap();
    println!("{:?}", tokenizer);
}

fn main() {
    newtok("../../tinybert-imdb");
}