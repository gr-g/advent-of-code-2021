// This function executes a generic list of ALU instructions.
// It is only used to verify the found solutions.
fn execute(program: &str, input: &[i64]) -> (i64, i64, i64, i64) {
    let (mut w, mut x, mut y, mut z) = (0, 0, 0, 0);
    let mut i = 0;
    let mut arg1;
    let mut arg2 = 0;

    for l in program.lines() {
        let mut l = l.split(' ');
        let (op, a1, a2) = (l.next().unwrap(), l.next(), l.next());

        if op != "inp" {
            arg2 = match a2.unwrap() {
                "w" => w,
                "x" => x,
                "y" => y,
                "z" => z,
                s => s.parse().unwrap(),
            };
        }

        arg1 = match a1.unwrap() {
            "w" => &mut w,
            "x" => &mut x,
            "y" => &mut y,
            "z" => &mut z,
            _ => panic!(),
        };

        match op {
            "inp" => { *arg1 = input[i]; i += 1; },
            "add" => { *arg1 += arg2; },
            "mul" => { *arg1 *= arg2; },
            "div" => { *arg1 /= arg2; },
            "mod" => { *arg1 %= arg2; },
            "eql" => { *arg1 = if *arg1 == arg2 { 1 } else { 0 }; },
            _ => panic!(),
        }
    }

    (w, x, y, z)
}

// A MONAD program is a set of 14 steps, each step being one of two
// possible operations:
//  - Push(n): "push" a value input[i]+n to z (z = z*26 + input[i]+n)
//  - PopIf(m): "pop" a value k from z (k = z%26; z = z/26), but only
//              if input[i] = k+m.
#[derive(Clone, Copy, Debug)]
enum MonadStep {
    Push(i64),
    PopIf(i64),
}

fn monad_decode(program: &str) -> Option<[MonadStep; 14]> {
    let mut result = [MonadStep::Push(0); 14];

    let mut lines = program.lines();
    for i in 0..14 {
        if lines.next()? != "inp w" { return None; }
        if lines.next()? != "mul x 0" { return None; }
        if lines.next()? != "add x z" { return None; }
        if lines.next()? != "mod x 26" { return None; }
        let v1 = lines.next()?.strip_prefix("div z ")?.parse().ok()?;
        let v2 = lines.next()?.strip_prefix("add x ")?.parse().ok()?;
        if lines.next()? != "eql x w" { return None; }
        if lines.next()? != "eql x 0" { return None; }
        if lines.next()? != "mul y 0" { return None; }
        if lines.next()? != "add y 25" { return None; }
        if lines.next()? != "mul y x" { return None; }
        if lines.next()? != "add y 1" { return None; }
        if lines.next()? != "mul z y" { return None; }
        if lines.next()? != "mul y 0" { return None; }
        if lines.next()? != "add y w" { return None; }
        let v3 = lines.next()?.strip_prefix("add y ")?.parse().ok()?;
        if lines.next()? != "mul y x" { return None; }
        if lines.next()? != "add z y" { return None; }

        result[i] = match v1 {
            1 if v2 >= 10 => MonadStep::Push(v3),
            26 => MonadStep::PopIf(v2),
            _ => return None,
        };
    }

    if !lines.next().is_none() { return None };

    Some(result)
}

// Computes the minimum and maximum valid inputs for a MONAD program.
fn monad_valid_inputs(monad_program: &[MonadStep; 14]) -> Option<([i64; 14], [i64; 14])> {
    let mut min_input = [1; 14];
    let mut max_input = [9; 14];
    let mut stack = vec![];

    for i in 0..14 {
        match monad_program[i] {
            MonadStep::Push(n) => {
                // input[i] + n is pushed to z.
                stack.push((i, n));
            },
            MonadStep::PopIf(m) => {
                let (j, n) = stack.pop()?;

                // A value is popped from z only if input[i] = input[j] + n + m.
                // Therefore input[i] and input[j] must be separated by n + m.
                if n + m < -8 || n + m > 8 {
                    return None;
                }
                if n + m >= 0 {
                    min_input[i] = 1 + n + m;
                    max_input[j] = 9 - n - m;
                } else {
                    max_input[i] = 9 + n + m;
                    min_input[j] = 1 - n - m;
                }
            },
        }
    }

    Some((min_input, max_input))
}

fn solve(input: &str) -> (i64, i64) {
    let monad_program = monad_decode(input).expect("Program not recognized.");
    let (min_valid_input, max_valid_input) = monad_valid_inputs(&monad_program).expect("No solutions found.");

    // Verify the solutions.
    assert_eq!(execute(input, &min_valid_input).3, 0);
    assert_eq!(execute(input, &max_valid_input).3, 0);

    // Convert the inputs to model numbers.
    let mut min_number = 0;
    let mut max_number = 0;
    for i in 0..14 {
        min_number *= 10; min_number += min_valid_input[i];
        max_number *= 10; max_number += max_valid_input[i];
    }

    (max_number, min_number)
}

fn main() {
    let input = std::fs::read_to_string("input/24.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(execute("\
inp x
mul x -1", &[123]), (0, -123, 0, 0));
        assert_eq!(execute("\
inp z
inp x
mul z 3
eql z x", &[4, 12]), (0, 12, 0, 1));
        assert_eq!(execute("\
inp z
inp x
mul z 3
eql z x", &[5, 12]), (0, 12, 0, 0));
        assert_eq!(execute("\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2", &[13]), (1, 1, 0, 1));
    }
}
