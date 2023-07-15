use wasmi_core::ValueType;

use crate::{
    core::{UntypedValue, F32},
    engine::{
        bytecode2::{AnyConst32, Instruction, Register},
        ConstRef,
    },
};
use core::fmt::Display;

pub trait WasmType: Copy + Display + Into<UntypedValue> + From<UntypedValue> {
    const NAME: &'static str;
    const VALUE_TYPE: ValueType;

    fn return_imm_instr(&self) -> Instruction;
}

impl WasmType for i32 {
    const NAME: &'static str = "i32";
    const VALUE_TYPE: ValueType = ValueType::I32;

    fn return_imm_instr(&self) -> Instruction {
        Instruction::return_imm32(*self)
    }
}

impl WasmType for i64 {
    const NAME: &'static str = "i64";
    const VALUE_TYPE: ValueType = ValueType::I64;

    fn return_imm_instr(&self) -> Instruction {
        match i32::try_from(*self) {
            Ok(value) => Instruction::return_i64imm32(value),
            Err(_) => Instruction::return_reg(Register::from_i16(-1)),
        }
    }
}

impl WasmType for f32 {
    const NAME: &'static str = "f32";
    const VALUE_TYPE: ValueType = ValueType::F32;

    fn return_imm_instr(&self) -> Instruction {
        Instruction::return_imm32(F32::from(*self))
    }
}

impl WasmType for f64 {
    const NAME: &'static str = "f64";
    const VALUE_TYPE: ValueType = ValueType::F64;

    fn return_imm_instr(&self) -> Instruction {
        Instruction::return_reg(Register::from_i16(-1))
    }
}
