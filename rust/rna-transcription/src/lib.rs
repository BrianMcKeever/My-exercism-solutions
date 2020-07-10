#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, n) in dna.chars().enumerate() {
            match n {
                'A' | 'C' | 'G' | 'T' => (),
                _ => return Err(i),
            }
        }
        Ok(DNA {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            rna: self
                .dna
                .chars()
                .map(|x| match x {
                    'A' => 'U',
                    'C' => 'G',
                    'G' => 'C',
                    'T' => 'A',
                    _ => panic!("Impossible branch"),
                })
                .collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, n) in rna.chars().enumerate() {
            match n {
                'A' | 'C' | 'G' | 'U' => (),
                _ => return Err(i),
            }
        }
        Ok(RNA {
            rna: rna.to_string(),
        })
    }
}
