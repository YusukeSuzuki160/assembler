use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct SymbolTable {
    table: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut new_table = HashMap::new();
        new_table.insert("SP".to_string(), 0);
        new_table.insert("LCL".to_string(), 1);
        new_table.insert("ARG".to_string(), 2);
        new_table.insert("THIS".to_string(), 3);
        new_table.insert("THAT".to_string(), 4);
        new_table.insert("R0".to_string(), 0);
        new_table.insert("R1".to_string(), 1);
        new_table.insert("R2".to_string(), 2);
        new_table.insert("R3".to_string(), 3);
        new_table.insert("R4".to_string(), 4);
        new_table.insert("R5".to_string(), 5);
        new_table.insert("R6".to_string(), 6);
        new_table.insert("R7".to_string(), 7);
        new_table.insert("R8".to_string(), 8);
        new_table.insert("R9".to_string(), 9);
        new_table.insert("R10".to_string(), 10);
        new_table.insert("R11".to_string(), 11);
        new_table.insert("R12".to_string(), 12);
        new_table.insert("R13".to_string(), 13);
        new_table.insert("R14".to_string(), 14);
        new_table.insert("R15".to_string(), 15);
        new_table.insert("SCREEN".to_string(), 16384);
        new_table.insert("KBD".to_string(), 24576);

        SymbolTable { table: new_table }
    }
    pub fn add_entry(&mut self, symbol: String, address: usize) {
        self.table.insert(symbol, address);
    }
    pub fn contains(&self, symbol: &str) -> bool {
        match self.table.get(symbol) {
            Some(_) => true,
            _ => false,
        }
    }
    pub fn get_adress(&self, symbol: &str) -> usize {
        *self.table.get(symbol).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
    Space,
    None,
}

struct Command {
    command: String,
    command_type: CommandType,
    counter: usize,
}

impl Command {
    fn judge_type(command: String) -> CommandType {
        if command == "" {
            CommandType::None
        } else if command.chars().collect::<Vec<_>>()[0] == ' ' {
            CommandType::Space
        } else if command.chars().collect::<Vec<_>>()[0] == '@' {
            CommandType::ACommand
        } else if command.chars().collect::<Vec<_>>()[0] == '(' {
            CommandType::LCommand
        } else {
            CommandType::CCommand
        }
    }
    fn new() -> Command {
        Command {
            command: String::new(),
            command_type: CommandType::None,
            counter: 16,
        }
    }
    fn load_command(&mut self, command_name: String) {
        let code = command_name.replace(" ", "");
        self.command = code;
        self.command_type = Command::judge_type(command_name);
    }
    fn symbol_a(&mut self, table: &mut SymbolTable) -> String {
        self.command.remove(0);
        match self.command.parse::<usize>() {
            Ok(_) => self.command.clone(),
            _ => {
                if table.contains(&self.command) {
                    table.get_adress(&self.command).to_string()
                } else {
                    table.add_entry(self.command.clone(), self.counter);
                    let ans = self.counter;
                    self.counter += 1;
                    ans.to_string()
                }
            }
        }
    }
    fn symbol_l(&mut self) -> String {
        self.command.replace("(", "").replace(")", "").clone()
    }
    fn parse_c(&mut self) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let v: Vec<&str>;
        if let Some(_) = self.command.find('=') {
            v = self.command.split('=').collect();
            ans.push(v[0].clone().to_string());
        } else {
            if let Some(_) = self.command.find(';') {
                v = self.command.split(';').collect();
                ans.push("No Code.".to_string());
                ans.push(v[0].clone().to_string());
                ans.push(v[1].clone().to_string());
            }
            ans.push("No Code.".to_string());
            ans.push("No Code.".to_string());
            ans.push("No Code.".to_string());
            return ans;
        }
        if let Some(_) = v[1].find(';') {
            let w: Vec<&str> = v[1].split(';').collect();
            ans.push(w[0].clone().to_string());
            ans.push(w[1].clone().to_string());
            ans
        } else {
            ans.push(v[1].clone().to_string());
            ans.push("No Code.".to_string());
            ans
        }
    }
}
pub struct Code {}

