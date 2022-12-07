pub fn jp_era(wareki: &str) -> Result<i32, String> {
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

        assert_eq!(jp_era(""), err);
        assert_eq!(jp_era("M"), err);
        assert_eq!(jp_era("M1"), err);
        assert_eq!(jp_era("M100"), err);
    }

    #[test]
    fn unknown_era() {
        let err = Err("Unknown era.".to_string());

        assert_eq!(jp_era("A01"), err);
        assert_eq!(jp_era("001"), err);
    }

    #[test]
    fn less_than_one() {
        let err = Err("Year is more than 1.".to_string());

        assert_eq!(jp_era("A00"), err);
        assert_eq!(jp_era("000"), err);

        assert_eq!(jp_era("M00"), err);
        assert_eq!(jp_era("T00"), err);
        assert_eq!(jp_era("S00"), err);
        assert_eq!(jp_era("H00"), err);
        assert_eq!(jp_era("R00"), err);
    }

    #[test]
    fn meiji() {
        assert_eq!(jp_era("M01"), Ok(1868));
        assert_eq!(jp_era("101"), Ok(1868));

        assert_eq!(jp_era("M25"), Ok(1892));
        assert_eq!(jp_era("125"), Ok(1892));

        assert_eq!(jp_era("M45"), Ok(1912));
        assert_eq!(jp_era("145"), Ok(1912));
    }

    #[test]
    fn meiji_err() {
        let err = Err("Meiji until 45.".to_string());

        assert_eq!(jp_era("M46"), err);
        assert_eq!(jp_era("146"), err);
        assert_eq!(jp_era("M99"), err);
    }

    #[test]
    fn taisho() {
        assert_eq!(jp_era("T01"), Ok(1912));
        assert_eq!(jp_era("201"), Ok(1912));

        assert_eq!(jp_era("T08"), Ok(1919));
        assert_eq!(jp_era("208"), Ok(1919));

        assert_eq!(jp_era("T15"), Ok(1926));
        assert_eq!(jp_era("215"), Ok(1926));
    }

    #[test]
    fn taisho_err() {
        let err = Err("Taisho until 15.".to_string());

        assert_eq!(jp_era("T16"), err);
        assert_eq!(jp_era("216"), err);
        assert_eq!(jp_era("T99"), err);
    }

    #[test]
    fn showa() {
        assert_eq!(jp_era("S01"), Ok(1926));
        assert_eq!(jp_era("301"), Ok(1926));

        assert_eq!(jp_era("S33"), Ok(1958));
        assert_eq!(jp_era("333"), Ok(1958));

        assert_eq!(jp_era("S64"), Ok(1989));
        assert_eq!(jp_era("364"), Ok(1989));
    }

    #[test]
    fn showa_err() {
        let err = Err("Showa until 64.".to_string());

        assert_eq!(jp_era("S65"), err);
        assert_eq!(jp_era("365"), err);
        assert_eq!(jp_era("S99"), err);
    }

    #[test]
    fn heisei() {
        assert_eq!(jp_era("H01"), Ok(1989));
        assert_eq!(jp_era("401"), Ok(1989));

        assert_eq!(jp_era("H16"), Ok(2004));
        assert_eq!(jp_era("416"), Ok(2004));

        assert_eq!(jp_era("H31"), Ok(2019));
        assert_eq!(jp_era("431"), Ok(2019));
    }

    #[test]
    fn heisei_err() {
        let err = Err("Heisei until 31.".to_string());

        assert_eq!(jp_era("H32"), err);
        assert_eq!(jp_era("432"), err);
        assert_eq!(jp_era("H99"), err);
    }

    #[test]
    fn reiwa() {
        assert_eq!(jp_era("R01"), Ok(2019));
        assert_eq!(jp_era("501"), Ok(2019));

        assert_eq!(jp_era("R07"), Ok(2025));
        assert_eq!(jp_era("507"), Ok(2025));

        assert_eq!(jp_era("R33"), Ok(2051));
        assert_eq!(jp_era("533"), Ok(2051));
    }
}
