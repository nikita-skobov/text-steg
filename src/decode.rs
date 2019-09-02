use std::fs;

use clap::ArgMatches;
use bitstream_io::{BigEndian, BitWriter};
use rpassword;

use super::utils;


pub fn decode_char_bit_mode(
  file: &str,
  output: &str,
  seed_str: &str,
  num_bits: usize,
  use_shuffle: bool,
  value_mode: utils::ValueMode,
) -> Result<(), String> {
  let mut rng = utils::create_rng_from_seed(seed_str);

  let mut bit_to_char_map = utils::make_bit_to_char_map(num_bits);
  utils::fill_bit_to_char_map(&mut rng, &mut bit_to_char_map);
  let mut char_to_bit_map = utils::make_char_to_bit_map(&bit_to_char_map);


  let mut bitwriter = BitWriter::endian(Vec::new(), BigEndian);
  let contents = utils::get_file_contents_as_string(file)?;

  let encoded_words = contents.split(' ').collect::<Vec<&str>>();

  let mut total_bits = ((encoded_words.len() * num_bits) / 8) * 8;

  for word in encoded_words {
    if utils::is_skip_word(word, &char_to_bit_map) {
      continue;
    }

    let value = utils::get_value_from_chars(word, &char_to_bit_map, &value_mode);

    if use_shuffle {
      utils::fill_bit_to_char_map(&mut rng, &mut bit_to_char_map);
      char_to_bit_map = utils::make_char_to_bit_map(&bit_to_char_map);
    }

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

pub fn decode_char_value_mode(
  file: &str,
  output: &str,
  seed_str: &str,
  num_bits: usize,
  use_shuffle: bool,
  value_mode: utils::ValueMode,
) -> Result<(), String> {
  let mut rng = utils::create_rng_from_seed(seed_str);

  let mut char_to_value_map = utils::make_char_to_value_map(num_bits);
  utils::shuffle_char_value_map(&mut rng, &mut char_to_value_map);

  let mut bitwriter = BitWriter::endian(Vec::new(), BigEndian);
  let contents = utils::get_file_contents_as_string(file)?;

  let encoded_words = contents.split(' ').collect::<Vec<&str>>();

  let mut total_bits = ((encoded_words.len() * num_bits) / 8) * 8;

  for word in encoded_words {
    let value = utils::get_value_from_chars(word, &char_to_value_map, &value_mode);

    if use_shuffle {
      utils::shuffle_char_value_map(&mut rng, &mut char_to_value_map);
    }

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

pub fn decode(matches: &ArgMatches) -> Result<(), String> {
  let file = utils::get_value(matches, "file")?;
  let output = utils::get_value(matches, "output")?;
  let password_str = utils::get_value(matches, "password")?;
  let mut seed_str = utils::get_value(matches, "seed")?;
  let alg_str = utils::get_value(matches, "algorithm")?;
  let num_bits = utils::get_numerical_value(matches, "bits")?;

  if num_bits > 8 || num_bits < 1 {
    return Err(format!("Bits must be between 1 and 8 inclusively, you provided {}", num_bits));
  }

  let pass;
  match password_str {
    "true" => {
      // get seed string interactively
      pass = rpassword::prompt_password_stderr("Enter password that file was encoded with: ").unwrap();
      seed_str = pass.as_str();
    },
    _ => (),
  };

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
      decode_char_bit_mode(
        file,
        output,
        seed_str, 
        num_bits, 
        use_shuffle,
        value_mode
      )
    },
    utils::ValueMode::CharValueMap(_) => {
      decode_char_value_mode(
        file,
        output,
        seed_str, 
        num_bits, 
        use_shuffle,
        value_mode
      )
    },
  }


  // let mut rng = utils::create_rng_from_seed(seed_str);

  // let mut bit_to_char_map = utils::make_bit_to_char_map(num_bits);
  // utils::fill_bit_to_char_map(&mut rng, &mut bit_to_char_map);
  // let mut char_to_bit_map = utils::make_char_to_bit_map(&bit_to_char_map);


  // let mut bitwriter = BitWriter::endian(Vec::new(), BigEndian);
  // let contents = utils::get_file_contents_as_string(file)?;

  // let encoded_words = contents.split(' ').collect::<Vec<&str>>();

  // let mut total_bits = ((encoded_words.len() * num_bits) / 8) * 8;

  // for word in encoded_words {
  //   if utils::is_skip_word(word, &char_to_bit_map) {
  //     continue;
  //   }

  //   let value = utils::get_value_from_chars(word, &char_to_bit_map, &value_mode);

  //   if use_shuffle {
  //     utils::fill_bit_to_char_map(&mut rng, &mut bit_to_char_map);
  //     char_to_bit_map = utils::make_char_to_bit_map(&bit_to_char_map);
  //   }

  //   let write_bits = if total_bits > num_bits {
  //     num_bits as u32
  //   } else {
  //     total_bits as u32
  //   };

  //   bitwriter.write(write_bits, value as u8).unwrap();
  //   total_bits -= write_bits as usize;
  // }


  // let out_vec = bitwriter.into_writer();
  // fs::write(output, out_vec).unwrap();


  // Ok(())
}