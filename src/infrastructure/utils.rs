pub fn is_unit_type<T>() -> bool {
    std::mem::size_of::<T>() == 0
}