use std::fs;
use std::io::{Cursor};
use std::collections::HashMap;

use clap::ArgMatches;
use rand::{Rng, prelude::StdRng};
use bitstream_io::{BigEndian, BitReader};
use ngrams::Ngram;

use super::utils;

pub fn generate_ngrams<'a>(text: &'a String, n: usize) -> (HashMap<Vec<&str>, usize>, Vec<&str>, usize) {
  let mut n_down = n;
  let mut hash: HashMap<Vec<&str>, usize> = HashMap::new();
  let mut unique_words: Vec<&str> = vec![];
  let mut total_words = 0;

  while n_down > 0 {
    // we wish to generate a hash map that contains n-grams for each value of n
    // from the user provided n, down to n == 1. this allows us to do a 'stupid backoff'
    // if there is not enough data at a certain n-depth for a certain word, we can
    // keep backing off to n - 1 until we find enough data to make a decision on which
    // word to use. (or until we hit n == 1, in which case we pick a random word).
    let mut grams: Vec<Vec<&str>> = vec![];
    match n_down {
      1 => {
        for t in text.split_whitespace() {
          grams.push(vec![t]);
        }
        // for some reason the ngrams crate has difficulty when n == 1,
        // so in this case we generate the n gram ourselves by a simple whitespace
        // delimiter.
      },
      _ => {
        grams = text.split_whitespace().ngrams(n_down).collect(); 
      }
    }

    for v in grams {
      if n_down == 1 {
        total_words += 1;
      }

      if let Some(val) = hash.get_mut(&v) {
        *val += 1;
      } else {
        if n_down == 1 {
          unique_words.push(v[0]);
          // if we are on the last n-depth (n == 1),
          // that means the vectors only contain one word.
          // if the hash does not have this vector of one word yet, then
          // this is the first time we are seeing it, so we will add it to
          // a vector of unique words.
        }
        hash.insert(v, 1);
        // if the hash does not have this vector yet, add it
        // with occurance 1, and next time we see this vector,
        // increment the occurance
      }
    }
    n_down -= 1;
  }

  (hash, unique_words, total_words)
}

pub fn get_restricted_chars(char_map: &HashMap<char, usize>, gib_word: &String) -> Vec<char> {
  let mut restricted_chars = vec![]; 
  for key in char_map.keys() {
    restricted_chars.push(*key);
  }
  let word_chars: Vec<char> = gib_word.chars().collect();
  for c in word_chars {
    if restricted_chars.contains(&c) {
      let char_index = restricted_chars.iter().position(|&r| r == c).unwrap();
      restricted_chars.remove(char_index);
    }
  }

  restricted_chars
}

pub fn can_use_word(word: &str, good_chars: &Vec<char>, restricted_chars: &Vec<char>) -> bool {
  let word_chars: Vec<char> = word.chars().collect();
  let mut good_chars_used = vec![0; good_chars.len()];
  for c in word_chars {
    if restricted_chars.contains(&c) {
      return false;
    }
    if good_chars.contains(&c) {
      let char_index = good_chars.iter().position(|&r| r == c).unwrap();
      good_chars_used[char_index] += 1;
    }
  }

  for i in good_chars_used {
    if i == 0 {
      return false;
    }
  }

  true
}

pub fn get_initial_words<'a>(hashmap: &'a HashMap<Vec<&str>, usize>, n: usize) -> Vec<&'a str> {
  let mut vecs_with_n_items = vec![];
  for key in hashmap.keys() {
    if key.len() == n {
      vecs_with_n_items.push(key);
    }
  }

  let mut count_hash = HashMap::new();

  for vec in vecs_with_n_items {
    let mut n_minus_1_slice = vec![];
    let mut counter = 1;

    for word in vec {
      if counter < n {
        n_minus_1_slice.push(*word);
      } else {
        break;
      }
      counter += 1;
    }

    if let Some(val) = count_hash.get_mut(&n_minus_1_slice) {
      *val += 1;
    } else {
      count_hash.insert(n_minus_1_slice, 1);
    }
  }

  let mut best_vec_count = 0;
  let mut best_vec = vec![];
  for vec in count_hash.keys() {
    let vec_count = count_hash.get(vec).unwrap();
    if vec_count > &best_vec_count {
      best_vec_count = *vec_count;
      best_vec = vec.to_vec();
    }
  }

  best_vec
}

