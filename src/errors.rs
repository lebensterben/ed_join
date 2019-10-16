error_chain! {

    foreign_links {
        Fmt(std::fmt::Error)
            #[doc = "A wrapper around `std::fmt::Error`"];
        ParseInt(std::num::ParseIntError)
            #[doc = "A wrapper around `std::num::ParseIntError`"];
        Io(std::io::Error)
            #[doc = "A wrapper around `std::io::Error`"];
        Cli(clap::Error)
            #[doc = "A wrapper around `clap::Error`"];
    }

    errors {
        InputFileNotReadable(f: String) {
            description("invalid input file"),
            display("input file not found/readable: 'filepath == {}'", f)
        }

        InvalidParameterQ(t: String) {
            description("invalid parameter"),
            display("q should be an integer, and 1 <= q <= 10: 'q == {}'", t)
        }

        InvalidParameterTau(t: String) {
            description("invalid parameter"),
            display("tau should be an integer, and tau >= 1: 'tau == {}'", t)
        }

    }
}
