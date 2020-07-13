use super::ast::*;
use std::collections::BTreeMap;

struct Bank {
    start: usize,
    len: usize,
    code: Vec<u8>,
}

struct Context<'a> {
    bank: Bank,
    emulation: bool,
    wide_math: bool,
    wide_index: bool,
    names: BTreeMap<&'a str, Name>,
    relocations: Vec<Relocation<'a>>,
}

#[derive(Clone)]
enum Name {
    Var(u32),
    Function(Option<usize>, Vec<Attribute>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Relocation<'a> {
    Function(&'a str, usize),
    Break(usize),
}

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    BadAssignment(&'a Operand<'a>, &'a Operand<'a>, &'a str),
    BadAndAssignment(&'a Operand<'a>, &'a Operand<'a>, &'a str),
    BadOrAssignment(&'a Operand<'a>, &'a Operand<'a>, &'a str),
    BadBitTest(&'a Operand<'a>, &'a Operand<'a>, &'a str),
    BadEquality(&'a Operand<'a>, &'a Operand<'a>, &'a str),
    BadPush(&'a Operand<'a>, &'a str),
    BadPop(&'a Operand<'a>, &'a str),
    ConflictingAttributes(Attribute, Attribute, &'a str),
    NoSpace(&'static str, &'a str),
    LoopTooLong(&'a str),
    IfTooLong(&'a str),
    InvalidAddress(u32, &'a str),
    InvalidValue(u32, &'a str),
    InvalidRegister(Register, Attribute, &'a str),
    UnknownVariable(&'a str, &'a str),
    UnknownFunction(&'a str, &'a str),
    InvalidInterrupt(&'static str),
    UnresolvedName(Relocation<'a>),
}

pub type Result<'a> = std::result::Result<(), Error<'a>>;

pub fn assemble<'p>(program: &'p Program<'p>) -> std::result::Result<Vec<u8>, Error<'p>> {
    let mut context = Context {
        bank: Bank {
            start: 0x8000,
            len: 0x8000 - 0x40, // leave room for the header
            code: vec![],
        },
        emulation: false,
        wide_math: false,
        wide_index: false,
        names: BTreeMap::new(),
        relocations: vec![],
    };

    // First, iterate over definitions to populate the name table
    for def in &program.definitions {
        match def {
            Definition::Function(func) => context.names.insert(
                func.name,
                Name::Function(None, func.body.attributes.clone()),
            ),
            Definition::Var(var) => context.names.insert(var.name, Name::Var(var.address)),
        };
    }

    // Then, start assembling the functions
    for def in &program.definitions {
        if let Definition::Function(function) = def {
            assemble_function(&mut context, &function)?;
        }
    }

    if !context.relocations.is_empty() {
        return Err(Error::UnresolvedName(context.relocations[0].clone()));
    }

    context.bank.code.resize(0x8000, 0);
    for i in 0..22 {
        context.bank.code[0x7FC0 + i] = b'Z';
    }
    context.bank.code[0x7FD5] = 0x20; // LoROM
    context.bank.code[0x7FD7] = 0x05; // 32KB ROM
    context.bank.code[0x7FD9] = 0x01; // North american ROM
    context.bank.code[0x7FDA] = 0x33;
    // TODO: checksums

    let mut code = context.bank.code.clone();
    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("cop") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FE4] = bytes[0];
            code[0x7FE5] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("cop"));
        }
    }
    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("brk") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FE6] = bytes[0];
            code[0x7FE7] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("brk"));
        }
    }
    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("nmi") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FEA] = bytes[0];
            code[0x7FEB] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("nmi"));
        }
    }
    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("irq") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FEE] = bytes[0];
            code[0x7FEF] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("irq"));
        }
    }

    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("cop_emu") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FF4] = bytes[0];
            code[0x7FF5] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("cop_emu"));
        }
    }
    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("nmi_emu") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FFA] = bytes[0];
            code[0x7FFB] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("nmi_emu"));
        }
    }
    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("reset") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FFC] = bytes[0];
            code[0x7FFD] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("reset"));
        }
    }
    if let Some(Name::Function(Some(addr), attributes)) = context.names.get("irq_emu") {
        if attributes.contains(&Attribute::Interrupt) {
            let bytes = addr.to_le_bytes();
            code[0x7FFE] = bytes[0];
            code[0x7FFF] = bytes[1];
        } else {
            return Err(Error::InvalidInterrupt("irq_emu"));
        }
    }

    // TODO: Calculate a checksum

    Ok(code)
}

