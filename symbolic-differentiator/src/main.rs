use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Term {
    coef: i32,
    power: i32,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term { coef, power: 0 } => {
                write!(f, "{}", coef)
            }
            Term { coef, power: 1 } => {
                write!(f, "{}x", coef)
            }
            Term { coef, power } => {
                write!(f, "{}x^{}", coef, power)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TermParseError {
    InvalidFormat,
    ParseIntError(std::num::ParseIntError),
}

impl fmt::Display for TermParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TermParseError::InvalidFormat => write!(f, "Invalid format for Term"),
            TermParseError::ParseIntError(e) => write!(f, "Integer parsing error: {}", e),
        }
    }
}

impl Error for TermParseError {}

impl From<std::num::ParseIntError> for TermParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        TermParseError::ParseIntError(err)
    }
}

impl FromStr for Term {
    type Err = TermParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.replace("^", "");
        let parts = binding.split("x").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(TermParseError::InvalidFormat);
        }

        let coef: i32 = match parts[0].parse::<i32>() {
            Ok(num) => Ok(num),
            Err(e) => match e.kind() {
                std::num::IntErrorKind::Empty => Ok(1),
                _ => Err(e),
            },
        }?;
        let power: i32 = match parts[1].parse::<i32>() {
            Ok(num) => Ok(num),
            Err(e) => match e.kind() {
                std::num::IntErrorKind::Empty => Ok(1),
                _ => Err(e),
            },
        }?;

        Ok(Term { coef, power })
    }
}

type Polynominal = Vec<Term>;

fn differentiate(input: String) -> String {
    input
        .replace(" ", "")
        .replace("-", "+-")
        .split("+")
        .into_iter()
        .map(|term| {
            term.parse::<Term>()
                .unwrap_or_else(|_| Term { coef: 0, power: 0 })
        })
        .map(power_rule)
        .map(|term| format!("{}", term))
        .collect::<Vec<String>>()
        .join(" + ")
        .replace(" + 0", "")
        .replace(" + -", " - ")
        .replace("0 - ", "-")
}

fn power_rule(Term { coef, power }: Term) -> Term {
    if power == 0 {
        return Term { coef: 0, power: 0 };
    }

    let next_coef = power * coef;
    let next_power = power - 1;

    Term {
        coef: next_coef,
        power: next_power,
    }
}

fn main() {
    println!("Hello, world!");
}

// f(x) = x^2 + 3x
// f'(x) = 2x + 3
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let input_1 = "3x^2";
        let input_2 = "2x";
        let input_3 = "x^3";
        let input_4 = "";

        let result_1 = input_1.parse::<Term>();
        let result_2 = input_2.parse::<Term>();
        let result_3 = input_3.parse::<Term>();
        let result_4 = input_4.parse::<Term>();

        let expected_1 = Ok(Term { coef: 3, power: 2 });
        let expected_2 = Ok(Term { coef: 2, power: 1 });
        let expected_3 = Ok(Term { coef: 1, power: 3 });
        let expected_4 = Err(TermParseError::InvalidFormat);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
    }

    #[test]
    fn test_power_rule() {
        let input_1 = Term { coef: 3, power: 2 };
        let input_2 = Term { coef: 4, power: 1 };
        let input_3 = Term { coef: 5, power: 0 };

        let result_1 = power_rule(input_1);
        let result_2 = power_rule(input_2);
        let result_3 = power_rule(input_3);

        let expected_1 = Term { coef: 6, power: 1 };
        let expected_2 = Term { coef: 4, power: 0 };
        let expected_3 = Term { coef: 0, power: 0 };

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_differential() {
        let input_1: String = "x^2 + 3x".to_string();
        let input_2: String = "x + 3".to_string();
        let input_3: String = "10x^2 - 5x + 2".to_string();
        let input_4: String = "-4x^2".to_string();
        // let input_5: String = "gibberish".to_string();

        let result_1 = differentiate(input_1);
        let result_2 = differentiate(input_2);
        let result_3 = differentiate(input_3);
        let result_4 = differentiate(input_4);
        // let result_5 = differentiate(input_5);

        let expected_1: String = "2x + 3".to_string();
        let expected_2: String = "1".to_string();
        let expected_3: String = "20x - 5".to_string();
        let expected_4: String = "-8x".to_string();
        // let expected_5: String = "-8x".to_string();

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        // assert_eq!(result_5, expected_5);
    }
}
