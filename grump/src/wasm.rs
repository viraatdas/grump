use crate::parser::Expr;
use wasm_encoder::*;

pub fn generate_wasm(ast: &Expr) -> Vec<u8> {
    let mut module = Module::new();
    let mut types = TypeSection::new();
    let func_type_index = types.len();
    types.function(vec![], vec![ValType::I32]); // main() -> i32

    let mut functions = FunctionSection::new();
    functions.function(func_type_index);

    let mut code = CodeSection::new();
    let mut func = Function::new(vec![]);

    // Generate code based on AST
    generate_code(ast, &mut func);

    func.instruction(&Instruction::End);
    code.function(&func);

    let mut exports = ExportSection::new();
    exports.export("main", ExportKind::Func, 0); // Ensure `main` is correctly exported

    module.section(&types);
    module.section(&functions);
    module.section(&exports);
    module.section(&code);

    module.finish()
}

fn generate_code(ast: &Expr, func: &mut Function) {
    match ast {
        Expr::Number(n) => {
            func.instruction(&Instruction::I32Const(*n as i32));
        }
        Expr::List(list) => {
            if let Some(Expr::Symbol(op)) = list.first() {
                match op.as_str() {
                    "+" => {
                        generate_code(&list[1], func);
                        generate_code(&list[2], func);
                        func.instruction(&Instruction::I32Add);
                    }
                    "-" => {
                        generate_code(&list[1], func);
                        generate_code(&list[2], func);
                        func.instruction(&Instruction::I32Sub);
                    }
                    _ => panic!("Unknown operator: {}", op),
                }
            }
        }
        _ => panic!("Unsupported AST node"),
    }
}
