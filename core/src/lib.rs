pub mod v310;
pub mod v311;
pub mod v312;

use pyc_editor::CodeObject;

/// Disassemble the code object, and optionally the constants
pub fn disassemble_code(code: &CodeObject, constants: bool) -> String {
    match code {
        CodeObject::V310(code) => v310::disassemble::disassemble_code(code, constants),
        CodeObject::V311(code) => v311::disassemble::disassemble_code(code, constants),
        CodeObject::V312(code) => v312::disassemble::disassemble_code(code, constants),
    }
}

pub fn disable_colors() {
    yansi::disable();
}
