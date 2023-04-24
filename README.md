# ghk

`ghk` is a simple git hooks manager.

At it's core all it does is symlink whatever files are in `.ghk` into
`.git/hooks`.

Some additional niceties include:

- stripping extensions (not yet)

This seems nice because you can get syntax highlighting and more. You can write,
`pre-commit.sh` and have it stripped to `pre-commit` on symlink.

- verifying `.ghk` (not yet)

This seems nice because it can keep your hooks healthy. Typos, etc., can be
avoided. Basically, it can error out if things aren't perfect

- finding git hooks defined elsewhere (not yet)

In theory you could find git hooks in language and framework dependent ways.
E.g., you could defined them in `package.hooks` in javascript projects.

## Intro to git hooks

See `git help hooks`.

## Usage

1. Define a `.ghk` folder of git hooks in your repo root
1. Run `ghk` in your repo root

## Installation

Is none for now. It's Rust so you can `cargo install --git ...` if you want to
experiement with the repo.
