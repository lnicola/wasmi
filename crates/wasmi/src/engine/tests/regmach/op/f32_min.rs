use super::*;

const WASM_OP: WasmOp = WasmOp::binary(WasmType::F32, "min");

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::f32_min)
}

#[test]
fn reg_imm() {
    test_binary_reg_imm32(WASM_OP, 1.0_f32, Instruction::f32_min_imm)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm32_rev(WASM_OP, 1.0_f32, Instruction::f32_min_imm)
}

#[test]
fn reg_nan() {
    test_reg_nan(WASM_OP, [Instruction::return_imm32(f32::NAN)]);
}

#[test]
fn nan_reg() {
    test_nan_reg(WASM_OP, [Instruction::return_imm32(f32::NAN)]);
}

#[test]
fn reg_pos_infinity() {
    let expected = [Instruction::ReturnReg {
        value: Register::from_u16(0),
    }];
    test_binary_reg_imm_with(WASM_OP, f32::INFINITY, expected)
}

#[test]
fn reg_pos_infinity_rev() {
    let expected = [Instruction::ReturnReg {
        value: Register::from_u16(0),
    }];
    test_binary_reg_imm_rev_with(WASM_OP, f32::INFINITY, expected)
}

#[test]
fn consteval() {
    let lhs = 1.0_f32;
    let rhs = 2.0;
    let result = if rhs < lhs { rhs } else { lhs };
    test_binary_consteval(
        WASM_OP,
        lhs,
        rhs,
        [Instruction::ReturnImm32 {
            value: Const32::from(result),
        }],
    )
}
