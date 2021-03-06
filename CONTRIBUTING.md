# Contributing

## VCS 

This project is hosted at [`git.nora.codes`](https://git.nora.codes/nora/rn2903)
and mirrored for convenience at [`github.com`](https://github.com/NoraCodes/rn2903).
The primary branch is called `main`.

## Changelog

This project uses semantic versioning and KeepAChangelog format. Always update
CHANGELOG.md.

## API Documentation

### Style

All method documentation is written in the present tense. For example, "Creates a new..."
rather than "Create a new...".

## Cutting a Release

When cutting a release:

- In CHANGELOG, rename the Unreleased section and add a new Unreleased section above it
- Make a commit with only that change
- Tag that commit like "v1.0.0"
- Push main and the tag
- `cargo publish`
