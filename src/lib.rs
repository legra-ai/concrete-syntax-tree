#![doc = include_str!("../README.md")]

//! A small, lossless concrete syntax tree for parser tooling.

use std::ops::Range;

/// Trait implemented by each language's concrete-syntax-tree node kind.
pub trait CstKind: Copy + Eq + std::fmt::Debug {
    /// Returns `true` if this node kind represents trivia such as whitespace
    /// or a comment.
    fn is_trivia(self) -> bool;

    /// Returns a human-readable name for this node kind, suitable for
    /// diagnostics.
    fn description(self) -> &'static str;
}

/// A node in a concrete syntax tree.
///
/// Each node represents either a leaf token or a branch grammar production.
/// The node kind is supplied by the language-specific parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CstNode<K> {
    /// The syntactic construct represented by this node.
    pub kind: K,
    /// The byte range in the source text covered by this node.
    pub span: Range<usize>,
    /// Child nodes. This is empty for leaf nodes.
    pub children: Vec<CstNode<K>>,
    /// The exact token text for a leaf node. Branch nodes contain `None`.
    pub text: Option<String>,
}

impl<K: CstKind> CstNode<K> {
    /// Creates a leaf node from a token.
    #[must_use]
    pub fn leaf(kind: K, span: Range<usize>, text: &str) -> Self {
        Self {
            kind,
            span,
            children: Vec::new(),
            text: Some(text.to_owned()),
        }
    }

    /// Creates a branch node containing the supplied children.
    #[must_use]
    pub fn branch(kind: K, children: Vec<CstNode<K>>) -> Self {
        let span = if children.is_empty() {
            0..0
        } else {
            let start = children.first().map_or(0, |c| c.span.start);
            let end = children.last().map_or(0, |c| c.span.end);
            start..end
        };
        Self {
            kind,
            span,
            children,
            text: None,
        }
    }

    /// Reconstructs source text from this node and its descendants.
    ///
    /// Leaf text is returned directly. Branch text is the concatenation of
    /// descendant leaf text in source order.
    #[must_use]
    pub fn to_source(&self) -> String {
        if let Some(text) = &self.text {
            return text.clone();
        }

        let mut buf = String::new();
        for child in &self.children {
            buf.push_str(&child.to_source());
        }
        buf
    }

    /// Returns whether this node represents trivia.
    #[must_use]
    pub fn is_trivia(&self) -> bool {
        self.kind.is_trivia()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestKind {
        Root,
        Word,
        Space,
    }

    impl CstKind for TestKind {
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

    #[test]
    fn leaf_round_trip() {
        let leaf = CstNode::leaf(TestKind::Word, 0..5, "hello");
        assert_eq!(leaf.to_source(), "hello");
        assert!(!leaf.is_trivia());
    }

    #[test]
    fn branch_round_trip() {
        let a = CstNode::leaf(TestKind::Word, 0..5, "hello");
        let ws = CstNode::leaf(TestKind::Space, 5..6, " ");
        let b = CstNode::leaf(TestKind::Word, 6..11, "world");
        let root = CstNode::branch(TestKind::Root, vec![a, ws, b]);
        assert_eq!(root.to_source(), "hello world");
        assert_eq!(root.span, 0..11);
    }

    #[test]
    fn trivia_detection() {
        let ws = CstNode::leaf(TestKind::Space, 0..1, " ");
        assert!(ws.is_trivia());
        let word = CstNode::leaf(TestKind::Word, 0..3, "foo");
        assert!(!word.is_trivia());
    }

    #[test]
    fn empty_branch() {
        let branch: CstNode<TestKind> = CstNode::branch(TestKind::Root, vec![]);
        assert_eq!(branch.span, 0..0);
        assert_eq!(branch.to_source(), "");
    }
}
