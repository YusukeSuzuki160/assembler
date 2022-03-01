mod assembler;
use assembler::Parser;
use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = args[0].clone();
    let mut command = Parser::new(&args[0]);
    command.load_file();
    command.write_binary(&filename);
}
