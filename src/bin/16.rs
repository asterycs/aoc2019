use std::env;
use std::fs;
use std::path::PathBuf;

fn to_num_vec(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>()
}

fn ex_data() -> (Vec<i32>, Vec<i32>) {
    let input = "12345678";
    let output = "48226158";

    (to_num_vec(input), to_num_vec(output))
}

fn test1_data() -> (Vec<i32>, Vec<i32>) {
    let input = "80871224585914546619083218645595";
    let output = "24176176";

    (to_num_vec(input), to_num_vec(output))
}

fn test2_data() -> (Vec<i32>, Vec<i32>) {
    let input = "19617804207202209144916044189917";
    let output = "73745418";

    (to_num_vec(input), to_num_vec(output))
}

fn test3_data() -> (Vec<i32>, Vec<i32>) {
    let input = "69317163492948606335995924319873";
    let output = "52432133";

    (to_num_vec(input), to_num_vec(output))
}

fn get_input() -> Vec<i32> {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/16.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    to_num_vec(&input)
}

fn get_base(repetitions: usize, len: usize) -> Vec<i32> {
    let base_seq: Vec<i32> = vec![0, 1, 0, -1];
    let base_seq_len = base_seq.len();
    let mut base: Vec<i32> = Vec::new();

    let sequences = (len + base_seq_len) / base_seq_len;

    for _ in 0..sequences {
        base.append(&mut base_seq.to_vec());
    }

    let mut out = Vec::new();
    out.reserve(base.len() * (repetitions + 1) - 1);

    for e in base.iter() {
        for _ in 0..repetitions + 1 {
            out.push(e.clone());
        }
    }

    out[1..len + 1].into_iter().cloned().collect()
}

fn fft(input: &Vec<i32>, phases: usize) -> Vec<i32> {
    let mut input = input.to_vec();
    let mut output;
    let input_len = input.len();

    for _ in 0..phases {
        output = Vec::new();

        for iter in 0..input_len {
            let base = get_base(iter, input_len);

            let dot: i32 = input.iter().zip(base.iter()).map(|(i, b)| i * b).sum();

            let ls_digit = dot
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();
            output.push(ls_digit as i32);
        }

        input = output;
    }

    input // contains output
}

fn main() {
    let input = get_input();
    let output = fft(&input, 100);

    println!("Part 1 output: {:?}", &output[0..8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let (input, ground_truth) = ex_data();

        let output = fft(&input, 1);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test1() {
        let (input, ground_truth) = test1_data();
        let output = fft(&input, 100);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test2() {
        let (input, ground_truth) = test2_data();
        let output = fft(&input, 100);

        assert_eq!(output[0..8], ground_truth[..]);
    }

    #[test]
    fn test3() {
        let (input, ground_truth) = test3_data();
        let output = fft(&input, 100);

        assert_eq!(output[0..8], ground_truth[..]);
    }
}
