use anyhow::Result;
use seq_io::fasta::{Reader, Record};

use crate::SymmetryMode;

use super::config::{Config, SearchParams};
use super::constants;
use super::find_irs;
use super::matrix;
use super::utils;

/// Attemps to extract the first sequence (string) from the fasta file. Returns a trimmed lowercase String.
///
/// Returns an error if there are no sequences.
fn extract_first_sequence(config: &Config) -> Result<String> {
    utils::check_file_exist(config.input_file)?;
    let mut reader = Reader::from_path(config.input_file)?;
    let record = reader
        .next()
        .expect("No sequences found")
        .expect("Error reading record");

    Ok(std::str::from_utf8(record.seq())
        .unwrap()
        .to_lowercase()
        .replace('\n', ""))
}

// Test for an edge case with truncation (needs complement and matrix).
fn correct_truncation_helper(config: &Config) {
    let string = extract_first_sequence(config).unwrap();
    let seq = string.to_ascii_lowercase().as_bytes().to_vec();
    let n = seq.len();
    config.params.check_bounds(n).unwrap();
    let irs = find_irs(&config.params, &seq).unwrap();

    let complement = constants::build_complement_array();
    let s_n = 2 * n + 2;
    let mut s = vec![0u8; s_n];
    for i in 0..n {
        s[i] = seq[i];
        s[n + 1 + i] = complement[seq[n - 1 - i] as usize] as u8;
    }
    s[n] = b'$';
    s[2 * n + 1] = b'#';
    let matrix = matrix::MatchMatrix::new();

    for (left, right, _) in irs {
        assert!(matrix.match_u8(s[left], complement[s[right] as usize]),);
    }
}

#[test]
fn test_correct_truncation_one() {
    let config = Config {
        params: SearchParams::new(8, 100, 10, 6).unwrap(),
        input_file: "tests/test_data/test1.fasta",
        ..Default::default()
    };
    correct_truncation_helper(&config);
}

#[test]
fn test_correct_truncation_two() {
    let config = Config {
        params: SearchParams::new(8, 100, 10, 6).unwrap(),
        input_file: "tests/test_data/truncation_edge_case.fasta",
        ..Default::default()
    };
    correct_truncation_helper(&config);
}

#[test]
fn test_correct_truncation_three() {
    let config = Config {
        params: SearchParams::new(6, 100, 0, 5).unwrap(),
        input_file: "tests/test_data/truncation_edge_case.fasta",
        ..Default::default()
    };
    correct_truncation_helper(&config);
}

// Tests from local files
//
// Test generator
fn find_irs_from_first_sequence(config: &Config) -> Vec<(usize, usize, usize)> {
    let string = extract_first_sequence(config).unwrap();
    let seq = string.to_ascii_lowercase().as_bytes().to_vec();
    config.params.check_bounds(seq.len()).unwrap(); // BUT THE OUTPUT FORMAT MIGHT BE WRONG?
    find_irs(&config.params, &seq).unwrap()
}

#[test]
fn test_irs_edge_gap() {
    // The original IUPACpal won't find this IR
    let config = Config {
        params: SearchParams::new(14, 100, 3, 0).unwrap(),
        input_file: "tests/test_data/edge_gap.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 1);
}

// #[test]
// fn test_irs_alys() {
//     let config = Config {
//         params: SearchParams::new(3, 100, 20, 0).unwrap(),
//         input_file: "tests/test_data/alys.fna",
//         ..Default::default()
//     };
//     assert_eq!(find_irs_from_first_sequence(&config).len(), 739_728);
// }

#[test]
fn test_irs_8100_n() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 0).unwrap(),
        input_file: "tests/test_data/8100N.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 16_189);
}

#[test]
fn test_irs_8100_n_with_mismatches() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 2).unwrap(),
        input_file: "tests/test_data/8100N.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 16_189);
}

#[test]
fn test_irs_d00596() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 0).unwrap(),
        input_file: "tests/test_data/d00596.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 5251);
}

#[test]
fn test_irs_d00596_with_mismatches() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 2).unwrap(),
        input_file: "tests/test_data/d00596.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 31_555);
}

#[test]
fn test_rand_1000() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 0).unwrap(),
        input_file: "tests/test_data/rand1000.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 254);
}

#[test]
fn test_rand_10000() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 0).unwrap(),
        input_file: "tests/test_data/rand10000.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 2484);
}

#[test]
fn test_rand_100000() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 0).unwrap(),
        input_file: "tests/test_data/rand100000.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 25_440);
}

#[test]
fn test_rand_1000000() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 0).unwrap(),
        input_file: "tests/test_data/rand1000000.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 253_566);
}

#[test]
fn test_test_1() {
    let config = Config {
        params: SearchParams::new(3, 100, 20, 0).unwrap(),
        input_file: "tests/test_data/test1.fasta",
        ..Default::default()
    };
    assert_eq!(find_irs_from_first_sequence(&config).len(), 84);
}

// TODO: add more tests! (and improve this one below!)

fn mk_test_symmetry(seq_str: &str, symmetry_mode: SymmetryMode, expected: usize) {
    let seq = seq_str.as_bytes();
    let params1 = SearchParams::with_mode(3, 7, 2, 0, symmetry_mode).unwrap();
    let irs = find_irs(&params1, &seq).unwrap();    // Print the results for debugging
    println!("IRs found: {:?}", irs);
    assert_eq!(irs.len(), expected);
}

#[test]
fn test_simple_direct_mode() {
    mk_test_symmetry("aataat", SymmetryMode::Direct, 1);
    mk_test_symmetry("aataat", SymmetryMode::default(), 0);
} 
#[test]
fn test_simple_direct_complementary_mode() {
    mk_test_symmetry("aaattt", SymmetryMode::DirectComplementary, 1);
    // mk_test_symmetry("aaattt", SymmetryMode::default(), 1);
}

#[test]
fn test_simple_inverted_complementary_mode() { 
    mk_test_symmetry("aattaa", SymmetryMode::InvertedComplementary, 1);
    mk_test_symmetry("aattaa", SymmetryMode::default(), 0);
}
 
#[test]
fn test_simple_inverted_complementary_mode__() {  
    println!("I");
    mk_test_symmetry("atatat", SymmetryMode::default(), 1);
    println!("II");
    mk_test_symmetry("atatat", SymmetryMode::Direct, 1);
}
 