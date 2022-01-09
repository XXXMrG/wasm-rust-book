// index space type
pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type LocalIdx = u32;
pub type LabelIdx = u32;

pub type ValType = u8;

// wasm value type
pub enum BaseValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C
}

// function type
pub struct FuncType {
    param_types: Vec<ValType>,
    result_types: Vec<ValType>
}

// limit type, limit for desc table and memory
pub struct Limits {
    tag: u8,
    min: u32,
    max: u32,
}

// memory type only desc limit
pub type MemType = Limits;

// table type
pub const FUNC_REF: u8 = 0x70;
pub struct TableType {
    // now only should be FUNC_REF
    element_type: u8,
    limits: Limits,
}

// global type
pub const MUT_CONST: u8 = 0;
pub const MUT_VAR: u8 = 1;

pub struct GlobalType {
    val_type: ValType,
    mutable: u8,
}
