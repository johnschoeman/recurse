use std::error::Error;
use std::fmt;
use std::str::FromStr;

// ---- Term
#[derive(Clone, Debug, PartialEq)]
struct Term {
    coef: i32,
    power: i32,
}

fn is_non_zero_term(term: &Term) -> bool {
    term.coef != 0 || term.power != 0
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term { coef, power: 0 } => {
                write!(f, "{}", coef)
            }
            Term { coef: 1, power: 1 } => {
                write!(f, "x")
            }
            Term { coef: 1, power } => {
                write!(f, "x^{}", power)
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
    TooManyElements,
    InvalidFormat,
    ParseIntError(std::num::ParseIntError),
}

impl fmt::Display for TermParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TermParseError::TooManyElements => write!(f, "Too many elements matched"),
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

        if parts.len() > 2 {
            return Err(TermParseError::TooManyElements);
        }

        let coef_raw: &str = match parts.get(0) {
            Some(s) => s,
            None => "1",
        };

        let power_raw: &str = match parts.get(1) {
            Some(s) => s,
            None => "0",
        };

        let coef: i32 = match coef_raw.parse::<i32>() {
            Ok(num) => Ok(num),
            Err(e) => match e.kind() {
                std::num::IntErrorKind::Empty => Ok(1),
                _ => Err(e),
            },
        }?;
        let power: i32 = match power_raw.parse::<i32>() {
            Ok(num) => Ok(num),
            Err(e) => match e.kind() {
                std::num::IntErrorKind::Empty => Ok(1),
                _ => Err(e),
            },
        }?;

        Ok(Term { coef, power })
    }
}

// ---- Polynomial
#[derive(Debug, PartialEq)]
struct Polynomial {
    terms: Vec<Term>,
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let terms = self
            .terms
            .clone()
            .into_iter()
            .map(|term| format!("{}", term))
            .collect::<Vec<String>>()
            .join(" + ")
            .replace(" + 0", "")
            .replace(" + -", " - ")
            .replace("0 - ", "-");
        write!(f, "{}", terms)
    }
}

#[derive(Debug, PartialEq)]
pub enum PolynomialParseError {
    InvalidFormat,
    ParseIntError(std::num::ParseIntError),
    TermParseError(TermParseError),
}

impl fmt::Display for PolynomialParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PolynomialParseError::InvalidFormat => write!(f, "Invalid format for Polynomial"),
            PolynomialParseError::TermParseError(e) => write!(f, "Term parsing error: {}", e),
            PolynomialParseError::ParseIntError(e) => write!(f, "Int parsing error: {}", e),
        }
    }
}

impl Error for PolynomialParseError {}

impl From<std::num::ParseIntError> for PolynomialParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        PolynomialParseError::ParseIntError(err)
    }
}

impl From<TermParseError> for PolynomialParseError {
    fn from(err: TermParseError) -> Self {
        PolynomialParseError::TermParseError(err)
    }
}

impl FromStr for Polynomial {
    type Err = PolynomialParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let terms = s
            .replace("- ", "+-")
            .replace(" ", "")
            .split("+")
            .into_iter()
            .map(|term| match term.parse::<Term>() {
                Ok(t) => Ok(t),
                Err(e) => Err(PolynomialParseError::TermParseError(e)),
            })
            .into_iter()
            .collect::<Result<Vec<Term>, PolynomialParseError>>()?;

        Ok(Polynomial { terms })
    }
}

fn differentiate(input: Polynomial) -> Polynomial {
    let next_terms = input
        .terms
        .into_iter()
        .map(power_rule)
        .filter(is_non_zero_term)
        .collect::<Vec<Term>>();

    Polynomial { terms: next_terms }
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
        let expected_1 = Ok(Term { coef: 3, power: 2 });

        let input_2 = "2x";
        let expected_2 = Ok(Term { coef: 2, power: 1 });

        let input_3 = "x^3";
        let expected_3 = Ok(Term { coef: 1, power: 3 });

        let input_4 = "3";
        let expected_4 = Ok(Term { coef: 3, power: 0 });

        let result_1 = input_1.parse::<Term>();
        let result_2 = input_2.parse::<Term>();
        let result_3 = input_3.parse::<Term>();
        let result_4 = input_4.parse::<Term>();

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
    fn test_parse_polynomial() {
        let input_1: String = "x^2 + 3x".to_string();
        let expected_1 = Ok(Polynomial {
            terms: [Term { coef: 1, power: 2 }, Term { coef: 3, power: 1 }].to_vec(),
        });

        let input_2: String = "x + 3".to_string();
        let expected_2 = Ok(Polynomial {
            terms: [Term { coef: 1, power: 1 }, Term { coef: 3, power: 0 }].to_vec(),
        });

        let input_3: String = "10x^2 - 5x + 2".to_string();
        let expected_3 = Ok(Polynomial {
            terms: [
                Term { coef: 10, power: 2 },
                Term { coef: -5, power: 1 },
                Term { coef: 2, power: 0 },
            ]
            .to_vec(),
        });

        let input_4: String = "-4x^2".to_string();
        let expected_4 = Ok(Polynomial {
            terms: [Term { coef: -4, power: 2 }].to_vec(),
        });

        let input_5: String = "gibberish".to_string();

        let result_1 = input_1.parse();
        let result_2 = input_2.parse();
        let result_3 = input_3.parse();
        let result_4 = input_4.parse();
        let result_5: Result<Polynomial, PolynomialParseError> = input_5.parse();

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert!(result_5.is_err());
        let result_5_err = result_5.unwrap_err();
        match result_5_err {
            PolynomialParseError::TermParseError(_) => {}
            e => panic!("Wrong Parse Error Type Raised: {}", e),
        }
    }

    #[test]
    fn test_format_polynomial() -> Result<(), Box<dyn std::error::Error>> {
        let input_1 = "x^2 + 3x".parse::<Polynomial>()?;
        let input_2 = "-4x".parse::<Polynomial>()?;
        let input_3 = "123x^456".parse::<Polynomial>()?;

        let result_1 = format!("{}", input_1);
        let result_2 = format!("{}", input_2);
        let result_3 = format!("{}", input_3);

        let expected_1 = "x^2 + 3x";
        let expected_2 = "-4x";
        let expected_3 = "123x^456";

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);

        Ok(())
    }

    #[test]
    fn test_differential() -> Result<(), Box<dyn std::error::Error>> {
        let input_1 = "x^2 + 3x".parse::<Polynomial>()?;
        let input_2 = "x + 3".parse::<Polynomial>()?;
        let input_3 = "10x^2 - 5x + 2".parse::<Polynomial>()?;
        let input_4 = "-4x^2".parse::<Polynomial>()?;

        let result_1 = differentiate(input_1);
        let result_2 = differentiate(input_2);
        let result_3 = differentiate(input_3);
        let result_4 = differentiate(input_4);

        let expected_1 = "2x + 3".parse::<Polynomial>()?;
        let expected_2 = "1".parse::<Polynomial>()?;
        let expected_3 = "20x - 5".parse::<Polynomial>()?;
        let expected_4 = "-8x".parse::<Polynomial>()?;

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);

        Ok(())
    }
}
