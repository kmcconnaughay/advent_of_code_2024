use itertools::Itertools;
use std::{collections::VecDeque, iter::zip};

pub fn part1(input: &str) -> u64 {
    let mut fragmented = parse(input);
    let mut compressed = VecDeque::default();

    while let Some(front) = fragmented.pop_front() {
        let mut free_len = fragmented
            .front()
            .map_or(0, |second| second.start - front.end);
        let mut free_start = front.end;

        compressed.push_back(front);
        if free_len == 0 {
            continue;
        }

        while let Some(back) = fragmented.pop_back() {
            if back.len() > free_len {
                let moved = File {
                    start: free_start,
                    end: free_start + free_len,
                    id: back.id,
                    fixed: false,
                };
                let leftover = File {
                    start: back.start,
                    end: back.end - free_len,
                    id: back.id,
                    fixed: false,
                };
                compressed.push_back(moved);
                fragmented.push_back(leftover);
                break;
            } else if back.len() == free_len {
                compressed.push_back(File {
                    start: free_start,
                    end: free_start + free_len,
                    id: back.id,
                    fixed: false,
                });
                break;
            } else {
                compressed.push_back(File {
                    start: free_start,
                    end: free_start + back.len(),
                    id: back.id,
                    fixed: false,
                });
                free_len -= back.len();
                free_start += back.len();
            }
        }
    }

    checksum(&compressed)
}

pub fn part2(input: &str) -> u64 {
    let mut disk_map = parse(input);
    let mut back_index = disk_map.len() - 1;

    while back_index > 0 {
        let File {
            start,
            end,
            id,
            fixed,
        } = &disk_map[back_index];
        if *fixed {
            back_index -= 1;
            continue;
        }

        let mut moved_file = false;

        for (first_index, second_index) in zip(0..back_index, 1..=back_index) {
            let first = &disk_map[first_index];
            let second = &disk_map[second_index];
            let free_len = second.start - first.end;

            if free_len >= end - start {
                disk_map.insert(
                    second_index,
                    File {
                        start: first.end,
                        end: first.end + end - start,
                        id: *id,
                        fixed: true,
                    },
                );
                disk_map.remove(back_index + 1);
                moved_file = true;
                break;
            }
        }

        if !moved_file {
            disk_map[back_index].fixed = true;
            back_index -= 1;
        }
    }

    checksum(&disk_map)
}

#[derive(Debug)]
struct File {
    start: u64,
    end: u64,
    id: u64,
    fixed: bool,
}

impl File {
    fn len(&self) -> u64 {
        self.end - self.start
    }
}

fn parse(input: &str) -> VecDeque<File> {
    let mut disk_map = VecDeque::default();
    let mut disk_index = 0;
    let mut file = true;

    for (index, number) in input.char_indices() {
        let len: u64 = number.to_digit(10).unwrap().into();
        if file {
            disk_map.push_back(File {
                start: disk_index,
                end: disk_index + len,
                id: (index as u64) / 2,
                fixed: false,
            });
        }
        file = !file;
        disk_index += len;
    }

    disk_map
}

fn checksum(disk_map: &VecDeque<File>) -> u64 {
    disk_map
        .iter()
        .map(|file| file.id * file.len() * (2 * file.start + file.len() - 1) / 2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "2333133121414131402";

    #[test]
    pub fn part1_todo() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    pub fn part2_todo() {
        assert_eq!(part2(INPUT), 2858);
    }
}
