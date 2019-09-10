use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {}
        x => return Err(x),
    };

    let mut count = 0;
    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {
                if c == nucleotide {
                    count += 1
                }
            }
            x => return Err(x),
        };
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res = "ACGT"
        .chars()
        .map(|c| (c, 0_usize))
        .collect::<HashMap<_, _>>();

    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => res.insert(c, res.get(&c).unwrap() + 1),
            x => return Err(x),
        };
    }
    Ok(res)
}
