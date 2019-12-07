fn main() {
    let lower = 382345;
    let upper = 843167;

    let mut k = 0;

    for num in lower..upper {
        let mut double = false;
        let mut non_dec = true;

        {
            let mut prev = num
                .to_string()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap();

            for c in num.to_string().chars().skip(1) {
                let d = c.to_digit(10).unwrap();
                if d == prev {
                    double = true;
                    break;
                } else {
                    prev = d;
                }
            }
        }

        {
            let mut prev = num
                .to_string()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap();

            for c in num.to_string().chars().skip(1) {
                let d = c.to_digit(10).unwrap();
                if d < prev {
                    non_dec = false;
                    break;
                } else {
                    prev = d;
                }
            }
        }

        if non_dec && double {
            k += 1;
        }
    }

    println!("k: {}", k);
}
