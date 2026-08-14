# concrete-syntax-tree

[![Crates.io](https://img.shields.io/crates/v/concrete-syntax-tree.svg)](https://crates.io/crates/concrete-syntax-tree)
[![Documentation](https://docs.rs/concrete-syntax-tree/badge.svg)](https://docs.rs/concrete-syntax-tree)
[![CI](https://github.com/legra-ai/concrete-syntax-tree/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/concrete-syntax-tree/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/concrete-syntax-tree.svg)](https://github.com/legra-ai/concrete-syntax-tree/blob/main/LICENSE-APACHE)

A small, lossless concrete syntax tree for parser tooling.

The crate provides a generic tree container. Language-specific parsers define
their own node-kind enum and implement [`CstKind`]. The tree can represent
grammar productions, tokens, whitespace, comments, and other trivia without
discarding the original token text.

## Example

```rust
use concrete_syntax_tree::{CstKind, CstNode};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Root,
    Word,
    Space,
}

impl CstKind for Kind {
    fn is_trivia(self) -> bool {
        matches!(self, Self::Space)
    }

    fn description(self) -> &'static str {
        match self {
            Self::Root => "root",
            Self::Word => "word",
            Self::Space => "space",
        }
    }
}

let tree = CstNode::branch(
    Kind::Root,
    vec![
        CstNode::leaf(Kind::Word, 0..5, "hello"),
        CstNode::leaf(Kind::Space, 5..6, " "),
        CstNode::leaf(Kind::Word, 6..11, "world"),
    ],
);

assert_eq!(tree.to_source(), "hello world");
assert_eq!(tree.span, 0..11);
```

## API

`CstKind` supplies language-specific trivia classification and diagnostic
names. `CstNode<K>` stores a node kind, byte span, child nodes, and—on leaf
nodes—the exact token text. `CstNode::to_source` concatenates descendant leaf
text in order, making lossless round-tripping possible when the parser records
all tokens and trivia.

This initial `0.1` API intentionally keeps the tree representation simple.
Future versions may add a document-owned source buffer and span-based text
access without changing the role of the crate as a generic CST foundation.

## Scope

This crate does not parse a language, define a grammar, recover from syntax
errors, or provide an AST. It is the reusable tree representation for those
layers.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT)
  or <https://opensource.org/licenses/MIT>)

at your option.
