# HyRCEd [![](https://img.shields.io/crates/v/hyrced.svg)](https://crates.io/crates/hyrced) [![License](https://img.shields.io/badge/license-ISC-blue.svg)](/LICENSE)

**HyRCEd** is a daemon providing remote command execution (RCE for short) over HTTP.

## Current Status

HyRCEd is still under active design and not yet ready for mainstream usage.

## Questions nobody has ever asked

### Is HyRCEd secure?

Of course it is: the “S” in the name stands for “secure”. It's just executing commands remotely over unencrypted HTTP, nothing more. What could go wrong?

Seriously, though: no, it's not. For now at least. I'll update the answer to this question when I'll consider it “reasonably secure”, but until then, it'd be foolish to run HyRCEd next to anything remotely important.

## Usage

```
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
```

## Compiling

Run `cargo build --release` in your working copy.

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/hyrced/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/hyrced/issues).

## License

hyrced is copyright (C) 2025 Jérémie Roquet <jroquet@arkanosis.net> and licensed under the ISC license.