pub fn get_probability_of(word: &str, given: &Vec<&str>, hashmap: &HashMap<Vec<&str>, usize>, num_words: f64) -> f64 {
  let count_of_given = match given.len() {
    0 => num_words,
    _ => {
      if let Some(count) = hashmap.get(given) {
        *count as f64
      } else {
        return 0.0;
      }
    },
  };

  let mut word_vec = given.clone();
  word_vec.push(word);
  let count_of_sequence = if let Some(count) = hashmap.get(&word_vec) {
    *count as f64
  } else {
    return 0.0;
  };

  count_of_sequence / count_of_given
}


pub fn get_best_word<'a>(
  gram: &HashMap<Vec<&str>, usize>,
  usable_words: &Vec<&'a str>,
  current_words: &Vec<&str>,
  n: usize,
  total_words: f64,
) -> (&'a str, usize) {
  let mut all_p_zero = true;
  let mut use_n = n;
  let mut max_p_index = 0;

  while all_p_zero {
    let mut ngram_slice = vec![];
    let mut counter = 1;
    for word in current_words.iter().rev() {
      if counter < use_n {
        ngram_slice.push(*word);
      }
      counter += 1;
    }
    ngram_slice.reverse();

    let last_word = ngram_slice.last().unwrap();
    let mut max_p = 0.0;
    max_p_index = 0;

    for i in 0..usable_words.len() {
      let w = &usable_words[i];
      if w == last_word {
        continue;
      }
      let p = get_probability_of(w, &ngram_slice, gram, total_words);
      // let p = get_interpolated_probability(w, &ngram_slice, gram, total_words);
      // println!("P({} | {:?}) = {}", w, ngram_slice, p);
      if p > max_p {
        all_p_zero = false;
        max_p_index = i;
        max_p = p;
      }
    }

    if all_p_zero {
      use_n -= 1;
    }

    // comment this if using interpolation
    if use_n == 1 {
      // no point in picking the word that appears the most...
      // take our chances and break, and pick a random word from the list.
      // println!("reached bottom of use n!");
      break;
    }
  }


  use_n -= 1;

  if use_n == 0 {
    let mut rng = rand::thread_rng();
    max_p_index = rng.gen_range(0, usable_words.len());
  }

  (usable_words[max_p_index], use_n)
}


