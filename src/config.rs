use anyhow::{Result, anyhow};

use crate::constants::{
    DEFAULT_INPUT_FILE, DEFAULT_MAX_GAP, DEFAULT_MAX_LEN, DEFAULT_MIN_LEN, DEFAULT_MISMATCHES,
    DEFAULT_OUTPUT_FILE, DEFAULT_SEQ_NAME, OutputFormat,
};

#[derive(Debug, Default, Clone)]
pub enum SymmetryMode {
    #[default]
    Inverted, // TODO: Pon comentario
    InvertedComplementary,
    Direct,
    DirectComplementary,
}

impl std::str::FromStr for SymmetryMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "inverted" | "i" => Ok(SymmetryMode::Inverted),
            "inverted_complementary" | "ic" => Ok(SymmetryMode::InvertedComplementary),
            "direct" | "d" => Ok(SymmetryMode::Direct),
            "direct_complementary" | "dc" => Ok(SymmetryMode::DirectComplementary),
            _ => Err(format!("Invalid symmetry mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub min_len: usize,
    pub max_len: usize,
    pub max_gap: usize,
    pub mismatches: usize,
    pub symmetry_mode: SymmetryMode,
}

impl SearchParams {
    /// Constructor with symmetry mode as an argument. For Inverted Repeats use
    /// SearchParams::new.
    pub fn with_mode(
        min_len: usize,
        max_len: usize,
        max_gap: usize,
        mismatches: usize,
        symmetry_mode: SymmetryMode,
    ) -> Result<Self> {
        if min_len < 2 {
            return Err(anyhow!("min_len={} must not be less than 2.", min_len));
        }
        if min_len > max_len {
            return Err(anyhow!(
                "min_len={} must be less than max_len={}.",
                min_len,
                max_len
            ));
        }
        if mismatches >= min_len {
            return Err(anyhow!(
                "mismatches={} must be less than min_len={}.",
                mismatches,
                min_len
            ));
        }

        Ok(Self {
            min_len,
            max_len,
            max_gap,
            mismatches,
            symmetry_mode,
        })
    }

    pub fn new(min_len: usize, max_len: usize, max_gap: usize, mismatches: usize) -> Result<Self> {
        Self::with_mode(
            min_len,
            max_len,
            max_gap,
            mismatches,
            SymmetryMode::default(),
        )
    }

    pub fn check_bounds(&self, n: usize) -> Result<()> {
        if self.min_len >= n {
            return Err(anyhow!(
                "min_len={} must be less than sequence length={}.",
                self.min_len,
                n
            ));
        }
        if self.max_gap >= n {
            return Err(anyhow!(
                "max_gap={} must be less than sequence length={}.",
                self.max_gap,
                n
            ));
        }

        if self.mismatches >= n {
            return Err(anyhow!(
                "mismatches={} must be less than sequence length={}.",
                self.mismatches,
                n
            ));
        }

        Ok(())
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        Self::new(
            DEFAULT_MIN_LEN,
            DEFAULT_MAX_LEN,
            DEFAULT_MAX_GAP,
            DEFAULT_MISMATCHES,
        )
        .unwrap()
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    pub input_file: &'a str,
    pub seq_name: &'a str,
    pub params: SearchParams,
    pub output_file: &'a str,
    pub output_format: OutputFormat,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Config {
            input_file: DEFAULT_INPUT_FILE,
            seq_name: DEFAULT_SEQ_NAME,
            params: SearchParams::default(),
            output_file: DEFAULT_OUTPUT_FILE,
            output_format: OutputFormat::default(),
        }
    }
}

impl std::fmt::Display for Config<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "input_file:  {}", self.input_file)?;
        writeln!(f, "seq_name:    {}", self.seq_name)?;
        writeln!(f, "min_len:     {}", self.params.min_len)?;
        writeln!(f, "max_len:     {}", self.params.max_len)?;
        writeln!(f, "max_gap:     {}", self.params.max_gap)?;
        writeln!(f, "mismatches:  {}", self.params.mismatches)?;
        writeln!(f, "output_file: {}", self.output_file)?;
        writeln!(f, "output_fmt:  {}", self.output_format)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_min_len_less_than_two() {
        assert!(SearchParams::new(0, 100, 0, 0).is_err());
    }
}
