use super::{Sorter, SortableListBuilder};

#[derive(Debug)]
pub struct BubbleSortableList<T>
where T: Ord + Clone {
  list: Vec<T>
}

impl<T> SortableListBuilder<T> for BubbleSortableList<T>
where T: Ord + Clone {
  fn new(list: &Vec<T>) -> Self {
    Self{ list: list.clone() }
  }
}

impl<T> Sorter for BubbleSortableList<T>
where T: Ord + Clone {
  fn sort_list(&mut self) {
    for i in 0..self.list.len() {
      for j in 0..self.list.len() - i - 1 {
        if self.list[j] > self.list[j + 1] {
          self.list.swap(j, j + 1);
        }
      } 
    } 
  }
}