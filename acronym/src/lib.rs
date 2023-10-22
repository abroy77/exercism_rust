pub fn abbreviate(phrase: &str) -> String {
    let delims = [' ', '-'];
    // phrase
    //     .to_string()
    //     .split(&delims[..])
    //     .map(|word| word.replace(|ch: char| !ch.is_alphabetic() || delims.contains(&ch), ""))
    //     .map(|word| word.chars().nth(0).unwrap_or(' ').to_ascii_uppercase())
    //     .collect::<String>()
    //     .replace(' ', "")

    let cleaned_vec = phrase
        .to_string()
        .split(&delims) // split on delims
        .map(|word| word.replace(|ch: char| !ch.is_alphabetic(), ""))
        .filter(|word| !word.is_empty())
        .collect::<Vec<String>>();
    // we now have a vector of strings (words),
    // where the non alphabetic chars && non-delim chars are removed
    // empty s

    let mut acronym = String::new();
    for word in cleaned_vec {
        acronym.push(
            word.chars()
                .nth(0)
                .expect("should be an accessible letter")
                .to_ascii_uppercase(),
        );

        // split the remainder of each word on uppercase letters
        // and add the first letter of each split to the acronym

        let components = word
            .split_inclusive(|ch: char| ch.is_uppercase()) // split including uppercase
            .filter(|component| component.len() > 1) // remove single letters. not camelcase
            .collect::<Vec<&str>>();

        // for 1..-1, push the last letter of each component to the acronym

        components[0..components.len().saturating_sub(1)]
            .iter()
            .for_each(|component| {
                acronym.push(
                    component
                        .chars()
                        .last()
                        .expect("should be an accessible letter"),
                )
            });
    }

    acronym
}
