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
    BinOp(BinOpExp),
    Num(String),
    Ident(String),
    Assign(Assignment),
    Undef,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum BinOp {
    PlusMinus(Additive),
    MulDiv(Multitive),
}

#[derive(Debug, Clone, Copy)]
pub enum Additive {
    Plus,
    Minus,
}

eq_enum!(Additive);

#[derive(Debug, Clone, Copy)]
pub enum Multitive {
    Mul,
    Div,
}

eq_enum!(Multitive);

#[derive(Debug, Clone)]
pub struct Assignment {
    pub ident: String,
    pub rhs: Box<Exp>,
}

pub fn make_assign(ident: &str) -> Exp {
    Exp::Assign(Assignment {
        ident: ident.to_string(),
        rhs: Box::new(Exp::Undef),
    })
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

    pub fn set_rhs(mut self, rhs: Exp) -> Self {
        self.rhs = Box::new(match self.rhs.as_mut() {
            Exp::BinOp(bo) => Exp::BinOp(bo.clone().set_rhs(rhs)),
            Exp::Undef => rhs,
            _ => unreachable!(),
        });
        self
    }
}

pub fn make_bexpl(mut lhs: Exp, op: BinOp) -> Exp {
    match lhs {
        Exp::BinOp(ref mut bo) => {
            if bo.op >= op {
                Exp::BinOp(BinOpExp::new(lhs, op, Exp::Undef))
            } else {
                bo.rhs = Box::new(make_bexpl(bo.rhs.as_ref().clone(), op));
                lhs
            }
        }
        Exp::Assign(subst) => Exp::Assign(Assignment {
            ident: subst.ident,
            rhs: Box::new(make_bexpl(subst.rhs.as_ref().clone(), op)),
        }),
        Exp::Num(s) => Exp::BinOp(BinOpExp::new(Exp::Num(s.to_string()), op, Exp::Undef)),
        Exp::Ident(s) => Exp::BinOp(BinOpExp::new(Exp::Ident(s.to_string()), op, Exp::Undef)),
        Exp::Undef => unreachable!(),
    }
}

pub fn comp_expr(exp: Exp, rhs: Exp) -> Exp {
    match exp {
        Exp::BinOp(bo) => Exp::BinOp(bo.set_rhs(rhs)),
        Exp::Assign(subst) => Exp::Assign(Assignment {
            ident: subst.ident,
            rhs: Box::new(match subst.rhs.as_ref() {
                Exp::Undef => rhs,
                Exp::BinOp(bo) => Exp::BinOp(bo.clone().set_rhs(rhs)),
                _ => unreachable!(),
            }),
        }),
        Exp::Undef => rhs,
        _ => panic!("Unknown Syntax"),
    }
}
