fn main() -> eyre::Result<()> {
    let passes = passes()?;
    let ids = passes.into_iter().map(|p| seat_id(&p));

    // first part
    let max = ids.clone().max().unwrap();
    println!("Max seatID = {}", max);

    // second part
    let mut sorted = ids.collect::<Vec<i32>>();
    sorted.sort_unstable();
    let first = sorted[0];

    for (idx, id) in sorted.iter().enumerate() {
        let maybe = first + idx as i32;
        if maybe == *id {
            continue;
        }

        println!("My seatID = {}", maybe);
        break;
    }

    Ok(())
}

fn seat_id(pass: &str) -> i32 {
    let mut row = 0;
    for (idx, c) in pass[0..7].chars().enumerate() {
        match c {
            'B' => row += 1 << (6 - idx),
            'F' => continue,
            _ => unreachable!(),
        }
    }

    let mut col = 0;
    for (idx, c) in pass[7..].chars().enumerate() {
        match c {
            'R' => col += 1 << (2 - idx),
            'L' => continue,
            _ => unreachable!(),
        }
    }

    row * 8 + col
}

fn passes() -> eyre::Result<Vec<String>> {
    use std::fs::File;
    use std::io::{BufRead, Lines};

    let lines: Lines<_> = std::io::BufReader::new(File::open("data/05.txt")?).lines();
    Ok(lines.map(|l| l.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(567, seat_id("BFFFBBFRRR"));
        assert_eq!(119, seat_id("FFFBBBFRRR"));
        assert_eq!(820, seat_id("BBFFBBFRLL"));
    }
}
