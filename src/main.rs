use clap::{CommandFactory, Parser, ValueEnum};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Cipher {
    Simple,
    Reversal,
    Prime,
    #[value(name = "r-prime")]
    RPrime,
    Claude,
    #[value(name = "r-claude")]
    RClaude,
    #[value(name = "p-claude")]
    PClaude,
    #[value(name = "rp-claude")]
    RPClaude
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum PalindromeCipher {
    Simple,
    Prime,
    Claude,
    PClaude
}

#[derive(Debug, Parser)]
#[command(
    name = "gematria-cli",
    version,
    about =
"A Command-line gematria calculator written in Rust which also includes minor ciphers method based on Claude Shannon’s perfect-pangram, and filtering words from a text file is available.",
    override_usage =
"
  gematria-cli [OPTIONS]
  Examples:
    gematria-cli -c simple -t gematria
    gematria-cli -c reversal -t 'The quick brown fox jumps over the lazy dog'
    gematria-cli -e -t gematria
    gematria-cli -c prime -f list.txt
    gematria-cli -c simple -f list.txt -n 74
    geamtria-cli -c claude -f list.txt -a 'the' -n 31
    gematria-cli -p -f list.txt
    gematria-cli -a gematria -c simple -f list.txt -n 74"
)]

struct Args {
    #[arg(
        short = 'c',
        long = "cipher",
        value_enum,
        conflicts_with = "every_cipher",
        help = "Select a cipher\n"
    )]
    cipher: Option<Cipher>,

    #[arg(
        short = 'e',
        long = "every-cipher",
        requires = "text",
        conflicts_with = "cipher",
        help = "Calculate using all available ciphers"
    )]
    every_cipher: bool,

    #[arg(
        short = 't',
        long = "text",
        conflicts_with_all = ["file", "addition"],
        value_name = "TEXT",
        help = "Calculate the gematria value of the input text"
    )]
    text: Option<String>,
    
    #[arg(
        short = 'a',
        long = "addition",
        requires = "file",
        conflicts_with_all = ["text", "palindrome", "every_cipher"],
        value_name = "WORD",
        help = "Add the specified word or words to each file word"
    )]
    addition: Option<String>,

    #[arg(
        short = 'f',
        long = "file",
        conflicts_with = "text",
        value_name = "FILE",
        help = "Read text from a file and extract words for calculating"
    )]
    file: Option<PathBuf>,

    #[arg(
        short = 'n',
        long = "number",
        conflicts_with = "text",
        requires = "file",
        value_name = "NUMBER",
        value_parser = clap::value_parser!(u64).range(1..),
        help = "Filter for words that match the specified value"
    )]
    number: Option<u64>,
    
    #[arg(
        short = 'p',
        long = "palindrome",
        conflicts_with_all = ["cipher", "every_cipher"],
        requires = "file",
        value_enum,
        value_name = "CIPHER",
        help = "Filter for words that have matching values ​​in both the forward and reverse calculation results\n"
    )]
    palindrome: Option<PalindromeCipher>,
}

fn letter_value(ch: char, cipher: Cipher) -> Option<u64> {
    if !ch.is_ascii_alphabetic() {
        return None;
    }

    let index = ch.to_ascii_lowercase() as u8 - b'a';

    match cipher {
        Cipher::Simple => Some(u64::from(index) + 1),
        Cipher::Reversal => Some(26 - u64::from(index)),
        Cipher::Prime => {
            const PRIMES: [u64; 26] = [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
            ];

            Some(PRIMES[index as usize])
        },
        Cipher::RPrime => {
            const RPRIMES: [u64; 26] = [
                101, 97, 89, 83, 79, 73, 71, 67, 61, 59, 53, 47, 43,
                41, 37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2,
            ];

            Some(RPRIMES[index as usize])
        },
        Cipher::Claude => {
            const CLAUDE: [u64; 26] = [
                11, 6, 14, 3, 8, 7, 4, 22, 16, 15, 13, 10, 17,
                12, 25, 18, 1, 19, 23, 21, 2, 24, 20, 26, 5, 9,
            ];

            Some(CLAUDE[index as usize])
        },
        Cipher::RClaude => {
            const RCLAUDE: [u64; 26] = [
                9, 5, 26, 20, 24, 2, 21, 23, 19, 1, 18, 25, 12,
                17, 10, 13, 15, 16, 22, 4, 7, 8, 3, 14, 6, 11,
            ];

            Some(RCLAUDE[index as usize])
        },
        Cipher::PClaude => {
            const PCLAUDE: [u64; 26] = [
                31, 13, 43, 5, 19, 17, 7, 79, 53, 47, 41, 29, 59,
                37, 97, 61, 2, 67, 83, 73, 3, 89, 71, 101, 11, 23,
            ];

            Some(PCLAUDE[index as usize])
        },
        Cipher::RPClaude => {
            const RPCLAUDE: [u64; 26] = [
                23, 11, 101, 71, 89, 3, 73, 83, 67, 2, 61, 97, 37,
                59, 29, 41, 47, 53, 79, 7, 17, 19, 5, 43, 13, 31,
            ];

            Some(RPCLAUDE[index as usize])
        }
    }
}

