use snazzy::{ast::*, parser};

#[test]
fn empty() {
    assert!(parser::program("").is_ok())
}

#[test]
fn simple() {
    let expected = Program {
        definitions: vec![
            Definition::Var(Var {
                address: 4096,
                name: "reg",
            }),
            Definition::Function(Function {
                name: "main",
                body: Block {
                    attributes: vec![],
                    instructions: vec![
                        Instruction::Assign(Operand::Register(Register::A), Operand::Immediate(48)),
                        Instruction::Assign(
                            Operand::Variable("reg"),
                            Operand::Register(Register::A),
                        ),
                    ],
                },
            }),
        ],
    };
    let result = parser::program(include_str!("input/simple.snz"));
    assert_eq!(result, Ok(expected));
}

#[test]
fn snes() {
    let expected = Program {
        definitions: vec![
            Definition::Function(Function {
                name: "cop",
                body: Block {
                    attributes: vec![Attribute::Interrupt],
                    instructions: vec![],
                },
            }),
            Definition::Function(Function {
                name: "brk",
                body: Block {
                    attributes: vec![Attribute::Interrupt],
                    instructions: vec![],
                },
            }),
            Definition::Function(Function {
                name: "irq",
                body: Block {
                    attributes: vec![Attribute::Interrupt],
                    instructions: vec![],
                },
            }),
            Definition::Function(Function {
                name: "cop_emu",
                body: Block {
                    attributes: vec![Attribute::Emulation, Attribute::Interrupt],
                    instructions: vec![],
                },
            }),
            Definition::Function(Function {
                name: "nmi_emu",
                body: Block {
                    attributes: vec![Attribute::Emulation, Attribute::Interrupt],
                    instructions: vec![],
                },
            }),
            Definition::Function(Function {
                name: "irq_emu",
                body: Block {
                    attributes: vec![Attribute::Emulation, Attribute::Interrupt],
                    instructions: vec![],
                },
            }),
            Definition::Function(Function {
                name: "reset",
                body: Block {
                    attributes: vec![Attribute::Emulation, Attribute::Interrupt],
                    instructions: vec![
                        Instruction::Sei,
                        Instruction::Block(Block {
                            attributes: vec![Attribute::Native],
                            instructions: vec![
                                Instruction::Block(Block {
                                    attributes: vec![Attribute::WideMath],
                                    instructions: vec![
                                        Instruction::Assign(
                                            Operand::Register(Register::C),
                                            Operand::Immediate(0x01FF),
                                        ),
                                        Instruction::Assign(
                                            Operand::Register(Register::S),
                                            Operand::Register(Register::C),
                                        ),
                                        Instruction::Assign(
                                            Operand::Register(Register::C),
                                            Operand::Immediate(0),
                                        ),
                                        Instruction::Assign(
                                            Operand::Register(Register::D),
                                            Operand::Register(Register::C),
                                        ),
                                    ],
                                }),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(0x8F),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2100),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2101),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2102),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2103),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2105),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2106),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2107),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2108),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2109),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210A),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210B),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210C),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210D),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210D),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(0xFF),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210E),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2110),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2112),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2114),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(0x07),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210E),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2110),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2112),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2114),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210F),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x210F),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2111),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2111),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2113),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2113),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(0x80),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2115),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2116),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2117),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211A),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211B),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(1),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211B),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211C),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211C),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211D),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211D),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211E),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211E),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211F),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x211F),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2120),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2120),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2121),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2123),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2124),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2125),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2126),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2127),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2128),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2129),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x212A),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x212B),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x212C),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x212D),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x212E),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x212F),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(0x30),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2130),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2131),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(0xE0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2132),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x2133),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Register(Register::A),
                                    Operand::Immediate(0xFF),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4200),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4201),
                                    Operand::Register(Register::A),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4202),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4203),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4204),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4205),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4206),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4207),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4208),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x4209),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x420A),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x420B),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x420C),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Assign(
                                    Operand::Absolute(0x420D),
                                    Operand::Immediate(0),
                                ),
                                Instruction::Cli,
                                Instruction::Call("main"),
                            ],
                        }),
                    ],
                },
            }),
            Definition::Var(Var {
                address: 0x2100,
                name: "inidisp",
            }),
            Definition::Var(Var {
                address: 0x2122,
                name: "cgdata",
            }),
            Definition::Var(Var {
                address: 0x4200,
                name: "nmitimen",
            }),
            Definition::Var(Var {
                address: 0x0000,
                name: "status",
            }),
            Definition::Function(Function {
                name: "main",
                body: Block {
                    attributes: vec![],
                    instructions: vec![
                        Instruction::Assign(
                            Operand::Register(Register::A),
                            Operand::Immediate(0x3C),
                        ),
                        Instruction::Assign(Operand::Variable("cgdata"), Operand::Immediate(0)),
                        Instruction::Assign(
                            Operand::Variable("cgdata"),
                            Operand::Register(Register::A),
                        ),
                        Instruction::Assign(Operand::Register(Register::A), Operand::Immediate(2)),
                        Instruction::Assign(
                            Operand::Variable("status"),
                            Operand::Register(Register::A),
                        ),
                        Instruction::Assign(
                            Operand::Register(Register::A),
                            Operand::Immediate(0b1000_0001),
                        ),
                        Instruction::Assign(
                            Operand::Variable("nmitimen"),
                            Operand::Register(Register::A),
                        ),
                        Instruction::Loop(
                            Block {
                                attributes: vec![],
                                instructions: vec![
                                    Instruction::Loop(
                                        Block {
                                            attributes: vec![],
                                            instructions: vec![Instruction::Assign(
                                                Operand::Register(Register::A),
                                                Operand::Variable("status"),
                                            )],
                                        },
                                        Some(Conditional::NotBitTest(
                                            Operand::Register(Register::A),
                                            Operand::Immediate(1),
                                        )),
                                    ),
                                    Instruction::Assign(
                                        Operand::Register(Register::A),
                                        Operand::Variable("status"),
                                    ),
                                    Instruction::OrAssign(
                                        Operand::Register(Register::A),
                                        Operand::Immediate(0b0000_0010),
                                    ),
                                    Instruction::AndAssign(
                                        Operand::Register(Register::A),
                                        Operand::Immediate(0b1111_1110),
                                    ),
                                    Instruction::Assign(
                                        Operand::Variable("status"),
                                        Operand::Register(Register::A),
                                    ),
                                ],
                            },
                            None,
                        ),
                    ],
                },
            }),
            Definition::Function(Function {
                name: "nmi",
                body: Block {
                    attributes: vec![Attribute::Interrupt],
                    instructions: vec![
                        Instruction::Block(Block {
                            attributes: vec![Attribute::WideMath, Attribute::WideIndex],
                            instructions: vec![
                                Instruction::Push(Operand::Register(Register::C)),
                                Instruction::Push(Operand::Register(Register::X)),
                                Instruction::Push(Operand::Register(Register::Y)),
                            ],
                        }),
                        Instruction::Assign(
                            Operand::Register(Register::A),
                            Operand::Variable("status"),
                        ),
                        Instruction::If(
                            Block {
                                attributes: vec![],
                                instructions: vec![
                                    Instruction::Assign(
                                        Operand::Register(Register::A),
                                        Operand::Immediate(0x8F),
                                    ),
                                    Instruction::Assign(
                                        Operand::Variable("inidisp"),
                                        Operand::Register(Register::A),
                                    ),
                                    Instruction::Assign(
                                        Operand::Register(Register::A),
                                        Operand::Immediate(0x0F),
                                    ),
                                    Instruction::Assign(
                                        Operand::Variable("inidisp"),
                                        Operand::Register(Register::A),
                                    ),
                                    Instruction::Assign(
                                        Operand::Register(Register::A),
                                        Operand::Variable("status"),
                                    ),
                                    Instruction::OrAssign(
                                        Operand::Register(Register::A),
                                        Operand::Immediate(1),
                                    ),
                                    Instruction::AndAssign(
                                        Operand::Register(Register::A),
                                        Operand::Immediate(0b1111_1101),
                                    ),
                                    Instruction::Assign(
                                        Operand::Variable("status"),
                                        Operand::Register(Register::A),
                                    ),
                                ],
                            },
                            Conditional::BitTest(
                                Operand::Register(Register::A),
                                Operand::Immediate(2),
                            ),
                        ),
                        Instruction::Block(Block {
                            attributes: vec![Attribute::WideMath, Attribute::WideIndex],
                            instructions: vec![
                                Instruction::Pop(Operand::Register(Register::Y)),
                                Instruction::Pop(Operand::Register(Register::X)),
                                Instruction::Pop(Operand::Register(Register::C)),
                            ],
                        }),
                    ],
                },
            }),
        ],
    };

    let result = parser::program(include_str!("input/snes.snz"));
    assert_eq!(result, Ok(expected));
}
