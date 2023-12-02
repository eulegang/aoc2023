use bstr::ByteSlice;

use crate::common::Day;

pub fn run() {
    let input = Trebuchet::parse(include_bytes!("input.txt"));
    let output = Trebuchet::calc2(input);

    println!("{output}");
}

pub struct Trebuchet;

pub struct Lines(Vec<Vec<u8>>);

impl Day for Trebuchet {
    type Input = Lines;
    type Output = usize;

    fn parse(buf: &[u8]) -> Self::Input {
        Self::Input::parse(buf)
    }

    fn calc(input: Self::Input) -> Self::Output {
        let mut sum = 0;

        for line in input.0 {
            let Some(first) = first_digit(&line) else {
                continue;
            };

            let Some(last) = last_digit(&line) else {
                continue;
            };

            let cur = first * 10 + last;

            sum += cur;
        }

        sum
    }

    fn calc2(input: Self::Input) -> Self::Output {
        let mut sum = 0;

        for line in input.0 {
            let Some(first) = first_text(&line) else {
                continue;
            };

            let Some(last) = last_text(&line) else {
                continue;
            };

            let cur = first * 10 + last;

            sum += cur;
        }

        sum
    }
}

fn first_digit(line: &[u8]) -> Option<usize> {
    for ch in line {
        if matches!(ch, b'0'..=b'9') {
            return Some((ch - b'0').into());
        }
    }

    None
}

fn last_digit(line: &[u8]) -> Option<usize> {
    for ch in line.iter().rev() {
        if matches!(ch, b'0'..=b'9') {
            return Some((ch - b'0').into());
        }
    }

    None
}

fn first_text(line: &[u8]) -> Option<usize> {
    let repr: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    let dig: [&[u8]; 9] = [b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9"];

    let mut pos = usize::MAX;
    let mut val = None::<usize>;
    for (i, r) in repr.iter().enumerate() {
        let v = i + 1;

        if let Some(p) = line.find(r) {
            if p < pos {
                val = Some(v);
                pos = p;
            }
        }
    }

    for (i, r) in dig.iter().enumerate() {
        let v = i + 1;

        if let Some(p) = line.find(r) {
            if p < pos {
                val = Some(v);
                pos = p;
            }
        }
    }

    val
}

fn last_text(line: &[u8]) -> Option<usize> {
    let repr: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    let dig: [&[u8]; 9] = [b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9"];

    let mut pos = None::<usize>;
    let mut val = None::<usize>;

    for (i, r) in repr.iter().enumerate() {
        let v = i + 1;

        if let Some(p) = line.rfind(r) {
            if let Some(x) = pos {
                if p > x {
                    val = Some(v);
                    pos = Some(p);
                }
            } else {
                val = Some(v);
                pos = Some(p);
            }
        }
    }

    for (i, r) in dig.iter().enumerate() {
        let v = i + 1;

        if let Some(p) = line.rfind(r) {
            if let Some(x) = pos {
                if p > x {
                    val = Some(v);
                    pos = Some(p);
                }
            } else {
                val = Some(v);
                pos = Some(p);
            }
        }
    }

    val
}

impl Lines {
    fn parse(buf: &[u8]) -> Lines {
        Lines(buf.split(|b| *b == b'\n').map(|t| t.to_vec()).collect())
    }
}

#[test]
fn example() {
    let input = Trebuchet::parse(include_bytes!("example.txt"));

    assert_eq!(Trebuchet::calc(input), 142)
}

#[test]
fn example2() {
    let input = Trebuchet::parse(include_bytes!("example2.txt"));

    assert_eq!(Trebuchet::calc2(input), 281)
}
