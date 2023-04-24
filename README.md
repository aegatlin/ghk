# ghk

`ghk` solves the problem of consistent git hooks within a single repository.

## Intro to git hooks

Git hooks are scripts in `.git/hooks` that git will trigger at certain points in
git's execution. These points are essentially life-cycle events that git allows
you to hook into, like `publish`, `pre-commit`, etc.

See `git help hooks` for more information.

## How ghk works 

When you run `ghk` within a repository, the following will happen.

1. Find the repo's git hooks

You can store a directory of scripts, e.g., `.ghk/pre-commit`. Or, if `ghk`
supports it, you can use a language or framework specific location, e.g., within
`package.json` `ghk.pre-commit`.

2. Enable `.git/hooks` based on findings

`ghk` can do a few things here. The simplest is linking your directory or scripts directly to
`.git/hooks`. If you are using a language or framework specific location, then the process will
depend on the specifics on where and how it is defined. For example, if the scripts are in `package.json` 
`ghk.pre-commit` as a string, `ghk` will create `.git/hooks/pre-commit` with that string as the content.

## Usage

On the command line run `ghk`. That's it!

## Installation

### Rust

`cargo install ghk`
