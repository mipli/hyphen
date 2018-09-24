# Hyphen

A simple hyphenation library written in Rust, using standard Knuth-Liang based hyphenation. Supports the normal [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8).

# Features to add
 - Compile to wasm
 - Support reading optimized (binary) pattern files, and create tool for generating these pattern files
 - Figure out how Unicode segmentation and normalization affects hyphenation
 - Add support for [extended Liang's hyphenatation](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf).
