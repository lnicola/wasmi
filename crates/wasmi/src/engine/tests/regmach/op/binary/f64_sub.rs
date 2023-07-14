use super::*;

const WASM_OP: WasmOp = WasmOp::binary(WasmType::F64, "sub");

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::f64_sub)
}

#[test]
fn reg_imm() {
    test_binary_reg_imm64(WASM_OP, 1.0_f32, Instruction::f64_sub_imm)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm64_rev(WASM_OP, 1.0_f32, Instruction::f64_sub_imm_rev)
}

#[test]
fn reg_nan() {
    test_binary_reg_imm_with(
        WASM_OP,
        f64::NAN,
        [Instruction::return_imm(ConstRef::from_u32(0))],
    )
    .expect_cref(ConstRef::from_u32(0), f64::NAN)
    .run()
}

#[test]
fn nan_reg() {
    test_binary_reg_imm_rev_with(
        WASM_OP,
        f64::NAN,
        [Instruction::return_imm(ConstRef::from_u32(0))],
    )
    .expect_cref(ConstRef::from_u32(0), f64::NAN)
    .run()
}

#[test]
fn reg_zero() {
    let expected = [Instruction::return_reg(0)];
    test_binary_reg_imm_with(WASM_OP, 0.0_f64, expected).run()
}

#[test]
fn consteval() {
    let lhs = 13.0_f64;
    let rhs = 5.5;
    test_binary_consteval(
        WASM_OP,
        lhs,
        rhs,
        [Instruction::ReturnImm {
            value: ConstRef::from_u32(0),
        }],
    )
}
