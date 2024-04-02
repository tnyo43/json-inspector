use getopts::Options;
use once_cell::sync::OnceCell;

pub struct Option {
    pub files: Vec<String>,
    pub silent: bool,
}
pub static INSTANCE: OnceCell<Option> = OnceCell::new();

impl Option {
    pub fn from_cli(args: &[String]) -> Self {
        let mut silent = false;

        let mut opts = Options::new();
        opts.optflag("s", "silent", "no console output");
        let matches = match opts.parse(args) {
            Ok(m) => m,
            Err(f) => panic!("{}", f.to_string()),
        };
        if matches.opt_present("s") {
            silent = true;
        }
        let files = matches.free.clone();

        Self { files, silent }
    }
}
