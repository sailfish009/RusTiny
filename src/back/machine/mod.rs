//! The RusTiny machine description
//!
//! # Memory layout
//!
//! The Tiny machine doesn't have any registers. We emulate them as special
//! memory addresses. The planned memory layout looks like this (from low
//! to high):
//!
//! - Stack management registers: SP, BP
//! - General purpose registers: AX, BX, CX, DX
//! - Global variables
//! - The runtime stack

mod cconv;
mod instructions;
mod registers;


pub use self::instructions::MachineCode;
pub use self::registers::MachineRegister;