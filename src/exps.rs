use std::cmp::Ordering;

macro_rules! eq_enum {
    ($EnumName:ident) => {
        impl PartialOrd for $EnumName {
            fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
                Some(Ordering::Equal)
            }
        }

        impl PartialEq for $EnumName {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }
    };
}

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

eq_enum!(MulDiv);

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
