use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult, Parser,
};

// Multiplication of all divisibility tests for my input
const MAGIC_CONSTANT: u64 = 11 * 2 * 5 * 17 * 19 * 7 * 3 * 13;

#[derive(Debug)]
pub enum Operation {
    Add((Value, Value)),
    Multiply((Value, Value)),
}

#[derive(Debug)]
pub enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
pub struct Test {
    divisible_by: u64,
    pass_if_true: u8,
    pass_if_false: u8,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub activity: u64,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn inspect(&mut self) -> u64 {
        self.activity += 1;
        let item = self.items.pop().unwrap();
        let get_value = |v: &Value| match v {
            Value::Old => item,
            Value::Num(num) => *num,
        };

        // // part 1 solution
        // (match &self.operation {
        //     Operation::Add((a, b)) => get_value(a) + get_value(b),
        //     Operation::Multiply((a, b)) => {
        //         get_value(a) * get_value(b)
        //     }
        // }) / 3

        match &self.operation {
            Operation::Add((a, b)) => {
                get_value(a) + get_value(b) % MAGIC_CONSTANT
            }
            Operation::Multiply((a, b)) => {
                get_value(a) * get_value(b) % MAGIC_CONSTANT
            }
        }
    }

    pub fn test(&self, item: u64) -> usize {
        match item % self.test.divisible_by {
            0 => self.test.pass_if_true as usize,
            _ => self.test.pass_if_false as usize,
        }
    }
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, op) = alt((tag("+"), tag("*")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value) = alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64.map(Value::Num),
    ))(input)?;

    let result = match op {
        "+" => Operation::Add((Value::Old, value)),
        "*" => Operation::Multiply((Value::Old, value)),
        _ => panic!(),
    };

    Ok((input, result))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible_by) = preceded(
        tag("Test: divisible by "),
        nom::character::complete::u64,
    )(input)?;

    let (input, _) = multispace1(input)?;

    let (input, pass_if_true) = preceded(
        tag("If true: throw to monkey "),
        nom::character::complete::u8,
    )(input)?;

    let (input, _) = multispace1(input)?;

    let (input, pass_if_false) = preceded(
        tag("If false: throw to monkey "),
        nom::character::complete::u8,
    )(input)?;

    Ok((
        input,
        Test {
            divisible_by,
            pass_if_true,
            pass_if_false,
        },
    ))
}

pub fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(
        tag("Monkey "),
        nom::character::complete::u8,
        tag(":"),
    )(input)?;

    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list0(tag(", "), nom::character::complete::u64),
    )(input)?;

    let (input, _) = multispace1(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;

    Ok((
        input,
        Monkey {
            items,
            activity: 0,
            operation,
            test,
        },
    ))
}
