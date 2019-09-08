use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Exp {
    BinaryOp(BinOpExp),
    Number(String),
    Undefined,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum BinOp {
    PlusMinus(PlusMinus),
    MulDiv(MulDiv),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PlusMinus {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy)]
pub enum MulDiv {
    Mul,
    Div,
}

impl PartialOrd for MulDiv {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialEq for MulDiv {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct BinOpExp {
    pub lhs: Box<Exp>,
    pub op: BinOp,
    pub rhs: Box<Exp>,
}

impl BinOpExp {
    pub fn new(lhs: Exp, op: BinOp, rhs: Exp) -> Self {
        BinOpExp {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}
