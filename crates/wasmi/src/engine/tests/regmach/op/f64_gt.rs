use super::*;

const WASM_OP: WasmOp = WasmOp::cmp(WasmType::F64, "gt");

#[test]
fn same_reg() {
    let expected = [Instruction::ReturnImm32 {
        value: Const32::from(false),
    }];
    test_binary_same_reg(WASM_OP, expected)
}

#[test]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::f64_gt)
}

#[test]
fn reg_imm() {
    test_binary_reg_imm64(WASM_OP, 1.0, Instruction::f64_gt_imm)
}

#[test]
fn reg_imm_rev() {
    test_binary_reg_imm64_rev(WASM_OP, 1.0, Instruction::f64_lt_imm)
}

#[test]
fn reg_pos_inf() {
    test_binary_reg_imm_with(
        WASM_OP,
        f64::INFINITY,
        [Instruction::ReturnImm32 {
            value: Const32::from(false),
        }],
    )
}

#[test]
fn neg_inf_reg() {
    test_binary_reg_imm_rev_with(
        WASM_OP,
        f64::NEG_INFINITY,
        [Instruction::ReturnImm32 {
            value: Const32::from(false),
        }],
    )
}

#[test]
fn reg_nan() {
    // Note: Unfortunately we cannot use convenience functions
    //       for test case since f32 NaN `Display` implementation
    //       differs from what the `wat2wasm` parser expects.
    let param_ty = WASM_OP.param_ty();
    let result_ty = WASM_OP.result_ty();
    let wasm = wat2wasm(&format!(
        r#"
        (module
            (func (param {param_ty}) (result {result_ty})
                local.get 0
                {param_ty}.const nan
                {WASM_OP}
            )
        )
    "#,
    ));
    let expected = [Instruction::ReturnImm32 {
        value: Const32::from(false),
    }];
    assert_func_bodies(wasm, [expected]);
}

#[test]
fn nan_reg() {
    // Note: Unfortunately we cannot use convenience functions
    //       for test case since f32 NaN `Display` implementation
    //       differs from what the `wat2wasm` parser expects.
    let param_ty = WASM_OP.param_ty();
    let result_ty = WASM_OP.result_ty();
    let wasm = wat2wasm(&format!(
        r#"
        (module
            (func (param {param_ty}) (result {result_ty})
                {param_ty}.const nan
                local.get 0
                {WASM_OP}
            )
        )
    "#,
    ));
    let expected = [Instruction::ReturnImm32 {
        value: Const32::from(false),
    }];
    assert_func_bodies(wasm, [expected]);
}

#[test]
fn consteval() {
    test_binary_consteval(
        WASM_OP,
        1.0,
        2.0,
        [Instruction::ReturnImm32 {
            value: Const32::from(false),
        }],
    );
    test_binary_consteval(
        WASM_OP,
        2.0,
        1.0,
        [Instruction::ReturnImm32 {
            value: Const32::from(true),
        }],
    );
}
