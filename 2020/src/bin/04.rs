fn main() -> eyre::Result<()> {
    let raw = std::fs::read_to_string("data/04.txt")?;

    // first part
    let batch: Batch<Passport> = Batch::parse(&raw);
    println!("There are {} valid passports", batch.number_of_valid());

    // second part
    let batch: Batch<StrictPassport> = Batch::parse(&raw);
    println!(
        "There are {} valid passports with strich validation",
        batch.number_of_valid()
    );

    Ok(())
}

trait PassportValidation {
    fn is_valid(&self) -> bool;
}

struct Batch<P>(Vec<P>);

impl<'r, P> Batch<P>
where
    P: PassportValidation + From<&'r str>,
{
    fn parse(raw: &'r str) -> Self {
        let list = raw.split("\n\n").map(|raw| P::from(raw)).collect();

        Self(list)
    }

    fn number_of_valid(&self) -> usize {
        self.0.iter().map(P::is_valid).filter(|&ok| ok).count()
    }
}

#[derive(Debug, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl PassportValidation for Passport {
    fn is_valid(&self) -> bool {
        !self.byr.is_empty()
            && !self.iyr.is_empty()
            && !self.eyr.is_empty()
            && !self.hgt.is_empty()
            && !self.hcl.is_empty()
            && !self.ecl.is_empty()
            && !self.pid.is_empty()
    }
}

impl From<&str> for Passport {
    fn from(raw: &str) -> Self {
        let mut passport = Passport::default();
        for raw_pair in raw.split_whitespace() {
            let parts: Vec<&str> = raw_pair.split(":").collect();
            match parts.as_slice() {
                ["byr", v] => passport.byr = v.to_string(),
                ["iyr", v] => passport.iyr = v.to_string(),
                ["eyr", v] => passport.eyr = v.to_string(),
                ["hgt", v] => passport.hgt = v.to_string(),
                ["hcl", v] => passport.hcl = v.to_string(),
                ["ecl", v] => passport.ecl = v.to_string(),
                ["pid", v] => passport.pid = v.to_string(),
                ["cid", v] => passport.cid = v.to_string(),
                [] | [..] => unreachable!(),
            };
        }

        passport
    }
}

trait PassportField {
    type Value;

    fn value(&self) -> Self::Value;
    fn parse(raw: &str) -> Self;
    fn is_valid(&self) -> bool;
}

#[derive(Default)]
struct BirthYear(Option<i16>);

impl PassportField for BirthYear {
    type Value = Option<i16>;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn parse(raw: &str) -> Self {
        let year = raw.parse::<i16>();
        match year {
            Ok(y) => Self(Some(y)),
            _ => Self(None),
        }
    }

    fn is_valid(&self) -> bool {
        if let Some(year) = self.value() {
            1920 <= year && year <= 2002
        } else {
            false
        }
    }
}

#[derive(Default)]
struct IssueYear(Option<i16>);

impl PassportField for IssueYear {
    type Value = Option<i16>;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn parse(raw: &str) -> Self {
        let year = raw.parse::<i16>();
        match year {
            Ok(y) => Self(Some(y)),
            _ => Self(None),
        }
    }

    fn is_valid(&self) -> bool {
        if let Some(year) = self.value() {
            2010 <= year && year <= 2020
        } else {
            false
        }
    }
}

#[derive(Default)]
struct ExpirationYear(Option<i16>);

impl PassportField for ExpirationYear {
    type Value = Option<i16>;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn parse(raw: &str) -> Self {
        let year = raw.parse::<i16>();
        match year {
            Ok(y) => Self(Some(y)),
            _ => Self(None),
        }
    }

