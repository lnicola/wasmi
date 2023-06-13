use super::{BinInstr, BinInstrImm16, Const16, Const32, Instruction, Register, UnaryInstr};

macro_rules! constructor_for {
    (
        $(
            fn $fn_name:ident($mode:ident) -> Self::$op_code:ident;
        )* $(,)?
    ) => {
        $( constructor_for! { @impl fn $fn_name($mode) -> Self::$op_code } )*
    };
    ( @impl fn $fn_name:ident(binary) -> Self::$op_code:ident ) => {
        #[doc = concat!("Creates a new [`Instruction::", stringify!($op_code), "`].")]
        pub fn $fn_name(result: Register, lhs: Register, rhs: Register) -> Self {
            Self::$op_code(BinInstr::new(result, lhs, rhs))
        }
    };
    ( @impl fn $fn_name:ident(binary_imm) -> Self::$op_code:ident ) => {
        #[doc = concat!("Creates a new [`Instruction::", stringify!($op_code), "`].")]
        pub fn $fn_name(result: Register, lhs: Register) -> Self {
            Self::$op_code(UnaryInstr::new(result, lhs))
        }
    };
    ( @impl fn $fn_name:ident(binary_imm16) -> Self::$op_code:ident ) => {
        #[doc = concat!("Creates a new [`Instruction::", stringify!($op_code), "`].")]
        pub fn $fn_name(result: Register, lhs: Register, rhs: Const16) -> Self {
            Self::$op_code(BinInstrImm16::new(result, lhs, rhs))
        }
    };
    ( @impl fn $fn_name:ident(binary_imm16_rev) -> Self::$op_code:ident ) => {
        #[doc = concat!("Creates a new [`Instruction::", stringify!($op_code), "`].")]
        pub fn $fn_name(result: Register, lhs: Const16, rhs: Register) -> Self {
            Self::$op_code(BinInstrImm16::new(result, rhs, lhs))
        }
    };
}

impl Instruction {
    /// Creates a new [`Instruction::Const32`] from the given `value`.
    pub fn const32(value: impl Into<Const32>) -> Self {
        Self::Const32(value.into())
    }

