use crate::engine::bytecode2::Instruction;
use alloc::vec::Vec;

/// Reference to an encoded [`Instruction`] in the [`Instruction`] stream of an [`InstrEncoder`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Instr(usize);

/// Encodes `wasmi` bytecode instructions to an [`Instruction`] stream.
#[derive(Debug)]
pub struct InstrEncoder {
    /// Already encoded [`Instruction`] words.
    _instrs: Vec<Instruction>,
}