    fn is_valid(&self) -> bool {
        if let Some(year) = self.value() {
            2020 <= year && year <= 2030
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum HeightUnit {
    In(i16),
    Cm(i16),
}

#[derive(Default)]
struct Height(Option<HeightUnit>);

impl PassportField for Height {
    type Value = Option<HeightUnit>;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn parse(raw: &str) -> Self {
        let len = raw.len();

        if len <= 2 {
            return Self(None);
        }

        let val = &raw[..(len - 2)];
        let val = val.parse::<i16>();
        if val.is_err() {
            return Self(None);
        }
        let val = val.unwrap();

        let unit = &raw[(len - 2)..];
        match unit {
            "cm" => Self(Some(HeightUnit::Cm(val))),
            "in" => Self(Some(HeightUnit::In(val))),
            _ => Self(None),
        }
    }

    fn is_valid(&self) -> bool {
        if self.value().is_none() {
            return false;
        }

        let val = self.value().unwrap();

        match val {
            HeightUnit::Cm(h) => 150 <= h && h <= 193,
            HeightUnit::In(h) => 59 <= h && h <= 76,
        }
    }
}

#[derive(Default)]
struct HairColor(Option<String>);

impl PassportField for HairColor {
    type Value = Option<String>;

    fn value(&self) -> Self::Value {
        self.0.clone()
    }

    fn parse(raw: &str) -> Self {
        Self(Some(raw.to_owned()))
    }

    fn is_valid(&self) -> bool {
        if self.value().is_none() {
            return false;
        }

        let val = self.value().unwrap();

        if val.len() != 7 {
            return false;
        }

        if &val[0..1] != "#" {
            return false;
        }

        val[1..].chars().into_iter().all(|c| c.is_ascii_hexdigit())
    }
}

#[derive(Clone, Copy, Debug)]
enum EyeColors {
    Ambar,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

#[derive(Default)]
struct EyeColor(Option<EyeColors>);

impl PassportField for EyeColor {
    type Value = Option<EyeColors>;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn parse(raw: &str) -> Self {
        use EyeColors::*;

        match raw {
            "amb" => Self(Some(Ambar)),
            "blu" => Self(Some(Blue)),
            "brn" => Self(Some(Brown)),
            "gry" => Self(Some(Grey)),
            "grn" => Self(Some(Green)),
            "hzl" => Self(Some(Hazel)),
            "oth" => Self(Some(Other)),
            _ => Self(None),
        }
    }

    fn is_valid(&self) -> bool {
        self.value().is_some()
    }
}

#[derive(Default)]
struct PassportID(Option<String>);

impl PassportField for PassportID {
    type Value = Option<String>;

    fn value(&self) -> Self::Value {
        self.0.clone()
    }

    fn parse(raw: &str) -> Self {
        Self(Some(raw.to_owned()))
    }

    fn is_valid(&self) -> bool {
        if self.value().is_none() {
            return false;
        }

        let val = self.value().unwrap();

        if val.len() != 9 {
            return false;
        }

        val.chars().all(|c| c.is_ascii_digit())
    }
}

#[derive(Default)]
struct StrictPassport {
    birth_year: BirthYear,
    issue_year: IssueYear,
    expiration_year: ExpirationYear,
    height: Height,
    hair_color: HairColor,
    eye_color: EyeColor,
    passport_id: PassportID,
    country_id: String,
}

impl PassportValidation for StrictPassport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_valid()
            && self.issue_year.is_valid()
            && self.expiration_year.is_valid()
            && self.height.is_valid()
            && self.hair_color.is_valid()
            && self.eye_color.is_valid()
            && self.passport_id.is_valid()
    }
}

impl From<&str> for StrictPassport {
    fn from(raw: &str) -> Self {
        let mut passport = StrictPassport::default();
        for raw_pair in raw.split_whitespace() {
            let parts: Vec<&str> = raw_pair.split(":").collect();
            match parts.as_slice() {
                ["byr", v] => passport.birth_year = BirthYear::parse(v),
                ["iyr", v] => passport.issue_year = IssueYear::parse(v),
                ["eyr", v] => passport.expiration_year = ExpirationYear::parse(v),
                ["hgt", v] => passport.height = Height::parse(v),
                ["hcl", v] => passport.hair_color = HairColor::parse(v),
                ["ecl", v] => passport.eye_color = EyeColor::parse(v),
                ["pid", v] => passport.passport_id = PassportID::parse(v),
                ["cid", v] => passport.country_id = v.to_string(),
                [] | [..] => unreachable!(),
            };
        }

        passport
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
              byr:1937 iyr:2017 cid:147 hgt:183cm\n\
              \n\
              iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
              hcl:#cfa07d byr:1929\n\
              \n\
              hcl:#ae17e1 iyr:2013\n\
              eyr:2024\n\
              ecl:brn pid:760753108 byr:1931\n\
              hgt:179cm\n\
              \n\
              hcl:#cfa07d eyr:2025 pid:166559648\n\
              iyr:2011 ecl:brn hgt:59in\n";

        let batch: Batch<Passport> = Batch::parse(data);

        assert_eq!(2, batch.number_of_valid());
    }

    macro_rules! assert_valid {
        ($expected:expr, $field:expr) => {
            assert_eq!(
                $expected,
                $field.is_valid(),
                "expected {:?} to be valid",
                $field.value()
            );
        };
    }

    #[test]
    fn birth_year() {
        assert_valid!(true, BirthYear::parse("2002"));
        assert_valid!(false, BirthYear::parse("2003"));
    }

    #[test]
    fn height() {
        assert_valid!(true, Height::parse("60in"));
        assert_valid!(true, Height::parse("190cm"));
        assert_valid!(false, Height::parse("190in"));
        assert_valid!(false, Height::parse("190"));
    }

    #[test]
    fn hair_color() {
        assert_valid!(true, HairColor::parse("#123abc"));
        assert_valid!(false, HairColor::parse("#123abz"));
        assert_valid!(false, HairColor::parse("123abc"));
    }

    #[test]
    fn eye_color() {
        assert_valid!(true, EyeColor::parse("brn"));
        assert_valid!(false, EyeColor::parse("wat"));
    }

    #[test]
    fn passport_id() {
        assert_valid!(true, PassportID::parse("000000001"));
        assert_valid!(false, PassportID::parse("0123456789"));
    }

    #[test]
    fn invalid_strict_examples() {
        let data = "eyr:1972 cid:100\n\
              hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
              \n\
              iyr:2019\n\
              hcl:#602927 eyr:1967 hgt:170cm\n\
              ecl:grn pid:012533040 byr:1946\n\
              \n\
              hcl:dab227 iyr:2012\n\
              ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
              \n\
              hgt:59cm ecl:zzz\n\
              eyr:2038 hcl:74454a iyr:2023\n\
              pid:3556412378 byr:2007";

        let batch: Batch<StrictPassport> = Batch::parse(data);

        assert_eq!(0, batch.number_of_valid());
    }

    #[test]
    fn valid_strict_examples() {
        let data = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
            hcl:#623a2f\n\
            \n\
            eyr:2029 ecl:blu cid:129 byr:1989\n\
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
            \n\
            hcl:#888785\n\
            hgt:164cm byr:2001 iyr:2015 cid:88\n\
            pid:545766238 ecl:hzl\n\
            eyr:2022\n\
            \n\
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let batch: Batch<StrictPassport> = Batch::parse(data);

        assert_eq!(5, batch.number_of_valid());
    }
}
