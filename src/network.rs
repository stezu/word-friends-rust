extern crate strsim;

struct SearchResult {
  distance: usize,
  word: String,
}

impl SearchResult {
  fn new(word: String, distance: usize) -> SearchResult {
    SearchResult {
      word: word,
      distance: distance,
    }
  }
}

struct NetworkEntry {
  matches: Vec<SearchResult>,
  word: String,
}

impl NetworkEntry {
  fn new(word: String) -> NetworkEntry {
    NetworkEntry {
      matches: Vec::new(),
      word: word,
    }
  }

  fn add_match(&mut self, m: SearchResult) -> &NetworkEntry {
    self.matches.push(m);
    self
  }

  pub fn search(&mut self, word: &String, max_distance: &usize) -> &NetworkEntry {
    let actual_distance = strsim::levenshtein(&self.word, &word);

    if actual_distance <= *max_distance {
      self.add_match(SearchResult::new(word.clone(), actual_distance));
    }
    self
  }
}

pub struct Network {
  entries: Vec<NetworkEntry>,
}

impl Network {
  pub fn new() -> Network {
    Network { entries: Vec::new() }
  }

  pub fn add_word(&mut self, word: &String) -> &Network {
    self.entries.push(NetworkEntry::new(word.clone()));
    self
  }

  pub fn search(&mut self, word: &String, distance: &usize) -> &Network {
    for entry in &mut self.entries {
      entry.search(&word, &distance);
    }
    self
  }

  pub fn print_results(&mut self, distance: &usize) {
    for entry in &self.entries {
      let filtered_matches: Vec<usize> = entry.matches
        .iter()
        .filter(|m| m.distance == *distance)
        .map(|m| m.distance)
        .collect();

      println!("{}", filtered_matches.len());
    }
  }
}
