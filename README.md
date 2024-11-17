# Sapling 🌱
Mini-tree CLI program, written in Rust

## Todo's
[ ] read command line arguments instead of asking for input<br/>
[ ] add tree-like UI (see below)<br/>

 ## Tree-like UI
**Box Drawings**
[Box-Drawing Characters Wikipedia page](https://en.wikipedia.org/wiki/Box-drawing_characters)
There's a nice table of box drawing characters and their Unicode codes [here](https://en.wikipedia.org/wiki/Box-drawing_characters#Box_Drawing)
```rust
'│' // show hierarchy - '\u{2502}'
'─' // child branch - '\u{2500}'
'└' // last child in parent '\u{2514}'
'├' // show sibling hierarchy - '\u{251c}'
```

**Ascii**
```rust
'|' // show hierarchy - pipe char
'-' // child branch - en-dash char
'L' // last child in parent - capital "ell" char
"|-" // show sibling hierarchy - pipe char + en-dash
```
