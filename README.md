# Sapling 🌱
Mini-tree CLI program, written in Rust<br/>

**Table of Contents**
* [TUI](#tui)
* [Todo's](#todos)

## TUI
**Box Drawings**<br/>
[Box-Drawing Characters Wikipedia page](https://en.wikipedia.org/wiki/Box-drawing_characters)<br/>
There's a nice table of box drawing characters and their Unicode codes [here](https://en.wikipedia.org/wiki/Box-drawing_characters#Box_Drawing)
```txt
'│' show hierarchy         - '\u{2502}'
'─' child branch           - '\u{2500}'
'└' last child in parent   - '\u{2514}'
'├' show sibling hierarchy - '\u{251c}'
```

**Ascii**
```txt
'|'  show hierarchy         - pipe char
'-'  child branch           - en-dash char
'L'  last child in parent   - capital "ell" char
"|-" show sibling hierarchy - pipe char + en-dash
```

## Todo's
- [x] add tree-like TUI (see above)
- [ ] add support for POSIX style opts / args
