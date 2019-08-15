use std::fs;

use clap::ArgMatches;
use bitstream_io::{BigEndian, BitReader, BitWriter};

use super::utils;

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

pub fn get_file_contents(file_name: &str) -> Result<String, String> {
  match fs::read_to_string(file_name) {
    Ok(data) => Ok(data),
    Err(_) => Err(format!("Failed to read file: '{}'", file_name)),
  }
}

pub fn decode(matches: &ArgMatches) -> Result<(), String> {
  let file = get_value(matches, "file")?;
  let output = get_value(matches, "output")?;
  let seed_str = get_value(matches, "seed")?;
  let num_bits = get_numerical_value(matches, "bits")?;

  if num_bits > 8 || num_bits < 1 {
    return Err(format!("Bits must be between 1 and 8 inclusively, you provided {}", num_bits));
  }

  let mut rng = utils::create_rng_from_seed(seed_str);

  let mut bit_to_char_map = utils::make_bit_to_char_map(num_bits);
  utils::fill_bit_to_char_map(&mut rng, &mut bit_to_char_map);
  let mut char_to_bit_map = utils::make_char_to_bit_map(&bit_to_char_map);


  let mut bitwriter = BitWriter::endian(Vec::new(), BigEndian);
  let contents = get_file_contents(file)?;

  let encoded_words = contents.split(' ').collect::<Vec<&str>>();

  let value_mode = utils::ValueMode::CharBitMap;
  let mut total_bits = ((encoded_words.len() * num_bits) / 8) * 8;

  for word in encoded_words {
    if utils::is_skip_word(word, &char_to_bit_map) {
      continue;
    }

    let value = utils::get_value_from_chars(word, &char_to_bit_map, &value_mode);
    utils::fill_bit_to_char_map(&mut rng, &mut bit_to_char_map);
    char_to_bit_map = utils::make_char_to_bit_map(&bit_to_char_map);

    let write_bits = if total_bits > num_bits {
      num_bits as u32
    } else {
      total_bits as u32
    };

    bitwriter.write(write_bits, value as u8).unwrap();
    total_bits -= write_bits as usize;
  }

  let out_vec = bitwriter.into_writer();
  fs::write(output, out_vec).unwrap();


  Ok(())
}