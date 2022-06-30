type MorseLUTPair<'a> = (&'a str, char);

const LUT: [MorseLUTPair; 36] = [
    (".-", 'a'),
    ("-...", 'b'),
    ("-.-.", 'c'),
    ("-..", 'd'),
    (".", 'e'),
    ("..-.", 'f'),
    ("--.", 'g'),
    ("....", 'h'),
    ("..", 'i'),
    (".---", 'j'),
    ("-.-", 'k'),
    (".-..", 'l'),
    ("--", 'm'),
    ("-.", 'n'),
    ("---", 'o'),
    (".--.", 'p'),
    ("--.-", 'q'),
    (".-.", 'r'),
    ("...", 's'),
    ("-", 't'),
    ("..-", 'u'),
    ("...-", 'v'),
    (".--", 'w'),
    ("-..-", 'x'),
    ("-.--", 'y'),
    ("--..", 'z'),
    (".----", '1'),
    ("..---", '2'),
    ("...--", '3'),
    ("....-", '4'),
    (".....", '5'),
    ("-....", '6'),
    ("--...", '7'),
    ("---..", '8'),
    ("----.", '9'),
    ("-----", '0'),
];

const WORD_SPACE: &str = "       ";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let words = &args[1..].join(" ");

    // If the binary is called 'morse', convert a sentence into morse
    // If it's called unmorse, convert morse into a sentence
    let output = match args[0].as_ref() {
        "morse" => from_morse(words),
        "unmorse" => to_morse(words),
        _ => String::new(),
    };
    println!("{output}");
}

fn to_morse(sentence: &str) -> String {
    let mut words = Vec::new();
    sentence.split(' ').into_iter().for_each(|word| {
        let unmorsed = word
            .chars()
            .into_iter()
            .flat_map(|letter| LUT.iter().position(|(_morse, ch)| ch == &letter))
            .map(|pos| LUT[pos].0.to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();
        words.push(unmorsed);
    });
    words.join(WORD_SPACE).trim().to_string()
}

fn from_morse(sentence: &str) -> String {
    let mut vars = Vec::new();
    for word in sentence.split(WORD_SPACE) {
        for letter in word.split(' ') {
            if let Some(pos) = LUT.iter().position(|(morse, _ch)| morse == &letter) {
                vars.push(LUT[pos].1.to_string());
            }
        }
        vars.push(String::from(" "));
    }
    vars.join("").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_words_test() {
        assert_eq!(from_morse("."), "e");
        assert_eq!(from_morse("..-. ..- -.-. -.-"), "fuck");
        assert_eq!(
            from_morse("- .... .. ...       .. ...       .-       ... . -. - . -. -.-. ."),
            "this is a sentence"
        );
        assert_eq!(from_morse(".----"), "1");
    }

    #[test]
    fn strip_invalid_chars_test() {
        assert_eq!(from_morse("///"), "");
    }

    #[test]
    fn unmorse_test() {
        assert_eq!(to_morse("e"), ".");
        assert_eq!(to_morse("fuck"), "..-. ..- -.-. -.-");
        assert_eq!(
            to_morse("this is a sentence"),
            "- .... .. ...       .. ...       .-       ... . -. - . -. -.-. .",
        );
        assert_eq!(to_morse("1"), ".----");
    }
}
