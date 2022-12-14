# Paro
> paro : _to prepare, get ready / set, put / furnish, supply._  

Tool for managing dotfiles directories; Heavily based on [rcm](http://thoughtbot.github.io/rcm).  
paro has first class support on macOS and Linux

## Installation

### Manual

For quick installation use:

```bash
bash < <(curl -s https://raw.githubusercontent.com/rafaeldelboni/paro/main/install.sh)
```

### Cargo

If you're a **Rust programmer**, paro can be installed with `cargo`.
```
git clone https://github.com/rafaeldelboni/paro
cd paro
cargo install --path .
```

## CLI Usage
`paro [OPTIONS]`

### Options

#### -a, --add-dir <folder-pattern>
Install dotfiles directories from the <folder-pattern>. This can be repeated with
additional patterns.

#### -B, --hostname <name>
Override the computer hostname by <name>. Shall return the standard host name for the
current machine.

#### -d, --down
Remove all the rc files that the paro suite knows about, This can be further controlled
with the -t, -B and -a flags.

#### -D, --dry-run
Shows what paro would do without causing the effects. A simulated or practice
performance; rehearsal.

#### -f, --force
Override if the file already exists in your home directory, does not prompt for how to
handle it.

#### -h, --help
Print help information

#### -i, --include <file-pattern>
Install files that match <file-pattern>. Despite being excluded by the -x flag or a
setting in the config.
This can be repeated with additional patterns.

#### -n, --destination <folder-name>
Override the destination folder by <folder-name>. By default this value is the current
user home directory.

#### -t, --tag <tag>
Do not install files that match <file-pattern>. Tagged files go in a directory named for
the tag, prefixed with tag-. Therefore, files under .dotfiles/tag-git are only installed
when installing using the git tag. This can be repeated with additional patterns.

#### -v, --verbose
Make the operation more talkative. This can be repeated for more verbosity.

#### -V, --version
Print version information

#### -x, --exclude <file-pattern>
Do not install files that match <file-pattern>. This can be repeated with additional
patterns.

> Note: `paro -h` prints a short and concise overview while `paro --help` gives all details.

## Configuration File
You can set any of the options above in the configuration using [TOML](https://github.com/toml-lang/toml) file format, check [`tests/settings.toml`](tests/settings.toml) for a full example. 

paro will search for the config file in the following locations: 
- `~/.parorc` 
- `~/.config/paro/parorc`
- `~/.dotfiles/parorc`
- `~/.dotfiles/config/paro/parorc`

### Sample
```toml
tags=["linux"]
excludes=["file.txt", "file2.txt"]
includes=[".file3.txt", ".hid/file4.txt"]
directories=["my-dotfiles/", ".dotfiles2/"]
destination="/home/user-name/"
hostname="override-my-computer-name"
```

### Defaults
By default paro defines these settings, that you can override with options above:
- directories: `~/.dotfiles`
- destination: `~/`
- hostname: `Unix Hostname (libc::gethostname)`

## Building

paro is written in Rust, so you'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.
paro compiles with Rust stable or newer. In general, paro tracks
the latest stable release of the Rust compiler.

To build paro:

```bash
git clone https://github.com/rafaeldelboni/paro
cd paro
cargo build --release
./target/release/paro --version
0.0.1
```
### Running tests

paro is relatively well-tested, including both unit tests and integration
tests. To run the full test suite, use:

```
$ cargo test --all
```

from the repository root.

## Progress
paro currently has all the features I use from rcm, but is in the plans to add more existing and new features.  
You can check the progress list: [`TODO.md`](TODO.md)

## Contributing
If you find any dead links, misinformation or any improvements in this software at all [Emails](https://github.com/rafaeldelboni), [PRs](https://github.com/rafaeldelboni/paro/pulls) and [Issues](https://github.com/rafaeldelboni/paro/issues) are highly encouraged.

## License
This is free and unencumbered software released into the public domain.  
For more information, please refer to <http://unlicense.org>
