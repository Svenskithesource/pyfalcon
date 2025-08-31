use std::fmt::format;

use pyc_editor::prelude::*;
use pyc_editor::v313::code_objects::JumpDirection;
use pyc_editor::v313::instructions::get_real_jump_index;
use pyc_editor::v313::{
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
        | ExtInstruction::BeforeAsyncWith(_)
        | ExtInstruction::BeforeWith(_)
        | ExtInstruction::BinarySlice(_)
        | ExtInstruction::BinarySubscr(_)
        | ExtInstruction::CheckEgMatch(_)
        | ExtInstruction::CheckExcMatch(_)
        | ExtInstruction::CleanupThrow(_)
        | ExtInstruction::DeleteSubscr(_)
        | ExtInstruction::EndAsyncFor(_)
        | ExtInstruction::EndFor(_)
        | ExtInstruction::EndSend(_)
        | ExtInstruction::ExitInitCheck(_)
        | ExtInstruction::FormatSimple(_)
        | ExtInstruction::FormatWithSpec(_)
        | ExtInstruction::GetAiter(_)
        | ExtInstruction::Reserved(_)
        | ExtInstruction::GetAnext(_)
        | ExtInstruction::GetIter(_)
        | ExtInstruction::GetLen(_)
        | ExtInstruction::GetYieldFromIter(_)
        | ExtInstruction::InterpreterExit(_)
        | ExtInstruction::LoadAssertionError(_)
        | ExtInstruction::LoadBuildClass(_)
        | ExtInstruction::LoadLocals(_)
        | ExtInstruction::MakeFunction(_)
        | ExtInstruction::MatchKeys(_)
        | ExtInstruction::MatchMapping(_)
        | ExtInstruction::MatchSequence(_)
        | ExtInstruction::Nop(_)
        | ExtInstruction::PopExcept(_)
        | ExtInstruction::PopTop(_)
        | ExtInstruction::PushExcInfo(_)
        | ExtInstruction::PushNull(_)
        | ExtInstruction::ReturnGenerator(_)
        | ExtInstruction::ReturnValue(_)
        | ExtInstruction::SetupAnnotations(_)
        | ExtInstruction::StoreSlice(_)
        | ExtInstruction::StoreSubscr(_)
        | ExtInstruction::ToBool(_)
        | ExtInstruction::UnaryInvert(_)
        | ExtInstruction::UnaryNegative(_)
        | ExtInstruction::UnaryNot(_)
        | ExtInstruction::WithExceptStart(_) => None,
        ExtInstruction::StoreName(name_index)
        | ExtInstruction::DeleteName(name_index)
        | ExtInstruction::StoreAttr(name_index)
        | ExtInstruction::DeleteAttr(name_index)
        | ExtInstruction::StoreGlobal(name_index)
        | ExtInstruction::DeleteGlobal(name_index)
        | ExtInstruction::LoadName(name_index)
        | ExtInstruction::ImportName(name_index)
        | ExtInstruction::ImportFrom(name_index) => Some(lookup_name!(
            code.names,
            name_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::LoadAttr(attr_name_index) => {
            let repr = lookup_name!(code.names, attr_name_index.index >> 1 as usize, |name| {
                name.value.clone()
            });

            let repr = if (attr_name_index.index & 1) != 0
                && code
                    .names
                    .get(attr_name_index.index as usize >> 1)
                    .is_some()
            {
                "NULL + ".to_string() + &repr
            } else {
                repr
            };

            Some(repr)
        }
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
                "NULL + ".to_string() + &repr
            } else {
                repr
            };

            Some(repr)
        }
        ExtInstruction::LoadSuperAttr(super_name_index) => {
            let repr = lookup_name!(code.names, super_name_index.index >> 2 as usize, |name| {
                name.value.clone()
            });

            let repr = if (super_name_index.index & 1) != 0
                && code
                    .names
                    .get(super_name_index.index as usize >> 2)
                    .is_some()
            {
                "NULL + ".to_string() + &repr
            } else {
                repr
            };

            Some(repr)
        }
        ExtInstruction::BinaryOpInplaceAddUnicode(_)
        | ExtInstruction::UnpackSequence(_)
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
        | ExtInstruction::YieldValue(_)
        | ExtInstruction::MatchClass(_)
        | ExtInstruction::BuildConstKeyMap(_)
        | ExtInstruction::BuildString(_)
        | ExtInstruction::ListExtend(_)
        | ExtInstruction::SetUpdate(_)
        | ExtInstruction::DictMerge(_)
        | ExtInstruction::DictUpdate(_)
        | ExtInstruction::EnterExecutor(_)
        | ExtInstruction::Call(_)
        | ExtInstruction::CallKw(_)
        | ExtInstruction::BinaryOpAddFloat(_)
        | ExtInstruction::BinaryOpAddInt(_)
        | ExtInstruction::BinaryOpAddUnicode(_)
        | ExtInstruction::BinaryOpMultiplyFloat(_)
        | ExtInstruction::BinaryOpMultiplyInt(_)
        | ExtInstruction::BinaryOpSubtractFloat(_)
        | ExtInstruction::BinaryOpSubtractInt(_)
        | ExtInstruction::BinarySubscrDict(_)
        | ExtInstruction::BinarySubscrGetitem(_)
        | ExtInstruction::BinarySubscrListInt(_)
        | ExtInstruction::BinarySubscrStrInt(_)
        | ExtInstruction::BinarySubscrTupleInt(_)
        | ExtInstruction::CallAllocAndEnterInit(_)
        | ExtInstruction::CallBoundMethodExactArgs(_)
        | ExtInstruction::CallBoundMethodGeneral(_)
        | ExtInstruction::CallBuiltinClass(_)
        | ExtInstruction::CallBuiltinFast(_)
        | ExtInstruction::CallBuiltinFastWithKeywords(_)
        | ExtInstruction::CallBuiltinO(_)
        | ExtInstruction::CallIsinstance(_)
        | ExtInstruction::CallLen(_)
        | ExtInstruction::CallListAppend(_)
        | ExtInstruction::CallMethodDescriptorFast(_)
        | ExtInstruction::CallMethodDescriptorFastWithKeywords(_)
        | ExtInstruction::CallMethodDescriptorNoargs(_)
        | ExtInstruction::CallMethodDescriptorO(_)
        | ExtInstruction::CallNonPyGeneral(_)
        | ExtInstruction::CallPyExactArgs(_)
        | ExtInstruction::CallPyGeneral(_)
        | ExtInstruction::CallStr1(_)
        | ExtInstruction::CallTuple1(_)
        | ExtInstruction::CallType1(_)
        | ExtInstruction::CompareOpFloat(_)
        | ExtInstruction::CompareOpInt(_)
        | ExtInstruction::CompareOpStr(_)
        | ExtInstruction::ContainsOpDict(_)
        | ExtInstruction::ContainsOpSet(_)
        | ExtInstruction::LoadAttrClass(_)
        | ExtInstruction::LoadAttrGetattributeOverridden(_)
        | ExtInstruction::LoadAttrInstanceValue(_)
        | ExtInstruction::LoadAttrMethodLazyDict(_)
        | ExtInstruction::LoadAttrMethodNoDict(_)
        | ExtInstruction::LoadAttrMethodWithValues(_)
        | ExtInstruction::LoadAttrModule(_)
        | ExtInstruction::LoadAttrNondescriptorNoDict(_)
        | ExtInstruction::LoadAttrNondescriptorWithValues(_)
        | ExtInstruction::LoadAttrProperty(_)
        | ExtInstruction::LoadAttrSlot(_)
        | ExtInstruction::LoadAttrWithHint(_)
        | ExtInstruction::LoadGlobalBuiltin(_)
        | ExtInstruction::LoadGlobalModule(_)
        | ExtInstruction::LoadSuperAttrAttr(_)
        | ExtInstruction::LoadSuperAttrMethod(_)
        | ExtInstruction::ResumeCheck(_)
        | ExtInstruction::SendGen(_)
        | ExtInstruction::StoreAttrInstanceValue(_)
        | ExtInstruction::StoreAttrSlot(_)
        | ExtInstruction::StoreAttrWithHint(_)
        | ExtInstruction::StoreSubscrDict(_)
        | ExtInstruction::StoreSubscrListInt(_)
        | ExtInstruction::ToBoolAlwaysTrue(_)
        | ExtInstruction::ToBoolBool(_)
        | ExtInstruction::ToBoolInt(_)
        | ExtInstruction::ToBoolList(_)
        | ExtInstruction::ToBoolNone(_)
        | ExtInstruction::ToBoolStr(_)
        | ExtInstruction::UnpackSequenceList(_)
        | ExtInstruction::UnpackSequenceTuple(_)
        | ExtInstruction::UnpackSequenceTwoTuple(_)
        | ExtInstruction::InstrumentedResume(_)
        | ExtInstruction::InstrumentedEndFor(_)
        | ExtInstruction::InstrumentedEndSend(_)
        | ExtInstruction::InstrumentedReturnValue(_)
        | ExtInstruction::InstrumentedReturnConst(_)
        | ExtInstruction::InstrumentedYieldValue(_)
        | ExtInstruction::InstrumentedLoadSuperAttr(_)
        | ExtInstruction::InstrumentedCall(_)
        | ExtInstruction::InstrumentedCallKw(_)
        | ExtInstruction::InstrumentedCallFunctionEx(_)
        | ExtInstruction::InstrumentedInstruction(_)
        | ExtInstruction::InstrumentedLine(_) => None,
        ExtInstruction::CallIntrinsic1(_) | ExtInstruction::CallIntrinsic2(_) => None,
        ExtInstruction::LoadConst(const_index) | ExtInstruction::ReturnConst(const_index) => {
            Some(lookup_name!(code.consts, const_index.index as usize))
        }
        ExtInstruction::CompareOp(cmp_op) => {
            let cmp_str = cmp_op.compare_op.to_string();
            Some(if !cmp_op.to_bool {
                format!("{}", cmp_str)
            } else {
                format!("bool({})", cmp_str)
            })
        }
        ExtInstruction::ForIter(jump)
        | ExtInstruction::JumpForward(jump)
        | ExtInstruction::PopJumpIfFalse(jump)
        | ExtInstruction::PopJumpIfTrue(jump)
        | ExtInstruction::Send(jump)
        | ExtInstruction::PopJumpIfNotNone(jump)
        | ExtInstruction::PopJumpIfNone(jump)
        | ExtInstruction::ForIterRange(jump)
        | ExtInstruction::ForIterList(jump)
        | ExtInstruction::ForIterGen(jump)
        | ExtInstruction::ForIterTuple(jump)
        | ExtInstruction::InstrumentedForIter(jump)
        | ExtInstruction::InstrumentedPopJumpIfNone(jump)
        | ExtInstruction::InstrumentedPopJumpIfNotNone(jump)
        | ExtInstruction::InstrumentedJumpForward(jump)
        | ExtInstruction::InstrumentedPopJumpIfFalse(jump)
        | ExtInstruction::InstrumentedPopJumpIfTrue(jump)
        | ExtInstruction::JumpBackwardNoInterrupt(jump)
        | ExtInstruction::JumpBackward(jump)
        | ExtInstruction::InstrumentedJumpBackward(jump) => {
            let index = get_real_jump_index(&code.code, index as usize).unwrap() as u32;
            let jump_target = match jump.direction {
                JumpDirection::Forward => index + jump.index + 1,
                JumpDirection::Backward => index - jump.index + 1,
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
        | ExtInstruction::DeleteFast(varname_index)
        | ExtInstruction::LoadFastCheck(varname_index)
        | ExtInstruction::LoadFastAndClear(varname_index) => Some(lookup_name!(
            code.localsplusnames,
            varname_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::LoadFastLoadFast((index_1, index_2))
        | ExtInstruction::StoreFastLoadFast((index_1, index_2))
        | ExtInstruction::StoreFastStoreFast((index_1, index_2)) => Some(
            lookup_name!(code.localsplusnames, index_1.index as usize, |name| name
                .value)
                + ", "
                + &lookup_name!(code.localsplusnames, index_2.index as usize, |name| name
                    .value),
        ),
        ExtInstruction::LoadFromDictOrDeref(_) => None,
        ExtInstruction::LoadFromDictOrGlobals(dynamic_index) => Some(
            // See https://docs.python.org/3.12/library/dis.html#opcode-LOAD_FROM_DICT_OR_GLOBALS
            code.names
                .get(dynamic_index.index as usize)
                .map(|s| s.value.to_string())
                .unwrap_or("NAME NOT FOUND".to_string()),
        ),
        ExtInstruction::RaiseVarargs(_) => None,
        ExtInstruction::GetAwaitable(_) => None,
        ExtInstruction::BuildSlice(_) => None,
        ExtInstruction::MakeCell(closure_index)
        | ExtInstruction::LoadDeref(closure_index)
        | ExtInstruction::StoreDeref(closure_index)
        | ExtInstruction::DeleteDeref(closure_index) => Some(lookup_name!(
            code.localsplusnames,
            closure_index.index as usize,
            |name| name.value
        )),
        ExtInstruction::CallFunctionEx(_) => None,
        ExtInstruction::SetFunctionAttribute(flags) => Some(flags.to_string()),
        ExtInstruction::ConvertValue(format) => Some(format.to_string()),
        ExtInstruction::Resume(_) => None,
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
                max_lineno.to_string().len()
            }
        }
        Err(_) => 0,
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
        } else if matches!(instruction, Instruction::Cache(_)) {
            // Don't print cache instructions
            continue;
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
    use pyc_editor::v311;
    use pyc_editor::v311::{
        code_objects::{Constant, FrozenConstant},
        instructions::{Instruction, Instructions},
    };
    use python_marshal::Kind::{ShortAscii, ShortAsciiInterned};
    use python_marshal::{CodeFlags, PyString};

    use crate::v311::disassemble::disassemble_code;

    #[test]
    fn test_invalid_opcode() {
        // 1.
        // 2. print("line 1")
        // 3. a = 2
        // 4.
        // 5. print(f"line 4, {a=}")

        let code_object = v311::code_objects::Code {
            argcount: 0,
            posonlyargcount: 0,
            kwonlyargcount: 0,
            stacksize: 4,
            flags: CodeFlags::from_bits_retain(0x0),
            code: Instructions::new(vec![
                Instruction::Resume(0),
                Instruction::PushNull(0),
                Instruction::LoadName(0),
                Instruction::LoadConst(0),
                Instruction::Precall(1),
                Instruction::Cache(0),
                Instruction::Call(1),
                Instruction::Cache(0),
                Instruction::Cache(0),
                Instruction::Cache(0),
                Instruction::Cache(0),
                Instruction::PopTop(0),
                Instruction::LoadConst(1),
                Instruction::StoreName(1),
                Instruction::PushNull(0),
                Instruction::LoadName(0),
                Instruction::LoadConst(2),
                Instruction::LoadName(1),
                Instruction::FormatValue(2),
                Instruction::InvalidOpcode((0, 5)),
                Instruction::Precall(1),
                Instruction::Cache(0),
                Instruction::Call(1),
                Instruction::Cache(0),
                Instruction::Cache(0),
                Instruction::Cache(0),
                Instruction::Cache(0),
                Instruction::PopTop(0),
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
            localsplusnames: vec![],
            localspluskinds: vec![],
            filename: PyString {
                value: "test.py".into(),
                kind: ShortAscii,
            },
            name: PyString {
                value: "<module>".into(),
                kind: ShortAsciiInterned,
            },
            qualname: PyString {
                value: "<module>".into(),
                kind: ShortAsciiInterned,
            },
            firstlineno: 1,
            linetable: vec![
                240, 3, 1, 1, 1, 224, 0, 5, 128, 5, 128, 104, 129, 15, 132, 15, 128, 15, 216, 4, 5,
                128, 1, 224, 0, 5, 128, 5, 128, 111, 144, 17, 128, 111, 128, 111, 209, 0, 22, 212,
                0, 22, 208, 0, 22, 208, 0, 22, 208, 0, 22,
            ],
            exceptiontable: vec![],
        };

        print!("{}", disassemble_code(&code_object, true));
    }
}
