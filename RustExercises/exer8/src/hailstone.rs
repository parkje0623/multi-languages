pub fn hailstone(n: u64) -> u64 {
    // TODO
    if n%2 == 0 {
        return n/2;
    } else {
        return 3*n +1;
    }
}

//append 7 -> time:   [202.18 ns 209.60 ns 217.07 ns]
//append 162964 -> time:   [277.77 ns 281.94 ns 286.69 ns]
//append 686901248 ->  time:   [1.0682 us 1.0777 us 1.0880 us]
pub fn hailstone_sequence_append(n: u64) -> Vec<u64> {
    // TODO
    let mut hailstones = Vec::new();
    let mut value = n;
    hailstones.push(value);
    while value > 1 {
        value = hailstone(value);
        hailstones.push(value);
    }
    return hailstones;
}

//prealloc 7 -> time:   [56.681 ns 57.472 ns 58.369 ns]
//prealloc 162964 -> time:   [127.96 ns 129.07 ns 130.52 ns]
//prealloc 686901248 -> time:   [1.0349 us 1.0637 us 1.0976 us]
pub fn hailstone_sequence_prealloc(n: u64) -> Vec<u64> {
    // TODO
    let mut count = 1;
    let mut value1 = n;
    let mut value2 = n;
    while value1 > 1 {
        value1 = hailstone(value1);
        count += 1;
    }

    let mut vec = Vec::with_capacity(count);
    vec.push(value2);
    while value2 > 1 {
        value2 = hailstone(value2);
        vec.push(value2);
    }
    return vec;
}
