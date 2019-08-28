use std::collections::HashMap;
use std::fs;

use clap::ArgMatches;
use rand::{Rng, SeedableRng, prelude::StdRng};
use arrayref::array_ref;
use sha2::{Sha256, Digest};


const COMMON_CHARS: [char; 26] = [
  'i', 't', 'a', 'o', 'e', 'n', 's',
  'h', 'r', 'd', 'l', 'c', 'u', 'm',
  'w', 'f', 'g', 'y', 'p', 'b', 'v',
  'k', 'j', 'x', 'q', 'z'
];

pub enum ValueMode {
  CharBitMap,
  // the map contains 1 character for each bit position
  // given a number of bits. ie: if 3 bits, the map contains 4 characters
  // (1 extra character to explicitly map 0), if 8 bits, the map contains 9 characters.
  // the value is determined by checking which characters are present, and if
  // a character is present, that means the bit is set for that char.
  // duplicate characters are irrelevant, since a bit can only be set once.
  CharValueMap(usize),
  // the map contains every character in the alphabet and assigns values ranging
  // from 0 to 2^(num bits) - 1 in increments of powers of 2. the value is determined
  // by adding the the value for each character present in a word. duplicate characters
  // are allowed, since it will increase the value. If the value reaches
  // 2^(num bits), it overflows and wraps back to 0.
}

pub enum Algorithm {
  Shuffle(ValueMode),
  NoShuffle(ValueMode),
}

pub fn get_algorithm_from_string(alg_str: &str, num_bits: usize) -> Result<Algorithm, String> {
  match alg_str {
    "char-bit" => Ok(Algorithm::NoShuffle(ValueMode::CharBitMap)),
    "char-bit-shuffle" => Ok(Algorithm::Shuffle(ValueMode::CharBitMap)),
    "char-value" => Ok(Algorithm::NoShuffle(ValueMode::CharValueMap(num_bits))),
    _ => Err(format!("Could not determine algorithm: {}", alg_str)),
  }
}

pub fn get_value<'a>(matches: &'a ArgMatches, value_name: &str) -> Result<&'a str, String> {
  match matches.value_of(value_name) {
    Some(val) => {
      Ok(val)
    },
    None => Err(format!("failed to get value of {}", value_name)),
  }
}

pub fn get_numerical_value(matches: &ArgMatches, value_name: &str) -> Result<usize, String> {
  let value_str = get_value(matches, value_name)?;

  match value_str.parse::<usize>() {
    Ok(parsed) => Ok(parsed),
    Err(_) => Err(format!("Failed to parse '{}' as a number", value_str)),
  }
}

pub fn make_bit_to_char_map(num_bits: usize) -> HashMap<usize, char> {
  let mut bit_to_char_map: HashMap<usize, char> = HashMap::new();
  for num in 0..(num_bits + 1) {
    let key = if num == 0 {
      0
    } else {
      1 << (num - 1)
    };
    bit_to_char_map.insert(key, COMMON_CHARS[num as usize]);
  }

  bit_to_char_map
}

pub fn get_max_value(exponent: usize) -> usize {
  (2 as usize).pow(exponent as u32) - 1
}

pub fn make_char_to_value_map(exponent: usize) -> HashMap<char, usize> {
  let mut char_to_value_map: HashMap<char, usize> = HashMap::new();

  let max_val = get_max_value(exponent);
  let mut current_val = 0;
  let mut max_it = COMMON_CHARS.len() / 2;
  for i in 0..max_it {
    let common_index = i;
    let uncommon_index = COMMON_CHARS.len() - i - 1;
    
    char_to_value_map.insert(COMMON_CHARS[common_index], current_val);
    char_to_value_map.insert(COMMON_CHARS[uncommon_index], current_val);

    current_val += 1;
    if current_val > max_val {
      current_val = 0;
    }
    // current val gets assigned simultaneously to the most common and
    // least common character at any timestep. for each timestep,
    // current val gets incremented until it surpasses the maximum val
    // then wraps back to 0.
    // eg: maximum val is 7 if max_exponent = 3,
    // because 2^3 = 8. 8 - 1 = 7.
    // the first 7 most common characters get mapped 0, 1, 2, ... 7,
    // as well as the 7 least common characters.
    // the 9th character on either side then gets mapped to 0 and the
    // cycle restarts.
  }

  char_to_value_map
}

fn create_hash(text: &str) -> String {
  let mut hasher = Sha256::default();
  hasher.input(text.as_bytes());
  format!("{:x}", hasher.result())
}

pub fn create_rng_from_seed(text: &str) -> StdRng {
  let hash = create_hash(text);
  let seed = array_ref!(hash.as_bytes(), 0, 32);
  SeedableRng::from_seed(*seed)
}

