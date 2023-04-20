mod core;

fn main() {
    let code = "VAR a = 1; VAR b = 3;";
    let tokens = core::lexer::process(code);

    println!("{:?}", tokens);
}
