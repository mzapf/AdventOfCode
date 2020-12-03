use anyhow::{ensure, Context};

pub struct Solver {
    // regex: regex::Regex,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            // regex: regex::Regex::new(r"(?-u)([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)")
            //     .expect("Invalid regex"),
        }
    }
}

impl crate::Solver for Solver {
    fn day(&self) -> u8 {
        2
    }

    fn is_input_safe(&self, input: &str) -> anyhow::Result<bool> {
        for line in input.lines() {
            let parts: Vec<_> = line.split(' ').collect();
            ensure!(parts.len() == 3, "Wrong number of parts: {}", line);
            let lowhigh: Vec<_> = parts[0].split('-').collect();
            ensure!(
                lowhigh.len() == 2,
                "Wrong number of parts for lowhigh: {}",
                line
            );
            let low = lowhigh[0]
                .parse::<u32>()
                .with_context(|| format!("Failed to convert low to int: {}", line))?;
            let high = lowhigh[1]
                .parse::<u32>()
                .with_context(|| format!("Failed to convert high to int: {}", line))?;

            ensure!(low > 0, "Low is 0: {}", line);
            ensure!(low < high, "Low not less than high: {}", line);
            ensure!(
                (high as usize) <= parts[2].len(),
                "High points outside the password: {}",
                line
            );
            ensure!(
                parts[1].len() == 2,
                "Character part has invalid length: {}",
                line
            );
            ensure!(
                &parts[1][1..2] == ":",
                "Character part is missing colon: {}",
                line
            );
        }
        Ok(true)
    }

    // fn solve(&self, input: &str) -> (String, String) {
    //     let mut output = (0, 0);

    //     for line in input.lines() {
    //         if let Some(captures) = self.regex.captures(line) {
    //             let low: usize = captures[1].parse().unwrap();
    //             let high: usize = captures[2].parse().unwrap();
    //             let char = captures[3].as_bytes()[0];

    //             let pwd = captures[4].as_bytes();
    //             let count = pwd.iter().copied().filter(|c| *c == char).count();
    //             if low <= count && count <= high {
    //                 output.0 += 1;
    //             }
    //             let char_at_low = pwd[low - 1] == char;
    //             let char_at_high = pwd[high - 1] == char;
    //             if char_at_low ^ char_at_high {
    //                 output.1 += 1;
    //             }
    //         }
    //     }

    //     (output.0.to_string(), output.1.to_string())
    // }

    unsafe fn solve(&self, input: &str) -> (String, String) {
        let mut output = (0, 0);

        let bytes = input.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut low = 0;
            while *bytes.get_unchecked(i) != b'-' {
                low = low * 10 + (*bytes.get_unchecked(i) - b'0');
                i += 1;
            }

            i += 1;
            let mut high = 0;
            while *bytes.get_unchecked(i) != b' ' {
                high = high * 10 + (*bytes.get_unchecked(i) - b'0');
                i += 1;
            }

            i += 1;
            let char = *bytes.get_unchecked(i);
            let mut count = 0;
            i += 2;

            let char_at_low = *bytes.get_unchecked(i + low as usize) == char;
            let char_at_high = *bytes.get_unchecked(i + high as usize) == char;
            if char_at_low ^ char_at_high {
                output.1 += 1;
            }

            i += 1;

            while *bytes.get_unchecked(i) != b'\n' {
                if *bytes.get_unchecked(i) == char {
                    count += 1;
                }
                i += 1;
            }

            if low <= count && count <= high {
                output.0 += 1;
            }

            i += 1;
        }

        (output.0.to_string(), output.1.to_string())
    }
}