fn assemble_function<'c, 'p: 'c>(
    context: &'c mut Context<'p>,
    function: &'p Function,
) -> Result<'p> {
    // Save state of codegen flags
    let emulation = context.emulation;
    let wide_math = context.wide_math;
    let wide_index = context.wide_index;

    let function_addr = context.bank.code.len() + context.bank.start;
    context.names.insert(
        function.name,
        Name::Function(Some(function_addr), function.body.attributes.clone()),
    );

    // TODO: Make this way more efficient (maybe a refcell?)
    let mut code = context.bank.code.clone();
    context.relocations.retain(|relo| {
        if let Relocation::Function(func, fixup) = relo {
            if *func == function.name {
                let addr_bytes = function_addr.to_le_bytes();
                code[*fixup] = addr_bytes[0];
                code[*fixup + 1] = addr_bytes[1];
                false
            } else {
                true
            }
        } else {
            true
        }
    });
    context.bank.code = code;

    update_codegen(context, &function.body.attributes, &function.name)?;

    for instruction in &function.body.instructions {
        assemble_instruction(context, instruction, function.name)?;
    }

    let opcode = if function.body.attributes.contains(&Attribute::Interrupt) {
        0x40 // RTI
    } else if function.body.attributes.contains(&Attribute::Extern) {
        0x6B // RTL
    } else {
        0x60 // RTS
    };

    context.bank.push_code("Return", function.name, &[opcode])?;

    context.emulation = emulation;
    context.wide_math = wide_math;
    context.wide_index = wide_index;
    Ok(())
}

