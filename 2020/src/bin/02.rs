fn main() -> eyre::Result<()> {
    // let sample: Vec<Record> = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
    //     .iter()
    //     .map(|l| parse_line(l).unwrap())
    //     .collect();
    // let records = sample;

    let records = read_data()?;
    let count = records
        .into_iter()
        // .filter(|rec| rec.valid_for_sled_rental()) // first part
        .filter(|rec| rec.valid_for_toboggan()) // second part
        .count();
    println!("There are {} valid records", count);

    Ok(())
}

#[derive(Debug)]
struct Policy {
    n1: usize,
    n2: usize,
    letter: String,
}

impl Policy {
    fn accepts_range(&self, password: &String) -> bool {
        let count = password
            .chars()
            .filter(|c| c.to_string() == self.letter)
            .count();
        self.n1 <= count && count <= self.n2
    }

    fn accepts_position(&self, password: &String) -> bool {
        let match_1 = password
            .chars()
            .nth(self.n1 - 1)
            .map_or(false, |c| c.to_string() == self.letter);
        let match_2 = password
            .chars()
            .nth(self.n2 - 1)
            .map_or(false, |c| c.to_string() == self.letter);

        match_1 ^ match_2
    }
}

#[derive(Debug)]
struct Record {
    policy: Policy,
    password: String,
}

impl Record {
    fn valid_for_sled_rental(&self) -> bool {
        self.policy.accepts_range(&self.password)
    }

    fn valid_for_toboggan(&self) -> bool {
        self.policy.accepts_position(&self.password)
    }
}

fn read_data() -> eyre::Result<Vec<Record>> {
    use std::fs::File;
    use std::io::{BufRead, Lines};

    let lines: Lines<_> = std::io::BufReader::new(File::open("data/02.txt")?).lines();

    let records = lines.map(|l| parse_line(l.unwrap()).unwrap()).collect();

    Ok(records)
}

fn parse_line<L: AsRef<str>>(line: L) -> eyre::Result<Record> {
    // line parts
    // 0: "1-3"   -> min-max
    // 1: "a"     -> letter
    // 2: ""      -> contiguous separator returns an empty string
    // 3: "abcde" -> password
    let parts = line
        .as_ref()
        .split(|c| c == ' ' || c == ':')
        .collect::<Vec<&str>>();

    let pol = parts[0].split('-').collect::<Vec<&str>>();
    Ok(Record {
        password: parts[3].to_owned(),
        policy: Policy {
            letter: parts[1].to_owned(),
            n1: pol[0].parse().unwrap(),
            n2: pol[1].parse().unwrap(),
        },
    })
}
