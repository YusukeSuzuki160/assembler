mod assembler;
use assembler::{Parser, SymbolTable};
use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filename = args[0].clone();
    let mut command = Parser::new(&args[0]);
    let mut table = SymbolTable::new();
    command.load_file();
    command.make_symbol_table(&mut table);
    command.reset();
    command.write_binary(&filename, &mut table);
}
