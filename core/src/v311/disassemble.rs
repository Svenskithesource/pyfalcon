use pyc_editor::prelude::*;
use pyc_editor::v311::code_objects::JumpDirection;
use pyc_editor::v311::{
    code_objects::{Code, Constant},
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
        ExtInstruction::Cache(_)
        | ExtInstruction::PopTop(_)
        | ExtInstruction::PushNull(_)
        | ExtInstruction::Nop(_)
        | ExtInstruction::UnaryPositive(_)
        | ExtInstruction::UnaryNegative(_)
        | ExtInstruction::UnaryNot(_)
        | ExtInstruction::UnaryInvert(_)
        | ExtInstruction::BinarySubscr(_)
        | ExtInstruction::GetLen(_)
        | ExtInstruction::MatchMapping(_)
        | ExtInstruction::MatchSequence(_)
        | ExtInstruction::MatchKeys(_)
        | ExtInstruction::PushExcInfo(_)
        | ExtInstruction::CheckExcMatch(_)
        | ExtInstruction::CheckEgMatch(_)
        | ExtInstruction::WithExceptStart(_)
        | ExtInstruction::GetAiter(_)
        | ExtInstruction::GetAnext(_)
        | ExtInstruction::BeforeAsyncWith(_)
        | ExtInstruction::BeforeWith(_)
        | ExtInstruction::EndAsyncFor(_)
        | ExtInstruction::StoreSubscr(_)
        | ExtInstruction::DeleteSubscr(_)
        | ExtInstruction::GetIter(_)
        | ExtInstruction::GetYieldFromIter(_)
        | ExtInstruction::PrintExpr(_)
        | ExtInstruction::LoadBuildClass(_)
        | ExtInstruction::LoadAssertionError(_)
        | ExtInstruction::ReturnGenerator(_)
        | ExtInstruction::ListToTuple(_)
        | ExtInstruction::ReturnValue(_)
        | ExtInstruction::ImportStar(_)
        | ExtInstruction::SetupAnnotations(_)
        | ExtInstruction::YieldValue(_)
        | ExtInstruction::AsyncGenWrap(_)
        | ExtInstruction::PrepReraiseStar(_)
        | ExtInstruction::PopExcept(_) => None,
        ExtInstruction::StoreName(name_index)
        | ExtInstruction::DeleteName(name_index)
        | ExtInstruction::LoadName(name_index)
        | ExtInstruction::ImportName(name_index)
        | ExtInstruction::ImportFrom(name_index)
        | ExtInstruction::StoreAttr(name_index)
        | ExtInstruction::DeleteAttr(name_index)
        | ExtInstruction::StoreGlobal(name_index)
        | ExtInstruction::DeleteGlobal(name_index)
        | ExtInstruction::LoadAttr(name_index)
        | ExtInstruction::LoadMethod(name_index) => Some(lookup_name!(
            code.names,
            name_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::LoadGlobal(global_name_index) => {
            let repr = lookup_name!(code.names, global_name_index.index >> 1 as usize, |name| {
                name.value.clone()
            });

            let repr = if (global_name_index.index & 1) != 0
                && code
                    .names
                    .get(global_name_index.index as usize >> 1)
                    .is_some()
            {
                "NULL".to_string() + &repr
            } else {
                repr
            };

            Some(repr)
        }
        ExtInstruction::UnpackSequence(_)
        | ExtInstruction::UnpackEx(_)
        | ExtInstruction::Swap(_)
        | ExtInstruction::BuildTuple(_)
        | ExtInstruction::BuildList(_)
        | ExtInstruction::BuildSet(_)
        | ExtInstruction::BuildMap(_)
        | ExtInstruction::Copy(_)
        | ExtInstruction::ListAppend(_)
        | ExtInstruction::SetAdd(_)
        | ExtInstruction::MapAdd(_)
        | ExtInstruction::CopyFreeVars(_)
        | ExtInstruction::MatchClass(_)
        | ExtInstruction::BuildConstKeyMap(_)
        | ExtInstruction::BuildString(_)
        | ExtInstruction::ListExtend(_)
        | ExtInstruction::SetUpdate(_)
        | ExtInstruction::DictMerge(_)
        | ExtInstruction::DictUpdate(_)
        | ExtInstruction::Precall(_)
        | ExtInstruction::Call(_)
        | ExtInstruction::BinaryOpAdaptive(_)
        | ExtInstruction::BinaryOpAddFloat(_)
        | ExtInstruction::BinaryOpAddInt(_)
        | ExtInstruction::BinaryOpAddUnicode(_)
        | ExtInstruction::BinaryOpInplaceAddUnicode(_)
        | ExtInstruction::BinaryOpMultiplyFloat(_)
        | ExtInstruction::BinaryOpMultiplyInt(_)
        | ExtInstruction::BinaryOpSubtractFloat(_)
        | ExtInstruction::BinaryOpSubtractInt(_)
        | ExtInstruction::BinarySubscrAdaptive(_)
        | ExtInstruction::BinarySubscrDict(_)
        | ExtInstruction::BinarySubscrGetitem(_)
        | ExtInstruction::BinarySubscrListInt(_)
        | ExtInstruction::BinarySubscrTupleInt(_)
        | ExtInstruction::CallAdaptive(_)
        | ExtInstruction::CallPyExactArgs(_)
        | ExtInstruction::CallPyWithDefaults(_)
        | ExtInstruction::CompareOpAdaptive(_)
        | ExtInstruction::CompareOpFloatJump(_)
        | ExtInstruction::CompareOpIntJump(_)
        | ExtInstruction::CompareOpStrJump(_)
        | ExtInstruction::LoadAttrAdaptive(_)
        | ExtInstruction::LoadAttrInstanceValue(_)
        | ExtInstruction::LoadAttrModule(_)
        | ExtInstruction::LoadAttrSlot(_)
        | ExtInstruction::LoadAttrWithHint(_)
        | ExtInstruction::LoadConstLoadFast(_)
        | ExtInstruction::LoadFastLoadConst(_)
        | ExtInstruction::LoadFastLoadFast(_)
        | ExtInstruction::LoadGlobalAdaptive(_)
        | ExtInstruction::LoadGlobalBuiltin(_)
        | ExtInstruction::LoadGlobalModule(_)
        | ExtInstruction::LoadMethodAdaptive(_)
        | ExtInstruction::LoadMethodClass(_)
        | ExtInstruction::LoadMethodModule(_)
        | ExtInstruction::LoadMethodNoDict(_)
        | ExtInstruction::LoadMethodWithDict(_)
        | ExtInstruction::LoadMethodWithValues(_)
        | ExtInstruction::PrecallAdaptive(_)
        | ExtInstruction::PrecallBoundMethod(_)
        | ExtInstruction::PrecallBuiltinClass(_)
        | ExtInstruction::PrecallBuiltinFastWithKeywords(_)
        | ExtInstruction::PrecallMethodDescriptorFastWithKeywords(_)
        | ExtInstruction::PrecallNoKwBuiltinFast(_)
        | ExtInstruction::PrecallNoKwBuiltinO(_)
        | ExtInstruction::PrecallNoKwIsinstance(_)
        | ExtInstruction::PrecallNoKwLen(_)
        | ExtInstruction::PrecallNoKwListAppend(_)
        | ExtInstruction::PrecallNoKwMethodDescriptorFast(_)
        | ExtInstruction::PrecallNoKwMethodDescriptorNoargs(_)
        | ExtInstruction::PrecallNoKwMethodDescriptorO(_)
        | ExtInstruction::PrecallNoKwStr1(_)
        | ExtInstruction::PrecallNoKwTuple1(_)
        | ExtInstruction::PrecallNoKwType1(_)
        | ExtInstruction::PrecallPyfunc(_)
        | ExtInstruction::StoreAttrAdaptive(_)
        | ExtInstruction::StoreAttrInstanceValue(_)
        | ExtInstruction::StoreAttrSlot(_)
        | ExtInstruction::StoreAttrWithHint(_)
        | ExtInstruction::StoreFastLoadFast(_)
        | ExtInstruction::StoreFastStoreFast(_)
        | ExtInstruction::StoreSubscrAdaptive(_)
        | ExtInstruction::StoreSubscrDict(_)
        | ExtInstruction::StoreSubscrListInt(_)
        | ExtInstruction::UnpackSequenceAdaptive(_)
        | ExtInstruction::UnpackSequenceList(_)
        | ExtInstruction::UnpackSequenceTuple(_)
        | ExtInstruction::UnpackSequenceTwoTuple(_)
        | ExtInstruction::DoTracing(_) => None,
        ExtInstruction::LoadConst(const_index) | ExtInstruction::KwNames(const_index) => {
            Some(lookup_name!(code.consts, const_index.index as usize))
        }
        ExtInstruction::CompareOp(cmp_op) => Some(cmp_op.to_string()),
        ExtInstruction::ForIter(jump)
        | ExtInstruction::JumpForward(jump)
        | ExtInstruction::JumpIfFalseOrPop(jump)
        | ExtInstruction::JumpIfTrueOrPop(jump)
        | ExtInstruction::PopJumpForwardIfFalse(jump)
        | ExtInstruction::PopJumpForwardIfTrue(jump)
        | ExtInstruction::Send(jump)
        | ExtInstruction::PopJumpForwardIfNotNone(jump)
        | ExtInstruction::PopJumpForwardIfNone(jump)
        | ExtInstruction::JumpBackwardNoInterrupt(jump)
        | ExtInstruction::JumpBackward(jump)
        | ExtInstruction::JumpBackwardQuick(jump)
        | ExtInstruction::PopJumpBackwardIfNotNone(jump)
        | ExtInstruction::PopJumpBackwardIfNone(jump)
        | ExtInstruction::PopJumpBackwardIfFalse(jump)
        | ExtInstruction::PopJumpBackwardIfTrue(jump) => {
            let jump_target = match jump.direction {
                JumpDirection::Forward => index + jump.index + 1,
                JumpDirection::Backward => index - jump.index - 1,
            };

            let num = format!("{}", jump_target * 2);
            Some(format!(
                "to {}",
                if jump_target < code.code.len() as u32 {
                    num
                } else {
                    // Put on red background if it's an invalid jump target
                    num.on_red().to_string()
                }
            ))
        }
        ExtInstruction::IsOp(_) | ExtInstruction::ContainsOp(_) => None,
        ExtInstruction::Reraise(_) => None,
        ExtInstruction::BinaryOp(binary_op) => Some(binary_op.to_string()),
        ExtInstruction::LoadFast(varname_index)
        | ExtInstruction::StoreFast(varname_index)
        | ExtInstruction::DeleteFast(varname_index) => Some(lookup_name!(
            code.localsplusnames,
            varname_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::RaiseVarargs(_) => None,
        ExtInstruction::GetAwaitable(_) => None,
        ExtInstruction::MakeFunction(flags) => Some(flags.to_string()),
        ExtInstruction::BuildSlice(_) => None,
        ExtInstruction::MakeCell(closure_index)
        | ExtInstruction::LoadClosure(closure_index)
        | ExtInstruction::LoadDeref(closure_index)
        | ExtInstruction::StoreDeref(closure_index)
        | ExtInstruction::DeleteDeref(closure_index)
        | ExtInstruction::LoadClassderef(closure_index) => Some(lookup_name!(
            code.localsplusnames,
            closure_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::CallFunctionEx(_) => None,
        ExtInstruction::Resume(_) | ExtInstruction::ResumeQuick(_) => None,
        ExtInstruction::FormatValue(format_flag) => Some(format_flag.to_string()),
        ExtInstruction::InvalidOpcode((_, _)) => None,
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
                code_object.code.len().to_string().len()
            }
        }
        Err(_) => LINENO_WIDTH as usize,
    };

    let offset_width = if code_object.code.len() - 1 < 10_000 {
        OFFSET_WIDTH as usize
    } else {
        code_object.code.len().to_string().len()
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

        match instruction {
            Instruction::InvalidOpcode((opcode, _)) => fields.push(format!(
                "{:<width$}",
                format!("{:?} <{opcode}>", instruction.get_opcode()),
                width = OPNAME_WIDTH as usize
            )),
            _ => fields.push(format!(
                "{:<width$}",
                format!("{:?}", instruction.get_opcode()),
                width = OPNAME_WIDTH as usize
            )),
        }

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
    use python_marshal::Kind::{ShortAscii, ShortAsciiInterned};
    use python_marshal::{CodeFlags, PyString};

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
