pub fn find_elt<T: Eq>(values: &Vec<T>, elt: T) -> Option<usize> {
    // TODO
    let mut index = 0;
    for i in values.iter() {
        if *i == elt {
            return Some(index);
        }
        index += 1;
    }
    return None;
}
