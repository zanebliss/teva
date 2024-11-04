# revw

`revw` (revision warden) is a CLI tool that makes it easier to maintain a git
revision history by running tests for changed files on every commit of a
branch. It creates a git worktree, checks out to each SHA, gets changed files,
and passes them a test runner until reaching HEAD.

It includes signal handling for SIGNINT, the ability to define repository setup
steps and test runners with a config file, and is isolated from the main
worktree and any unstashed changes.

`revw` is a rewrite of `teva`, which was the first version of this tool.

Numerous improvements were made, but the primary improvement is the usage of
the `git2` crate instead of relying on process I/O.

Check out the original blog post introducing `teva` here.
[here](https://zanebliss.github.io/blog/rusty-revisions).
