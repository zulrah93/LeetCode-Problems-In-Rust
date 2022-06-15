use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = HashMap::new();
        climb_stairs_with_cache(n, &mut cache)
    }
}

fn climb_stairs_with_cache(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if n == 1 {
        1
    } else if n == 2 {
        2
    } else if cache.contains_key(&n) {
        *cache.get(&n).unwrap()
    } else {
        let x = climb_stairs_with_cache(n - 1, cache);
        let y = climb_stairs_with_cache(n - 2, cache);

        cache.insert(n - 1, x);
        cache.insert(n - 2, y);

        x + y
    }
}
