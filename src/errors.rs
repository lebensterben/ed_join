error_chain! {

    foreign_links {
        Fmt(std::fmt::Error)
            #[doc = "A wrapper around `std::fmt::Error`"];
        ParseInt(std::num::ParseIntError)
            #[doc = "A wrapper around `std::num::ParseIntError`"];
        Io(std::io::Error)
            #[doc = "A wrapper around `std::io::Error`"];
        Cli(clap::Error)
            #[doc = "A wrapper around `clap::Error`"] #[cfg(feature = "cli")];
    }

    errors {
        InputFileNotReadable(f: String) {
            description("invalid input file"),
            display("input file not found/readable: 'filepath = {}'", f)
        }

        QTooSmall(q: usize) {
            description("q is too small"),
            display("q should be an integer, and q >= 1 : 'q = {}'", q)
        }

        QTooLarge(q: usize, min_line_len: usize) {
            description("q is to large"),
            display("q cannot excess the length of records: 'q = {} > {}'", q, min_line_len)
        }

        TauTooSmall(t: usize) {
            description("tau is to small"),
            display("tau should be an integer, and tau >= 1: 'tau = {}'", t)
        }

    }
}
