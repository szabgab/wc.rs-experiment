pub fn wc(file: &str) -> usize {
    let content = std::fs::read_to_string(file).unwrap();
    return content.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = wc("files/empty.txt");
        assert_eq!(result, 0);

        let result = wc("files/hello.txt");
        assert_eq!(result, 11);

        let result = wc("files/hello_with_newline.txt");
        assert_eq!(result, 12);

        let result = wc("files/spaces.txt");
        assert_eq!(result, 47);

        let result = wc("files/unicode.txt");
        assert_eq!(result, 12);

    }
}
