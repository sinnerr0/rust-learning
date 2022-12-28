#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn it_not_works() {
        panic!("테스트 실패!!");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn struct_eq_test() {
        let struct1 = Rectangle {
            length: 1,
            width: 1,
        };
        let struct2 = Rectangle {
            length: 1,
            width: 1,
        };

        assert_eq!(struct1, struct2);
    }

    #[test]
    #[should_panic(expected = "struct 구조가 동일하지 않습니다.")]
    fn struct_assert_test_with_note() {
        let struct1 = Rectangle {
            length: 1,
            width: 1,
        };
        let struct2 = Rectangle {
            length: 1,
            width: 2,
        };

        assert_eq!(
            struct1, struct2,
            "struct 구조가 동일하지 않습니다. {:?} != {:?}",
            struct1, struct2
        );
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[should_panic]
    fn this_test_will_fail() {
        let value = prints_and_return_10(8);
        assert_eq!(5, value);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        let list = 0..99999999;
        for i in list {
            println!("{}", i);
        }
        assert!(true);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn prints_and_return_10(a: i32) -> i32 {
    println!("입력값: {}", a);
    10
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
