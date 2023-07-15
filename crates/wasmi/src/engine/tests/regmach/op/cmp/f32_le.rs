use super::*;

const WASM_OP: WasmOp = WasmOp::cmp(WasmType::F32, "le");

#[test]
fn same_reg() {
    let expected = [Instruction::ReturnImm32 {
        value: Const32::from(true),
    }];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::f32_le)
}

#[test]
fn reg_imm() {
    test_binary_reg_imm32(WASM_OP, 1.0_f32, Instruction::f32_le)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm32_rev(WASM_OP, 1.0_f32, Instruction::f32_le)
}

#[test]
fn reg_nan() {
    test_binary_reg_imm_with(WASM_OP, f32::NAN, [Instruction::return_imm32(false)]).run()
}

#[test]
fn nan_reg() {
    test_binary_reg_imm_rev_with(WASM_OP, f32::NAN, [Instruction::return_imm32(false)]).run()
}

#[test]
fn consteval() {
    fn test_with(lhs: f32, rhs: f32, result: bool) {
        test_binary_consteval(
            WASM_OP,
            lhs,
            rhs,
            [Instruction::ReturnImm32 {
                value: Const32::from(result),
            }],
        );
    }
    test_with(1.0, 1.0, true);
    test_with(1.0, 2.0, true);
    test_with(2.0, 1.0, false);
}
