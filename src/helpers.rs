pub fn is_same_pointer<T>(a: &T, b: &T) -> bool {
    a as *const _ == b as *const _
}
