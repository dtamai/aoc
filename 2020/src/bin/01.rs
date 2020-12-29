fn main() -> eyre::Result<()> {
    // let numbers = &mut [1721, 979, 366, 299, 675, 1456];

    let mut numbers = read_data()?;
    numbers.sort();

    for (idx_i, i) in numbers.iter().enumerate() {
        for (idx_j, j) in numbers[idx_i..].iter().enumerate() {
            // let nuple = &[*i, *j]; // first part
            // loop for the second part
            for k in numbers[idx_j..].iter() {
                let nuple = &[*i, *j, *k]; // second part
                match nuple.iter().sum::<i32>().cmp(&2020) {
                    // common part
                    std::cmp::Ordering::Greater => break,
                    std::cmp::Ordering::Equal => {
                        println!("sum({:?}) = 2020", nuple);
                        println!("product({:?}) = {}", nuple, nuple.iter().product::<i32>());
                        return Ok(());
                    }
                    std::cmp::Ordering::Less => continue,
                }
            } // end of second part
        }
    }

    Err(eyre::eyre!("No answer found!"))
}

fn read_data() -> eyre::Result<Vec<i32>> {
    use std::fs::File;
    use std::io::{BufRead, Lines};

    let lines: Lines<_> = std::io::BufReader::new(File::open("data/01.txt")?).lines();
    let numbers = lines
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Ok(numbers)
}
