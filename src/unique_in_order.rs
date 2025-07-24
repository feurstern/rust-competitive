pub fn unique_in_order_display() {}

fn unique_in_order_fn<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + std::fmt::Debug,
{
    let mut result = Vec::new();
    let mut iter = sequence.into_iter();

    if let Some(mut prev) = iter.next() {
        // result.push(prev.clone());

        for curr in iter {
            // result.push(curr.clone());
            prev = curr;
        }
    }
    result
}
