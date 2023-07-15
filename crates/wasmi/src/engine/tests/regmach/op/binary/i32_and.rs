use super::*;

const WASM_OP: WasmOp = WasmOp::binary(WasmType::I32, "and");

#[test]
fn same_reg() {
    let expected = [Instruction::return_reg(0)];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::i32_and)
}

#[test]
fn reg_imm16() {
    test_binary_reg_imm16::<i32>(WASM_OP, 100, Instruction::i32_and_imm16)
}

#[test]
fn reg_imm16_rev() {
    test_binary_reg_imm16_rev::<i32>(WASM_OP, 100, swap_ops!(Instruction::i32_and_imm16))
}

#[test]
fn reg_imm() {
    test_binary_reg_imm32(WASM_OP, i32::MAX, Instruction::i32_and)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm32_rev_commutative(WASM_OP, i32::MAX, Instruction::i32_and)
}

#[test]
fn reg_zero() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from_i32(0),
    }];
    test_binary_reg_imm_with(WASM_OP, 0i32, expected).run()
}

#[test]
fn reg_zero_rev() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from_i32(0),
    }];
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
    test_binary_consteval(
        WASM_OP,
        lhs,
        rhs,
        [Instruction::ReturnImm32 {
            value: AnyConst32::from_i32(lhs & rhs),
        }],
    )
}
