#!/bin/bash

# Build the docs.
cargo doc
# Get the current rev SHA.
rev=$(git rev-parse --short HEAD)
# Move into the doc's directory.
cd target/doc
# Create a git repo for the docs.
git init
# Add our repo as a remote.
git remote rm upstream
git remote add upstream "https://github.com/nixpulvis/hell"
# Fetch the upstream gh-pages branch.
git fetch upstream && git reset upstream/gh-pages
# Add the changed docs, and push them.
touch .
git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
