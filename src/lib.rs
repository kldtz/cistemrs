//! CISTEM Stemmer for German. Rust translation of [CISTEM](https://github.com/LeonieWeissweiler/CISTEM).
use std::borrow::Cow;

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
pub fn stem<'a>(word: &'a str, case_insensitive: bool) -> Cow<'a, str> {
    if word.len() == 0 {
        return word.into();
    }
    let upper = word.chars().next().unwrap().is_uppercase();
    // replace single chars
    let mut stem = String::with_capacity(word.len());
    // keep track of number of chars
    let mut num_chars = 0;
    for ch in word.chars() {
        num_chars += 1;
        for c in ch.to_lowercase() {
            match c {
                'ü' => stem.push('u'),
                'ö' => stem.push('o'),
                'ä' => stem.push('a'),
                'ß' => {
                    stem.push_str("ss");
                    num_chars += 1;
                }
                _ => stem.push(c),
            };
        }
    }

    // cut off prefix
    if stem.starts_with("ge") && num_chars >= 6 {
        stem = stem[2..].to_string();
        num_chars -= 2;
    }

    let num_bytes = stem.len();
    //replace multiple chars
    stem = stem
        .replace("sch", "$")
        .replace("ei", "%")
        .replace("ie", "&");
    // only 8-bit chars are replaced, so this is valid
    num_chars -= num_bytes - stem.len();
    stem = replxx(stem);

    // trim suffixes
    while num_chars > 3 {
        if num_chars > 5 && (stem.ends_with("em") || stem.ends_with("er") || stem.ends_with("nd")) {
            stem.truncate(stem.len() - 2);
            num_chars -= 2;
        } else if ((!upper || case_insensitive) && stem.ends_with("t"))
            || stem.ends_with(&['e', 's', 'n'][..])
        {
            stem.truncate(stem.len() - 1);
            num_chars -= 1;
        } else {
            break;
        }
    }

    stem = undo_replacements(stem, word.len());
    stem.into()
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

fn undo_replacements(stem: String, word_len: usize) -> String {
    let mut result = String::with_capacity(word_len);
    let mut last_repl = "*";
    let mut b = [0; 4];
    for c in stem.chars() {
        let repl = match c {
            '*' => last_repl,
            '%' => "ei",
            '&' => "ie",
            '$' => "sch",
            _ => c.encode_utf8(&mut b),
        };
        result.push_str(repl);
        last_repl = repl;
    }
    result
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

    //replace multiple chars
    let mut stem = word
        .replace("sch", "$")
        .replace("ei", "%")
        .replace("ie", "&");
    stem = replxx(stem);

    let mut num_chars = stem.chars().count();
    
    // trim suffixes
    while num_chars > 3 {
        if num_chars > 5 && (stem.ends_with("em") || stem.ends_with("er") || stem.ends_with("nd")) {
            stem.truncate(stem.len() - 2);
            num_chars -= 2;
        } else if ((!upper || case_insensitive) && stem.ends_with("t"))
            || stem.ends_with(&['e', 's', 'n'][..])
        {
            stem.truncate(stem.len() - 1);
            num_chars -= 1;
        } else {
            break;
        }
    }

    stem = undo_replacements(stem, word.len());

    return (word[..stem.len()].into(), word[stem.len()..].into());
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
            let actual_line = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}", word, stem_cs, stem_ci, left, right, left_ic, right_ic);
            assert_eq!(actual_line, line);
        }
    }
}
