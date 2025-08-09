use crate::v310;
use pyc_editor::CodeObject;

/// Disassemble the code object, and optionally the constants
pub fn disassemble_code(code: &CodeObject, constants: bool) -> String {
    match code {
        CodeObject::V310(code) => v310::disassemble::disassemble_code(code, constants),
    }
}
