pub mod sorting;

use sorting::{
  Sorter,
  SortableListBuilder,
  insertion_sort::InsertionSortableList,
  selection_sort::SelectionSortableList,
  bubble_sort::BubbleSortableList,
  quick_sort::QuickSortableList
};

use crate::sorting::{merge_sort::MergeSortableList, heap_sort::HeapSortableList};


fn test_sorter<U, T>(list: & Vec<T>)
where T : Ord + Clone, U : std::fmt::Debug + Sorter + SortableListBuilder<T> {
  let mut sorter = U::new(&list);
  sorter.sort_list();
  println!("{:?}", sorter);
}


fn test_sorters() {
  println!("Testing sorters...");
  let list = vec![9, 1, 7, 2, 1, 3];
  println!("Testing insertion sort");
  test_sorter::<InsertionSortableList<i32>, i32>(&list);
  println!("Testing selection sort");
  test_sorter::<SelectionSortableList<i32>, i32>(&list);
  println!("Testing bubble sort");
  test_sorter::<BubbleSortableList<i32>, i32>(&list);
  println!("Testing quick sort");
  test_sorter::<QuickSortableList<i32>, i32>(&list);
  println!("Testing merge sort");
  test_sorter::<MergeSortableList<i32>, i32>(&list);
  println!("Testing heap sort");
  test_sorter::<HeapSortableList<i32>, i32>(&list);
}

fn main() {
  test_sorters();
}
