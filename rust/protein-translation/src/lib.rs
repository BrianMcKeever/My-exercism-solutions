use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        let rna_vec: Vec<char> = rna.chars().collect();
        let codons: Vec<Option<&str>> = rna_vec
            .chunks(3)
            .map(|chunk| self.name_for(chunk.iter().copied().collect::<String>().as_str()))
            .take_while(|x| match x {
                Some(x) => *x != "stop codon",
                None => true,
            })
            .collect();
        if codons.iter().any(|x| x.is_none()) {
            return None;
        }
        let codons: Vec<&str> = codons
            .iter()
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        if codons.is_empty() {
            return None;
        }
        Some(codons)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut map = HashMap::new();
    for (codon, name) in pairs {
        map.insert(codon, name);
    }
    CodonsInfo { pairs: map }
}
