use std::ops::Range;

use logos::{Logos, Lexer};

fn parse_picture(lex: &mut Lexer<Attribute>) -> (String, String) {
    let len = lex.slice().len();
    let title_end = lex.slice().find("]").unwrap();
    let title = lex.slice()[2..title_end].to_string();
    let location = lex.slice()[title_end+2..len-1].to_string();
    (title, location)
}

fn parse_link(lex: &mut Lexer<Attribute>) -> (String, String) {
    let len = lex.slice().len();
    let title_end = lex.slice().find("]").unwrap();
    let title = lex.slice()[1..title_end].to_string();
    let link = lex.slice()[title_end+2..len-1].to_string();
    (title, link)
}

fn skip_x_chars_start_and_end(lex: &mut Lexer<Attribute>, num_skip: usize) -> String {
    let len = lex.slice().len();
    lex.slice()[num_skip..len-num_skip].to_string()
}

fn header_to_text(lex: &mut Lexer<Attribute>, num_skip: usize) -> String {
    lex.slice()[num_skip..].trim().to_string()
}

#[derive(Debug, Logos, PartialEq, Clone)]
pub enum Attribute {
    #[regex("#[^\n]*", |lex| header_to_text(lex, 1))]
    Heading1(String),
    #[regex("##[^\n]*", |lex| header_to_text(lex, 2))]
    Heading2(String),
    #[regex("###[^\n]*", |lex| header_to_text(lex, 3))]
    Heading3(String),
    #[regex("####[^\n]*", |lex| header_to_text(lex, 4))]
    Heading4(String),
    #[regex("#####[^\n]*", |lex| header_to_text(lex, 5))]
    Heading5(String),

    #[regex(r"\*[^\n\*]*\*", |lex| skip_x_chars_start_and_end(lex, 1))]
    #[regex(r"\_[^\n\_]*\_", |lex| skip_x_chars_start_and_end(lex, 1))]
    Italic(String),
    #[regex(r"\*\*[^\n\*]*\*\*", |lex| skip_x_chars_start_and_end(lex, 2))]
    #[regex(r"\_\_[^\n\_]*\_\_", |lex| skip_x_chars_start_and_end(lex, 2))]
    Bold(String),

    #[regex(r"\[[^\n\]]*\]\([^\n\)]*\)", |lex| parse_link(lex))]
    Link((String, String)),
    #[regex(r"!\[[^\n\]]*\]\([^\n\)]*\)", |lex| parse_picture(lex))]
    Picture((String, String)),

    #[regex(r"[^#*_\[\!\n]+", |lex| lex.slice().to_string())]
    Text(String),
}

#[derive(Debug)]
pub struct Block {
    pub span: Range<usize>,
    pub attr: Attribute,
}

impl Block {
    pub fn new(span: Range<usize>, attr: Attribute) -> Self {
        Self { span, attr }
    }
}

