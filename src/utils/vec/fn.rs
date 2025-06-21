use crate::*;

pub(crate) fn split_whitespace(input: &[u8]) -> Vec<&[u8]> {
    let mut parts: Vec<&[u8]> = Vec::new();
    let mut start: usize = 0;
    for (i, &byte) in input.iter().enumerate() {
        if byte == SPACE_U8 || byte == TAB_U8 {
            if i > start {
                parts.push(&input[start..i]);
            }
            start = i + 1;
        }
    }
    if start < input.len() {
        parts.push(&input[start..]);
    }
    parts
}

pub(crate) fn split_multi_byte<'a>(data: &'a [u8], delimiter: &'a [u8]) -> Vec<&'a [u8]> {
    let mut result: Vec<&[u8]> = Vec::new();
    let mut start: usize = 0;
    for i in 0..=data.len() {
        if data[i..].starts_with(delimiter) {
            result.push(&data[start..i]);
            start = i + delimiter.len();
        }
    }
    if start < data.len() {
        result.push(&data[start..]);
    }
    result
}
