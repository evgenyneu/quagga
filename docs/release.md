# Insruction how to release new version

1. Update the version in your Cargo.toml file.
2. Document changes in CHANGELOG.md.
3. Public to crates.io

```
cargo login
cargo package
cargo publish
```

4. Make a release on Github

```
git tag v0.1.2
git push origin v0.1.2
```

5. Update homebrew formula

Update the version in the homebrew formula:

https://github.com/evgenyneu/homebrew-quagga/blob/main/quagga.rb
