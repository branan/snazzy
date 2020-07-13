use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, tag_no_case, take_until},
    character::complete::{digit1, hex_digit1, multispace0, oct_digit1},
    combinator::{all_consuming, complete, cut, map, opt, value},
    error::context,
    multi::{many0, separated_list},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};

use super::ast::*;

pub type Error<'a> = nom::Err<nom::error::VerboseError<&'a str>>;
type IResult<'a, T> = Result<(&'a str, T), Error<'a>>;

pub fn program(input: &str) -> Result<Program, nom::Err<nom::error::VerboseError<&str>>> {
    let (_, definitions) =
        complete(all_consuming(terminated(many0(definition), multispace0)))(input)?;
    Ok(Program { definitions })
}

fn definition(input: &str) -> IResult<Definition> {
    context(
        "definition",
        alt((
            preceded(ws(tag("FUN")), map(cut(function), Definition::Function)),
            preceded(ws(tag("VAR")), map(cut(var), Definition::Var)),
        )),
    )(input)
}

fn function(input: &str) -> IResult<Function> {
    context(
        "function",
        map(pair(ws(identifier), block), |(name, body)| Function {
            name,
            body,
        }),
    )(input)
}

fn var(input: &str) -> IResult<Var> {
    map(
        terminated(
            separated_pair(ws(identifier), ws(tag(":=")), ws(number)),
            ws(tag(";")),
        ),
        |(name, address)| Var { name, address },
    )(input)
}

fn block(input: &str) -> IResult<Block> {
    context(
        "block",
        map(
            pair(
                attributes,
                context(
                    "body",
                    delimited(
                        context("open", ws(tag("{"))),
                        context("inner", many0(instruction)),
                        context("close", ws(tag("}"))),
                    ),
                ),
            ),
            |(attributes, instructions)| Block {
                attributes,
                instructions,
            },
        ),
    )(input)
}

fn attributes(input: &str) -> IResult<Vec<Attribute>> {
    context(
        "attributes",
        map(
            opt(delimited(
                ws(tag("[")),
                separated_list(ws(tag(",")), ws(attribute)),
                ws(tag("]")),
            )),
            |attributes| attributes.unwrap_or_else(Vec::new),
        ),
    )(input)
}

fn attribute(input: &str) -> IResult<Attribute> {
    alt((
        value(Attribute::Emulation, tag("EMU")),
        value(Attribute::Extern, tag("EXTERN")),
        value(Attribute::Interrupt, tag("INTR")),
        value(Attribute::Native, tag("NAT")),
        value(Attribute::NarrowIndex, tag("NARROWX")),
        value(Attribute::NarrowMath, tag("NARROWM")),
        value(Attribute::WideIndex, tag("WIDEX")),
        value(Attribute::WideMath, tag("WIDEM")),
    ))(input)
}

fn instruction(input: &str) -> IResult<Instruction> {
    context(
        "instruction",
        alt((
            terminated(
                alt((
                    assign,
                    and_assign,
                    or_assign,
                    call,
                    push,
                    pop,
                    value(Instruction::Sei, ws(tag("SEI"))),
                    value(Instruction::Cli, ws(tag("CLI"))),
                )),
                ws(tag(";")),
            ),
            do_loop,
            if_block,
            map(block, Instruction::Block),
        )),
    )(input)
}

fn do_loop(input: &str) -> IResult<Instruction> {
    map(
        preceded(
            ws(tag("DO")),
            pair(block, opt(preceded(ws(tag("WHILE")), conditional))),
        ),
        |(block, cond)| Instruction::Loop(block, cond),
    )(input)
}

fn if_block(input: &str) -> IResult<Instruction> {
    map(
        preceded(ws(tag("IF")), pair(conditional, block)),
        |(cond, block)| Instruction::If(block, cond),
    )(input)
}

fn assign(input: &str) -> IResult<Instruction> {
    map(
        separated_pair(ws(operand), ws(tag(":=")), ws(operand)),
        |(l, r)| Instruction::Assign(l, r),
    )(input)
}

fn and_assign(input: &str) -> IResult<Instruction> {
    map(
        separated_pair(ws(operand), ws(tag("&=")), ws(operand)),
        |(l, r)| Instruction::AndAssign(l, r),
    )(input)
}

