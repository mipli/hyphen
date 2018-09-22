# Hyphen

A simple hyphenation library written in Rust, using standard Knuth-Liang based hyphenation. Supports the normal [TeX UTF-8 patterns](http://www.ctan.org/tex-archive/language/hyph-utf8).

# Features to add
 - Support reading default TeX pattern files
 - Add `Hyphenate` trait to `&str`, for both single word hyphenation and full text hyphenation
 - Compile to wasm
 - Figure out how Unicode segmentation and normalization affects hyphenation
 - Add support for [extended Liang's hyphenatation](https://www.tug.org/TUGboat/tb27-1/tb86nemeth.pdf).
