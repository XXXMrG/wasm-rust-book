use super::types::{GlobalType, MemType, TableType, TypeIdx, TableIdx, FuncIdx, ValType, MemIdx};

// binary module type

// '\0asm'
pub const MAGIC_NUMBER: u32 = 0x6D736100;
pub const VERSION: u32 = 0x00000001;

// import
pub enum ImportTag {
    Func = 0,
    Table,
    Mem,
    Global,
}
pub struct ImportDesc {
    tag: ImportTag,
    func_type: TypeIdx,
    table: TableType,
    memory: MemType,
    global: GlobalType,
}

pub struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

// global
pub struct Expr {}

pub struct Global {
    _type: GlobalType,
    init: Expr,
}

// export
pub enum ExportTag {
    Func = 0,
    Table,
    Mem,
    Global
}

pub struct ExportDesc {
    tag: u8,
    idx: u32,
}

pub struct Export {
    name: String,
    desc: ExportDesc,
}

// element
pub struct Elem {
    table: TableIdx,
    offset: Expr,
    init: Vec<FuncIdx>,
}

// code
pub struct Locals {
    n: u32,
    _type: ValType,
}
pub struct Code {
    locals: Vec<Locals>,
    expr: Expr,
}

// data
pub struct Data {
    memory: MemIdx,
    offset: Expr,
    init: Vec<u8>,
}

// custom sec
pub struct CustomSec {
    name: String,
    bytes: Vec<u8>,
}
