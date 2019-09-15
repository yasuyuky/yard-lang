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
    Subst(Substitution),
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
pub struct Substitution {
    pub ident: String,
    pub rhs: Box<Exp>,
}

pub fn make_subst(ident: &str) -> Exp {
    Exp::Subst(Substitution {
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
}

pub fn make_bexpl(mut lhs: Exp, op: BinOp) -> Exp {
    match lhs {
        Exp::Num(s) => Exp::BinOp(BinOpExp::new(Exp::Num(s.to_string()), op, Exp::Undef)),
        Exp::BinOp(ref mut bo) => {
            if bo.op >= op {
                Exp::BinOp(BinOpExp::new(lhs, op, Exp::Undef))
            } else {
                bo.rhs = Box::new(make_bexpl(bo.rhs.as_ref().clone(), op));
                lhs
            }
        }
        Exp::Ident(s) => Exp::BinOp(BinOpExp::new(Exp::Ident(s.to_string()), op, Exp::Undef)),
        Exp::Subst(_) => unreachable!(),
        Exp::Undef => unreachable!(),
    }
}

pub fn comp_bexpr(bexpl: &mut BinOpExp, rhs: Exp) {
    match bexpl.rhs.as_mut() {
        Exp::BinOp(bo) => comp_bexpr(bo, rhs),
        _ => bexpl.rhs = Box::new(rhs),
    }
}
