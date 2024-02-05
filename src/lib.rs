use std::collections::HashSet;

///
/// Unique64 is a very specific crate created to keep track of unique IDs
/// while only using 8 bytes of data.
///
/// This works, as a Java dev might put it, as a VecQueueSet.
///
/// If you've ever used OpenGL, this might seem familiar.
///
pub struct Unique64 {
  available_ids: HashSet<u64>,
  next_id: u64,
}

impl Unique64 {
  pub fn new() -> Self {
    Self {
      available_ids: HashSet::new(),
      next_id: 0,
    }
  }

  ///
  /// Get the next available u64 from the queue.
  ///
  pub fn get_next(&mut self) -> u64 {
    // We want to clear out the internal queue. Do it.
    // We have to do this a bit...strangely.
    let mut selection_option: Option<u64> = None;
    // Avoid borrowing twice in same scope.
    if let Some(id) = self.available_ids.iter().next() {
      selection_option = Some(*id);
    }
    if let Some(selection) = selection_option {
      self.available_ids.remove(&selection);
      // And now you have an old id that got removed before.
      // Recycling is cool. 8)
      return selection;
    }

    // Get and increment.
    // I don't think this will ever overflow because your computer will just run out of RAM first.
    let selection = self.next_id;
    self.next_id += 1;

    selection
  }

  ///
  /// Remove a used u64 from the queue.
  ///
  pub fn remove(&mut self, value: u64) {
    // You can't remove a value, if it doesn't exist.
    if self.available_ids.contains(&value) || value >= self.next_id {
      panic!("Unique64: Attempted to remove a non-existent ID.")
    }

    self.available_ids.insert(value);
  }

  // ! There is no reset function, make a new Unique64. It avoids a whole boat load of errors this way.
}

impl Default for Unique64 {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn working_correctly() {
    let mut dispatcher = Unique64::new();

    for _ in 0..1_000 {
      dispatcher.get_next();
    }

    assert!(dispatcher.next_id == 1_000);

    for i in 500..1000 {
      dispatcher.remove(i);
    }

    assert!(dispatcher.next_id == 1_000);
    assert!(dispatcher.available_ids.len() == 500);

    for _ in 500..1_000 {
      // g stands for gotten.
      let g = dispatcher.get_next();
      assert!((500..1_000).contains(&g))
    }

    assert!(dispatcher.next_id == 1_000);
    assert!(dispatcher.available_ids.is_empty());

    let cool = dispatcher.get_next();

    assert!(cool == 1_000);
    assert!(dispatcher.available_ids.is_empty());
    assert!(dispatcher.next_id == 1_001);

    for i in 0..1_000 {
      dispatcher.remove(i);
    }

    assert!(dispatcher.available_ids.len() == 1_000);
    assert!(dispatcher.next_id == 1_001);
  }

  #[test]
  fn very_specific() {
    let mut dispatcher = Unique64::new();

    // 5 values, 0,1,2,3,4
    for _ in 0..5 {
      dispatcher.get_next();
    }

    // It's gonna get kind of weird.

    dispatcher.remove(1);

    assert!(dispatcher.available_ids.get(&1).is_some());
    assert!(dispatcher.get_next() == 1);
    assert!(dispatcher.next_id == 5);

    dispatcher.remove(4);

    assert!(dispatcher.available_ids.get(&4).is_some());
    assert!(dispatcher.get_next() == 4);
    assert!(dispatcher.next_id == 5);

    dispatcher.remove(2);
    dispatcher.remove(3);
    assert!(dispatcher.available_ids.get(&3).is_some());
    assert!(dispatcher.available_ids.get(&2).is_some());
    let testing = dispatcher.get_next();
    assert!(testing == 2 || testing == 3);
    let testing = dispatcher.get_next();
    assert!(testing == 2 || testing == 3);

    assert!(dispatcher.next_id == 5);
  }

  #[test]
  #[should_panic]
  pub fn wrong() {
    let mut dispatcher = Unique64::new();
    dispatcher.remove(5)
  }

  #[test]
  #[should_panic]
  pub fn wrong_again() {
    let mut dispatcher = Unique64::new();
    dispatcher.remove(0)
  }

  #[test]
  #[should_panic]
  pub fn common_mistake() {
    let mut dispatcher = Unique64::new();
    for _ in 0..1_000 {
      dispatcher.get_next();
    }

    // Lucky 7
    dispatcher.remove(7);

    // Oops.
    dispatcher.remove(7);
  }

  #[test]
  pub fn readme_example() {
    let mut dispatcher = Unique64::new();

    // 0
    let x = dispatcher.get_next();

    assert!(x == 0);

    // 1
    let y = dispatcher.get_next();

    assert!(y == 1);

    // 0 is free again.
    dispatcher.remove(x);

    // 0
    let z = dispatcher.get_next();

    assert!(z == 0);
  }
}
