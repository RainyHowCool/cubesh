# CubeSH - A Shell for UNIX-Like System
[English](README.md) [简体中文](README.zh_CN.md)
1. A powerful shell written in Rust.🎉
2. Simplify in one line.💪
3. Source code written by a 13-year-old developer.😰
4. Memory Safe.😊
# Third-Party crates
1. `regex` -- This crate provides routines for searching strings for matches of a regular expression (aka “regex”). The regex syntax supported by this crate is similar to other regex engines, but it lacks several features that are not known how to implement efficiently. This includes, but is not limited to, look-around and backreferences. In exchange, all regex searches in this crate have worst case O(m * n) time complexity, where m is proportional to the size of the regex and n is proportional to the size of the string being searched.
2. `crossterm` -- Crossterm is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces. It supports all UNIX and Windows terminals down to Windows 7.
# Build it
1. Install Rust.
2. Download Source Code.
3. Build it! `cargo build --release`
# Contact us
1. xiaokuai *rainyhowcool@outlook.com*
