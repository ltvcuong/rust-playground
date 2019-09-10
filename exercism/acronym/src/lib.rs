use std::iter;

pub fn abbreviate(phrase: &str) -> String {
    iter::once(' ')
        .chain(phrase.chars().into_iter())
        .collect::<Vec<char>>()
        .windows(2)
        .filter(|c| {
            (!c[0].is_alphabetic() && c[1].is_alphabetic())
                || (c[0].is_lowercase() && c[1].is_uppercase())
        })
        .map(|c| c[1].to_ascii_uppercase())
        .collect()
}
