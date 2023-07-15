use super::*;

const WASM_OP: WasmOp = WasmOp::cmp(WasmType::F64, "ne");

#[test]
fn same_reg() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from(false),
    }];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::f64_ne)
}

#[test]
fn reg_imm() {
    test_binary_reg_imm32(WASM_OP, 1.0_f64, Instruction::f64_ne)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm32_rev_commutative(WASM_OP, 1.0_f64, Instruction::f64_ne)
}

#[test]
fn reg_nan() {
    test_binary_reg_imm_with(WASM_OP, f64::NAN, [Instruction::return_imm32(true)]).run()
}

#[test]
fn nan_reg() {
    test_binary_reg_imm_rev_with(WASM_OP, f64::NAN, [Instruction::return_imm32(true)]).run()
}

#[test]
fn consteval() {
    test_binary_consteval(
        WASM_OP,
        1.0,
        1.0,
        [Instruction::ReturnImm32 {
            value: AnyConst32::from(false),
        }],
    );
    test_binary_consteval(
        WASM_OP,
        0.0,
        1.0,
        [Instruction::ReturnImm32 {
            value: AnyConst32::from(true),
        }],
    );
}
