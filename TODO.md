# TODOs

## Initial
- [x] Rust Boilerplate
- [x] CI Pipeline to lint and run test

## Base Features
- [x] Parse inputs using Clap
  - [x] Add inputs to a internal settings structure
- [x] Parse config files
  - [x] Add config settings to the internal settings structure
- [x] Option to override host name from config (-B --hostname)
- [x] Add defaults to the internal settings structure if nothing is defined
- [x] Read .dotfile folder(s) and files structure and store it
- [x] Consider multiple dotfiles folders configuration and reorganize file list (-a --add-dir)
- [x] Consider ignore files and filters then from the list (-x --exclude)
- [x] Consider .dot files and filters then from the list
- [x] Consider inclusion list for extra files and reorganize file list (-i --include)
- [x] Consider tags and tag folders and reorganize file list (-t --tag)
  - [x] Consider hosts and host folders and reorganize file list
- [x] Make the top level files hidden in the destination file list
- [x] Get action list and link/copy the files
- [x] Dialog to ask to override existing files if already exists in your home directory but does not match the file in your dotfiles directory
- [x] Option to always override (-f --force)
- [x] Add version command (-V --version)
- [x] Add dry-run command (-D --dry-run)
- [x] Add verbosity command (-v --verbose)
- [x] Integration tests
- [x] Documentation
- [x] Add drop/delete command, this deletes dotfiles managed by paro (-d --down)
- [ ] Instalation Script (Like rustup install)

### CI Pipeline to build releases
- [x] Linux (x86_64)
- [x] Linux (arm)
- [x] Macos (x86_64)
- [x] Macos (arm)

## Extras Features
- [x] Configuration to override the destination file path will be symlinked or copied (-n --destination)
- [ ] Sync command (delete files that are set to be ignored) (-S --sync)
- [ ] Split config files in two where you have configs and ignore files in different files

### Maybe Features
- [ ] Execute Hooks (Pre/Post)
- [ ] Skip hooks (-K --skip-hooks)
- [ ] Option to always copy files instead symlinking them. (-C --copy)
- [ ] Consider inclusion list to always copy files instead symlinking them. (-c --include-copy)
- [ ] Consider argument list to don't hide the file in the destination. (-N --no-hide)
- [ ] Option to install git hooks to run paro sync on post-commit and post-merge hooks
