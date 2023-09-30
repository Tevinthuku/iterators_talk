struct GreaterThan12(usize);

impl GreaterThan12 {
    fn new(item: usize) -> Result<Self, String> {
        if item > 12 {
            Ok(GreaterThan12(item))
        } else {
            Err(format!("Value {} is less than 12", item))
        }
    }
}

fn initial_all_values_are_greater_than_12(items: Vec<usize>) -> Result<Vec<GreaterThan12>, String> {
    let mut greater_than_12 = Vec::with_capacity(items.len());
    for item in items {
        greater_than_12.push(GreaterThan12::new(item)?);
    }
    Ok(greater_than_12)
}

fn better_all_values_are_greater_than_12(items: Vec<usize>) -> Result<Vec<GreaterThan12>, String> {
    items
        .into_iter()
        .map(GreaterThan12::new)
        .collect::<Result<Vec<_>, _>>()
}
