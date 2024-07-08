pub fn wc(file: &str) -> (usize, usize) {
    let content = std::fs::read_to_string(file).unwrap();
    let rows = content.split("\n").collect::<Vec<&str>>();
    return (rows.len()-1, content.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = wc("files/empty.txt");
        assert_eq!(result, (0,0));

        let result = wc("files/hello.txt");
        assert_eq!(result, (0, 11));

        let result = wc("files/hello_with_newline.txt");
        assert_eq!(result, (1, 12));

        let result = wc("files/spaces.txt");
        assert_eq!(result, (3, 47));

        let result = wc("files/unicode.txt");
        assert_eq!(result, (1, 12));

    }
}
