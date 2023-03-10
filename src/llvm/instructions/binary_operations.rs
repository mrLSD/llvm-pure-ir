//! # Binary Operations
//!
//! Binary operators are used to do most of the computation in a program.
//! They require two operands of the same type, execute an operation on
//! them, and produce a single value. The operands might represent multiple
//! data, as is the case with the vector data type. The result value has
//! the same type as its operands.
//!
//! https://llvm.org/docs/LangRef.html#binary-operations

use crate::llvm::{fast_math_flags::FastMathFlags, types::Type};

/// The ‘add’ instruction returns the sum of its two operands.
///
/// The two arguments to the ‘add’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// The value produced is the integer sum of the two operands.
///
/// https://llvm.org/docs/LangRef.html#add-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Add {
    pub result: String,
    pub nuw: Option<()>,
    pub nsw: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘fadd’ instruction returns the sum of its two operands.
///
/// The two arguments to the ‘fadd’ instruction must be floating-point
/// or vector of floating-point values. Both arguments must have
/// identical types.
///
/// https://llvm.org/docs/LangRef.html#fadd-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FAdd {
    pub result: String,
    pub fast_math_flags: Option<FastMathFlags>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// Note that the ‘sub’ instruction is used to represent the ‘neg’
/// instruction present in most other intermediate representations.
///
/// The two arguments to the ‘sub’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
/// https://llvm.org/docs/LangRef.html#sub-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sub {
    pub result: String,
    pub nuw: Option<()>,
    pub nsw: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘mul’ instruction returns the product of its two operands.
///
/// The two arguments to the ‘mul’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
/// https://llvm.org/docs/LangRef.html#mul-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Mul {
    pub result: String,
    pub nuw: Option<()>,
    pub nsw: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘fmul’ instruction returns the product of its two operands.
///
/// The two arguments to the ‘fmul’ instruction must be floating-point
/// or vector of floating-point values. Both arguments must have
/// identical types.
///
/// https://llvm.org/docs/LangRef.html#fmul-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FMul {
    pub result: String,
    pub fast_math_flags: Option<FastMathFlags>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘udiv’ instruction returns the quotient of its two operands.
///
/// The two arguments to the ‘udiv’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// https://llvm.org/docs/LangRef.html#udiv-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UDiv {
    pub result: String,
    pub exact: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘sdiv’ instruction returns the quotient of its two operands.
///
/// The two arguments to the ‘sdiv’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
/// The value produced is the signed integer quotient of the two
/// operands rounded towards zero.
/// Note that signed integer division and unsigned integer division
/// are distinct operations; for unsigned integer division, use ‘udiv’.
/// Division by zero is undefined behavior. For vectors, if any
/// element of the divisor is zero, the operation has undefined
/// behavior. Overflow also leads to undefined behavior; this is a
/// rare case, but can occur, for example, by doing a 32-bit division
/// of -2147483648 by -1.
/// If the exact keyword is present, the result value of the sdiv is
/// a poison value if the result would be rounded.
///
/// https://llvm.org/docs/LangRef.html#sdiv-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SDiv {
    pub result: String,
    pub exact: Option<()>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘fdiv’ instruction returns the quotient of its two operands.
///
/// The two arguments to the ‘fdiv’ instruction must be floating-point
/// or vector of floating-point values. Both arguments must have
/// identical types.
///
/// https://llvm.org/docs/LangRef.html#fdiv-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FDiv {
    pub result: String,
    pub fast_math_flags: Option<FastMathFlags>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘urem’ instruction returns the remainder from the unsigned
/// division of its two arguments.
///
/// The two arguments to the ‘urem’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// https://llvm.org/docs/LangRef.html#urem-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct URem {
    pub result: String,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘srem’ instruction returns the remainder from the signed
/// division of its two operands. This instruction can also take
/// vector versions of the values in which case the elements must be
/// integers.
///
/// The two arguments to the ‘srem’ instruction must be integer or
/// vector of integer values. Both arguments must have identical
/// types.
///
/// https://llvm.org/docs/LangRef.html#srem-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SRem {
    pub result: String,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

/// The ‘frem’ instruction returns the remainder from the division of
/// its two operands.
///
/// The two arguments to the ‘frem’ instruction must be floating-point
/// or vector of floating-point values. Both arguments must have
/// identical types.
///
/// https://llvm.org/docs/LangRef.html#frem-instruction
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FRem {
    pub result: String,
    pub fast_math_flags: Option<FastMathFlags>,
    pub ty: Type,
    pub op1: String,
    pub op2: String,
}

impl std::fmt::Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "add".to_string();
        if self.nuw.is_some() {
            s = format!("{s} nuw")
        }
        if self.nsw.is_some() {
            s = format!("{s} nsw")
        }
        s = format!(
            "{} = {} {} {}, {}",
            self.result, s, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for FAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "fadd".to_string();
        if let Some(v) = &self.fast_math_flags {
            s = format!("{s} {v}")
        }
        s = format!(
            "{} = {} {} {}, {}",
            self.result, s, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for Sub {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "sub".to_string();
        if self.nuw.is_some() {
            s = format!("{s} nuw")
        }
        if self.nsw.is_some() {
            s = format!("{s} nsw")
        }
        s = format!(
            "{s} = {} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for Mul {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "mul".to_string();
        if self.nuw.is_some() {
            s = format!("{s} nuw")
        }
        if self.nsw.is_some() {
            s = format!("{s} nsw")
        }
        s = format!(
            "{s} = {} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for FMul {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "fmul".to_string();
        if let Some(v) = &self.fast_math_flags {
            s = format!("{s} {v}")
        }
        s = format!(
            "{} = {s} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for UDiv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "udiv".to_string();
        if self.exact.is_some() {
            s = format!("{s} exact")
        }
        s = format!(
            "{} = {s} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for SDiv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "sdiv".to_string();
        if self.exact.is_some() {
            s = format!("{s} exact")
        }
        s = format!(
            "{} = {} {} {}, {}",
            self.result, s, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for FDiv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "fdiv".to_string();
        if let Some(v) = &self.fast_math_flags {
            s = format!("{s} {v}")
        }
        s = format!(
            "{} = {s} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for URem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "urem".to_string();
        s = format!(
            "{} = {s} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for SRem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "srem".to_string();
        s = format!(
            "{} = {s} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}

impl std::fmt::Display for FRem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "frem".to_string();
        if let Some(v) = &self.fast_math_flags {
            s = format!("{s} {v}")
        }
        s = format!(
            "{} = {s} {} {}, {}",
            self.result, self.ty, self.op1, self.op2
        );
        write!(f, "{s}")
    }
}
