extern crate tokesies;

use std::vec::Vec;
use std::mem;
use tokesies::{filters, FilteredTokenizer, Token};

pub enum ShaderType {
    Vertex,
    Fragment
}

type Tokens<'a> = Vec<Vec<&'a str>>;
struct GlslFilter;


fn tokenize(code: &str) -> Tokens {
    let mut code_tokens = Vec::new();
    for line in code.lines() {
        code_tokens.push(line.split(" ").collect());
    }
    code_tokens
}

fn get_str_token<'a>(tokens: &'a Tokens, line: usize, column: usize) -> &'a str {
    tokens[line][column]
}

fn get_version(tokens: &Tokens) -> u32 {
    let mut version = 0;

    // First, try with #version
    if get_str_token(tokens, 0, 0) == "#version" {
        version = get_str_token(tokens, 0, 1).parse::<u32>().unwrap();
    }

    version
}

fn transpile120(mut tokens: Tokens, shader_type: ShaderType) -> String {
    // remove version if exists
    if get_str_token(&tokens, 0, 0) == "#version" {
        tokens.remove(0);
    }

    for line in tokens.iter() {
        for token in line.iter() {
            token.replace("attribute", "out");
        }
    }

    let mut result = String::from("#version 450\n");
    for line in tokens.iter() {
        for token in line.iter() {
            result.push_str(token);
            result.push(' ');
        }
        result.push('\n');
    }

    result
}

fn transpile100(tokens: Tokens, shader_type: ShaderType) -> String {
    //TODO: remove precision qualifier
    transpile120(tokens, shader_type)
}

pub fn transpile(code: &str, shader_type: ShaderType) {
    let code_tokens = tokenize(code);
    let version = get_version(&code_tokens);

    let result = match version {
        100 => transpile100(code_tokens, shader_type),
        120 => transpile120(code_tokens, shader_type),
        _ => panic!("Can't determine shader version")
    };

    println!("{:?}", result);
}
