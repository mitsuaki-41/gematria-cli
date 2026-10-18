# gematria-cli
A Command-line gematria calculator written in Rust which also includes minor ciphers method based on Claude Shannon's perfect-pangram, and filtering words from a text file is available.

## Features

- Calculate values with eight ciphers: `simple`, `reversal`, `prime`, `r-prime`, `claude`, `r-claude`, `p-claude`, and `rp-claude`.
- Calculate a value for one or more words.
- Calculate results using every available cipher all at once.
- Read words from a file and filter them by gematria value.
- Find words whose values match when calculated with a forward cipher and its reverse counterpart.
- Combine each word in a file with one or more specified words.

## Requirements

- Rust 1.99.0+

Install Rust on your system: https://rust-lang.org/tools/install/

## Build

Linux, Mac, Windows:

Clone the repository, enter the project directory, and build the release binary:

```sh
git clone https://github.com/mitsuaki-41/gematria-cli
cd gematria-cli
cargo build --release
```

The executable will be available at `target/release/gematria-cli`.

You can also run it directly from the project directory with Cargo:

```sh
cargo run -- --help
```

Now you can also install:

```sh
cargo install --path .
```

## Usage

```text
gematria-cli [OPTIONS]
```

### Options

| Command | Short Option | Description |
|---|---|---|
| --cipher | -c | Select a cipher |
| --every-cipher | -e | Calculate using all available cipher |
| --text | -t | Calculate the gematria value of the input text |
| --addition | -a | Add the specified word or words to each file word |
| --file | -f | Read text from a file and extract words for calculating |
| --number | -n | Filter for words that match the specified number |
| --palindrome | -p | Filter for words that have matching values ​​in both the forward and reverse calculation results |
| --help | -h | Print help |
| --version | -V | Print version |


### Calculate a word

```sh
gematria-cli -c simple -t gematria
```

The output will be as follows: `74`

### Calculate a phrase with one cipher

```sh
gematria-cli -c simple -t "computer technology"
```

The output will be as follows:

    18: 14508 = cumputer(117) × technology(124)

Each word is evaluated separately. When the input contains multiple words, the program multiplies their values and prints the product, its digit sum, and the individual word values.

### Calculate with every cipher

```sh
gematria-cli -e -t "value of the humanity"
```

`--every-cipher` cannot be used together with `--cipher`.

### Read words from a file

You can load the text file by navigating to the directory where it is located or by specifying an absolute path.

```sh
cd /your/text/file/path
```

Now you can execute:

```sh
gematria-cli -c prime -f list.txt
```

Without a number filter, matching file words are printed as `word(value)`.

### Filter by an exact value

```sh
gematria-cli -c simple -f list.txt -n 111
```

The regular file-search mode keeps words whose calculated value is exactly the specified number. The output omits the value in parentheses when `--number` is used.

## Available ciphers

Letter values are case-insensitive. Non-alphabetic punctuation is ignored when calculating letter values. Numeric sequences in input tokens are added as numbers.

| Cipher | Description |
|---|---|
| `simple` | A=1, B=2, ..., Z=26 |
| `reversal` | A=26, B=25, ..., Z=1 |
| `prime` | Maps A–Z to the first 26 prime numbers: A=2, B=3, C=5, ..., Z=101 |
| `r-prime` | Reverse order of the first 26 prime numbers: A=101, B=97, ..., Z=2 |
| `claude` | Custom letter-value mapping based on the Claude Shannon's perfect-pangram |
| `r-claude` | Reverse counterpart of the `claude` mapping |
| `p-claude` | Assigns the first 26 prime numbers according to the `claude` letter ordering |
| `rp-claude` | Reverse counterpart of the `p-claude` mapping |

### Sample: Claude cipher (personal customized option)

    squdgy	23, 01, 02, 03, 04, 05
    fez	07, 08, 09
    blank	06, 10, 11, 12, 13
    jimp	15, 16, 17, 18
    crwth	14, 19, 20, 21, 22
    vox	24, 25, 26

### Find matching forward/reverse values

Use `--palindrome` with one of the supported cipher families:

```sh
gematria-cli -p simple -f list.txt
```

A word is selected when its value is the same under the chosen cipher and its reverse counterpart:

| `--palindrome` value | Forward cipher | Reverse cipher |
|---|---|---|
| `simple` | `simple` | `reversal` |
| `prime` | `prime` | `r-prime` |
| `claude` | `claude` | `r-claude` |
| `p-claude` | `p-claude` | `rp-claude` |

When used without `--number`, results are printed as `word(value)`. When used with `--number`, only matches whose value equals that number are printed, as words without values.

Example:

```sh
gematria-cli -p prime -f list.txt -n 174
```

### Combine file words with additional words

```sh
gematria-cli -c simple -f list.txt -a "number of the"
```

The program calculates the sum of the values of the words supplied with `--addition`, multiplies that sum by each file word's value, and prints the product, its digit sum, and the terms used.

In this mode, `--number` filters by the digit sum of that product:

```sh
gematria-cli -c prime -f list.txt -a "the underdog of" -n 37
```

## Uninstall

```sh
cargo uninstall gematria-cli
rm -r gematria-cli
```

Windows:

```cmd
cargo uninstall cipher-finder-cli
rmdir /s cipher-finder-cli
```

## License

MIT
