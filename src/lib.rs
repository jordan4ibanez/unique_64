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
  /// Delete a used u64 from the queue.
  ///
  pub fn delete(&mut self, value: u64) {
    // You can't remove a value, if it doesn't exist.
    if !self.available_ids.contains(&value) || value <= self.next_id {
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
