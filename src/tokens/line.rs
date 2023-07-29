use super::LineItem;

pub enum Line {
    Paragraph(Vec<LineItem>),
    Conversation(Vec<LineItem>),
    Quotation(Vec<LineItem>),
    Comment(String),
}