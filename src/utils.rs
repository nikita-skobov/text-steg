use std::collections::HashMap;

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
    ValueMode::CharValueMap(max_value) => out_value % max_value,
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