pub fn wordify(
  gram: &HashMap<Vec<&str>, usize>,
  n: usize,
  file_words: Vec<String>,
  rng: &mut StdRng,
  bit_to_char_map: &mut HashMap<usize, char>,
  unique_words: &Vec<&str>,
  total_words: f64,
  consecutive_skips: usize,
  depth_skip_threshold: usize,
  use_shuffle: bool,
) -> Result<String, String> {
  let mut char_to_bit_map = HashMap::new();
  let mut num_bits = 0;
  for bit_val in bit_to_char_map.keys() {
    num_bits += 1;
    char_to_bit_map.insert(*bit_to_char_map.get(&bit_val).unwrap(), *bit_val);
  }
  num_bits -= 1;

  // let num_words = unique_words.len();
  let mut succ_count = 0;
  let mut fail_count = 0;
  let mut skip_count = 0;
  let mut n_gram_used = vec![0; n];
  let mut text_data = String::from("");
  let mut current_words = get_initial_words(gram, n);
  let mut i = 0;
  let mut consecutive_skips_used = 0;

  let mut skip_words = vec![];
  if !use_shuffle {
    for w in unique_words {
      if utils::is_skip_word(w, &char_to_bit_map) {
        skip_words.push(*w);
      }
    }
    // if not shuffling, skip words get filled in once before
    // iterating.
  }

  while i < file_words.len() {
    let gibberish_word = &file_words[i];
    let mut used_skip_word = false;

    let mut use_keys = vec![]; 
    for key in char_to_bit_map.keys() {
      use_keys.push(*key);
    }

    let restricted_chars = get_restricted_chars(&char_to_bit_map, gibberish_word);

    let mut usable_words = vec![];
    if use_shuffle {
      skip_words = vec![];
      // if shuffling the bit to char map,
      // we have to reset the skip words every time because they might
      // be different
    }

    // let mut value_num_list = vec![0; max_value];
    for w in unique_words {
      // let word_val = get_value_from_word(w, char_value_map, max_value);
      // // println!("value for {}: {}", w, get_value_from_word(w, char_value_map, 2));
      // value_num_list[word_val] += 1;

      if use_shuffle && utils::is_skip_word(w, &char_to_bit_map) {
        skip_words.push(*w);
      }

      if can_use_word(w, &gibberish_word.chars().collect(), &restricted_chars) {
        usable_words.push(*w);
      }
    }

    match usable_words.len() {
      0 => {
        fail_count += 1;
        text_data.push_str(&gibberish_word);
        text_data.push_str(" ");
        current_words.push(".");
        consecutive_skips_used = 0;
        // if there are NO usable words at all then we 'failed'
        // to encode this word. we push the gibberish word as is to
        // the text data output because we still need to be able to decode it.
        // we add a . to current words to stimulate the ngram probability
        // for the next word.
      },
      1 => {
        succ_count += 1;
        let best_word = &usable_words[0];
        text_data.push_str(best_word);
        current_words.push(best_word);
        text_data.push_str(" ");
        n_gram_used[0] += 1;
        consecutive_skips_used = 0;
        // there is only one usable word, so use it without
        // estimating any probabilities. ngram used a depth
        // of 0 since we are not evaluating ngrams here.
      },
      _ => {
        let (best_word, n_used) = get_best_word(
          gram,
          &usable_words,
          &current_words,
          n,
          total_words,
        );

        // user can fine-tune the quality of the text output using depth_skip_threshold
        // and consecutive skips allowed. The higher both are, the more skip words are used
        // which can potentially make the output look more like real text, at the
        // expense of encoding less bits per word on average.
        // consecutive skips used sets a limit to this such that it forces
        // the program to eventually encode a word, otherwise it might
        // loop forever in certain situations.
        // depth skip threshold allows user to say which n-depths are acceptable.
        // lower n-depths produce less realistic.
        if n_used <= depth_skip_threshold && consecutive_skips_used < consecutive_skips && skip_words.len() > 0 {
          let (best_word2, n_used2) = get_best_word(
            gram,
            &skip_words,
            &current_words,
            n,
            total_words
          );

          n_gram_used[n_used2] += 1;
          current_words.push(best_word2);
          text_data.push_str(best_word2);
          text_data.push_str(" ");
          skip_count += 1;
          used_skip_word = true;
          consecutive_skips_used += 1;
          i -= 1;
          // we used a skip word, make sure to keep i at its current
          // level so that we try to encode this word again
        } else {
          succ_count += 1;
          n_gram_used[n_used] += 1;
          text_data.push_str(best_word);
          current_words.push(best_word);
          text_data.push_str(" ");
          consecutive_skips_used = 0;
          // if not using a skip word, we encoded the best possible word according
          // to ngrams. add the best word to the text output, as well as the current
          // words vec which is used to determine word probabilities for the next
          // iteration
        }
      }
    };

    if !used_skip_word && use_shuffle {
      // only shuffle the bit to char map if we encoded a word
      // if we used a skip word, we do NOT want to shuffle as we
      // will not be able to properly decode
      utils::fill_bit_to_char_map(rng, bit_to_char_map);
      char_to_bit_map = utils::make_char_to_bit_map(bit_to_char_map);
    }

    i += 1;
  }

  text_data.pop(); // remove trailing space

  let num_bytes = (file_words.len() * num_bits) / 8;
  // print summary
  println!("\nencoding using {} bits per word. file had {} bytes, ie: {} words to wordify", num_bits, num_bytes, file_words.len());
  println!("succesfully filled {} words", (succ_count + skip_count));
  println!("of the {} words, {} were skip words", (succ_count + skip_count), skip_count);
  println!("failed to find a word {} times", fail_count);
  println!("average bits per word: {}\n", ((num_bytes * 8) as f64 / (succ_count + skip_count) as f64));

  println!("\nN-depth summary: {:?}", n_gram_used);

  Ok(text_data)
}

pub fn get_value_vec_from_char_value_mode(
  file_contents: &Vec<u8>,
  num_bits: usize,
) -> Vec<u8> {
  let mut cursor = Cursor::new(&file_contents);
  let mut num_bits_remain = file_contents.len() * 8;
  let mut bitreader = BitReader::endian(&mut cursor, BigEndian);
  let mut value_vec = vec![];

  while num_bits_remain > 0 {
    let num_bits_to_read = if num_bits_remain < num_bits as usize {
      num_bits_remain as u32
    } else {
      num_bits as u32
    };
    let value: u8 = bitreader.read(num_bits_to_read).unwrap();
    
    // if use_shuffle {
    //   utils::fill_bit_to_char_map(rng, bit_to_char_map);
    // }

    value_vec.push(value);
    num_bits_remain -= num_bits_to_read as usize;
  }

  value_vec
}

