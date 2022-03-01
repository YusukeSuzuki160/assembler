use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
#[derive(Clone, PartialEq, Eq, Debug)]
enum CommandType {
    ACommand,
    CCommand,
    Space,
}

struct Command {
    command: String,
    command_type: CommandType,
}

impl Command {
    fn judge_type(command: String) -> CommandType {
        if command.chars().collect::<Vec<_>>()[0] == ' ' {
            CommandType::Space
        } else if command.chars().collect::<Vec<_>>()[0] == '@' {
            CommandType::ACommand
        } else {
            CommandType::CCommand
        }
    }
    fn new(command_name: String) -> Command {
        let code = command_name.replace(" ", "");
        Command {
            command: code,
            command_type: Command::judge_type(command_name),
        }
    }
    fn symbol_a(&mut self) -> String {
        self.command.remove(0);
        self.command.clone()
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

#[test]
fn test_command() {
    let command1 = Command::new("@100".to_string());
    let command2 = Command::new("D".to_string());
    let command3 = Command::new("D=A".to_string());
    let command4 = Command::new("D=D+A".to_string());
    let command5 = Command::new("0;JMP".to_string());
    assert_eq!(command1.command_type, CommandType::ACommand);
    assert_eq!(command2.command_type, CommandType::CCommand);
    assert_eq!(command3.command_type, CommandType::CCommand);
    assert_eq!(command4.command_type, CommandType::CCommand);
    assert_eq!(command5.command_type, CommandType::CCommand);
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
}

impl<'a> Parser<'a> {
    pub fn new(filename: &'a str) -> Parser {
        Parser {
            commands: Vec::new(),
            now_command: Command {
                command: String::new(),
                command_type: CommandType::None,
            },
            counter: 0,
            file: filename,
        }
    }
    pub fn load_file(&mut self) {
        let v = fs::read_to_string(self.file).unwrap();
        let mut flags = 0;
        'outer: for lines in v.lines() {
            for c in lines.to_string().as_str().chars() {
                if c == '/' {
                    flags += 1;
                }
                if flags == 2 {
                    flags = 0;
                    continue 'outer;
                }
            }
            if lines.to_string().as_str() == "" {
                continue 'outer;
            }
            self.commands.push(lines.to_string());
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
            self.now_command = Command::new(self.commands[self.counter].clone());
            self.counter += 1;
            true
        } else {
            false
        }
    }
    fn command_type(&self) -> CommandType {
        self.now_command.command_type.clone()
    }
    fn symbol(&mut self) -> String {
        self.now_command.symbol_a().clone()
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
    fn write_code(&mut self) -> String {
        if self.command_type() == CommandType::ACommand {
            "0".to_string() + &(format!("{:015b}", self.symbol().parse::<i32>().unwrap())) + "\n"
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
    pub fn write_binary(&mut self, filename: &str) {
        let outfile = filename.replace(".asm", ".hack");
        let mut write_str: String = String::new();
        while self.advance() {
            write_str += &self.write_code();
        }
        let mut f = BufWriter::new(File::create(outfile).unwrap());
        write!(f, "{}", write_str).unwrap();
    }
}
#[test]
fn test_parser() {
    let mut command = Parser::new("add/Add.asm");
    command.load_file();
    assert_eq!(
        command.commands,
        vec![
            "@2".to_string(),
            "D=A".to_string(),
            "@3".to_string(),
            "D=D+A".to_string(),
            "@0".to_string(),
            "M=D".to_string(),
        ]
    );
    command.advance();
    assert_eq!(command.write_code(), "0000000000000010\n".to_string());
    command.advance();
    assert_eq!(command.write_code(), "1110110000010000\n".to_string());
}
