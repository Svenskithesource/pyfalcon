use pyc_editor::prelude::*;
use pyc_editor::v310::{
    code_objects::{ClosureRef, Code, Constant},
    ext_instructions::ExtInstruction,
    instructions::{Instruction, starts_line_number},
};
use yansi::Paint;

static LINENO_WIDTH: u8 = 3;
static OFFSET_WIDTH: u8 = 4;
static OPNAME_WIDTH: u8 = 20;
static OPARG_WIDTH: u8 = 5;

macro_rules! lookup_name {
    ($table:expr, $index:expr) => {
        $table
            .get($index as usize)
            .map(|name| name.to_string())
            .unwrap_or(format!("Invalid index {}", $index).red().to_string())
    };
    ($table:expr, $index:expr, |$var:ident| $body:expr) => {
        $table
            .get($index as usize)
            .map(|$var| $body.to_string())
            .unwrap_or(format!("Invalid index {}", $index).red().to_string())
    };
}

/// Returns a string with the argument formatted if applicable.
pub fn get_instruction_arg_repr(
    code: &Code,
    index: u32,
    instruction: ExtInstruction,
) -> Option<String> {
    match instruction {
        ExtInstruction::PopTop(_)
        | ExtInstruction::RotTwo(_)
        | ExtInstruction::RotThree(_)
        | ExtInstruction::DupTop(_)
        | ExtInstruction::DupTopTwo(_)
        | ExtInstruction::RotFour(_)
        | ExtInstruction::Nop(_)
        | ExtInstruction::UnaryPositive(_)
        | ExtInstruction::UnaryNegative(_)
        | ExtInstruction::UnaryNot(_)
        | ExtInstruction::UnaryInvert(_)
        | ExtInstruction::BinaryMatrixMultiply(_)
        | ExtInstruction::InplaceMatrixMultiply(_)
        | ExtInstruction::BinaryPower(_)
        | ExtInstruction::BinaryMultiply(_)
        | ExtInstruction::BinaryModulo(_)
        | ExtInstruction::BinaryAdd(_)
        | ExtInstruction::BinarySubtract(_)
        | ExtInstruction::BinarySubscr(_)
        | ExtInstruction::BinaryFloorDivide(_)
        | ExtInstruction::BinaryTrueDivide(_)
        | ExtInstruction::InplaceFloorDivide(_)
        | ExtInstruction::InplaceTrueDivide(_)
        | ExtInstruction::GetLen(_)
        | ExtInstruction::MatchMapping(_)
        | ExtInstruction::MatchSequence(_)
        | ExtInstruction::MatchKeys(_)
        | ExtInstruction::CopyDictWithoutKeys(_)
        | ExtInstruction::WithExceptStart(_)
        | ExtInstruction::GetAiter(_)
        | ExtInstruction::GetAnext(_)
        | ExtInstruction::BeforeAsyncWith(_)
        | ExtInstruction::EndAsyncFor(_)
        | ExtInstruction::InplaceAdd(_)
        | ExtInstruction::InplaceSubtract(_)
        | ExtInstruction::InplaceMultiply(_)
        | ExtInstruction::InplaceModulo(_)
        | ExtInstruction::StoreSubscr(_)
        | ExtInstruction::DeleteSubscr(_)
        | ExtInstruction::BinaryLshift(_)
        | ExtInstruction::BinaryRshift(_)
        | ExtInstruction::BinaryAnd(_)
        | ExtInstruction::BinaryXor(_)
        | ExtInstruction::BinaryOr(_)
        | ExtInstruction::InplacePower(_)
        | ExtInstruction::GetIter(_)
        | ExtInstruction::GetYieldFromIter(_)
        | ExtInstruction::PrintExpr(_)
        | ExtInstruction::LoadBuildClass(_)
        | ExtInstruction::YieldFrom(_)
        | ExtInstruction::GetAwaitable(_)
        | ExtInstruction::LoadAssertionError(_)
        | ExtInstruction::InplaceLshift(_)
        | ExtInstruction::InplaceRshift(_)
        | ExtInstruction::InplaceAnd(_)
        | ExtInstruction::InplaceXor(_)
        | ExtInstruction::InplaceOr(_)
        | ExtInstruction::ListToTuple(_)
        | ExtInstruction::ReturnValue(_)
        | ExtInstruction::ImportStar(_)
        | ExtInstruction::SetupAnnotations(_)
        | ExtInstruction::YieldValue(_)
        | ExtInstruction::PopBlock(_)
        | ExtInstruction::PopExcept(_)
        | ExtInstruction::InvalidOpcode(_) => None,
        ExtInstruction::StoreName(name_index)
        | ExtInstruction::DeleteName(name_index)
        | ExtInstruction::StoreAttr(name_index)
        | ExtInstruction::DeleteAttr(name_index)
        | ExtInstruction::StoreGlobal(name_index)
        | ExtInstruction::DeleteGlobal(name_index)
        | ExtInstruction::LoadName(name_index)
        | ExtInstruction::LoadAttr(name_index)
        | ExtInstruction::ImportName(name_index)
        | ExtInstruction::ImportFrom(name_index)
        | ExtInstruction::LoadGlobal(name_index)
        | ExtInstruction::LoadMethod(name_index) => Some(lookup_name!(
            code.names,
            name_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::UnpackSequence(_)
        | ExtInstruction::UnpackEx(_)
        | ExtInstruction::RotN(_)
        | ExtInstruction::BuildTuple(_)
        | ExtInstruction::BuildList(_)
        | ExtInstruction::BuildSet(_)
        | ExtInstruction::BuildMap(_)
        | ExtInstruction::CallFunction(_)
        | ExtInstruction::BuildSlice(_)
        | ExtInstruction::CallFunctionKW(_)
        | ExtInstruction::ListAppend(_)
        | ExtInstruction::SetAdd(_)
        | ExtInstruction::MapAdd(_)
        | ExtInstruction::MatchClass(_)
        | ExtInstruction::BuildConstKeyMap(_)
        | ExtInstruction::BuildString(_)
        | ExtInstruction::CallMethod(_)
        | ExtInstruction::ListExtend(_)
        | ExtInstruction::SetUpdate(_)
        | ExtInstruction::DictUpdate(_)
        | ExtInstruction::DictMerge(_) => None,
        ExtInstruction::ForIter(jump)
        | ExtInstruction::JumpForward(jump)
        | ExtInstruction::SetupFinally(jump)
        | ExtInstruction::SetupWith(jump)
        | ExtInstruction::SetupAsyncWith(jump) => {
            let num = format!("{}", (index + jump.index + 1) * 2);
            Some(format!(
                "to {}",
                if index + jump.index + 1 < code.code.len() as u32 {
                    num
                } else {
                    // Put on red background if it's an invalid jump target
                    num.on_red().to_string()
                }
            ))
        }
        ExtInstruction::LoadConst(const_index) => {
            Some(lookup_name!(code.consts, const_index.index as usize))
        }
        ExtInstruction::CompareOp(comp_op) => Some(comp_op.to_string()),
        ExtInstruction::JumpIfFalseOrPop(jump)
        | ExtInstruction::JumpIfTrueOrPop(jump)
        | ExtInstruction::JumpAbsolute(jump)
        | ExtInstruction::PopJumpIfFalse(jump)
        | ExtInstruction::PopJumpIfTrue(jump)
        | ExtInstruction::JumpIfNotExcMatch(jump) => {
            Some(format!(
                "to {}",
                if jump.index < code.code.len() as u32 {
                    (jump.index * 2).to_string()
                } else {
                    // Put on red background if it's an invalid jump target
                    (jump.index * 2).to_string().on_red().to_string()
                }
            ))
        }
        ExtInstruction::Reraise(_) => None,
        ExtInstruction::IsOp(_) | ExtInstruction::ContainsOp(_) => None,
        ExtInstruction::LoadFast(varname_index)
        | ExtInstruction::StoreFast(varname_index)
        | ExtInstruction::DeleteFast(varname_index) => Some(lookup_name!(
            code.varnames,
            varname_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::GenStart(_) => None,
        ExtInstruction::RaiseVarargs(_) => None,
        ExtInstruction::MakeFunction(flags) => Some(flags.to_string()),
        ExtInstruction::LoadClosure(closure_ref_index)
        | ExtInstruction::LoadDeref(closure_ref_index)
        | ExtInstruction::StoreDeref(closure_ref_index)
        | ExtInstruction::DeleteDeref(closure_ref_index)
        | ExtInstruction::LoadClassderef(closure_ref_index) => {
            let index = closure_ref_index.into_closure_ref(&code.cellvars, &code.freevars);

            match index {
                ClosureRef::Cell { index } => {
                    Some(lookup_name!(code.cellvars, index as usize, |name| name.value))
                }
                ClosureRef::Free { index } => {
                    Some(lookup_name!(code.freevars, index as usize, |name| name.value))
                }
                ClosureRef::Invalid(index) => {
                    Some(format!("{}", format!("Invalid index {}", index).red()))
                }
            }
        }
        ExtInstruction::CallFunctionEx(_) => None,
        ExtInstruction::FormatValue(format_flag) => Some(format_flag.to_string()),
    }
}

/// Disassemble the code object, and optionally the constants
pub fn disassemble_code(code: &Code, constants: bool) -> String {
    let mut text = disassemble_code_object(&code);

    if constants {
        for constant in &code.consts {
            match constant {
                Constant::CodeObject(code) => {
                    text += &format!("Disassembly of {}:\n", code);

                    text += &disassemble_code(&code, constants);
                }
                _ => {}
            }
        }
    }

    text
}

/// This only disassembles the given code object, not its constants
fn disassemble_code_object(code_object: &Code) -> String {
    let mut text = String::new();

    let co_lines = code_object.co_lines();
    let jump_map = code_object.code.get_jump_map();

    // This is the width used to show the line number
    let lineno_width = match co_lines {
        Ok(ref co_lines) => {
            let max_lineno = co_lines
                .iter()
                .filter_map(|entry| entry.line_number)
                .max()
                .unwrap_or(0);

            if max_lineno < 1000 {
                LINENO_WIDTH as usize
            } else {
                max_lineno.to_string().len()
            }
        }
        Err(_) => LINENO_WIDTH as usize,
    };

    let maxoffset = (code_object.code.len() - 1) * 2;
    let offset_width = if maxoffset < 10_000 {
        OFFSET_WIDTH as usize
    } else {
        maxoffset.to_string().len()
    };

    for (index, instruction) in code_object.code.into_iter().enumerate() {
        let mut fields = vec![];

        let line_number = if let Ok(ref co_lines) = co_lines {
            if let Some(line) = starts_line_number(co_lines, index as u32) {
                if index != 0 {
                    text += "\n" // Newline between line numbers
                }
                format!("{:>lineno_width$}", line)
            } else {
                " ".repeat(lineno_width)
            }
        } else {
            " ".repeat(lineno_width)
        };

        fields.push(line_number);

        fields.push("   ".to_string()); // Current instruction indicator (only here so we match `dis`'s output 1:1)

        if jump_map
            .iter()
            .filter(|(_, to)| **to == index as u32)
            .peekable()
            .peek()
            .is_some()
        {
            fields.push(">>".to_string()); // Jump target indicator
        } else {
            fields.push("  ".to_string());
        }

        fields.push(format!("{:>offset_width$}", (index * 2)));

        fields.push(format!(
            "{:<width$}",
            format!("{:?}", instruction.get_opcode()),
            width = OPNAME_WIDTH as usize
        ));

        let arg = code_object.code.get_full_arg(index).unwrap();

        fields.push(format!(
            "{:>width$}",
            format!("{:?}", arg), // It's safe to unwrap since the index is always within bounds
            width = OPARG_WIDTH as usize
        ));

        let formatted_oparg = match instruction {
            Instruction::ExtendedArg(_) => None,
            _ => get_instruction_arg_repr(
                &code_object,
                index as u32,
                (instruction.get_opcode(), arg)
                    .try_into()
                    .expect("We know it's not an extended arg so we can safely convert."),
            ),
        };

        if let Some(formatted_oparg) = formatted_oparg {
            fields.push(format!("({})", formatted_oparg));
        }

        if matches!(instruction, Instruction::InvalidOpcode(_)) {
            // Show invalid instructions clearly
            text += &(fields.join(" ").on_bright_red().to_string());
        } else {
            text += &fields.join(" ");
        }

        text += "\n";
    }

    text += "\n"; // `dis` also includes an empty line at the end

    text
}

#[cfg(test)]
mod tests {
    use pyc_editor::v310::{
        code_objects::{Constant, FrozenConstant},
        instructions::{Instruction, Instructions},
    };
    use python_marshal::PyString;
    use python_marshal::{
        CodeFlags,
        Kind::{ShortAscii, ShortAsciiInterned},
    };

    use crate::v310::disassemble::disassemble_code;

    #[test]
    fn test_invalid_opcode() {
        let code_object = pyc_editor::v310::code_objects::Code {
            argcount: 0,
            posonlyargcount: 0,
            kwonlyargcount: 0,
            nlocals: 0,
            stacksize: 3,
            flags: CodeFlags::from_bits_truncate(CodeFlags::NOFREE.bits()),
            code: Instructions::new(vec![
                Instruction::LoadName(0),
                Instruction::LoadConst(0),
                Instruction::CallFunction(1),
                Instruction::PopTop(0),
                Instruction::LoadConst(1),
                Instruction::StoreName(1),
                Instruction::LoadName(0),
                Instruction::LoadConst(2),
                Instruction::LoadName(1),
                Instruction::FormatValue(2),
                Instruction::BuildString(2),
                Instruction::CallFunction(1),
                Instruction::InvalidOpcode((0, 0)),
                Instruction::LoadConst(3),
                Instruction::ReturnValue(0),
            ]),
            consts: vec![
                Constant::FrozenConstant(FrozenConstant::String(PyString {
                    value: "line 1".into(),
                    kind: ShortAscii,
                })),
                Constant::FrozenConstant(FrozenConstant::Long(2.into())),
                Constant::FrozenConstant(FrozenConstant::String(PyString {
                    value: "line 4, a=".into(),
                    kind: ShortAscii,
                })),
                Constant::FrozenConstant(FrozenConstant::None),
            ],
            names: vec![
                PyString {
                    value: "print".into(),
                    kind: ShortAsciiInterned,
                },
                PyString {
                    value: "a".into(),
                    kind: ShortAsciiInterned,
                },
            ],
            varnames: vec![],
            freevars: vec![],
            cellvars: vec![],
            filename: PyString {
                value: "test.py".into(),
                kind: ShortAscii,
            },
            name: PyString {
                value: "<module>".into(),
                kind: ShortAsciiInterned,
            },
            firstlineno: 1,
            linetable: vec![8, 0, 4, 1, 18, 2],
        };

        print!("{}", disassemble_code(&code_object, true));
    }
}
