use std::{fmt, ops};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Major {
    Row,
    Col
}