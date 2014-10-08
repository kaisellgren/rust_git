pub fn max_by<T: Clone>(items: &[T], predicate: |&T| -> uint) -> Option<T> {
    let mut item: Option<T> = None;
    let mut max_value: uint = 0;

    for i in items.iter() {
        let value = predicate(i);
        if  value > max_value {
            max_value = value;
            item = Some((*i).clone());
        }
    }

    item
}
