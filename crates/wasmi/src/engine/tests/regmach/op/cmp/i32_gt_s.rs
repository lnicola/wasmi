use super::*;

const WASM_OP: WasmOp = WasmOp::cmp(WasmType::I32, "gt_s");

#[test]
fn same_reg() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from(false),
    }];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::i32_gt_s)
}

#[test]
fn reg_imm16() {
    test_binary_reg_imm16::<i16>(WASM_OP, 100, Instruction::i32_gt_s_imm16)
}

#[test]
fn reg_imm16_rev() {
    test_binary_reg_imm16_rev::<i16>(WASM_OP, 100, swap_ops!(Instruction::i32_lt_s_imm16))
}

#[test]
fn reg_imm() {
    test_binary_reg_imm32(WASM_OP, 100_000, Instruction::i32_gt_s)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm32_rev(WASM_OP, 100_000, Instruction::i32_gt_s)
}

#[test]
fn reg_max() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from(false),
    }];
    test_binary_reg_imm_with(WASM_OP, i32::MAX, expected).run()
}

#[test]
fn min_reg() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from(false),
    }];
    test_binary_reg_imm_rev_with(WASM_OP, i32::MIN, expected).run()
}

#[test]
fn consteval() {
    let lhs = 1_i32;
    let rhs = 2;
    test_binary_consteval(
        WASM_OP,
        lhs,
        rhs,
        [Instruction::ReturnImm32 {
            value: AnyConst32::from(lhs > rhs),
        }],
    )
}
