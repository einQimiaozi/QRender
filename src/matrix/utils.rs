/// Mark whether the current vector is a row vector or a column vector
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Major {
    Row,
    Col
}