use wrapper_sys;


pub fn add_numbers(a: i32, b: i32) -> i32 {
    unsafe {
        wrapper_sys::add_numbers(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        assert_eq!(super::add_numbers(1, 2), 3);
        assert_eq!(super::add_numbers(4, 5), 9);
    }
}
