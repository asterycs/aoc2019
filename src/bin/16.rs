use std::env;
use std::fs;
use std::path::PathBuf;

struct InputGenerator {
    sequence: Vec<i32>,
    idx: usize,
    current_repetition: usize,
    repetitions: usize,
}

impl InputGenerator {
    fn new(sequence: Vec<i32>, repetitions: usize) -> InputGenerator {
        InputGenerator {
            sequence: sequence,
            idx: 0,
            current_repetition: 0,
            repetitions: repetitions,
        }
    }

    fn reset(&mut self) {
        self.idx = 0;
        self.current_repetition = 0;
    }

    fn reverse(&mut self) {
        self.sequence.reverse();
    }
}

impl Iterator for InputGenerator {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.idx >= self.sequence.len() {
            self.idx = 0;
            self.current_repetition += 1;

            self.current_repetition = std::cmp::min(self.current_repetition, self.repetitions);
        }

        self.idx += 1;

        if self.current_repetition >= self.repetitions {
            None
        } else {
            Some(self.sequence[self.idx - 1])
        }
    }
}

impl InputGenerator {
    fn len(&self) -> usize {
        self.repetitions * self.sequence.len()
    }
}

fn to_num_vec(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>()
}

fn ex_data() -> (InputGenerator, Vec<i32>) {
    let input = "12345678";
    let output = "48226158";

    (
        InputGenerator::new(to_num_vec(input), 1),
        to_num_vec(output),
    )
}

fn test11_data() -> (InputGenerator, Vec<i32>) {
    let input = "80871224585914546619083218645595";
    let output = "24176176";

    (
        InputGenerator::new(to_num_vec(input), 1),
        to_num_vec(output),
    )
}

fn test12_data() -> (InputGenerator, Vec<i32>) {
    let input = "19617804207202209144916044189917";
    let output = "73745418";

    (
        InputGenerator::new(to_num_vec(input), 1),
        to_num_vec(output),
    )
}

fn test13_data() -> (InputGenerator, Vec<i32>) {
    let input = "69317163492948606335995924319873";
    let output = "52432133";

    (
        InputGenerator::new(to_num_vec(input), 1),
        to_num_vec(output),
    )
}

fn test21_data() -> (InputGenerator, usize, Vec<i32>) {
    let input = "03036732577212944063491565474664";
    let output = "84462026";

    let skip = input
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (
        InputGenerator::new(to_num_vec(input), 10000),
        skip,
        to_num_vec(output),
    )
}

fn test22_data() -> (InputGenerator, usize, Vec<i32>) {
    let input = "02935109699940807407585447034323";
    let output = "78725270";

    let skip = input
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (
        InputGenerator::new(to_num_vec(input), 10000),
        skip,
        to_num_vec(output),
    )
}

fn test23_data() -> (InputGenerator, usize, Vec<i32>) {
    let input = "03081770884921959731165446850517";
    let output = "53553731";
    let skip = input
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (
        InputGenerator::new(to_num_vec(input), 10000),
        skip,
        to_num_vec(output),
    )
}

fn get_input() -> String {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/16.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    input
}

struct BaseGenerator {
    base_sequence: Vec<i32>,
    base_idx: usize,
    current_repetition: usize,
    repetitions: usize,
}

impl BaseGenerator {
    fn new(repetitions: usize) -> BaseGenerator {
        BaseGenerator {
            base_sequence: vec![0, 1, 0, -1],
            base_idx: 0,
            current_repetition: 1,
            repetitions: repetitions,
        }
    }
}

impl Iterator for BaseGenerator {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.current_repetition > self.repetitions {
            self.base_idx = (self.base_idx + 1) % self.base_sequence.len();
            self.current_repetition = 0;
        }

        self.current_repetition += 1;
        Some(self.base_sequence[self.base_idx])
    }
}

fn fft(mut input: InputGenerator, phases: usize) -> Vec<i32> {
    let mut output;
    let input_len = input.len();

    for _phase in 0..phases {
        output = Vec::new();

        for iter in 0..input_len {
            input.reset();
            let mut base = BaseGenerator::new(iter);
            let mut dot = 0;

            while let Some(i) = input.next() {
                dot += i * base.next().unwrap();
            }

            let ls_digit = dot.abs() % 10;
            output.push(ls_digit as i32);
        }

        input = InputGenerator::new(output, 1);
    }

    input.sequence // contains output
}

fn fft_fake(mut input: InputGenerator, phases: usize, skip: usize) -> Vec<i32> {
    input.reverse();

    let input_len = input.len();
    let rev_len = input_len - skip;
    let mut output;

    for _phase in 0..phases {
        output = Vec::new();

        input.reset();
        let mut dot = 0;

        for _ in 0..rev_len {
            dot += input.next().unwrap();
            output.push(dot.abs() % 10);
        }

        input = InputGenerator::new(output, 1);
    }

    let mut output = input.sequence; //contains output
    output.reverse();
    output
}

fn main() {
    let input = get_input();

    let input_gen = InputGenerator::new(to_num_vec(&input), 1);
    let output = fft(input_gen, 100);

    println!("Part 1 output: {:?}", &output[0..8]);

    let skip = input
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let input_gen = InputGenerator::new(to_num_vec(&input), 10000);
    println!("Part 2 input length: {}", input_gen.len());
    let output = fft_fake(input_gen, 100, skip);

    println!("Part 2 output: {:?}", &output[0..8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let (input, ground_truth) = ex_data();

        let output = fft(input, 1);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test11() {
        let (input, ground_truth) = test11_data();
        let output = fft(input, 100);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test12() {
        let (input, ground_truth) = test12_data();
        let output = fft(input, 100);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test13() {
        let (input, ground_truth) = test13_data();
        let output = fft(input, 100);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test21() {
        let (input, skip, ground_truth) = test21_data();
        let output = fft_fake(input, 100, skip);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test22() {
        let (input, skip, ground_truth) = test22_data();
        let output = fft_fake(input, 100, skip);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test23() {
        let (input, skip, ground_truth) = test23_data();
        let output = fft_fake(input, 100, skip);

        assert_eq!(output[0..8], ground_truth[..]);
    }
}
