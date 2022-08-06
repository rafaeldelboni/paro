# Paro
> paro : _to prepare, get ready / set, put / furnish, supply._  

Tool for managing dotfiles directories; Heavily based on [rcm](http://thoughtbot.github.io/rcm).

## TODO
- [ ] Rust Boilerplate
- Base Features
  - [ ] Parse inputs using Clap
    - [ ] Add inputs to a internal config structure
  - [ ] Parse config files
    - [ ] Add config settings to the internal config structure
  - [ ] Read .dotfile folder structure and store it
  - [ ] Consider ignore files and filters then from the list
  - [ ] Consider tags and tag folders and reorganize file list (-t --tag)
  - [ ] Consider hosts and host folders and reorganize file list
  - [ ] Option to override host name from config (-B --hostname)
  - [ ] Consider inclusion list for extra files and reorganize file list (-i --include)
  - [ ] Consider multiple dotfiles folders configuration and reorganize file list (-a --add-dir)
  - [ ] Consider .dot files and filters then from the list
  - [ ] Dialog to ask to override existing files if already exists in your home directory but does not match the file in your dotfiles directory
  - [ ] Option to always override (-f --force)
  - [ ] Add drop/delete command, this deletes dotfiles managed by paro (-d --down)
  - [ ] Add dry-run command (-D --dry-run)
  - [ ] Add version command (-v --version)
- Extras Features
  - [ ] Configuration to override the destination file path will be symlinked or copied (-o --override-path)
  - [ ] Sync command (delete files that are set to be ignored) (-S --sync)
  - [ ] Create an inclusion list for already doted files in your dotfiles directory to be included as symlink or copy (-I --include-dotted)
  - [ ] Split config files in two where you have configs and ignore files in different files
- Maybe Features
  - [ ] Execute Hooks (Pre/Post)
  - [ ] Skip hooks (-K --skip-hooks)
  - [ ] Option to always copy files instead symlinking them. (-C --copy)
  - [ ] Consider inclusion list to always copy files instead symlinking them. (-c --include-copy)
- [ ] CI Pipeline to lint and run test
- CI Pipeline to build releases
  - [ ] Linux (x86_64)
  - [ ] Linux (arm)
  - [ ] Macos (x86_64)
  - [ ] Macos (arm)
- Instalation Script (Like rustup install)

## Contributing
If you find any dead links, misinformation or any improvements in this software at all [Emails](https://github.com/rafaeldelboni), [PRs](https://github.com/rafaeldelboni/paro/pulls) and [Issues](https://github.com/rafaeldelboni/paro/issues) are highly encouraged.

## License
This is free and unencumbered software released into the public domain.  
For more information, please refer to <http://unlicense.org>
