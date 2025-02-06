use wasm_encoder::*;

pub fn generate_wasm() -> Vec<u8> {
    let mut module = Module::new();
    let mut func = Function::new(vec![]);
    // Add return value (i32) and single instruction (return constant 42)
    func.instruction(&Instruction::I32Const(42));
    func.instruction(&Instruction::End);
    let func_index = module.functions.add(func);
    module.exports.add("main", ExportKind::Func, func_index);

    let wasm_bytes = module.finish();
    wasm_bytes
}
