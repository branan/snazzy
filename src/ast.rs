#[derive(Clone, Debug, PartialEq)]
pub struct Program<'a> {
    pub definitions: Vec<Definition<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Definition<'a> {
    Function(Function<'a>),
    Var(Var<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Var<'a> {
    pub address: u32,
    pub name: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function<'a> {
    pub body: Block<'a>,
    pub name: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    pub attributes: Vec<Attribute>,
    pub instructions: Vec<Instruction<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Emulation,
    Extern,
    Interrupt,
    NarrowIndex,
    NarrowMath,
    Native,
    WideIndex,
    WideMath,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction<'a> {
    Assign(Operand<'a>, Operand<'a>),
    Block(Block<'a>),
    Call(&'a str),
    Loop(Block<'a>),
    Cli,
    Sei,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    A,
    B,
    C,
    X,
    Y,

    S,
    D,

    DB,
    PB,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operand<'a> {
    Immediate(u32),
    Absolute(u32),
    Register(Register),
    Variable(&'a str),
}
