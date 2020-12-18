use std::io::{self, Read};

use peg;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

peg::parser! {
    grammar parser_p1() for str {
        pub rule eval_expr() -> u64 = precedence!{
                x:(@) _ "+" _ y:@ { x + y }
                x:(@) _ "*" _ y:@ { x * y }
                "(" _ e:eval_expr() _ ")" { e }
                n:num() { n }
        }

        rule num() -> u64
            = x:$([ASCII_DIGIT]) { x.parse::<u64>().unwrap() }

        rule _() = " "?
    }
}

peg::parser! {
    grammar parser_p2() for str {
        pub rule eval_expr() -> u64 = precedence!{
                x:(@) _ "*" _ y:@ { x * y }
                --
                x:(@) _ "+" _ y:@ { x + y }
                --
                "(" _ e:eval_expr() _ ")" { e }
                n:num() { n }
        }

        rule num() -> u64
            = x:$([ASCII_DIGIT]) { x.parse::<u64>().unwrap() }

        rule _() = " "?
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let start = std::time::Instant::now();
    part1(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    let start = std::time::Instant::now();
    part2(&input)?;
    eprintln!("elapsed {:?}", start.elapsed());

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let soln: u64 = input
        .lines()
        .map(|l| {
            let n = parser_p1::eval_expr(l)?;
            Ok(n)
        })
        .sum::<Result<_>>()?;
    println!("part 1 solution: {}", soln);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let soln: u64 = input
        .lines()
        .map(|l| {
            let n = parser_p2::eval_expr(l)?;
            Ok(n)
        })
        .sum::<Result<_>>()?;
    println!("part 2 solution: {}", soln);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let expr = "2 * 3 + (4 * 5)";
        let soln = parser_p1::eval_expr(expr).unwrap();
        assert!(soln == 26);

        let expr = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let soln = parser_p1::eval_expr(expr).unwrap();
        assert!(soln == 437);

        let expr = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let soln = parser_p1::eval_expr(expr).unwrap();
        assert!(soln == 12240);

        let expr = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let soln = parser_p1::eval_expr(expr).unwrap();
        assert!(soln == 13632);
    }

    #[test]
    fn part2_test() {
        let expr = "1 + (2 * 3) + (4 * (5 + 6))";
        let soln = parser_p2::eval_expr(expr).unwrap();
        assert!(soln == 51);

        let expr = "2 * 3 + (4 * 5)";
        let soln = parser_p2::eval_expr(expr).unwrap();
        assert!(soln == 46);

        let expr = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let soln = parser_p2::eval_expr(expr).unwrap();
        assert!(soln == 1445);

        let expr = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let soln = parser_p2::eval_expr(expr).unwrap();
        assert!(soln == 669060);

        let expr = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let soln = parser_p2::eval_expr(expr).unwrap();
        assert!(soln == 23340);
    }
}
