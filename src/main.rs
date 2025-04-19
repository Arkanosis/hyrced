use std::process;

use serde_derive::Deserialize;

const USAGE: &str = "
Usage: hyrced start [--hostname=<hostname>] [--port=<port>]
       hyrced -h | --help
       hyrced --version

Commands:
    start                    Start the remote command execution daemon.

Options:
    -h, --help               Show this screen.
    --hostname=<hostname>    Hostname to resolve to find the network interface to listen on [default: localhost].
    --port=<port>            Port to listen to [default: 8080].
    --version                Show version.
";

#[derive(Deserialize)]
struct Args {
    cmd_start: bool,
    flag_hostname: String,
    flag_port: u16,
    flag_version: bool,
}

fn main() {
    let args: Args =
        docopt::Docopt::new(USAGE)
            .and_then(|docopts|
                docopts.argv(std::env::args().into_iter())
                   .deserialize()
            )
            .unwrap_or_else(|error|
                error.exit()
            );

    if args.flag_version {
        println!("hyrced v{}", hyrced::version());
    } else {
        if args.cmd_start {
            if hyrced::start(args.flag_hostname, args.flag_port).is_err() {
                process::exit(1);
            }
        }
    }
}