pub fn get_blocks(text: &str) -> Vec<Block> {
    let mut v = vec![];
    let mut lexer = Attribute::lexer(text);

    while let Some(res) = lexer.next() {
        if let Ok(token) = res {
            let span = Block::new(lexer.span().start..lexer.span().end, token.clone());
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

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 8);

        assert_eq!(attributes[0].attr, Attribute::Heading1("Hello".to_string()));
        let slice: &str = &text[attributes[0].span.clone()];
        assert_eq!(slice, "# Hello");

        assert_eq!(attributes[1].attr, Attribute::Text("body".to_string()));
        assert_eq!(attributes[2].attr, Attribute::Text("the ".to_string()));

        assert_eq!(attributes[3].attr, Attribute::Heading2("Middle of a line".to_string()));
        let slice: &str = &text[attributes[3].span.clone()];
        assert_eq!(slice, "## Middle of a line");

        assert_eq!(attributes[4].attr, Attribute::Text("body2".to_string()));

        assert_eq!(attributes[5].attr, Attribute::Heading3("Heading3with#insideit".to_string()));
        let slice: &str = &text[attributes[5].span.clone()];
        assert_eq!(slice, "### Heading3with#insideit");

        assert_eq!(attributes[6].attr, Attribute::Heading4("heading4".to_string()));
        let slice: &str = &text[attributes[6].span.clone()];
        assert_eq!(slice, "#### heading4");

        assert_eq!(attributes[7].attr, Attribute::Heading5("Heading 5".to_string()));
        let slice: &str = &text[attributes[7].span.clone()];
        assert_eq!(slice, "##### Heading 5");
    }

    #[test]
    fn bold() {
        let text = "this is **bold** but not anymore";

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 3);

        assert_eq!(attributes[0].attr, Attribute::Text("this is ".to_string()));
        assert_eq!(attributes[1].attr, Attribute::Bold("bold".to_string()));
        let slice: &str = &text[attributes[1].span.clone()];
        assert_eq!(slice, "**bold**");
        assert_eq!(attributes[2].attr, Attribute::Text(" but not anymore".to_string()));

        let text = "this is __bold__ but not anymore";

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 3);

        assert_eq!(attributes[0].attr, Attribute::Text("this is ".to_string()));
        assert_eq!(attributes[1].attr, Attribute::Bold("bold".to_string()));
        let slice: &str = &text[attributes[1].span.clone()];
        assert_eq!(slice, "__bold__");
        assert_eq!(attributes[2].attr, Attribute::Text(" but not anymore".to_string()));
    }

    #[test]
    fn italic() {
        let text = "this is *italic* but not anymore";

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 3);

        assert_eq!(attributes[0].attr, Attribute::Text("this is ".to_string()));
        assert_eq!(attributes[1].attr, Attribute::Italic("italic".to_string()));
        let slice: &str = &text[attributes[1].span.clone()];
        assert_eq!(slice, "*italic*");
        assert_eq!(attributes[2].attr, Attribute::Text(" but not anymore".to_string()));

        let text = "this is _italic_ but not anymore";

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 3);

        assert_eq!(attributes[0].attr, Attribute::Text("this is ".to_string()));
        assert_eq!(attributes[1].attr, Attribute::Italic("italic".to_string()));
        let slice: &str = &text[attributes[1].span.clone()];
        assert_eq!(slice, "_italic_");
        assert_eq!(attributes[2].attr, Attribute::Text(" but not anymore".to_string()));
    }

    #[test]
    fn link() {
        let text = "this is [a link](http://google.com) but not anymore";

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 3);

        assert_eq!(attributes[0].attr, Attribute::Text("this is ".to_string()));
        assert_eq!(attributes[1].attr, Attribute::Link(("a link".to_string(), "http://google.com".to_string())));
        let slice: &str = &text[attributes[1].span.clone()];
        assert_eq!(slice, "[a link](http://google.com)");
        assert_eq!(attributes[2].attr, Attribute::Text(" but not anymore".to_string()));
    }

    #[test]
    fn picture() {
        let text = "this is ![a picture](mypic.png) but not anymore";

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 3);

        assert_eq!(attributes[0].attr, Attribute::Text("this is ".to_string()));
        assert_eq!(attributes[1].attr, Attribute::Picture(("a picture".to_string(), "mypic.png".to_string())));
        let slice: &str = &text[attributes[1].span.clone()];
        assert_eq!(slice, "![a picture](mypic.png)");
        assert_eq!(attributes[2].attr, Attribute::Text(" but not anymore".to_string()));
    }

    #[test]
    fn text() {
        let text = "This is just plain text without any markdown.";

        let attributes = get_blocks(text);
        assert_eq!(attributes.len(), 1);

        assert_eq!(attributes[0].attr, Attribute::Text("This is just plain text without any markdown.".to_string()));
        let slice: &str = &text[attributes[0].span.clone()];
        assert_eq!(slice, "This is just plain text without any markdown.");
    }
}
