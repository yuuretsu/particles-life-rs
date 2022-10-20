pub fn compare_pointers<T>(a: &T, b: &T) -> bool {
    a as *const _ == b as *const _
}
