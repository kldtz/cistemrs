//! CISTEM Stemmer for German. Rust translation of [CISTEM](https://github.com/LeonieWeissweiler/CISTEM).

/// This method takes the word to be stemmed and a boolean specifiying if case-insensitive stemming should be used and returns the stemmed word.
///
/// Case sensitivity improves performance only if words in the text may be incorrectly upper case.
/// For all-lowercase and correctly cased text, best performance is achieved by
/// using the case-sensitive version.
///
/// ```
/// use cistemrs::stem;
///
/// let s = stem("schönes", false);
/// assert_eq!(s, "schon");
/// ```
pub fn stem(word: &str, case_insensitive: bool) -> String {
    if word.len() == 0 {
        return "".into();
    }
    let upper = word.chars().next().unwrap().is_uppercase();
    
    // normalize string
    let mut normalized = String::with_capacity(word.len());
    // keep track of number of chars
    let mut num_chars = 0;
    for ch in word.chars() {
        num_chars += 1;
        for c in ch.to_lowercase() {
            match c {
                'ü' => normalized.push('u'),
                'ö' => normalized.push('o'),
                'ä' => normalized.push('a'),
                'ß' => {
                    normalized.push_str("ss");
                    num_chars += 1;
                }
                _ => normalized.push(c),
            };
        }
    }

    // compute start offset
    let start = if normalized.starts_with("ge") && num_chars >= 6 {
        num_chars -= 2;
        2
    } else {
        0
    };

    let end = start + compute_end_offset(&normalized[start..], num_chars, upper, case_insensitive);
    String::from(&normalized[start..end]).into()
}

/// This method works very similarly to stem. The only difference is that in
/// addition to returning the stem, it also returns the rest that was removed at
/// the end. To be able to return the stem unchanged so the stem and the rest
/// can be concatenated to form the original word, all subsitutions that altered
/// the stem in any other way than by removing letters at the end were left out.
///
/// ```
/// use cistemrs::segment;
///
/// let (l, r) = segment("schönes", false);
/// assert_eq!(l, "schön");
/// assert_eq!(r, "es");
/// ```
pub fn segment(word: &str, case_insensitive: bool) -> (String, String) {
    if word.len() == 0 {
        return ("".into(), "".into());
    }
    let upper = word.chars().next().unwrap().is_uppercase();
    let word = word.to_lowercase();
    let end = compute_end_offset(&word, word.chars().count(), upper, case_insensitive);
    return (word[..end].into(), word[end..].into());
}

fn compute_end_offset(word: &str, mut num_chars: usize, upper: bool, case_insensitive: bool) -> usize {
    // temporary replacements
    let mut repl = word
        .replace("sch", "$")
        .replace("ei", "%")
        .replace("ie", "&");
    repl = replxx(repl);
    // only 8-bit chars are replaced, so this is valid
    num_chars -= word.len() - repl.len();

    //let mut num_chars = repl.chars().count();
    let mut end = word.len();
    // compute end offset
    while num_chars > 3 {
        if num_chars > 5 && (repl.ends_with("em") || repl.ends_with("er") || repl.ends_with("nd")) {
            repl.truncate(repl.len() - 2);
            end -= 2;
            num_chars -= 2;
        } else if ((!upper || case_insensitive) && repl.ends_with("t"))
            || repl.ends_with(&['e', 's', 'n'][..])
        {
            repl.truncate(repl.len() - 1);
            end -= 1;
            num_chars -= 1;
        } else {
            break;
        }
    }
    end
}

fn replxx(stem: String) -> String { 
    let mut result = String::with_capacity(stem.len());
    for c in stem.chars() {
        if result.ends_with(c) {
            result.push('*');
        } else {
            result.push(c);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    #[test]
    fn test_all() {
        let file = File::open("data/perl.txt").unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let fields: Vec<&str> = line.trim().split("\t").collect();
            if fields.len() < 2 {
                continue;
            }
            let word = fields[0];
            let stem_cs = stem(word, false);
            let stem_ci = stem(word, true);
            let (left, right) = segment(word, false);
            let (left_ic, right_ic) = segment(word, true);
            let actual_line = format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                word, stem_cs, stem_ci, left, right, left_ic, right_ic
            );
            assert_eq!(actual_line, line);
        }
    }
}
