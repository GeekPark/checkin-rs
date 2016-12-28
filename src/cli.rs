use docopt::Docopt;

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_server: bool,
    cmd_init: bool,
}

pub enum CLIAction {
    Server,
    Init,
}

static USAGE: &'static str = include_str!("usage.txt");

pub fn parse_argv() -> Option<CLIAction> {
    let arg: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    if arg.cmd_server {
        Some(CLIAction::Server)
    } else if arg.cmd_init {
        Some(CLIAction::Init)
    } else {
        None
    }
}
