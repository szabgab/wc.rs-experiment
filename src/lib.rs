#[derive(PartialEq, Debug)]
pub struct Wc {
    rows: usize,
    words: usize,
    bytes: usize,
}

pub fn wc(file: &str) -> Wc {
    let content = std::fs::read_to_string(file).unwrap();
    let rows = content.split("\n").collect::<Vec<&str>>();
    let words = content.split_whitespace().collect::<Vec<&str>>();
    let res = Wc {
        rows: rows.len() - 1,
        words: words.len(),
        bytes: content.len(),
    };

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = wc("files/empty.txt");
        assert_eq!(
            result,
            Wc {
                rows: 0,
                words: 0,
                bytes: 0
            }
        );
        assert_eq!(result.rows, 0);

        let result = wc("files/hello.txt");
        assert_eq!(
            result,
            Wc {
                rows: 0,
                words: 2,
                bytes: 11
            }
        );

        let result = wc("files/hello_with_newline.txt");
        assert_eq!(
            result,
            Wc {
                rows: 1,
                words: 2,
                bytes: 12
            }
        );

        let result = wc("files/spaces.txt");
        assert_eq!(
            result,
            Wc {
                rows: 3,
                words: 7,
                bytes: 47
            }
        );

        let result = wc("files/unicode.txt");
        assert_eq!(
            result,
            Wc {
                rows: 1,
                words: 2,
                bytes: 12
            }
        );
    }
}