pub fn format_text_for_ngrams(text: &str) -> String {
  let mut new_text: String = text.to_string().to_lowercase();
  if text.ends_with('.') {
    new_text.push(' ');
  }

  let without_newlines = new_text.replace("\n", " \n ");
  let without_carriage_returns = without_newlines.replace("\r", "");
  let without_quotes = without_carriage_returns.replace("\"", "");
  let without_single_quotes = without_quotes.replace("'", "");
  let without_dashes = without_single_quotes.replace("-", " ");
  let without_commas = without_dashes.replace(", ", " , ");
  let without_exclamation = without_commas.replace("! ", " ! ");
  let without_questions = without_exclamation.replace("? ", " ? ");
  let without_semicolons = without_questions.replace("; ", " ; ");
  let without_colons = without_semicolons.replace(": ", " : ");
  let mut with_spaces = without_colons.replace(". ", " . ");
  with_spaces.pop();
  with_spaces = [". ", &with_spaces].join("");
  with_spaces
}


pub fn is_skip_word(word: &str, char_to_bit_map: &HashMap<char, usize>) -> bool {
  let mut restricted_chars = vec![]; 
  for key in char_to_bit_map.keys() {
    restricted_chars.push(*key);
  }
  let mut skip_word = true;
  for c in word.chars() {
    if restricted_chars.contains(&c) {
      skip_word = false;
      break;
    }
  }

  skip_word
}

pub fn get_file_contents(file_name: &str) -> Result<Vec<u8>, String> {
  match fs::read(file_name) {
    Ok(data) => Ok(data),
    Err(_) => Err(format!("Failed to read file: '{}'", file_name)),
  }
}

pub fn get_file_contents_as_string(file_name: &str) -> Result<String, String> {
  match fs::read_to_string(file_name) {
    Ok(data) => Ok(data),
    Err(_) => Err(format!("Failed to read file: '{}'", file_name)),
  }
}

pub fn get_chars_from_value(val: u8, char_map: &HashMap<usize, char>, sorted_keys: &Vec<usize>) -> String {
  let mut out_str = String::from("");
  let mut val_remaining = val;
  for num in 0..sorted_keys.len() {
    let current_byte_val = sorted_keys[num];

    if current_byte_val as u8 == val_remaining {
      let some_char = char_map.get(&current_byte_val).unwrap();
      out_str.push(*some_char);
      // perfect match: ie if map is { 0: 'a', 1: 'b', 2: 'c' }
      // and the val == 3, first we get to 2, which is less than 3
      // so we add a c. next val_remaining is 1. 1 == 1 which maps to 'b'
      // we push b to the out string, break, and return "cb"
      break
    } else if (current_byte_val as u8) < val_remaining {
      let some_char = char_map.get(&current_byte_val).unwrap();
      out_str.push(*some_char);
      val_remaining -= current_byte_val as u8;
    }
  }
  
  out_str
}

pub fn get_value_from_chars(chars: &str, char_map: &HashMap<char, usize>, mode: &ValueMode) -> usize {
  let mut out_value = 0;
  let mut chars_checked = vec![];
  for c in chars.chars() {

    match mode {
      ValueMode::CharBitMap => {
        if chars_checked.contains(&c) {
          // if we already checked this character,
          // dont bother checking again. since
          // each character represents a bit being set. if there are
          // multiple characters that does not mean that
          // the bit is set multiple times...
          continue
        }
      },
      _ => (),
    }


    if let Some(val) = char_map.get(&c) {
      out_value += val;
    }
    chars_checked.push(c);
  }

  match mode {
    ValueMode::CharBitMap => out_value,
    ValueMode::CharValueMap(exp) => out_value % (get_max_value(*exp) + 1),
  }
}


pub fn fill_bit_to_char_map(rng: &mut StdRng, bit_to_char_map: &mut HashMap<usize, char>) {
  let mut bit_keys = vec![];
  let mut bit_values = vec![];
  for key in bit_to_char_map.keys() {
    bit_keys.push(key.clone());
  }

  bit_keys.sort_by(|a, b| a.cmp(b));

  for key in &bit_keys {
    bit_values.push(bit_to_char_map.get(key).unwrap().clone());
  }

  let mut chars = COMMON_CHARS.to_vec();

  for num in 0..bit_keys.len() {
    let key = bit_keys[num];
    let random_index = rng.gen_range(0, chars.len());
    let random_char = chars[random_index];
    chars.remove(random_index);
    bit_to_char_map.remove(&key);
    bit_to_char_map.insert(key, random_char);
  }
}


pub fn make_char_to_bit_map(bit_to_char_map: &HashMap<usize, char>) -> HashMap<char, usize> {
  let mut char_to_bit_map = HashMap::new();
  for bit_val in bit_to_char_map.keys() {
    char_to_bit_map.insert(*bit_to_char_map.get(&bit_val).unwrap() , *bit_val);
  }

  char_to_bit_map
}
