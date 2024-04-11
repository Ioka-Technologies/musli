use musli::{Decode, Encode};

use crate::tag::{Kind, Tag, MAX_INLINE_LEN};

#[derive(Debug, PartialEq, Encode, Decode)]
struct From<const N: usize> {
    #[musli(name = 0)]
    prefix: Option<u32>,
    #[musli(name = 1)]
    field: Field<N>,
    #[musli(name = 2)]
    suffix: Option<u32>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct To {
    #[musli(name = 0)]
    prefix: Option<u32>,
    #[musli(name = 2)]
    suffix: Option<u32>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[musli(packed)]
struct Field<const N: usize> {
    #[musli(bytes)]
    value: [u8; N],
}

#[test]
fn pack_max() {
    macro_rules! test {
        ($size:expr, $len:expr) => {
            let value = From {
                prefix: Some(10),
                field: Field { value: [1; $size] },
                suffix: Some(20),
            };

            let bytes = crate::to_vec(&value).unwrap();
            let actual: From<$size> = crate::from_slice(&bytes).unwrap();
            let to: To = crate::from_slice(&bytes).unwrap();

            assert_eq!(value, actual);
            assert_eq!(
                to,
                To {
                    prefix: Some(10),
                    suffix: Some(20)
                }
            );

            assert_eq!(Tag::from_byte(bytes[5]), Tag::new(Kind::Prefix, $len));
            assert_eq!(bytes.len(), $size + 9);
        };
    }

    test!(0, 0);
    test!(23, 23);
    test!(MAX_INLINE_LEN, 62);
}

#[test]
fn pow2() {
    macro_rules! test_case {
        ($size:expr, $pow:expr, $pad:expr) => {
            let value = From {
                prefix: Some(10),
                field: Field { value: [1; $size] },
                suffix: Some(20),
            };

            let bytes = crate::to_vec(&value).unwrap();
            let actual: From<$size> = crate::from_slice(&bytes).unwrap();
            let to: To = crate::from_slice(&bytes).unwrap();

            assert_eq!(actual, value);
            assert_eq!(
                to,
                To {
                    prefix: Some(10),
                    suffix: Some(20)
                }
            );

            assert_eq!(Tag::from_byte(bytes[5]), Tag::new(Kind::Pack, $pow));
            assert_eq!(bytes.len(), $size + $pad);
            let start = $size + 6;
            assert!(&bytes[start..start + 18].iter().all(|b| *b == 0));
        };
    }

    test_case!(110, 7, 27);
    test_case!(200, 8, 65);
}
