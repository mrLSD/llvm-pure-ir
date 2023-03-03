//! # Basic LLVM types

use super::type_system::{
    aggregate::{ArrayType, StructureType},
    single_value::{
        FloatingPointType, Integer128Type, Integer16Type, Integer1Type, Integer32Type,
        Integer64Type, Integer8Type, IntegerType, PointerType, VectorType,
    },
    FunctionType, VoidType,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Void,
    Function(FunctionType),
    Integer1,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Integer128,
    Integer(i128),
    FloatingPoint(FloatingPointType),
    Pointer(PointerType),
    Vector(VectorType),
    Array(ArrayType),
    Structure(StructureType),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Type::Void => format!("{VoidType}"),
            Type::Function(x) => format!("{x}"),
            Type::Integer1 => format!("{Integer1Type}"),
            Type::Integer8 => format!("{Integer8Type}"),
            Type::Integer16 => format!("{Integer16Type}"),
            Type::Integer32 => format!("{Integer32Type}"),
            Type::Integer64 => format!("{Integer64Type}"),
            Type::Integer128 => format!("{Integer128Type}"),
            Type::Integer(n) => format!("{}", IntegerType::new(n)),
            Type::FloatingPoint(x) => format!("{x}"),
            Type::Pointer(x) => format!("{x}"),
            Type::Vector(x) => format!("{x}"),
            Type::Array(x) => format!("{x}"),
            Type::Structure(x) => format!("{x}"),
        };
        write!(f, "{s}")
    }
}

impl Type {
    #[must_use]
    pub fn pointer() -> Self {
        Type::Pointer(PointerType::new(None))
    }

    #[must_use]
    pub fn pointer_with_addrspace(addr: i64) -> Self {
        Type::Pointer(PointerType::new(Some(addr)))
    }
}
