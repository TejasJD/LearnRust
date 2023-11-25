pub mod insertion_sort;
pub mod selection_sort;
pub mod bubble_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod heap_sort;

pub trait SortableListBuilder<T> {
  fn new(list : &Vec<T>) -> Self;
}

pub trait Sorter {
  fn sort_list(&mut self);
}
