use super::super::*;

#[test]
fn test_open_wrong_path() {
    let filename = "src/tests/test_files/test1.fi";
    let content = read_from(filename);
    match content {
        Ok(content) => {
            assert_eq!(content, "a = 1;\nb = 2;\nc = 3;\nd = 4;\n");
        }
        Err(e) => assert_eq!(e.raw_os_error(), Some(2))
    }
}

#[test]
fn test_open_correct_path() {
    let filename = "right.fi";
    let content = read_from(filename);
    match content {
        Ok(content) => {
            print!("{}", content.as_str());
            assert_eq!(content, "i = 12+5*3;\nafficher i;\naff_ral;\n\n       ");
        }
        Err(e) => assert_eq!(e.raw_os_error(), Some(2))
    }
}