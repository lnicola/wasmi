use super::*;

const WASM_OP: WasmOp = WasmOp::binary(WasmType::I64, "and");

#[test]
fn same_reg() {
    let expected = [Instruction::return_reg(0)];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::i64_and)
}

#[test]
fn reg_imm16() {
    test_binary_reg_imm16::<i16>(WASM_OP, 100, Instruction::i64_and_imm16)
}

#[test]
fn reg_imm16_rev() {
    test_binary_reg_imm16_rev::<i16>(WASM_OP, 100, swap_ops!(Instruction::i64_and_imm16))
}

#[test]
fn reg_imm() {
    test_binary_reg_imm32(WASM_OP, i64::MAX, Instruction::i64_and)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm32_rev_commutative(WASM_OP, i64::MAX, Instruction::i64_and)
}

#[test]
fn reg_zero() {
    let expected = [Instruction::return_i64imm32(0)];
    test_binary_reg_imm_with(WASM_OP, 0i32, expected).run()
}

#[test]
fn reg_zero_rev() {
    let expected = [Instruction::return_i64imm32(0)];
    test_binary_reg_imm_rev_with(WASM_OP, 0i32, expected).run()
}

#[test]
fn reg_ones() {
    let expected = [Instruction::return_reg(0)];
    test_binary_reg_imm_with(WASM_OP, -1_i32, expected).run()
}

#[test]
fn reg_ones_rev() {
    let expected = [Instruction::return_reg(0)];
    test_binary_reg_imm_rev_with(WASM_OP, -1_i32, expected).run()
}

#[test]
fn consteval() {
    let lhs = 10;
    let rhs = 20;
    test_binary_consteval(WASM_OP, lhs, rhs, [Instruction::return_i64imm32(lhs & rhs)])
}