fn assemble_instruction<'c, 'p: 'c>(
    context: &'c mut Context<'p>,
    instruction: &'p Instruction,
    function_name: &'p str,
) -> Result<'p> {
    match instruction {
        Instruction::Assign(lhs, rhs) => match (lhs, rhs) {
            (Operand::Register(Register::A), Operand::Immediate(value)) => {
                if context.wide_math {
                    Err(Error::InvalidRegister(
                        Register::A,
                        Attribute::WideMath,
                        function_name,
                    ))
                } else {
                    if *value > 0xFF {
                        Err(Error::InvalidValue(*value, function_name))
                    } else {
                        let bytes = value.to_le_bytes();
                        let instruction = [0xA9, bytes[0]]; // LDA imm
                        context
                            .bank
                            .push_code("Load A", function_name, &instruction)
                    }
                }
            }
            (Operand::Register(Register::A), Operand::Variable(var)) => {
                if context.wide_math {
                    Err(Error::InvalidRegister(
                        Register::A,
                        Attribute::WideMath,
                        function_name,
                    ))
                } else {
                    if let Some(Name::Var(addr)) = context.names.get(var) {
                        if *addr > 0xFFFF {
                            Err(Error::InvalidAddress(*addr, function_name))
                        } else {
                            let bytes = addr.to_le_bytes();
                            let instruction = [0xAD, bytes[0], bytes[1]]; // LDA abs
                            context
                                .bank
                                .push_code("Load A", function_name, &instruction)
                        }
                    } else {
                        Err(Error::UnknownVariable(var, function_name))
                    }
                }
            }
            (Operand::Register(Register::C), Operand::Immediate(value)) => {
                if context.wide_math {
                    if *value > 0xFFFF {
                        Err(Error::InvalidValue(*value, function_name))
                    } else {
                        let bytes = value.to_le_bytes();
                        let instruction = [0xA9, bytes[0], bytes[1]]; // LDA imm
                        context
                            .bank
                            .push_code("Load C", function_name, &instruction)
                    }
                } else {
                    Err(Error::InvalidRegister(
                        Register::C,
                        Attribute::NarrowMath,
                        function_name,
                    ))
                }
            }
            (Operand::Register(Register::C), Operand::Variable(var)) => {
                if context.wide_math {
                    if let Some(Name::Var(addr)) = context.names.get(var) {
                        if *addr > 0xFFFF {
                            Err(Error::InvalidAddress(*addr, function_name))
                        } else {
                            let bytes = addr.to_le_bytes();
                            let instruction = [0xAD, bytes[0], bytes[1]]; // LDA abs
                            context
                                .bank
                                .push_code("Load A", function_name, &instruction)
                        }
                    } else {
                        Err(Error::UnknownVariable(var, function_name))
                    }
                } else {
                    Err(Error::InvalidRegister(
                        Register::C,
                        Attribute::NarrowMath,
                        function_name,
                    ))
                }
            }
            (Operand::Register(Register::D), Operand::Register(Register::C)) => context
                .bank
                .push_code("Transfer C to D", function_name, &[0x5B]),
            (Operand::Register(Register::S), Operand::Register(Register::C)) => context
                .bank
                .push_code("Transfer C to S", function_name, &[0x1B]),
            (Operand::Register(Register::X), Operand::Immediate(value)) => {
                if context.wide_index {
                    if *value > 0xFFFF {
                        Err(Error::InvalidValue(*value, function_name))
                    } else {
                        let bytes = value.to_le_bytes();
                        let instruction = [0xA2, bytes[0], bytes[1]]; // LDX imm
                        context
                            .bank
                            .push_code("Load X imm", function_name, &instruction)
                    }
                } else {
                    if *value > 0xFF {
                        Err(Error::InvalidValue(*value, function_name))
                    } else {
                        let bytes = value.to_le_bytes();
                        let instruction = [0xA2, bytes[0]]; // LX imm
                        context
                            .bank
                            .push_code("Load X imm", function_name, &instruction)
                    }
                }
            }
            (Operand::Register(Register::X), Operand::Variable(var)) => {
                if let Some(Name::Var(addr)) = context.names.get(var) {
                    if *addr > 0xFFFF {
                        Err(Error::InvalidAddress(*addr, function_name))
                    } else {
                        let bytes = addr.to_le_bytes();
                        let instruction = [0xAE, bytes[0], bytes[1]]; // LDX abs
                        context
                            .bank
                            .push_code("Load X abs", function_name, &instruction)
                    }
                } else {
                    Err(Error::UnknownVariable(var, function_name))
                }
            }
            (Operand::Absolute(addr), Operand::Register(Register::A)) => {
                if *addr > 0xFFFF {
                    Err(Error::InvalidAddress(*addr, function_name))
                } else {
                    let bytes = addr.to_le_bytes();
                    let instruction = [0x8D, bytes[0], bytes[1]];
                    context
                        .bank
                        .push_code("Store A", function_name, &instruction)
                }
            }
            (Operand::Absolute(addr), Operand::Immediate(0)) => {
                if *addr > 0xFFFF {
                    Err(Error::InvalidAddress(*addr, function_name))
                } else {
                    let bytes = addr.to_le_bytes();
                    let instruction = [0x9C, bytes[0], bytes[1]];
                    context
                        .bank
                        .push_code("Store Zero", function_name, &instruction)
                }
            }
            (Operand::Variable(var), Operand::Register(Register::A)) => {
                if let Some(Name::Var(addr)) = context.names.get(var) {
                    if *addr > 0xFFFF {
                        Err(Error::InvalidAddress(*addr, function_name))
                    } else {
                        let bytes = addr.to_le_bytes();
                        let instruction = [0x8D, bytes[0], bytes[1]]; // STA abs
                        context
                            .bank
                            .push_code("Store A", function_name, &instruction)
                    }
                } else {
                    Err(Error::UnknownVariable(var, function_name))
                }
            }
            (Operand::Variable(var), Operand::Register(Register::X)) => {
                if let Some(Name::Var(addr)) = context.names.get(var) {
                    if *addr > 0xFFFF {
                        Err(Error::InvalidAddress(*addr, function_name))
                    } else {
                        let bytes = addr.to_le_bytes();
                        let instruction = [0x8E, bytes[0], bytes[1]]; // STX abs
                        context
                            .bank
                            .push_code("Store X", function_name, &instruction)
                    }
                } else {
                    Err(Error::UnknownVariable(var, function_name))
                }
            }
            (Operand::Variable(var), Operand::Immediate(0)) => {
                if let Some(Name::Var(addr)) = context.names.get(var) {
                    if *addr > 0xFFFF {
                        Err(Error::InvalidAddress(*addr, function_name))
                    } else {
                        let bytes = addr.to_le_bytes();
                        let instruction = [0x9C, bytes[0], bytes[1]]; // STZ abs
                        context
                            .bank
                            .push_code("Store Zero", function_name, &instruction)
                    }
                } else {
                    Err(Error::UnknownVariable(var, function_name))
                }
            }
            (l, r) => Err(Error::BadAssignment(l, r, function_name)),
        },
        Instruction::AndAssign(l, r) => {
            match (l, r) {
                (Operand::Register(Register::A), Operand::Immediate(value)) => {
                    if context.wide_math {
                        Err(Error::InvalidRegister(
                            Register::A,
                            Attribute::WideMath,
                            function_name,
                        ))
                    } else {
                        if *value > 0xFF {
                            Err(Error::InvalidValue(*value, function_name))
                        } else {
                            let bytes = value.to_le_bytes();
                            let instruction = [0x29, bytes[0]]; // AND imm
                            context
                                .bank
                                .push_code("And A imm", function_name, &instruction)
                        }
                    }
                }
                _ => Err(Error::BadAndAssignment(l, r, function_name)),
            }
        }
        Instruction::OrAssign(l, r) => {
            match (l, r) {
                (Operand::Register(Register::A), Operand::Immediate(value)) => {
                    if context.wide_math {
                        Err(Error::InvalidRegister(
                            Register::A,
                            Attribute::WideMath,
                            function_name,
                        ))
                    } else {
                        if *value > 0xFF {
                            Err(Error::InvalidValue(*value, function_name))
                        } else {
                            let bytes = value.to_le_bytes();
                            let instruction = [0x09, bytes[0]]; // ORA imm
                            context
                                .bank
                                .push_code("Or A imm", function_name, &instruction)
                        }
                    }
                }
                _ => Err(Error::BadOrAssignment(l, r, function_name)),
            }
        }
        Instruction::Block(block) => {
            let mut emulation = context.emulation;
            let mut wide_math = context.wide_math;
            let mut wide_index = context.wide_index;
            update_codegen(context, &block.attributes, function_name)?;
            update_emulation(context, emulation, function_name)?;
            update_mx(context, wide_math, wide_index, function_name)?;
            for instruction in &block.instructions {
                assemble_instruction(context, instruction, function_name)?;
            }
            std::mem::swap(&mut emulation, &mut context.emulation);
            std::mem::swap(&mut wide_math, &mut context.wide_math);
            std::mem::swap(&mut wide_index, &mut context.wide_index);
            update_emulation(context, emulation, function_name)?;
            update_mx(context, wide_math, wide_index, function_name)?;
            Ok(())
        }
        Instruction::Call(target) => {
            let target_fun = context.names.get(target).cloned();
            if let Some(Name::Function(addr, attributes)) = target_fun {
                let mut emulation = context.emulation;
                let mut wide_math = context.wide_math;
                let mut wide_index = context.wide_index;
                update_codegen(context, &attributes, function_name)?;
                update_emulation(context, emulation, function_name)?;
                update_mx(context, wide_math, wide_index, function_name)?;
                let addr = addr.unwrap_or_else(|| {
                    context
                        .relocations
                        .push(Relocation::Function(target, context.bank.code.len() + 1));
                    0
                });
                let bytes = addr.to_le_bytes();
                if attributes.contains(&Attribute::Extern) {
                    unimplemented!("Calling extern functions")
                } else {
                    context
                        .bank
                        .push_code("Call", function_name, &[0x20, bytes[0], bytes[1]])?;
                }
                std::mem::swap(&mut emulation, &mut context.emulation);
                std::mem::swap(&mut wide_math, &mut context.wide_math);
                std::mem::swap(&mut wide_index, &mut context.wide_index);
                update_emulation(context, emulation, function_name)?;
                update_mx(context, wide_math, wide_index, function_name)?;
                Ok(())
            } else {
                Err(Error::UnknownFunction(target, function_name))
            }
        }
        Instruction::If(block, cond) => {
            assemble_conditional(context, cond, None, true, function_name)?;
            let block_start = context.bank.code.len();
            let mut emulation = context.emulation;
            let mut wide_math = context.wide_math;
            let mut wide_index = context.wide_index;
            update_codegen(context, &block.attributes, function_name)?;
            update_emulation(context, emulation, function_name)?;
            update_mx(context, wide_math, wide_index, function_name)?;
            for instruction in &block.instructions {
                assemble_instruction(context, instruction, function_name)?;
            }
            std::mem::swap(&mut emulation, &mut context.emulation);
            std::mem::swap(&mut wide_math, &mut context.wide_math);
            std::mem::swap(&mut wide_index, &mut context.wide_index);
            update_emulation(context, emulation, function_name)?;
            update_mx(context, wide_math, wide_index, function_name)?;

            let block_len = context.bank.code.len() - block_start;
            if block_len > 127 {
                return Err(Error::IfTooLong(function_name));
            }

            let fixup = block_start - 1;
            context.bank.code[fixup] = block_len as u8;
            Ok(())
        }
        Instruction::Loop(block, cond) => {
            let mut emulation = context.emulation;
            let mut wide_math = context.wide_math;
            let mut wide_index = context.wide_index;
            update_codegen(context, &block.attributes, function_name)?;
            update_emulation(context, emulation, function_name)?;
            update_mx(context, wide_math, wide_index, function_name)?;
            let loop_start = context.bank.code.len();
            for instruction in &block.instructions {
                assemble_instruction(context, instruction, function_name)?;
            }
            context.relocations.retain(|relo| {
                if let Relocation::Break(_) = relo {
                    unimplemented!("Break relocations");
                } else {
                    true
                }
            });
            if let Some(cond) = cond {
                assemble_conditional(context, cond, Some(loop_start), false, function_name)?;
            } else {
                let loop_length = context.bank.code.len() - loop_start;
                if loop_length <= 126 {
                    // Short loop, use a BRA
                    let offset = (!(loop_length as u8 + 2)) + 1;
                    context
                        .bank
                        .push_code("Loop", function_name, &[0x80, offset])?;
                } else {
                    // Long loop, must use a JMP
                    let bytes = (loop_start + context.bank.start).to_le_bytes();
                    context
                        .bank
                        .push_code("Loop", function_name, &[0x4C, bytes[0], bytes[1]])?;
                }
            }
            std::mem::swap(&mut emulation, &mut context.emulation);
            std::mem::swap(&mut wide_math, &mut context.wide_math);
            std::mem::swap(&mut wide_index, &mut context.wide_index);
            update_emulation(context, emulation, function_name)?;
            update_mx(context, wide_math, wide_index, function_name)?;
            Ok(())
        }
        Instruction::Cli => context.bank.push_code("Cli", function_name, &[0x58]),
        Instruction::Sei => context.bank.push_code("Sei", function_name, &[0x78]),
        Instruction::Push(reg) => match reg {
            Operand::Register(Register::A) => {
                if !context.wide_math {
                    context.bank.push_code("PHA", function_name, &[0x48])
                } else {
                    Err(Error::InvalidRegister(
                        Register::A,
                        Attribute::NarrowMath,
                        function_name,
                    ))
                }
            }
            Operand::Register(Register::C) => {
                if context.wide_math {
                    context.bank.push_code("PHA", function_name, &[0x48])
                } else {
                    Err(Error::InvalidRegister(
                        Register::C,
                        Attribute::NarrowMath,
                        function_name,
                    ))
                }
            }
            Operand::Register(Register::X) => context.bank.push_code("PHX", function_name, &[0xDA]),
            Operand::Register(Register::Y) => context.bank.push_code("PHY", function_name, &[0x5A]),
            _ => Err(Error::BadPush(reg, function_name)),
        },
        Instruction::Pop(reg) => match reg {
            Operand::Register(Register::A) => {
                if !context.wide_math {
                    context.bank.push_code("PLA", function_name, &[0x68])
                } else {
                    Err(Error::InvalidRegister(
                        Register::A,
                        Attribute::NarrowMath,
                        function_name,
                    ))
                }
            }
            Operand::Register(Register::C) => {
                if context.wide_math {
                    context.bank.push_code("PLA", function_name, &[0x68])
                } else {
                    Err(Error::InvalidRegister(
                        Register::C,
                        Attribute::NarrowMath,
                        function_name,
                    ))
                }
            }
            Operand::Register(Register::X) => context.bank.push_code("PLX", function_name, &[0xFA]),
            Operand::Register(Register::Y) => context.bank.push_code("PLY", function_name, &[0x7A]),
            _ => Err(Error::BadPop(reg, function_name)),
        },
    }
}