fn or_assign(input: &str) -> IResult<Instruction> {
    map(
        separated_pair(ws(operand), ws(tag("|=")), ws(operand)),
        |(l, r)| Instruction::OrAssign(l, r),
    )(input)
}

fn call(input: &str) -> IResult<Instruction> {
    map(terminated(ws(identifier), ws(tag("()"))), Instruction::Call)(input)
}

fn push(input: &str) -> IResult<Instruction> {
    map(preceded(ws(tag("PUSH")), ws(operand)), Instruction::Push)(input)
}

fn pop(input: &str) -> IResult<Instruction> {
    map(preceded(ws(tag("POP")), ws(operand)), Instruction::Pop)(input)
}

fn conditional(input: &str) -> IResult<Conditional> {
    context(
        "conditional",
        delimited(
            ws(tag("(")),
            alt((equality, bit_test, not_bit_test)),
            ws(tag(")")),
        ),
    )(input)
}

fn bit_test(input: &str) -> IResult<Conditional> {
    context(
        "bit_test",
        map(
            separated_pair(ws(operand), ws(tag("&&")), ws(operand)),
            |(lhs, rhs)| Conditional::BitTest(lhs, rhs),
        ),
    )(input)
}

fn not_bit_test(input: &str) -> IResult<Conditional> {
    context(
        "not_bit_test",
        map(
            separated_pair(ws(operand), ws(tag("!&")), ws(operand)),
            |(lhs, rhs)| Conditional::NotBitTest(lhs, rhs),
        ),
    )(input)
}

fn equality(input: &str) -> IResult<Conditional> {
    context(
        "equality",
        map(
            separated_pair(ws(operand), ws(tag("==")), ws(operand)),
            |(lhs, rhs)| Conditional::Equality(lhs, rhs),
        ),
    )(input)
}

fn operand(input: &str) -> IResult<Operand> {
    context(
        "operand",
        alt((
            map(number, Operand::Immediate),
            map(preceded(tag("*"), number), Operand::Absolute),
            map(register, Operand::Register),
            map(identifier, Operand::Variable),
        )),
    )(input)
}

fn register(input: &str) -> IResult<Register> {
    context(
        "register",
        alt((
            value(Register::A, tag("A")),
            value(Register::B, tag("B")),
            value(Register::C, tag("C")),
            value(Register::X, tag("X")),
            value(Register::Y, tag("Y")),
            value(Register::S, tag("S")),
            value(Register::D, tag("D")),
            value(Register::DB, tag("DB")),
            value(Register::PB, tag("PB")),
        )),
    )(input)
}

fn identifier(input: &str) -> IResult<&str> {
    is_a("abcdefghijklmnopqrstuvwxyz_")(input)
}

fn number(input: &str) -> IResult<u32> {
    context("number", alt((octal, hexadecimal, decimal)))(input)
}

fn decimal(input: &str) -> IResult<u32> {
    map(digit1, |digits| u32::from_str_radix(digits, 10).unwrap())(input)
}

fn hexadecimal(input: &str) -> IResult<u32> {
    map(preceded(tag_no_case("0x"), hex_digit1), |digits| {
        u32::from_str_radix(digits, 16).unwrap()
    })(input)
}

fn octal(input: &str) -> IResult<u32> {
    map(preceded(tag_no_case("0o"), oct_digit1), |digits| {
        u32::from_str_radix(digits, 8).unwrap()
    })(input)
}

fn ws<'a, T, F: Fn(&'a str) -> IResult<'a, T>>(
    combinator: F,
) -> impl Fn(&'a str) -> IResult<'a, T> {
    preceded(comment, combinator)
}

fn comment(input: &str) -> IResult<Vec<&str>> {
    context(
        "comment",
        preceded(
            multispace0,
            many0(delimited(tag("#"), take_until("\n"), multispace0)),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use nom::combinator::{all_consuming, complete};

    #[test]
    fn var() {
        let result = complete(all_consuming(super::var))("identifier := 100;");
        assert_eq!(
            result,
            Ok((
                "",
                Var {
                    address: 100,
                    name: "identifier"
                }
            ))
        );
    }

    #[test]
    fn empty_function() {
        let result = complete(all_consuming(super::function))("main [] {}");
        assert_eq!(
            result,
            Ok((
                "",
                Function {
                    body: Block {
                        attributes: vec![],
                        instructions: vec![]
                    },
                    name: "main"
                }
            ))
        );
    }
}
