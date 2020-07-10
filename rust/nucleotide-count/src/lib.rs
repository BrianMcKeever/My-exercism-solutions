use std::collections::HashMap;

fn is_nucleotide(letter: char) -> bool {
    ['A', 'C', 'G', 'T'].contains(&letter)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }
    for c in dna.chars() {
        //it's dumb that we have to validate the dna string in count.
        if !is_nucleotide(c) {
            return Err(c);
        }
    }
    Ok(dna.chars().filter(|x| *x == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    match (
        count('A', dna),
        count('C', dna),
        count('G', dna),
        count('T', dna),
    ) {
        (Err(a), _, _, _) => Err(a),
        /*
        (_, Err(c), _, _) => Err(c),
        (_, _, Err(g), _) => Err(g),
        (_, _, _, Err(t)) => Err(t),
        */
        (Ok(a), Ok(c), Ok(g), Ok(t)) => {
            let mut result = HashMap::new();
            result.insert('A', a);
            result.insert('C', c);
            result.insert('G', g);
            result.insert('T', t);

            Ok(result)
        }
        _ => panic!("This case should be impossible"),
    }
}