    constructor_for! {
        fn i32_add(binary) -> Self::I32Add;
        fn i64_add(binary) -> Self::I32Add;
        fn i32_add_imm(binary_imm) -> Self::I32AddImm;
        fn i64_add_imm(binary_imm) -> Self::I32AddImm;
        fn i32_add_imm16(binary_imm16) -> Self::I32AddImm16;
        fn i64_add_imm16(binary_imm16) -> Self::I32AddImm16;

        fn i32_sub(binary) -> Self::I32Sub;
        fn i64_sub(binary) -> Self::I64Sub;
        fn i32_sub_imm(binary_imm) -> Self::I32SubImm;
        fn i64_sub_imm(binary_imm) -> Self::I64SubImm;
        fn i32_sub_imm_rev(binary_imm) -> Self::I32SubImm;
        fn i64_sub_imm_rev(binary_imm) -> Self::I64SubImm;
        fn i32_sub_imm16(binary_imm16) -> Self::I32SubImm16;
        fn i64_sub_imm16(binary_imm16) -> Self::I64SubImm16;
        fn i32_sub_imm16_rev(binary_imm16_rev) -> Self::I32SubImm16;
        fn i64_sub_imm16_rev(binary_imm16_rev) -> Self::I64SubImm16;

        fn i32_shl(binary) -> Self::I32Shl;
        fn i32_shl_imm(binary_imm16) -> Self::I32ShlImm;
        fn i32_shl_imm_rev(binary_imm) -> Self::I32ShlImmRev;
        fn i32_shl_imm16_rev(binary_imm16_rev) -> Self::I32ShlImm16Rev;

        fn i64_shl(binary) -> Self::I64Shl;
        fn i64_shl_imm(binary_imm16) -> Self::I64ShlImm;
        fn i64_shl_imm_rev(binary_imm) -> Self::I64ShlImmRev;
        fn i64_shl_imm16_rev(binary_imm16_rev) -> Self::I64ShlImm16Rev;

        fn i32_shr_u(binary) -> Self::I32ShrU;
        fn i32_shr_u_imm(binary_imm16) -> Self::I32ShrUImm;
        fn i32_shr_u_imm_rev(binary_imm) -> Self::I32ShrUImmRev;
        fn i32_shr_u_imm16_rev(binary_imm16_rev) -> Self::I32ShrUImm16Rev;

        fn i64_shr_u(binary) -> Self::I64ShrU;
        fn i64_shr_u_imm(binary_imm16) -> Self::I64ShrUImm;
        fn i64_shr_u_imm_rev(binary_imm) -> Self::I64ShrUImmRev;
        fn i64_shr_u_imm16_rev(binary_imm16_rev) -> Self::I64ShrUImm16Rev;

        fn i32_shr_s(binary) -> Self::I32ShrS;
        fn i32_shr_s_imm(binary_imm16) -> Self::I32ShrSImm;
        fn i32_shr_s_imm_rev(binary_imm) -> Self::I32ShrSImmRev;
        fn i32_shr_s_imm16_rev(binary_imm16_rev) -> Self::I32ShrSImm16Rev;

        fn i64_shr_s(binary) -> Self::I64ShrS;
        fn i64_shr_s_imm(binary_imm16) -> Self::I64ShrSImm;
        fn i64_shr_s_imm_rev(binary_imm) -> Self::I64ShrSImmRev;
        fn i64_shr_s_imm16_rev(binary_imm16_rev) -> Self::I64ShrSImm16Rev;

        fn i32_rotl(binary) -> Self::I32Rotl;
        fn i32_rotl_imm(binary_imm16) -> Self::I32RotlImm;
        fn i32_rotl_imm_rev(binary_imm) -> Self::I32RotlImmRev;
        fn i32_rotl_imm16_rev(binary_imm16_rev) -> Self::I32RotlImm16Rev;

        fn i32_rotr(binary) -> Self::I32Rotr;
        fn i32_rotr_imm(binary_imm16) -> Self::I32RotrImm;
        fn i32_rotr_imm_rev(binary_imm) -> Self::I32RotrImmRev;
        fn i32_rotr_imm16_rev(binary_imm16_rev) -> Self::I32RotrImm16Rev;

        fn i32_mul(binary) -> Self::I32Mul;
        fn i64_mul(binary) -> Self::I64Mul;
        fn i32_mul_imm(binary_imm) -> Self::I32MulImm;
        fn i64_mul_imm(binary_imm) -> Self::I64MulImm;
        fn i32_mul_imm16(binary_imm16) -> Self::I32MulImm16;
        fn i64_mul_imm16(binary_imm16) -> Self::I64MulImm16;

        fn i32_and(binary) -> Self::I32And;
        fn i64_and(binary) -> Self::I64And;
        fn i32_and_imm(binary_imm) -> Self::I32AndImm;
        fn i64_and_imm(binary_imm) -> Self::I64AndImm;
        fn i32_and_imm16(binary_imm16) -> Self::I32AndImm16;
        fn i64_and_imm16(binary_imm16) -> Self::I64AndImm16;

        fn i32_or(binary) -> Self::I32Or;
        fn i64_or(binary) -> Self::I64Or;
        fn i32_or_imm(binary_imm) -> Self::I32OrImm;
        fn i64_or_imm(binary_imm) -> Self::I64OrImm;
        fn i32_or_imm16(binary_imm16) -> Self::I32OrImm16;
        fn i64_or_imm16(binary_imm16) -> Self::I64OrImm16;

        fn i32_xor(binary) -> Self::I32Xor;
        fn i64_xor(binary) -> Self::I64Xor;
        fn i32_xor_imm(binary_imm) -> Self::I32XorImm;
        fn i64_xor_imm(binary_imm) -> Self::I64XorImm;
        fn i32_xor_imm16(binary_imm16) -> Self::I32XorImm16;
        fn i64_xor_imm16(binary_imm16) -> Self::I64XorImm16;
    }
}
