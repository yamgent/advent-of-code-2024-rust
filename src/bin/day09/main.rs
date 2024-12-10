use std::fmt::Display;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/09/input.txt");

fn p1(input: &str) -> String {
    #[derive(Clone, Copy, PartialEq, Eq)]
    struct FileId(usize);

    impl FileId {
        const EMPTY_FID: Self = Self(usize::MAX);
    }

    struct Filesystem {
        blocks: Vec<FileId>,
    }

    impl Filesystem {
        fn parse_input(input: &str) -> Self {
            let input = input
                .trim()
                .chars()
                .map(|x| x.to_digit(10).expect("a digit"))
                .collect::<Vec<_>>();

            let capacity = input.iter().sum::<u32>() as usize;

            Self {
                blocks: input
                    .into_iter()
                    .enumerate()
                    .fold(
                        (Vec::with_capacity(capacity), true),
                        |(mut acc, is_file), (idx, val)| {
                            if is_file {
                                (0..val).for_each(|_| acc.push(FileId(idx / 2)));
                            } else {
                                (0..val).for_each(|_| acc.push(FileId::EMPTY_FID));
                            }
                            (acc, !is_file)
                        },
                    )
                    .0,
            }
        }

        fn compact(mut self) -> Self {
            let mut left = self
                .blocks
                .iter()
                .enumerate()
                .find(|(_, val)| **val == FileId::EMPTY_FID)
                .expect("at least one empty space")
                .0;

            let mut right = self
                .blocks
                .iter()
                .enumerate()
                .rev()
                .find(|(_, val)| **val != FileId::EMPTY_FID)
                .expect("at least one allocated file")
                .0;

            while left < right {
                self.blocks[left] = self.blocks[right];
                self.blocks[right] = FileId::EMPTY_FID;
                while left < self.blocks.len() && self.blocks[left] != FileId::EMPTY_FID {
                    left += 1;
                }
                while right > 0 && self.blocks[right] == FileId::EMPTY_FID {
                    right -= 1;
                }
            }

            self
        }

        fn checksum(&self) -> usize {
            self.blocks
                .iter()
                .enumerate()
                .filter(|(_, id)| **id != FileId::EMPTY_FID)
                .map(|(pos, id)| pos * id.0)
                .sum::<usize>()
        }
    }

    Filesystem::parse_input(input)
        .compact()
        .checksum()
        .to_string()
}

fn p2(input: &str) -> String {
    #[derive(Debug)]
    struct Block {
        fid: Option<usize>,
        len: usize,
    }

    impl Display for Block {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let ch = match self.fid {
                Some(fid) => fid.to_string(),
                None => ".".to_string(),
            };
            for _ in 0..self.len {
                write!(f, "{ch}")?;
            }
            Ok(())
        }
    }

    let mut filesystem = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).expect("a digit"))
        .enumerate()
        .fold(vec![], |mut acc, (index, len)| {
            acc.push(Block {
                fid: if index % 2 == 0 {
                    Some(index / 2)
                } else {
                    None
                },
                len: len as usize,
            });
            acc
        });

    let right_block_idx = filesystem
        .iter()
        .enumerate()
        .rev()
        .find(|(_, block)| block.fid.is_some())
        .expect("at least one file block")
        .0;

    (0..=right_block_idx).rev().for_each(|idx| {
        if filesystem[idx].fid.is_none() {
            // not a file
            return;
        }

        if let Some(replace_idx) = (0..idx).find(|replace_idx| {
            filesystem[*replace_idx].fid.is_none()
                && filesystem[*replace_idx].len >= filesystem[idx].len
        }) {
            let fid = filesystem[idx].fid.take();
            let remaining_space = filesystem[replace_idx].len - filesystem[idx].len;

            if remaining_space == 0 {
                filesystem[replace_idx].fid = fid;
            } else {
                filesystem[replace_idx].len = remaining_space;
                filesystem.insert(
                    replace_idx,
                    Block {
                        fid,
                        len: filesystem[idx].len,
                    },
                );
            }
        }
    });

    filesystem
        .into_iter()
        .fold((0, 0), |(mut acc, idx), block| {
            if let Some(fid) = block.fid {
                acc += (idx..(idx + block.len)).map(|idx| idx * fid).sum::<usize>();
            }
            (acc, idx + block.len)
        })
        .0
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"2333133121414131402";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "1928");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "6385338159127");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "2858");
    }

    #[test]
    #[ignore = "brute-force, took 1s on local"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "6415163624282");
    }
}
