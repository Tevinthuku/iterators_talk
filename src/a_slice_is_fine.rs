// If you are just reading the elements, then a slice is just fine,
// but if you want ownership, don't pass a slice and then clone, but rather, pass an owned collection.


#[derive(Clone)]
struct Item {
    name: String
}

// passing an entire vec when we only wanted to read elements.
fn initial_solution(items: Vec<Item>) {
    for item in items {
        println!("{}", item.name);
    }
}

// taking a reference of the Vec
fn intermediate_solution(items: &Vec<Item>) {
    for item in items {
        println!("{}", item.name);
    }
}


// takes slice, works with &Vec<_> or even arrays.
fn best_solution(items: &[Item]) {
    for item in items {
        println!("{}", item.name);
    }
}


#[test]
mod tests {
    use crate::a_slice_is_fine::{best_solution, Item};

    fn test_best_solution() {
        let items = vec![Item { name: Default::default() }];
        best_solution(&items);
    }
}