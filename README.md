# Teva

<p align="center">
  <img alt="Teva logo" width="200" src="teva.png" />
</p>

> Adventure sandals for continuous commits.

Teva is a command line program that enables performing actions on a series of commits in a git repository, typically in a feature branch. It creates a separate worktree and checks out to every commit from `main` to `HEAD` and performs an action. Out of the box, Teva supports `bin/rails`, but other prebaked commands are coming.

Teva manages it's worktree in a separate directory, so it is idempotent and does not affect the "main worktree". It includes signal handling for SIGNINT to interrupt long-running processes, and it deletes the worktree after it's finished.

## Installation

The executable is not available via package managers, but installation is simple by downloading the executable from the releases and installing it manually.

After selecting the executable that matches the platform architecture, you can rename it to `teva`.

```bash
mv <some_path>/teva-arm64 <some_path>/teva
```

### Locally
If you only want the executable available from a specific directory, download the executable and copy it to the desired directory.

```bash
cp <some_path>/teva my_directory/bin
cd my_directory
./teva
```

### Adding executable to `PATH`
Another option is to place the executable in an accessible directory such as `~/bin`, then export that directory to your `PATH`.  The example below uses `.zshrc`.

```bash
cp <some_path>/teva ~/bin
echo 'export PATH=$PATH:~/bin' >> ~/.zshrc
source ~/.zshrc
teva
```

### Adding executable to `/usr/local/bin`
Another option is to add the executable to `/usr/local/bin`. Note, this may require root privileges.
```bash
sudo cp <some_path>/teva /usr/local/bin
source /usr/local/bin
teva
```

## Platforms

This is currently only tested on macOS, Pop!_OS and Ubuntu.