fn assemble_conditional<'a, 'c, 'p: 'c>(
    context: &'c mut Context<'p>,
    conditional: &'p Conditional<'p>,
    target: Option<usize>,
    invert: bool,
    function_name: &'p str,
) -> Result<'p> {
    let zero = invert
        ^ match conditional {
            Conditional::NotBitTest(l, r) => match (l, r) {
                (Operand::Register(Register::A), Operand::Immediate(value)) => {
                    if context.wide_math {
                        return Err(Error::InvalidRegister(
                            Register::A,
                            Attribute::WideMath,
                            function_name,
                        ));
                    }

                    if *value > 0xFF {
                        return Err(Error::InvalidValue(*value, function_name));
                    }
                    let bytes = value.to_le_bytes();
                    let instruction = [0x89, bytes[0]]; // BIT imm
                    context
                        .bank
                        .push_code("BIT A imm", function_name, &instruction)?;

                    true
                }
                _ => return Err(Error::BadBitTest(l, r, function_name)),
            },
            Conditional::BitTest(l, r) => match (l, r) {
                (Operand::Register(Register::A), Operand::Immediate(value)) => {
                    if context.wide_math {
                        return Err(Error::InvalidRegister(
                            Register::A,
                            Attribute::WideMath,
                            function_name,
                        ));
                    }

                    if *value > 0xFF {
                        return Err(Error::InvalidValue(*value, function_name));
                    }
                    let bytes = value.to_le_bytes();
                    let instruction = [0x89, bytes[0]]; // BIT imm
                    context
                        .bank
                        .push_code("BIT A imm", function_name, &instruction)?;

                    false
                }
                (Operand::Register(Register::C), Operand::Immediate(value)) => {
                    if !context.wide_math {
                        return Err(Error::InvalidRegister(
                            Register::C,
                            Attribute::WideMath,
                            function_name,
                        ));
                    }

                    if *value > 0xFFFF {
                        return Err(Error::InvalidValue(*value, function_name));
                    }
                    let bytes = value.to_le_bytes();
                    let instruction = [0x89, bytes[0], bytes[1]]; // BIT imm
                    context
                        .bank
                        .push_code("BIT A imm", function_name, &instruction)?;

                    false
                }
                _ => return Err(Error::BadBitTest(l, r, function_name)),
            },
            Conditional::Equality(l, r) => match (l, r) {
                (Operand::Register(Register::A), Operand::Immediate(value)) => {
                    if context.wide_math {
                        return Err(Error::InvalidRegister(
                            Register::A,
                            Attribute::WideMath,
                            function_name,
                        ));
                    }

                    if *value > 0xFF {
                        return Err(Error::InvalidValue(*value, function_name));
                    }
                    let bytes = value.to_le_bytes();
                    let instruction = [0xC9, bytes[0]]; // CMP imm
                    context
                        .bank
                        .push_code("CMP A imm", function_name, &instruction)?;

                    true
                }
                _ => return Err(Error::BadEquality(l, r, function_name)),
            },
        };
    let loop_length = target
        .map(|target| context.bank.code.len() - target)
        .unwrap_or(0);
    let offset = (!(loop_length as u8 + 2)) + 1;
    if loop_length > 126 {
        return Err(Error::LoopTooLong(function_name));
    }
    let opcode = if zero { 0xF0 } else { 0xD0 };
    context
        .bank
        .push_code("Loop CC", function_name, &[opcode, offset])?;
    Ok(())
}