pub fn get_value_vec(
  bit_to_char_map: &mut HashMap<usize, char>,
  file_contents: &Vec<u8>,
  num_bits: usize,
  use_shuffle: bool,
  rng: &mut StdRng,
) -> Vec<String> {
  let mut cursor = Cursor::new(&file_contents);
  let mut num_bits_remain = file_contents.len() * 8;
  let mut bitreader = BitReader::endian(&mut cursor, BigEndian);
  let mut sorted_keys = vec![];
  let mut value_vec = vec![];

  for byte_val in bit_to_char_map.keys() {
    sorted_keys.push(*byte_val);
  }
  sorted_keys.sort_by(|a, b| b.cmp(a));
  // sort keys once so you dont need to do it in the iteration.
  // the bit to char map maps bit positions: (0, 1, 2, 4, 8, 16, 32, etc)
  // to characters. we iterate over the bit position values, and push the value
  // to a sorted_keys vec, and then sort in descending order
  // (ie: 0th element is largest)
  // we do this because the user provides the number of bits.
  // so if the user says number
  // of bits is 3, then the sorted keys will look like: [4, 2, 1, 0]

  while num_bits_remain > 0 {
    let num_bits_to_read = if num_bits_remain < num_bits as usize {
      num_bits_remain as u32
    } else {
      num_bits as u32
    };
    let value: u8 = bitreader.read(num_bits_to_read).unwrap();
    let char_str = utils::get_chars_from_value(value, bit_to_char_map, &sorted_keys);
    
    if use_shuffle {
      utils::fill_bit_to_char_map(rng, bit_to_char_map);
    }

    value_vec.push(char_str);
    num_bits_remain -= num_bits_to_read as usize;
  }
  // iterate the file that you wish to encode, reading num_bits at a time.
  // for each value you read, generate characters that map to the value using the bit to char map
  // if using shuffle, the bit to char map gets shuffled according to a seeded rng.
  // at the end you have a vector of gibberish strings that you will try to hide
  // in words using ngrams.

  value_vec
}

pub fn wordify_from_char_value_mode(
  gram: &HashMap<Vec<&str>, usize>,
  char_to_value_map: &HashMap<char, usize>,
  n: usize,
  file_values: Vec<u8>,
  num_bits: usize,
  unique_words: &Vec<&str>,
  total_words: f64,
  use_shuffle: bool,
  value_mode: utils::ValueMode,
) -> Result<String, String> {
  let mut succ_count = 0;
  let mut n_gram_used = vec![0; n];
  let mut text_data = String::from("");
  let mut current_words = get_initial_words(gram, n);
  let mut i = 0;


  while i < file_values.len() {
    let current_val = file_values[i];

    let mut usable_words = vec![];

    for w in unique_words {
      if *w == "." || *w == "," || *w == "?" || *w == ";" || *w == "!" {
        // dont use punctuation in char_value mode because
        // punctuation isnt ignored by the decoder. if you want
        // to leave punctuation in, you would also have to leave
        // the spaces around them which would result in a stego text
        // like: he likes cars , toys , and trucks .
        // for that reason, I chose to ignore punctuation
        continue;
      }

      let w_val = utils::get_value_from_chars(w, &char_to_value_map, &value_mode);
      if w_val == current_val as usize {
        usable_words.push(*w);
      }
    }


    match usable_words.len() {
      0 => {
        panic!("NOT ENOUGH WORDS WITH VALUE {}", current_val);
      },
      1 => {
        succ_count += 1;
        let best_word = &usable_words[0];
        text_data.push_str(best_word);
        current_words.push(best_word);
        text_data.push_str(" ");
        n_gram_used[0] += 1;
      },
      _ => {
        let (best_word, n_used) = get_best_word(
          gram,
          &usable_words,
          &current_words,
          n,
          total_words,
        );

        succ_count += 1;
        n_gram_used[n_used] += 1;
        text_data.push_str(best_word);
        current_words.push(best_word);
        text_data.push_str(" ");
      }
    };

    i += 1;
  }

  text_data.pop(); // remove trailing space

  let num_bytes = (file_values.len() * num_bits) / 8;
  // print summary
  println!("\nencoding using {} bits per word. file had {} bytes, ie: {} words to wordify", num_bits, num_bytes, file_values.len());
  println!("succesfully filled {} words", succ_count);
  println!("average bits per word: {}\n", ((num_bytes * 8) as f64 / succ_count as f64));

  println!("\nN-depth summary: {:?}", n_gram_used);

  Ok(text_data)
}

