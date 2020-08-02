#[derive(PartialEq, Debug)]
struct Chaussure {
    pointure: u32,
    style: String,
}

fn chaussures_a_la_pointure(chaussures: Vec<Chaussure>, pointure_chaussure: u32) -> Vec<Chaussure> {
    chaussures.into_iter()
              .filter(|s| s.pointure == pointure_chaussure)
              .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtres_par_pointure() {
        let chaussures = vec![
            Chaussure {
                pointure: 10,
                style: String::from("baskets"),
            },
            Chaussure {
                pointure: 13,
                style: String::from("sandale"),
            },
            Chaussure {
                pointure: 10,
                style: String::from("bottes"),
            },
        ];

        let a_ma_pointure = chaussures_a_la_pointure(chaussures, 10);

        assert_eq!(
            a_ma_pointure,
            vec![
                Chaussure {
                    pointure: 10,
                    style: String::from("baskets")
                },
                Chaussure {
                    pointure: 10,
                    style: String::from("bottes")
                },
            ]
        );
    }
}

fn main() {}
