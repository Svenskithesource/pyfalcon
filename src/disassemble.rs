use pyc_editor::{CodeObject, v310::instructions::starts_line_number};
use yansi::Paint;

static LINENO_WIDTH: u8 = 3;
static OFFSET_WIDTH: u8 = 4;
static OPNAME_WIDTH: u8 = 20;
static OPARG_WIDTH: u8 = 5;

pub fn disassemble_code(code: &CodeObject) -> String {
    let mut text = String::new();

    match code {
        CodeObject::V310(code) => {
            let co_lines = code.co_lines();

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
                        code.code.len().to_string().len()
                    }
                }
                Err(_) => LINENO_WIDTH as usize,
            };

            let offset_width = if code.code.len() - 1 < 10_000 {
                OFFSET_WIDTH as usize
            } else {
                code.code.len().to_string().len()
            };

            for (index, instruction) in code.code.into_iter().enumerate() {
                let mut fields = vec![];

                let line_number = if let Ok(ref co_lines) = co_lines {
                    if let Some(line) = starts_line_number(co_lines, index as u32) {
                        format!("{:>lineno_width$}", line)
                    } else {
                        " ".repeat(lineno_width)
                    }
                } else {
                    " ".repeat(lineno_width)
                };

                fields.push(line_number);

                fields.push("   ".to_string()); // Current instruction indicator (only here so we match dis's output 1:1)

                fields.push("  ".to_string()); // Jump target indicator

                fields.push(format!("{:>offset_width$}", (index * 2)));

                fields.push(format!(
                    "{:<width$}",
                    format!("{:?}", instruction.get_opcode()),
                    width = OPNAME_WIDTH as usize
                ));

                fields.push(format!(
                    "{:<width$}",
                    format!("{:?}", code.code.get_full_arg(index).unwrap()), // It's safe to unwrap since the index is always within bounds
                    width = OPARG_WIDTH as usize
                ));

                text += &fields.join(" ");
                text += "\n";
            }
        }
    }

    text
}
