use super::*;
use wasmi_core::TrapCode;

const WASM_OP: WasmOp = WasmOp::binary(WasmType::I64, "rem_s");

#[test]
fn same_reg() {
    let expected = [Instruction::return_i64imm32(0)];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::i64_rem_s)
}

#[test]
fn reg_imm16() {
    test_binary_reg_imm16::<i16>(WASM_OP, 100, Instruction::i64_rem_s_imm16)
}

#[test]
fn reg_imm16_rev() {
    test_binary_reg_imm16_rev::<i16>(WASM_OP, 100, Instruction::i64_rem_s_imm16_rev)
}

#[test]
fn reg_imm() {
    test_binary_reg_imm64(WASM_OP, i64::MAX, Instruction::i64_rem_s_imm)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm64_rev(WASM_OP, i64::MAX, Instruction::i64_rem_s_imm)
}

#[test]
fn reg_zero() {
    let expected = [Instruction::Trap(TrapCode::IntegerDivisionByZero)];
    test_binary_reg_imm_with(WASM_OP, 0_i64, expected).run()
}

#[test]
fn reg_one() {
    let expected = [Instruction::return_i64imm32(0)];
    test_binary_reg_imm_with(WASM_OP, 1_i64, expected).run()
}

#[test]
fn reg_minus_one() {
    let expected = [Instruction::return_i64imm32(0)];
    test_binary_reg_imm_with(WASM_OP, -1_i64, expected).run()
}

#[test]
fn consteval() {
    let lhs = -13;
    let rhs = 5;
    test_binary_consteval(WASM_OP, lhs, rhs, [Instruction::return_i64imm32(lhs % rhs)])
}

#[test]
fn consteval_2() {
    let lhs = i64::MIN;
    let rhs = -1;
    test_binary_consteval(WASM_OP, lhs, rhs, [Instruction::return_i64imm32(0)])
}

#[test]
fn consteval_div_by_zero() {
    let lhs = -4;
    let rhs = 0;
    test_binary_consteval(
        WASM_OP,
        lhs,
        rhs,
        [Instruction::Trap(TrapCode::IntegerDivisionByZero)],
    )
}
