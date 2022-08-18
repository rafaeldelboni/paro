# Paro
> paro : _to prepare, get ready / set, put / furnish, supply._  

Tool for managing dotfiles directories; Heavily based on [rcm](http://thoughtbot.github.io/rcm).

## TODO
- [x] Rust Boilerplate
- [x] CI Pipeline to lint and run test
- Base Features
  - [x] Parse inputs using Clap
    - [x] Add inputs to a internal settings structure
  - [x] Parse config files
    - [x] Add config settings to the internal settings structure
  - [x] Option to override host name from config (-B --hostname)
  - [x] Add defaults to the internal settings structure if nothing is defined
  - [x] Read .dotfile folder(s) and files structure and store it
  - [x] Consider ignore files and filters then from the list (-x --exclude)
  - [ ] Consider inclusion list for extra files and reorganize file list (-i --include)
  - [ ] Consider multiple dotfiles folders configuration and reorganize file list (-a --add-dir)
  - [ ] Consider tags and tag folders and reorganize file list (-t --tag)
  - [ ] Consider hosts and host folders and reorganize file list
  - [ ] Dialog to ask to override existing files if already exists in your home directory but does not match the file in your dotfiles directory
  - [ ] Option to always override (-f --force)
  - [ ] Add drop/delete command, this deletes dotfiles managed by paro (-d --down)
  - [ ] Add dry-run command (-D --dry-run)
  - [ ] Add version command (-v --version)
  - [ ] Consider .dot files and filters then from the list
- CI Pipeline to build releases
  - [ ] Linux (x86_64)
  - [ ] Linux (arm)
  - [ ] Macos (x86_64)
  - [ ] Macos (arm)
- [ ] Documentation
- [ ] Instalation Script (Like rustup install)
- Extras Features
  - [x] Configuration to override the destination file path will be symlinked or copied (-n --destination)
  - [ ] Sync command (delete files that are set to be ignored) (-S --sync)
  - [ ] Create an inclusion list for already doted files in your dotfiles directory to be included as symlink or copy (-I --include-dotted)
  - [ ] Split config files in two where you have configs and ignore files in different files
- Maybe Features
  - [ ] Execute Hooks (Pre/Post)
  - [ ] Skip hooks (-K --skip-hooks)
  - [ ] Option to always copy files instead symlinking them. (-C --copy)
  - [ ] Consider inclusion list to always copy files instead symlinking them. (-c --include-copy)
  - [ ] Option to install git hooks to run paro sync on post-commit and post-merge hooks

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

#### -V, --version
Print version information

#### -x, --exclude <file-pattern>
Do not install files that match <file-pattern>. This can be repeated with additional
patterns.

> Note: `paro -h` prints a short and concise overview while `paro --help` gives all details.

## Contributing
If you find any dead links, misinformation or any improvements in this software at all [Emails](https://github.com/rafaeldelboni), [PRs](https://github.com/rafaeldelboni/paro/pulls) and [Issues](https://github.com/rafaeldelboni/paro/issues) are highly encouraged.

## License
This is free and unencumbered software released into the public domain.  
For more information, please refer to <http://unlicense.org>