fn update_codegen<'a, 'c, 'p: 'c>(
    context: &'c mut Context<'p>,
    attributes: &'a [Attribute],
    function_name: &'p str,
) -> Result<'p> {
    for attr in attributes {
        match attr {
            Attribute::Emulation => {
                if attributes.contains(&Attribute::Native) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::Emulation,
                        Attribute::Native,
                        function_name,
                    ));
                }
                if attributes.contains(&Attribute::WideIndex) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::NarrowIndex,
                        Attribute::WideIndex,
                        function_name,
                    ));
                }
                if attributes.contains(&Attribute::WideMath) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::NarrowMath,
                        Attribute::WideMath,
                        function_name,
                    ));
                }
                context.emulation = true;
            }
            Attribute::NarrowIndex => {
                if attributes.contains(&Attribute::WideIndex) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::NarrowIndex,
                        Attribute::WideIndex,
                        function_name,
                    ));
                }
                context.wide_index = false;
            }
            Attribute::NarrowMath => {
                if attributes.contains(&Attribute::WideMath) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::NarrowMath,
                        Attribute::WideMath,
                        function_name,
                    ));
                }
                context.wide_math = false;
            }
            Attribute::Native => {
                if attributes.contains(&Attribute::Emulation) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::Native,
                        Attribute::Emulation,
                        function_name,
                    ));
                }
                context.emulation = false;
            }
            Attribute::WideIndex => {
                if attributes.contains(&Attribute::NarrowIndex) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::WideIndex,
                        Attribute::NarrowIndex,
                        function_name,
                    ));
                }
                if attributes.contains(&Attribute::Emulation) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::Native,
                        Attribute::Emulation,
                        function_name,
                    ));
                }
                context.emulation = false;
                context.wide_index = true;
            }
            Attribute::WideMath => {
                if attributes.contains(&Attribute::NarrowMath) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::WideMath,
                        Attribute::NarrowMath,
                        function_name,
                    ));
                }
                if attributes.contains(&Attribute::Emulation) {
                    return Err(Error::ConflictingAttributes(
                        Attribute::Native,
                        Attribute::Emulation,
                        function_name,
                    ));
                }
                context.emulation = false;
                context.wide_math = true;
            }
            _ => {}
        }
    }
    Ok(())
}

