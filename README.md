# Teva

<p align="center">
  <img alt="Teva logo" width="200" src="teva.png" />
</p>

> Adventure sandals for continuous commits.

Teva is a command line program to run arbitrary actions on a series commits in a repository, typically from `main` to `HEAD` in a feature branch. It is idempotent and does not affect the "main worktree". This is because it manages a separate directory and worktree.  It includes signal handling to interrupt long running processes, and it always cleans up after itself.

As of version `0.1.0` it only supports a pre-baked `rspec` test runner, but config file defined runners are coming soon.

## Installation

The executable is not available via package managers. There are a few ways to install it.

### Locally
If you only want the executable available from a specific place, simply download the release and copy it to the desired directory.

```bash
$ cp <some_path>/teva my_directory
$ cd my_directory
$ ./teva
```

### Adding executable to `PATH`
Place the executable in an accessible directory such as `~/bin` and add it to `PATH`. The example below uses `.zshrc`. Be sure to substitute your own config.

```bash
$ cp <some_path>/teva ~/bin
$ echo 'export PATH=$PATH:~/bin' >> ~/.zshrc
$ source ~/.zshrc
$ teva
```

### Adding executable to `/usr/local/bin`
Add the executable to `/usr/local/bin`. Note, this may require root privledges.
```bash
$ sudo cp <some_path>/teva /usr/local/bin
$ source /usr/local/bin
$ teva
```

## Platforms

This is currently only tested on macOS, Pop!_OS and Ubuntu.
