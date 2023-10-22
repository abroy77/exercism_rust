use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (a.len()).cmp(&b.len()) {
        Ordering::Equal => {
            if a == b {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if x_sub_y(b, a) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if x_sub_y(a, b) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn x_sub_y<T: PartialEq>(x: &[T], y: &[T]) -> bool {
    // check if the first is a sublist of the second

    x.is_empty() || y.windows(x.len()).any(|slice| slice == x)
}
