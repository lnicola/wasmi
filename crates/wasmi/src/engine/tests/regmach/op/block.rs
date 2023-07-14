use super::*;
use crate::engine::bytecode::BranchOffset;

#[test]
fn empty_block() {
    let wasm = wat2wasm(
        r"
        (module
            (func (block))
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::Return])
        .run()
}

#[test]
fn nested_empty_block() {
    let wasm = wat2wasm(
        r"
        (module
            (func (block (block)))
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::Return])
        .run()
}

#[test]
fn identity_block_1() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32) (result i32)
                (local.get 0)
                (block (param i32) (result i32))
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::return_reg(Register::from_i16(0))])
        .run()
}

#[test]
fn identity_block_2() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32 i64) (result i32)
                (local.get 0)
                (local.get 1)
                (block (param i32 i64) (result i32 i64))
                (drop)
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::return_reg(Register::from_i16(0))])
        .run()
}

#[test]
fn nested_identity_block_1() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32) (result i32)
                (local.get 0)
                (block (param i32) (result i32)
                    (block (param i32) (result i32))
                )
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::return_reg(Register::from_i16(0))])
        .run()
}

#[test]
fn nested_identity_block_2() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32 i64) (result i32)
                (local.get 0)
                (local.get 1)
                (block (param i32 i64) (result i32 i64)
                    (block (param i32 i64) (result i32 i64))
                )
                (drop)
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::return_reg(Register::from_i16(0))])
        .run()
}

#[test]
fn branched_block_0() {
    let wasm = wat2wasm(
        r"
        (module
            (func
                (block
                    (br 0)
                )
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([
            Instruction::branch(BranchOffset::from(1)),
            Instruction::Return,
        ])
        .run()
}

#[test]
fn branched_block_1() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32) (result i32)
                (local.get 0)
                (block (param i32) (result i32)
                    (br 0)
                )
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([
            Instruction::copy(Register::from_i16(1), Register::from_i16(0)),
            Instruction::branch(BranchOffset::from(1)),
            Instruction::return_reg(Register::from_i16(1)),
        ])
        .run()
}

#[test]
fn branched_block_2() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32 i64) (result i32)
                (local.get 0)
                (local.get 1)
                (block (param i32 i64) (result i32 i64)
                    (br 0)
                )
                (drop)
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([
            Instruction::copy(Register::from_i16(2), Register::from_i16(0)),
            Instruction::copy(Register::from_i16(3), Register::from_i16(1)),
            Instruction::branch(BranchOffset::from(1)),
            Instruction::return_reg(Register::from_i16(2)),
        ])
        .run()
}

#[test]
fn branch_if_block_0() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32)
                (local.get 0)
                (block (param i32)
                    (br_if 0)
                )
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([
            Instruction::branch_nez(Register::from_i16(0), BranchOffset::from(1)),
            Instruction::Return,
        ])
        .run()
}

#[test]
fn branch_if_block_1() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32 i32) (result i32)
                (local.get 0)
                (local.get 1)
                (block (param i32 i32) (result i32)
                    (br_if 0)
                )
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([
            Instruction::branch_eqz(Register::from_i16(1), BranchOffset::from(3)),
            Instruction::copy(Register::from_i16(2), Register::from_i16(0)),
            Instruction::branch(BranchOffset::from(2)),
            Instruction::copy(Register::from_i16(2), Register::from_i16(0)),
            Instruction::return_reg(Register::from_i16(2)),
        ])
        .run()
}

#[test]
fn branch_to_func_block_0() {
    let wasm = wat2wasm(
        r"
        (module
            (func
                (br 0)
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::Return])
        .run()
}

#[test]
fn branch_to_func_block_1() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32) (result i32)
                (local.get 0)
                (br 0)
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::return_reg(Register::from_i16(0))])
        .run()
}

#[test]
fn branch_to_func_block_nested_0() {
    let wasm = wat2wasm(
        r"
        (module
            (func
                (block
                    (br 1)
                )
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::Return])
        .run()
}

#[test]
fn branch_to_func_block_nested_1() {
    let wasm = wat2wasm(
        r"
        (module
            (func (param i32) (result i32)
                (local.get 0)
                (block (param i32) (result i32)
                    (br 1)
                )
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func([Instruction::return_reg(Register::from_i16(0))])
        .run()
}
