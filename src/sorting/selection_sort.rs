use super::{Sorter, SortableListBuilder};

#[derive(Debug)]
pub struct SelectionSortableList<T>
where T: Ord + Clone {
  list: Vec<T>
}

impl<T> SortableListBuilder<T> for SelectionSortableList<T>
where T: Ord + Clone {
  fn new(list: &Vec<T>) -> Self {
    Self{ list: list.clone() } 
  }
}

impl<T> Sorter for SelectionSortableList<T>
where T: Ord + Clone {
  fn sort_list(&mut self) {
    for index in 0..self.list.len() { 
      let mut min_elem = &self.list[index];
      let mut min_index = index;
      for i in index + 1..self.list.len() {
        if self.list[i] < *min_elem {
          min_elem = &self.list[i];
          min_index = i;
        }
      }
      self.list.swap(index, min_index);
    }
  }
}