impl Code {
    fn dest(mnemonic: &str) -> &str {
        match mnemonic {
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => "000",
        }
    }
    fn comp(mnemonic: &str) -> &str {
        match mnemonic {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => "1101010",
        }
    }
    fn jump(mnemonic: &str) -> &str {
        match mnemonic {
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => "000",
        }
    }
}

pub struct Parser<'elt> {
    commands: Vec<String>,
    now_command: Command,
    counter: usize,
    file: &'elt str,
    ram_number: usize,
}

impl<'a> Parser<'a> {
    pub fn new(filename: &'a str) -> Parser {
        Parser {
            commands: Vec::new(),
            now_command: Command::new(),
            counter: 0,
            file: filename,
            ram_number: 0,
        }
    }
    pub fn load_file(&mut self) {
        let v = fs::read_to_string(self.file).unwrap();
        for lines in v.lines() {
            self.commands.push(
                lines.to_string().split('/').collect::<Vec<&str>>()[0]
                    .replace(" ", "")
                    .to_string(),
            );
        }
    }
    fn has_more_commands(&self) -> bool {
        if self.counter == self.commands.len() {
            false
        } else {
            true
        }
    }
    fn advance(&mut self) -> bool {
        if self.has_more_commands() {
            self.now_command
                .load_command(self.commands[self.counter].clone());
            self.counter += 1;
            true
        } else {
            false
        }
    }
    fn command_type(&self) -> CommandType {
        self.now_command.command_type.clone()
    }
    fn symbol(&mut self, table: &mut SymbolTable) -> String {
        if self.command_type() == CommandType::ACommand {
            self.now_command.symbol_a(table).clone()
        } else if self.command_type() == CommandType::LCommand {
            self.now_command.symbol_l().clone()
        } else {
            "".to_string()
        }
    }
    fn dest(&mut self) -> String {
        self.now_command.parse_c()[0].clone()
    }
    fn comp(&mut self) -> String {
        self.now_command.parse_c()[1].clone()
    }
    fn jump(&mut self) -> String {
        self.now_command.parse_c()[2].clone()
    }
    pub fn make_symbol_table(&mut self, table: &mut SymbolTable) {
        while self.advance() {
            if self.now_command.command_type == CommandType::ACommand {
                self.ram_number += 1;
                continue;
            } else if self.now_command.command_type == CommandType::CCommand {
                self.ram_number += 1;
                continue;
            } else if self.now_command.command_type != CommandType::LCommand {
                continue;
            } else {
            }
            {
                let symbol = &mut self.now_command.command;
                symbol.pop();
                symbol.remove(0);
            }
            let symbol = self.now_command.command.clone();
            table.table.insert(symbol, self.ram_number);
        }
    }
    pub fn reset(&mut self) {
        self.now_command.command = String::new();
        self.now_command.command_type = CommandType::None;
        self.counter = 0;
    }
    fn write_code(&mut self, table: &mut SymbolTable) -> String {
        if self.command_type() == CommandType::ACommand {
            "0".to_string()
                + &(format!("{:015b}", self.symbol(table).parse::<i32>().unwrap()))
                + "\n"
        } else if self.command_type() == CommandType::CCommand {
            "111".to_string()
                + Code::comp(&self.comp())
                + Code::dest(&self.dest())
                + Code::jump(&self.jump())
                + "\n"
        } else {
            "".to_string()
        }
    }
    pub fn write_binary(&mut self, filename: &str, table: &mut SymbolTable) {
        let outfile = filename.replace(".asm", ".hack");
        let mut write_str: String = String::new();
        while self.advance() {
            write_str += &self.write_code(table);
        }
        let mut f = BufWriter::new(File::create(outfile).unwrap());
        write!(f, "{}", write_str).unwrap();
    }
}

#[test]

fn table_test() {
    let mut command = Parser::new("max/Max.asm");
    let mut table = SymbolTable::new();
    command.load_file();
    command.make_symbol_table(&mut table);
    for (k, v) in &table.table {
        println!("{}: {}", k, v);
    }
}
