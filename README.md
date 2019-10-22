[![Build Status](https://travis-ci.org/lebensterben/ed_join.svg?branch=master)](https://travis-ci.org/lebensterben/ed_join)

# `ed_join`

This is an implementation of Ed-join Algorithm proposed by Xiao et al. (2008) in Rust programming language. There're two major deviation from the original paper:

1. In this implementation I used parallel iteratiors instead of matching strings with a single thread.
2. I created an inverted index before the actual matching, while the paper generates an inverted index on-the-fly.

On a system with Intel i7-8700k and 32 GB ram, the program performed a self-match on a file with 1000 lines in 5 seconds.
Note that, this algorithm excels at matching long records, and the aforementioned testfile has rather short lines.

## Installation

To add this crate as a dependency, add it into your `Cargo.toml` or execute `cargo add ed_join`.

This crate also comes with an binary `ed-join`, which could be installed with `cargo install ed_join --features cli`.

## Reference

* Xiao, Chuan, Wei Wang, and Xuemin Lin. "Ed-join: an efficient algorithm for similarity joins with edit distance constraints." Proceedings of the VLDB Endowment 1.1 (2008): 933-944.

License: Apache-2.0 AND MIT