fn update_emulation<'c, 'p: 'c>(
    context: &'c mut Context<'p>,
    emulation: bool,
    function_name: &'p str,
) -> Result<'p> {
    if emulation != context.emulation {
        if context.emulation {
            context
                .bank
                .push_code("Enable Emulation", function_name, &[0x38, 0xFB])?;
        } else {
            context
                .bank
                .push_code("Disable Emulation", function_name, &[0x18, 0xFB])?;
        }
    }
    Ok(())
}

fn update_mx<'c, 'p: 'c>(
    context: &'c mut Context<'p>,
    wide_math: bool,
    wide_index: bool,
    function_name: &'p str,
) -> Result<'p> {
    if context.wide_math == context.wide_index {
        // Set both, even if one didn't change. Easier codegen
        if context.wide_math != wide_math && context.wide_math {
            context
                .bank
                .push_code("Enable Wide Math + Index", function_name, &[0xC2, 0x30])?;
        } else if context.wide_math != wide_math {
            context
                .bank
                .push_code("Disable wide math + index", function_name, &[0xE2, 0x30])?;
        }
    } else {
        if wide_math != context.wide_math {
            if context.wide_math {
                context
                    .bank
                    .push_code("Enable Wide Math", function_name, &[0xC2, 0x20])?;
            } else {
                context
                    .bank
                    .push_code("Disable Wide Math", function_name, &[0xE2, 0x20])?;
            }
        }

        if wide_index != context.wide_index {
            if context.wide_index {
                context
                    .bank
                    .push_code("Enable Wide Index", function_name, &[0xC2, 0x10])?;
            } else {
                context
                    .bank
                    .push_code("Disable Wide Index", function_name, &[0xE2, 0x10])?;
            }
        }
    }
    Ok(())
}

impl Bank {
    fn push_code<'c, 'p: 'c, 'a>(
        &'c mut self,
        operation: &'static str,
        function_name: &'p str,
        code: &'a [u8],
    ) -> Result<'p> {
        if self.code.len() + code.len() > self.len {
            return Err(Error::NoSpace(operation, function_name));
        }
        self.code.extend_from_slice(code);
        Ok(())
    }
}
