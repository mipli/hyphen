# Hyphen

A simple hyphenation library written in Rust, using standard Knuth-Liang based hyphenation. Supports the normal [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8).

# Features to add
 - Support settings min/max word length, and how near the end hyphenation can occur
 - Support reading default TeX pattern files
 - Compile to wasm
 - Figure out how Unicode segmentation and normalization affects hyphenation
 - Add support for [extended Liang's hyphenatation](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf).
