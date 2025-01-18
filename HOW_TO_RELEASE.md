To generate a new version, you need to follow these steps:

1. In the `main` branch, you must bump the version inside the `Cargo.toml` file.
2. Run `make rs-check` so that the version is changed in the `Cargo.lock` file.
3. Run the command `git add -A && git commit -m "release: bump version"`.
4. Run the command `git tag -a <your.new.version> -m "version <your.new.version>"`.
5. Run the command `make doc-changelog && make doc-readme`.
6. Run the command `git add -A && git commit -m "release: <your.new.version>"`.
7. Run `git push` to `main`.
