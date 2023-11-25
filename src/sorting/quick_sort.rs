use super::{Sorter, SortableListBuilder};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct QuickSortableList<T>
where T: Ord + Clone {
  list: Vec<T>
}

impl<T> QuickSortableList<T>
where T : Ord + Clone {
  
  fn partition(&mut self, start: isize, end: isize) -> isize {
    let partition_index = thread_rng().gen_range(start..=end);
    let mut pivot = start; 
    self.list.swap(partition_index as usize, end as usize);
    for i in start..end {
      if self.list[i as usize] < self.list[end as usize] {
        self.list.swap(pivot as usize, i as usize);
        pivot += 1;
      }
    }
    self.list.swap(pivot as usize, end as usize);
    pivot as isize
  }
  
  fn quick_sort(&mut self, start: isize, end: isize) {
    if start >= end {
      return;
    }
    let pivot = self.partition(start, end);
    self.quick_sort(start, pivot - 1);
    self.quick_sort(pivot + 1, end);
  }
}

impl<T> SortableListBuilder<T> for QuickSortableList<T>
where T: Ord + Clone {
  fn new(list: &Vec<T>) -> Self {
    Self{ list: list.clone() } 
  }
}

impl<T> Sorter for QuickSortableList<T>
where T: Ord + Clone {
  fn sort_list(&mut self) {
    if self.list.len() <= 0 {
      return;
    }
 
    self.quick_sort(0, self.list.len() as isize - 1); 
  }
}