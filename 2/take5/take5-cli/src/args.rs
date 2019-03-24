use docopt::Docopt;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(feature = "ext")]
const USAGE: &'static str = "
The card game take5 (or 6nimmit!).

Usage:
    take5 <players>
    take5 [--conf=<file>, --bulls=<file>, --human] <players>
    take5 [-hv]

Options:
    -h, --help      Show this screen.
    -v, --version   Show the version of take5.
    --conf=<file>   Global configurations for the game.
    --bulls=<file>  Designates file as the configuration for bull values.
                    This file should contian exactly `deck_size` comma separated bull values.
    -H, --human     Use human players instead of AI players.
";

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(not(feature = "ext"))]
const USAGE: &'static str = "
The card game take5 (or 6nimmit!).

Usage:
    take5 <players>
    take5 [--conf=<file>] <players>
    take5 [-hv]

Options:
    -h, --help      Show this screen.
    -v, --version   Show the version of take5.
    --conf=<file>   Global configurations for the game.
";

/// Command line arguments for the game.
#[derive(RustcDecodable)]
pub struct Args {
    /// Number of players to create.
    pub arg_players: u8,
    /// Optional configuration file.
    pub flag_conf: Option<String>,
    /// Optional bull value mapping.
    #[cfg(feature = "ext")]
    pub flag_bulls: Option<String>,
    /// If the game should be played with human players.
    #[cfg(feature = "ext")]
    pub flag_human: bool,
}

impl Args {
    /// Create an `Args` struct from the command line arguments, and
    /// validate them.
    pub fn parse() -> Self {
        let version = format!("{}.{}.{}{}",
                              env!("CARGO_PKG_VERSION_MAJOR"),
                              env!("CARGO_PKG_VERSION_MINOR"),
                              env!("CARGO_PKG_VERSION_PATCH"),
                              option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));
        let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.version(Some(version)).decode())
                             .unwrap_or_else(|e| e.exit());
        args.validate();
        args
    }

    fn validate(&self) {
        if self.arg_players < 2 || self.arg_players > 10 {
            panic!("Invalid number of players.");
        }
    }
}
