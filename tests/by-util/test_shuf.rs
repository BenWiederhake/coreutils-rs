// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
use crate::common::util::TestScenario;

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails().code_is(1);
}

#[test]
fn test_output_is_random_permutation() {
    let input_seq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = input_seq
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n");

    let result = new_ucmd!().pipe_in(input.as_bytes()).succeeds();
    result.no_stderr();

    let mut result_seq: Vec<i32> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    result_seq.sort_unstable();
    assert_ne!(result.stdout_str(), input, "Output is not randomized");
    assert_eq!(result_seq, input_seq, "Output is not a permutation");
}

#[test]
fn test_zero_termination() {
    let input_seq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = new_ucmd!().arg("-z").arg("-i1-10").succeeds();
    result.no_stderr();

    let mut result_seq: Vec<i32> = result
        .stdout_str()
        .split('\0')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    result_seq.sort_unstable();
    assert_eq!(result_seq, input_seq, "Output is not a permutation");
}

#[test]
fn test_very_large_range() {
    let num_samples = 10;
    let result = new_ucmd!()
        .arg("-n")
        .arg(&num_samples.to_string())
        .arg("-i0-1234567890")
        .succeeds();
    result.no_stderr();

    let result_seq: Vec<isize> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(result_seq.len(), num_samples, "Miscounted output length!");
    assert!(
        result_seq.iter().all(|x| (0..=1234567890).contains(x)),
        "Output includes element not from range: {}",
        result.stdout_str()
    );
}

#[test]
fn test_very_large_range_offset() {
    let num_samples = 10;
    let result = new_ucmd!()
        .arg("-n")
        .arg(&num_samples.to_string())
        .arg("-i1234567890-2345678901")
        .succeeds();
    result.no_stderr();

    let result_seq: Vec<isize> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(result_seq.len(), num_samples, "Miscounted output length!");
    assert!(
        result_seq
            .iter()
            .all(|x| (1234567890..=2345678901).contains(x)),
        "Output includes element not from range: {}",
        result.stdout_str()
    );
}

#[test]
fn test_very_high_range_full() {
    let input_seq = vec![
        2345678901, 2345678902, 2345678903, 2345678904, 2345678905, 2345678906, 2345678907,
    ];
    let result = new_ucmd!().arg("-i2345678901-2345678907").succeeds();
    result.no_stderr();

    let mut result_seq: Vec<isize> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    result_seq.sort_unstable();
    assert_eq!(result_seq, input_seq, "Output is not a permutation");
}

#[test]
fn test_range_repeat() {
    let num_samples = 500;
    let result = new_ucmd!()
        .arg("-r")
        .arg("-n")
        .arg(&num_samples.to_string())
        .arg("-i12-34")
        .succeeds();
    result.no_stderr();

    let result_seq: Vec<isize> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(result_seq.len(), num_samples, "Miscounted output length!");
    assert!(
        result_seq.iter().all(|x| (12..=34).contains(x)),
        "Output includes element not from range: {}",
        result.stdout_str()
    );
}

#[test]
fn test_empty_input() {
    let result = new_ucmd!().pipe_in(vec![]).succeeds();
    result.no_stderr();
    result.no_stdout();
}

#[test]
fn test_echo() {
    let input_seq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = new_ucmd!()
        .arg("-e")
        .args(
            &input_seq
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        )
        .succeeds();
    result.no_stderr();

    let mut result_seq: Vec<i32> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    result_seq.sort_unstable();
    assert_eq!(result_seq, input_seq, "Output is not a permutation");
}

#[test]
fn test_head_count() {
    let repeat_limit = 5;
    let input_seq = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = input_seq
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n");

    let result = new_ucmd!()
        .args(&["-n", &repeat_limit.to_string()])
        .pipe_in(input.as_bytes())
        .succeeds();
    result.no_stderr();

    let mut result_seq: Vec<i32> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    result_seq.sort_unstable();
    assert_eq!(result_seq.len(), repeat_limit, "Output is not limited");
    assert!(
        result_seq.iter().all(|x| input_seq.contains(x)),
        "Output includes element not from input: {}",
        result.stdout_str()
    );
}

#[test]
fn test_repeat() {
    let repeat_limit = 15000;
    let input_seq = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input = input_seq
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n");

    let result = new_ucmd!()
        .arg("-r")
        .args(&["-n", &repeat_limit.to_string()])
        .pipe_in(input.as_bytes())
        .succeeds();
    result.no_stderr();

    let result_seq: Vec<i32> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(
        result_seq.len(),
        repeat_limit,
        "Output is not repeating forever"
    );
    assert!(
        result_seq.iter().all(|x| input_seq.contains(x)),
        "Output includes element not from input: {:?}",
        result_seq
            .iter()
            .filter(|x| !input_seq.contains(x))
            .collect::<Vec<&i32>>()
    );
}

#[test]
fn test_file_input() {
    let expected_seq = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let result = new_ucmd!().arg("file_input.txt").succeeds();
    result.no_stderr();

    let mut result_seq: Vec<i32> = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    result_seq.sort_unstable();
    assert_eq!(result_seq, expected_seq, "Output is not a permutation");
}

#[test]
fn test_shuf_echo_and_input_range_not_allowed() {
    new_ucmd!()
        .args(&["-e", "0", "-i", "0-2"])
        .fails()
        .stderr_contains("cannot be used with");
}

#[test]
fn test_shuf_input_range_and_file_not_allowed() {
    new_ucmd!()
        .args(&["-i", "0-9", "file"])
        .fails()
        .stderr_contains("cannot be used with");
}

#[test]
fn test_shuf_invalid_input_range_one() {
    new_ucmd!()
        .args(&["-i", "0"])
        .fails()
        .stderr_contains("invalid input range");
}

#[test]
fn test_shuf_invalid_input_range_two() {
    new_ucmd!()
        .args(&["-i", "a-9"])
        .fails()
        .stderr_contains("invalid input range: 'a'");
}

#[test]
fn test_shuf_invalid_input_range_three() {
    new_ucmd!()
        .args(&["-i", "0-b"])
        .fails()
        .stderr_contains("invalid input range: 'b'");
}

#[test]
fn test_shuf_invalid_input_line_count() {
    new_ucmd!()
        .args(&["-n", "a"])
        .fails()
        .stderr_contains("invalid line count: 'a'");
}

#[test]
fn test_shuf_multiple_input_line_count() {
    let result = new_ucmd!()
        .args(&["-i10-200", "-n", "10", "-n", "5"])
        .succeeds();

    result.no_stderr();

    let result_count = result
        .stdout_str()
        .split('\n')
        .filter(|x| !x.is_empty())
        .count();
    assert_eq!(result_count, 5, "Output should have 5 items");
}
