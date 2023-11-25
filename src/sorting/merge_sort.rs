use super::{Sorter, SortableListBuilder};

#[derive(Debug)]
pub struct MergeSortableList<T>
where T: Ord + Clone {
  list: Vec<T>
}

impl<T> MergeSortableList<T>
where T: Ord + Clone {
  fn merge(&mut self, start: usize, mid: usize, end: usize) {

  }
  
  fn merge_sort(&mut self, start: usize, end: usize) {
    if start >= end {
      return;
    }
    let mid = start + (end - start) / 2;
    self.merge_sort(start, mid);
    self.merge_sort(mid + 1, end);
    self.merge(start, mid, end)
  }
}

impl<T> SortableListBuilder<T> for MergeSortableList<T>
where T: Ord + Clone {
  fn new(list : &Vec<T>) -> Self {
    Self{ list: list.clone() }      
  }
}

impl<T> Sorter for MergeSortableList<T>
where T: Ord + Clone {
  fn sort_list(&mut self) {
    if self.list.len() <= 0 {
      return;
    }
    self.merge_sort(0, self.list.len() - 1);
  }
}