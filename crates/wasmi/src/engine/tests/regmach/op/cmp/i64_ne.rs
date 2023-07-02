use super::*;

const WASM_OP: WasmOp = WasmOp::cmp(WasmType::I64, "ne");

#[test]
fn same_reg() {
    let expected = [Instruction::ReturnImm32 {
        value: Const32::from_i32(0),
    }];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::i64_ne)
}

#[test]
fn reg_imm16() {
    test_binary_reg_imm16::<i16>(WASM_OP, 100, Instruction::i64_ne_imm16)
}

#[test]
fn reg_imm16_rev() {
    test_binary_reg_imm16_rev::<i16>(WASM_OP, 100, swap_ops!(Instruction::i64_ne_imm16))
}

#[test]
fn reg_imm() {
    test_binary_reg_imm64(WASM_OP, i64::MAX, Instruction::i64_ne_imm)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm64_rev(WASM_OP, i64::MAX, Instruction::i64_ne_imm)
}

#[test]
fn consteval() {
    test_binary_consteval(
        WASM_OP,
        1,
        1,
        [Instruction::ReturnImm32 {
            value: Const32::from_i32(0),
        }],
    );
    test_binary_consteval(
        WASM_OP,
        42,
        5,
        [Instruction::ReturnImm32 {
            value: Const32::from_i32(1),
        }],
    );
}
