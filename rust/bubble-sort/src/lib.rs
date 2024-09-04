pub fn bubble_sort<const N: usize>(array: [u32; N]) {
    for x in array {
        print!("{x}, ")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn normal_array() {
        let array: [u32; 8] = [2, 3, 1, 4, 9, 6, 2, 1];

        super::bubble_sort(array);

        assert_eq!(array, [1, 1, 2, 2, 3, 4, 6, 9])
    }
}
