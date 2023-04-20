mod core;

fn main() {
    let code = "VAR a = 1.4; VAR b = 3; VAR c = 'asd'";
    let tokens = core::lexer::process(code);

    println!("{:?}", tokens);
}