pub fn encode_char_bit_map(
  file: &str,
  output: &str,
  seed_str: &str,
  word_file_name: &str,
  n_depth: usize,
  consecutive_skips: usize,
  depth_skip_threshold: usize,
  num_bits: usize,
  use_shuffle: bool,
) -> Result<(), String> {
  let mut rng = utils::create_rng_from_seed(seed_str);
  let mut original_rng = utils::create_rng_from_seed(seed_str);
  let contents = utils::get_file_contents(file)?;
  let mut word_file_data = utils::get_file_contents_as_string(word_file_name)?;


  let mut bit_to_char_map = utils::make_bit_to_char_map(num_bits);
  let mut original_bit_to_char_map = bit_to_char_map.clone();
  utils::fill_bit_to_char_map(&mut rng, &mut bit_to_char_map);
  utils::fill_bit_to_char_map(&mut original_rng, &mut original_bit_to_char_map);


  let value_vec = get_value_vec(&mut bit_to_char_map, &contents, num_bits, use_shuffle, &mut rng);


  word_file_data = word_file_data.to_lowercase();
  word_file_data = utils::format_text_for_ngrams(&word_file_data);
  let (
    gram_hash,
    unique_words,
    total_words,
  ) = generate_ngrams(&word_file_data, n_depth);


  let text_data = wordify(
    &gram_hash,
    n_depth,
    value_vec,
    &mut original_rng,
    &mut original_bit_to_char_map,
    &unique_words,
    total_words as f64,
    consecutive_skips,
    depth_skip_threshold,
    use_shuffle,
  )?;

  fs::write(output, text_data).unwrap();

  Ok(())
}

pub fn encode_char_value_map(
  file: &str,
  output: &str,
  seed_str: &str,
  word_file_name: &str,
  n_depth: usize,
  consecutive_skips: usize,
  depth_skip_threshold: usize,
  num_bits: usize,
  use_shuffle: bool,
  value_mode: utils::ValueMode,
) -> Result<(), String> {
  let mut rng = utils::create_rng_from_seed(seed_str);
  let mut original_rng = utils::create_rng_from_seed(seed_str);
  let contents = utils::get_file_contents(file)?;
  let mut word_file_data = utils::get_file_contents_as_string(word_file_name)?;

  let mut char_to_value_map = utils::make_char_to_value_map(num_bits);

  let mut value_vec = get_value_vec_from_char_value_mode(&contents, num_bits);


  word_file_data = word_file_data.to_lowercase();
  word_file_data = utils::format_text_for_ngrams(&word_file_data);
  let (
    gram_hash,
    unique_words,
    total_words,
  ) = generate_ngrams(&word_file_data, n_depth);

  let text_data = wordify_from_char_value_mode(
    &gram_hash,
    &char_to_value_map,
    n_depth,
    value_vec,
    num_bits,
    &unique_words,
    total_words as f64,
    use_shuffle,
    value_mode,
  )?;

  fs::write(output, text_data).unwrap();

  Ok(())
}


pub fn encode(matches: &ArgMatches) -> Result<(), String> {
  let file = utils::get_value(matches, "file")?;
  let output = utils::get_value(matches, "output")?;
  let seed_str = utils::get_value(matches, "seed")?;
  let alg_str = utils::get_value(matches, "algorithm")?;
  let word_file_name = utils::get_value(matches, "words")?;
  let n_depth = utils::get_numerical_value(matches, "n")?;
  let consecutive_skips = utils::get_numerical_value(matches, "consecutive_skips")?;
  let depth_skip_threshold = utils::get_numerical_value(matches, "depth_skip")?;
  let num_bits = utils::get_numerical_value(matches, "bits")?;

  if num_bits > 8 || num_bits < 1 {
    return Err(format!("Bits must be between 1 and 8 inclusively, you provided {}", num_bits));
  }

  let alg = utils::get_algorithm_from_string(alg_str, num_bits)?;

  let (use_shuffle, value_mode) = match alg {
    utils::Algorithm::Shuffle(mode) => {
      (true, mode)
    },
    utils::Algorithm::NoShuffle(mode) => {
      (false, mode)
    },
  };

  match value_mode {
    utils::ValueMode::CharBitMap => {
      encode_char_bit_map(
        file,
        output,
        seed_str,
        word_file_name,
        n_depth,
        consecutive_skips,
        depth_skip_threshold,
        num_bits,
        use_shuffle,
      )
    },
    utils::ValueMode::CharValueMap(val) => {
      println!("using char value instead of char bit with num bits: {}", val);
      encode_char_value_map(
        file,
        output,
        seed_str,
        word_file_name,
        n_depth,
        consecutive_skips,
        depth_skip_threshold,
        num_bits,
        use_shuffle,
        value_mode,
      )
    },
  }
}