fn calculate_gematria(word: &str, cipher: Cipher) -> u64 {
    let mut total = 0_u64;
    let mut digits = String::new();

    for ch in word.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else {
            if !digits.is_empty() {
                total += digits.parse::<u64>().unwrap_or(0);
                digits.clear();
            }

            if let Some(value) = letter_value(ch, cipher) {
                total += value;
            }
        }
    }

    if !digits.is_empty() {
        total += digits.parse::<u64>().unwrap_or(0);
    }

    total
}

fn extract_words(text: &str) -> Vec<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}

fn sum_digits(n: u64) -> u64 {
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(u64::from)
        .sum()
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    if let Some(text) = args.text {
        let words = extract_words(&text);

        if words.is_empty() {
            return Err("The input does not contain any English words".into());
        }

        let ciphers = [
            (Cipher::Simple, "simple"),
            (Cipher::Reversal, "reversal"),
            (Cipher::Prime, "prime"),
            (Cipher::RPrime, "r-prime"),
            (Cipher::Claude, "claude"),
            (Cipher::RClaude, "r-claude"),
            (Cipher::PClaude, "p-claude"),
            (Cipher::RPClaude, "rp-claude")
        ];

        if args.every_cipher {
            for (cipher, name) in ciphers {               
                let mut product = 1u64;
                let mut terms = Vec::new();

                for word in &words {
                    let value = calculate_gematria(word, cipher);

                    product = product
                        .checked_mul(value)
                        .ok_or("Gematria product exceeds u64 range")?;

                    terms.push(format!("{word}({value})"));
                }

                if words.len() > 1 {
                    let digits_sum = sum_digits(product);

                    println!("{name}: {digits_sum} = {product} = {}", terms.join(" × "));
			    } else {
					println!("{name}: {product}");
				}
            }
        } else {
			let cipher = args.cipher.ok_or("Specify --cipher or --every-cipher")?;
            let mut product = 1u64;
            let mut terms = Vec::new();

            for word in &words {
                let value = calculate_gematria(word, cipher);

                product = product
                    .checked_mul(value)
                    .ok_or("Gematria product exceeds u64 range")?;

                terms.push(format!("{word}({value})"));
            }

            if words.len() > 1 {
                let digits_sum = sum_digits(product);

                println!("{digits_sum}: {product} = {}", terms.join(" × "));
		    } else {
				println!("{product}");
            }
        }

        return Ok(());
    }


    if let Some(file) = args.file {
		let path = file;
        let text = fs::read_to_string(path)?;
        let words = extract_words(&text);
        let mut results = Vec::new();
        
        if let Some(addition) = args.addition {
            let cipher = args.cipher.ok_or("Specify --cipher.")?;
            let addition_words = extract_words(&addition);

            if addition_words.is_empty() {
                return Err(
                    "The addition does not contain any English words.".into()
                );
            }

            let addition_value: u64 = addition_words
                .iter()
                .map(|word| calculate_gematria(word, cipher))
                .sum();

            let addition_display: Vec<String> = addition_words
                .iter()
                .map(|word| {
                    let value = calculate_gematria(word, cipher);
                    format!("{word}({value})")
                })
                .collect();

            for word in words {
                let value = calculate_gematria(&word, cipher);
                let product = value * addition_value;
                let digits_sum = sum_digits(product);

                if let Some(number) = args.number {
                    if digits_sum != number {
                        continue;
                    }
                }

                let mut results = vec![format!("{word}({value})")];
                results.extend(addition_display.iter().cloned());

                println!("{digits_sum}: {product} = {}", results.join(" × "));
            }

            return Ok(());
        }

        for word in words {
            if let Some(cipher) = args.palindrome {
                let (cipher, reverse_cipher) = match cipher {
                    PalindromeCipher::Simple => (Cipher::Simple, Cipher::Reversal),
                    PalindromeCipher::Prime => (Cipher::Prime, Cipher::RPrime),
                    PalindromeCipher::Claude => (Cipher::Claude, Cipher::RClaude),
                    PalindromeCipher::PClaude => (Cipher::PClaude, Cipher::RPClaude),
                };

                let value = calculate_gematria(&word, cipher);
                let reverse_value = calculate_gematria(&word, reverse_cipher);

                if value == reverse_value {
                    if args.number.is_none_or(|number| value == number) {
                        if args.number.is_some() {
                            results.push(format!("{word}"));
                        } else {
                            results.push(format!("{word}({value})"));
                        }
                    }
                }
            } else {
                let cipher = args.cipher.ok_or("Specify --cipher.")?;
                let value = calculate_gematria(&word, cipher);

                match args.number {
                    Some(number) if value == number => {
                        results.push(format!("{word}"));
                    }
                    Some(_) => {}
                    None => {
                        results.push(format!("{word}({value})"));
                    }
                }
            }
        }
        
        if results.len() > 0 {
		    println!("{}", results.join(", "));
		}

        return Ok(());
    }

    Err("Specify either --word or --file.".into())
}

fn main() {
    if std::env::args_os().len() == 1 {
        let mut command = Args::command();
        command.print_help().unwrap();
        println!();
        return;
    }

    let args = Args::parse();

    if let Err(error) = run(args) {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
