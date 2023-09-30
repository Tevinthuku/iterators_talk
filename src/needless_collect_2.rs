use itertools::Itertools;

// Count
fn initial_len_solution(items: Vec<usize>) {
    let count_of_items_above_10 = items.iter().filter(|item| *item > &10).collect_vec().len();
    println!("{count_of_items_above_10}")
}

fn better_len_computation(items: Vec<usize>) {
    let count_of_items_above_10 = items.iter().filter(|item| *item > &10).count();
    println!("{count_of_items_above_10}")
}

fn count_and_still_compute(items: Vec<usize>) {
    let mut count = 0;
    for item in items.iter().filter(|item| *item > &10) {
        count += 1;
        println!("{item}");
    }
    println!("count of items above 10 {count}")
}
