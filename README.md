# Teva

<p align="center">
  <img alt="Teva logo" width="200" src="teva.png" />
</p>

Teva is a CLI tool that makes it easier to maintain a git revision history by
running tests for changed files on every commit of a branch. It creates a git
worktree, checks out to each SHA, gets changed files, and passes them a test
runner until reaching HEAD.

It includes signal handling for SIGNINT, the ability to define repository setup
steps and test runners with a config file, and is isolated from the main
worktree and any unstashed changes.

Check out the blog post introducing it
[here](https://zanebliss.github.io/blog/rusty-revisions).

## Installation

### Download the binary

#### Locally
If you only want the executable available from a specific directory, download the executable and copy it to the desired directory.

```bash
curl -L -o {project-dir}/bin/teva https://github.com/zanebliss/teva/releases/download/v1.0/teva-{architecture}
chmod u+x {project-dir}/bin/teva
```

#### Putting it in  `PATH`
Another option is to place the executable in an accessible directory such as `~/bin`, then export that directory to your `PATH`.  The example below uses `.zshrc`.

```bash
mkdir ~/.local/bin
curl -L -o ~/.local/bin/teva https://github.com/zanebliss/teva/releases/download/v1.0/teva-{architecture}
chmod u+x ~/.local/bin/teva
echo 'export PATH=$PATH:~/.local/bin' >> ~/.zshrc
source ~/.zshrc
```

#### Adding executable to `/usr/local/bin`
Another option is to add the executable to `/usr/local/bin`. Note, this will likely require `sudo`.
```bash
curl -L -o /usr/local/bin https://github.com/zanebliss/teva/releases/download/v1.0/teva-{architecture}
chmod u+x /usr/local/bin/teva
```

## Platforms

This is currently only tested on macOS, Pop!_OS and Ubuntu.
