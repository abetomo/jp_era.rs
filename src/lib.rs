pub fn to_gregorian_year(wareki: &str) -> Result<i32, String> {
    if wareki.chars().count() != 3 {
        return Err("`wareki` must have 3 characters.".to_string());
    }

    let year: i32 = wareki[1..3].parse().unwrap();
    if year < 1 {
        return Err("Year is more than 1.".to_string());
    }

    let prefix = &wareki[..1];
    match prefix {
        "M" | "1" => {
            if year > 45 {
                return Err("Meiji until 45.".to_string());
            }
            Ok(year + 1867)
        }
        "T" | "2" => {
            if year > 15 {
                return Err("Taisho until 15.".to_string());
            }
            Ok(year + 1911)
        }
        "S" | "3" => {
            if year > 64 {
                return Err("Showa until 64.".to_string());
            }
            Ok(year + 1925)
        }
        "H" | "4" => {
            if year > 31 {
                return Err("Heisei until 31.".to_string());
            }
            Ok(year + 1988)
        }
        "R" | "5" => {
            // if year > x {
            //     return Err("Reiwa until x.".to_string());
            // }
            Ok(year + 2018)
        }
        _ => Err("Unknown era.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_character_count() {
        let err = Err("`wareki` must have 3 characters.".to_string());

        assert_eq!(to_gregorian_year(""), err);
        assert_eq!(to_gregorian_year("M"), err);
        assert_eq!(to_gregorian_year("M1"), err);
        assert_eq!(to_gregorian_year("M100"), err);
    }

    #[test]
    fn unknown_era() {
        let err = Err("Unknown era.".to_string());

        assert_eq!(to_gregorian_year("A01"), err);
        assert_eq!(to_gregorian_year("001"), err);
    }

    #[test]
    fn less_than_one() {
        let err = Err("Year is more than 1.".to_string());

        assert_eq!(to_gregorian_year("A00"), err);
        assert_eq!(to_gregorian_year("000"), err);

        assert_eq!(to_gregorian_year("M00"), err);
        assert_eq!(to_gregorian_year("T00"), err);
        assert_eq!(to_gregorian_year("S00"), err);
        assert_eq!(to_gregorian_year("H00"), err);
        assert_eq!(to_gregorian_year("R00"), err);
    }

    #[test]
    fn meiji() {
        assert_eq!(to_gregorian_year("M01"), Ok(1868));
        assert_eq!(to_gregorian_year("101"), Ok(1868));

        assert_eq!(to_gregorian_year("M25"), Ok(1892));
        assert_eq!(to_gregorian_year("125"), Ok(1892));

        assert_eq!(to_gregorian_year("M45"), Ok(1912));
        assert_eq!(to_gregorian_year("145"), Ok(1912));
    }

    #[test]
    fn meiji_err() {
        let err = Err("Meiji until 45.".to_string());

        assert_eq!(to_gregorian_year("M46"), err);
        assert_eq!(to_gregorian_year("146"), err);
        assert_eq!(to_gregorian_year("M99"), err);
    }

    #[test]
    fn taisho() {
        assert_eq!(to_gregorian_year("T01"), Ok(1912));
        assert_eq!(to_gregorian_year("201"), Ok(1912));

        assert_eq!(to_gregorian_year("T08"), Ok(1919));
        assert_eq!(to_gregorian_year("208"), Ok(1919));

        assert_eq!(to_gregorian_year("T15"), Ok(1926));
        assert_eq!(to_gregorian_year("215"), Ok(1926));
    }

    #[test]
    fn taisho_err() {
        let err = Err("Taisho until 15.".to_string());

        assert_eq!(to_gregorian_year("T16"), err);
        assert_eq!(to_gregorian_year("216"), err);
        assert_eq!(to_gregorian_year("T99"), err);
    }

    #[test]
    fn showa() {
        assert_eq!(to_gregorian_year("S01"), Ok(1926));
        assert_eq!(to_gregorian_year("301"), Ok(1926));

        assert_eq!(to_gregorian_year("S33"), Ok(1958));
        assert_eq!(to_gregorian_year("333"), Ok(1958));

        assert_eq!(to_gregorian_year("S64"), Ok(1989));
        assert_eq!(to_gregorian_year("364"), Ok(1989));
    }

    #[test]
    fn showa_err() {
        let err = Err("Showa until 64.".to_string());

        assert_eq!(to_gregorian_year("S65"), err);
        assert_eq!(to_gregorian_year("365"), err);
        assert_eq!(to_gregorian_year("S99"), err);
    }

    #[test]
    fn heisei() {
        assert_eq!(to_gregorian_year("H01"), Ok(1989));
        assert_eq!(to_gregorian_year("401"), Ok(1989));

        assert_eq!(to_gregorian_year("H16"), Ok(2004));
        assert_eq!(to_gregorian_year("416"), Ok(2004));

        assert_eq!(to_gregorian_year("H31"), Ok(2019));
        assert_eq!(to_gregorian_year("431"), Ok(2019));
    }

    #[test]
    fn heisei_err() {
        let err = Err("Heisei until 31.".to_string());

        assert_eq!(to_gregorian_year("H32"), err);
        assert_eq!(to_gregorian_year("432"), err);
        assert_eq!(to_gregorian_year("H99"), err);
    }

    #[test]
    fn reiwa() {
        assert_eq!(to_gregorian_year("R01"), Ok(2019));
        assert_eq!(to_gregorian_year("501"), Ok(2019));

        assert_eq!(to_gregorian_year("R07"), Ok(2025));
        assert_eq!(to_gregorian_year("507"), Ok(2025));

        assert_eq!(to_gregorian_year("R33"), Ok(2051));
        assert_eq!(to_gregorian_year("533"), Ok(2051));
    }
}
