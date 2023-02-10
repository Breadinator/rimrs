# RimRs
A 3rd party mod manager for RimWorld based on RimPy. Made because (1) RimPy is closed source and (2) RimPy is pretty slow.

Under heavy development, wouldn't recommend.

# Usage
Just launch the binary `rimrs.exe`. I recommend you run it a command prompt because most errors are logged there.

# Installation
## Prebuilt binaries
See the [GitHub releases page](https://github.com/Breadinator/rimrs/releases).

## From source
```
cargo install rimrs
```

or

```
git install --git https://github.com/Breadinator/rimrs.git
```

or

```
git clone https://github.com/Breadinator/rimrs.git
cd rimrs
cargo install --path .
```

After using [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html), it'll probably be in `$HOME/.cargo/bin/rimrs.exe`.

# Roadmap
Not sure if this is the actual order, but this is what I intend on doing.

- [x] 0.1.0: Basic mod ordering
- [ ] 0.2.0: Basic mod sorting
- [ ] 0.3.0: Settings independent of RimPy (but still use RimPy config if none detected?)
- [ ] 1.0.0: Windows release

## Other planned features
Features that I plan on adding but don't remotely know when.

- [ ] Sorting based on Fluffy's Mod Manager files
- [ ] Sorting based on RimPy's community rules
- [ ] Linux support

