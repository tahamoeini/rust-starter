#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;
    if first_list == second_list {
        Equal
    } else if first_list.is_empty() || second_list.windows(first_list.len()).any(|w| w == first_list) {
        Sublist
    } else if second_list.is_empty() || first_list.windows(second_list.len()).any(|w| w == second_list) {
        Superlist
    } else {
        Unequal
    }
}


pub fn main() {
    let list1 = vec![1, 3, 2];
    let list2 = vec![0, 1, 2, 3, 4];

    println!("{:?}", sublist(&list1, &list2)); // Sublist
    println!("{:?}", sublist(&list2, &list1)); // Superlist
    println!("{:?}", sublist(&list1, &list1)); // Equal
    println!("{:?}", sublist(&[], &list2));    // Sublist
    println!("{:?}", sublist(&list1, &[]));    // Superlist
}
