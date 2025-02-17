use std::ops::Range;

use logos::Logos;

#[derive(Debug, Logos, PartialEq, Clone)]
pub enum Attribute {
    #[regex("#[^\n]*")]
    Heading1,
    #[regex("##[^\n]*")]
    Heading2,
    #[regex("###[^\n]*")]
    Heading3,
    #[regex("####[^\n]*")]
    Heading4,
    #[regex("#####[^\n]*")]
    Heading5,

    #[regex(r"\*[^\n\*]*\*")]
    #[regex(r"\_[^\n\_]*\_")]
    Italic,
    #[regex(r"\*\*[^\n\*]*\*\*")]
    #[regex(r"\_\_[^\n\_]*\_\_")]
    Bold,

    #[regex(r"\[[^\n\]]*\]\([^\n\)]*\)")]
    Link,
    #[regex(r"!\[[^\n\]]*\]\([^\n\)]*\)")]
    Picture,
}

#[derive(Debug)]
pub struct Span {
    pub range: Range<usize>,
    pub attr: Attribute,
}

impl Span {
    pub fn new(range: Range<usize>, attr: Attribute) -> Self {
        Self { range, attr }
    }
}

pub fn get_attributes(text: &str) -> Vec<Span> {
    let mut v = vec![];
    let mut lexer = Attribute::lexer(text);

    while let Some(res) = lexer.next() {
        if let Ok(token) = res {
            let span = Span::new(lexer.span().start..lexer.span().end, token.clone());
            v.push(span);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings() {
        let text = "# Hello\nbody\nthe ## Middle of a line\nbody2\n### Heading3with#insideit\n#### heading4\n##### Heading 5";

        let attributes = get_attributes(text);
        assert_eq!(attributes.len(), 5);

        assert_eq!(attributes[0].attr, Attribute::Heading1);
        let slice: &str = &text[attributes[0].range.clone()];
        assert_eq!(slice, "# Hello");

        assert_eq!(attributes[1].attr, Attribute::Heading2);
        let slice: &str = &text[attributes[1].range.clone()];
        assert_eq!(slice, "## Middle of a line");

        assert_eq!(attributes[2].attr, Attribute::Heading3);
        let slice: &str = &text[attributes[2].range.clone()];
        assert_eq!(slice, "### Heading3with#insideit");

        assert_eq!(attributes[3].attr, Attribute::Heading4);
        let slice: &str = &text[attributes[3].range.clone()];
        assert_eq!(slice, "#### heading4");

        assert_eq!(attributes[4].attr, Attribute::Heading5);
        let slice: &str = &text[attributes[4].range.clone()];
        assert_eq!(slice, "##### Heading 5");
    }

    #[test]
    fn bold() {
        let text = "this is **bold** but not anymore";

        let attributes = get_attributes(text);
        assert_eq!(attributes.len(), 1);

        assert_eq!(attributes[0].attr, Attribute::Bold);
        let slice: &str = &text[attributes[0].range.clone()];
        assert_eq!(slice, "**bold**");

        let text = "this is __bold__ but not anymore";

        let attributes = get_attributes(text);
        assert_eq!(attributes.len(), 1);

        assert_eq!(attributes[0].attr, Attribute::Bold);
        let slice: &str = &text[attributes[0].range.clone()];
        assert_eq!(slice, "__bold__");
    }

    #[test]
    fn italic() {
        let text = "this is *italic* but not anymore";

        let attributes = get_attributes(text);
        assert_eq!(attributes.len(), 1);

        assert_eq!(attributes[0].attr, Attribute::Italic);
        let slice: &str = &text[attributes[0].range.clone()];
        assert_eq!(slice, "*italic*");

        let text = "this is _italic_ but not anymore";

        let attributes = get_attributes(text);
        assert_eq!(attributes.len(), 1);

        assert_eq!(attributes[0].attr, Attribute::Italic);
        let slice: &str = &text[attributes[0].range.clone()];
        assert_eq!(slice, "_italic_");
    }

    #[test]
    fn link() {
        let text = "this is [http://google.com](a link) but not anymore";

        let attributes = get_attributes(text);
        assert_eq!(attributes.len(), 1);

        assert_eq!(attributes[0].attr, Attribute::Link);
        let slice: &str = &text[attributes[0].range.clone()];
        assert_eq!(slice, "[http://google.com](a link)");
    }

    #[test]
    fn picture() {
        let text = "this is ![mypic.png](a picture) but not anymore";

        let attributes = get_attributes(text);
        assert_eq!(attributes.len(), 1);

        assert_eq!(attributes[0].attr, Attribute::Picture);
        let slice: &str = &text[attributes[0].range.clone()];
        assert_eq!(slice, "![mypic.png](a picture)");
    }
}
