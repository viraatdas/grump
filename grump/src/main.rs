mod lexer;
mod parser;
mod token;
mod wasm;

fn main() {
    let input = "(+ 10 20)";
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("Tokens: {:?}", tokens);

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    println!("Parsed AST: {:?}", ast);

    // Compile AST to WebAssembly
    let wasm_code = wasm::generate_wasm(&ast);
    std::fs::write("output.wasm", wasm_code).expect("Failed to write Wasm file");

    println!("Compiled to WebAssembly: output.wasm");
}
