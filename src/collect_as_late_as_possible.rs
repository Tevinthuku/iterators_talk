

fn initial_filter_and_map_even_numbers(nums: &[usize]) {
    let mut filtered_and_mapped = Vec::new();

    for num in nums {
        if num % 2 == 0 {
            filtered_and_mapped.push(num * num);
        }
    }
}



fn better_solution(nums: &[usize]) {
    let filtered_and_mapped = nums.iter().filter(|x| *x % 2 == 0).map(|x| x * x).collect::<Vec<_>>();
}