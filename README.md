# Sapling 🌱
Mini-tree CLI program, written in Rust<br/>

**Table of Contents**
* [Goals](#goals)
* [Tree-like TUI](#tree-like-tui)
* [Todo's](#todos)

## Goals
* no dependencies (except `std`)
* put it on ...
    * [crates.io](https://crates.io/)
    * [nixpkgs](https://search.nixos.org/packages?)
    * [Homebrew](https://brew.sh/)

 ## Tree-like TUI
**Box Drawings**<br/>
[Box-Drawing Characters Wikipedia page](https://en.wikipedia.org/wiki/Box-drawing_characters)<br/>
There's a nice table of box drawing characters and their Unicode codes [here](https://en.wikipedia.org/wiki/Box-drawing_characters#Box_Drawing)
```rust
'│' // show hierarchy         - '\u{2502}'
'─' // child branch           - '\u{2500}'
'└' // last child in parent   - '\u{2514}'
'├' // show sibling hierarchy - '\u{251c}'
```

**Ascii**
```rust
'|'  // show hierarchy         - pipe char
'-'  // child branch           - en-dash char
'L'  // last child in parent   - capital "ell" char
"|-" // show sibling hierarchy - pipe char + en-dash
```

## Todo's
- [ ] read command line arguments instead of asking for input
- [ ] add support for POSIX style opts / args
- [ ] make the formatting work
- [ ] add tree-like TUI (see above)
