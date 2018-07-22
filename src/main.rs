use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::path::Path;

mod network;
use network::Network;

const DISTANCE: usize = 1;

fn get_file_from_stdin() -> Result<Box<BufRead>, Error> {
  Ok(Box::new(BufReader::new(io::stdin())))
}

fn get_file_from_filename(file_name: &str) -> Result<Box<BufRead>, Error> {
  let path = Path::new(&file_name);

  match File::open(path) {
    Ok(file) => Ok(Box::new(BufReader::new(file))),
    Err(why) => Err(why),
  }
}

fn help() {
  println!("usage:
word_friends <file_name>
  Load a file from disk within the word_friends program.
cat <file_name> | word_friends
  Pipe a file into the word_friends program.");
}

fn parse_args() -> Option<Result<Box<BufRead>, Error>> {
  let args: Vec<String> = env::args().collect();

  // Get a file from the user, can be stdin or a filename
  match args.len() {
    1 => Some(get_file_from_stdin()),
    2 => Some(get_file_from_filename(&args[1])),
    _ => {
      help();
      None
    }
  }
}

fn build_network(reader: Box<BufRead>) -> Network {
  let mut network = Network::new();
  let mut network_defined = false;

  for line in reader.lines().map(|l| l.unwrap()) {
    if line.contains("END OF INPUT") {
      network_defined = true;
    } else if !network_defined {
      network.add_word(&line);
    } else {
      network.search(&line, &DISTANCE);
    }
  }

  network
}

fn main() {
  let file = parse_args();

  if let Some(reader_result) = file {
    match reader_result {
      Ok(reader) => build_network(reader).print_results(&DISTANCE),
      Err(_) => panic!("File failed to open!"),
    }
  }
}
