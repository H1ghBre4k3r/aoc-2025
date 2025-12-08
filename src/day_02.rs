use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range(u64, u64);

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<Range> {
    input
        .split(",")
        .map(|range| {
            let Some((left, right)) = range.split_once("-") else {
                unreachable!()
            };
            Range(left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(ranges: &[Range]) -> u64 {
    let mut result = 0;
    for Range(start, end) in ranges {
        'outter: for i in *start..=*end {
            let len = i.ilog10() + 1;
            if len % 2 != 0 {
                continue 'outter;
            }

            let str_val = i.to_string().chars().collect::<Vec<_>>();
            let half_len = (len / 2) as usize;
            for j in 0..half_len {
                if str_val[j] != str_val[j + half_len] {
                    continue 'outter;
                }
            }

            result += i;
        }
    }

    result
}

#[aoc(day2, part2)]
fn part2(ranges: &[Range]) -> u64 {
    let mut result = 0;
    for Range(start, end) in ranges {
        for i in *start..=*end {
            let val = i.to_string().chars().collect::<Vec<_>>();
            let len = val.len();
            let half_len = len / 2;

            'chunks: for chunk_length in 1..=half_len {
                if !len.is_multiple_of(chunk_length) {
                    continue;
                }

                let chunks = val.chunks(chunk_length).collect::<Vec<_>>();
                let first_chunk = chunks[0];

                for index_in_chunk in 0..chunk_length {
                    for chunk_index in 1..chunks.len() {
                        let current_chunk = chunks[chunk_index];
                        if first_chunk[index_in_chunk] != current_chunk[index_in_chunk] {
                            continue 'chunks;
                        }
                    }
                }

                result += i;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_generator_part_1() {
        let generated = generator(INPUT);

        assert_eq!(generated[0], Range(11, 22));
        assert_eq!(generated[10], Range(2121212118, 2121212124));
    }

    #[test]
    fn test_part_1() {
        let generated = generator(INPUT);
        let result = part1(&generated);

        assert_eq!(result, 1227775554)
    }

    #[test]
    fn test_part_2() {
        let generated = generator(INPUT);
        let result = part2(&generated);

        assert_eq!(result, 4174379265)
    }
}
