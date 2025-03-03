# CubeSH - A Shell for UNIX-Like System
[简体中文](README.zh_CN.md) [English](README.md)
1. 由 Rust 编写的强大 shell。🎉
2. 在一行内，化繁为简。💪
3. 由一个 13 岁的开发者编写😰
4. 由 Rust 带来的内存安全。😊
# 第三方库
1. `regex` -- This crate provides routines for searching strings for matches of a regular expression (aka “regex”). The regex syntax supported by this crate is similar to other regex engines, but it lacks several features that are not known how to implement efficiently. This includes, but is not limited to, look-around and backreferences. In exchange, all regex searches in this crate have worst case O(m * n) time complexity, where m is proportional to the size of the regex and n is proportional to the size of the string being searched.
2. `crossterm` -- Crossterm is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces. It supports all UNIX and Windows terminals down to Windows 7.
# 如何构建
1. 安装 Rust。
2. 下载源代码。
3. 开始编译！ `cargo build --release`
# 联系我们
1. xiaokuai *rainyhowcool@outlook.com*
