use super::{Sorter, SortableListBuilder};

#[derive(Debug)]
pub struct HeapSortableList<T>
where T: Ord + Clone {
  list: Vec<T>
}

impl<T> SortableListBuilder<T> for HeapSortableList<T>
where T: Ord + Clone {
  fn new(list: &Vec<T>) -> Self {
    Self{ list: list.clone() }      
  }
}

impl<T> Sorter for HeapSortableList<T> 
where T: Ord + Clone {
  fn sort_list(&mut self) {
  } 
}