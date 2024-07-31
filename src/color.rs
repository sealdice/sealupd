//! This module provides methods to color console output. Coloring is disabled if
//! the console runs on Windows and is not Windows Terminal.
#![allow(unused)]

use std::{env, fmt::Display, sync::LazyLock};

static WINTERM_RUNNING: LazyLock<bool> = LazyLock::new(|| env::var("WT_SESSION").is_ok());

/// Contains a [`String`] and a series of [ANSI color codes].
///
/// [ANSI color codes]: https://talyian.github.io/ansicolors/
pub struct ColoredString {
    str: String,
    codes: Vec<i32>,
}

impl ColoredString {
    /// Creates a new [`ColoredString`] with an initial color code.
    /// If the code is 0, it will be ignored.
    pub fn new(s: &str, codes: &[i32]) -> Self {
        let mut vec = Vec::with_capacity(codes.len());
        for &code in codes {
            vec.push(code);
        }

        ColoredString {
            str: s.to_owned(),
            codes: vec,
        }
    }

    fn append_colors(&mut self, codes: &[i32]) {
        for &code in codes {
            self.codes.push(code);
        }
    }
}

impl Display for ColoredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ansi_ok = !cfg!(windows) || *WINTERM_RUNNING;
        if !ansi_ok || self.codes.is_empty() {
            write!(f, "{}", self.str)
        } else {
            let code_str = self
                .codes
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(";");
            write!(f, "\x1b[{}m{}\x1b[0m", code_str, self.str)
        }
    }
}

/// The convenience trait that allows plain strings to be colored.

// Implementation is on-demand; unused ANSI color codes will not be added.
pub trait Colorable {
    fn error(self) -> ColoredString;
    fn warn(self) -> ColoredString;
    fn success(self) -> ColoredString;
}

// Implementing for `&str` also implements for `String`.
impl<'a> Colorable for &'a str {
    fn error(self) -> ColoredString {
        ColoredString::new(self, &[31, 1])
    }

    fn warn(self) -> ColoredString {
        ColoredString::new(self, &[33])
    }

    fn success(self) -> ColoredString {
        ColoredString::new(self, &[32, 1])
    }
}

impl Colorable for ColoredString {
    fn error(mut self) -> ColoredString {
        self.append_colors(&[31, 1]);
        self
    }

    fn warn(mut self) -> ColoredString {
        self.append_colors(&[33]);
        self
    }

    fn success(mut self) -> ColoredString {
        self.append_colors(&[32, 1]);
        self
    }
}
