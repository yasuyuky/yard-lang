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
    Bin(BinaryExp),
    Num(String),
    Ident(String),
    Assign(Assignment),
    Return(Box<Exp>),
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
pub struct BinaryExp {
    pub lhs: Box<Exp>,
    pub op: BinOp,
    pub rhs: Box<Exp>,
}

impl BinaryExp {
    pub fn new(lhs: Exp, op: BinOp) -> Self {
        BinaryExp {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(Exp::Undef),
        }
    }

    pub fn set_rhs(mut self, rhs: Exp) -> Self {
        self.rhs = Box::new(match self.rhs.as_mut() {
            Exp::Bin(b) => Exp::Bin(b.clone().set_rhs(rhs)),
            Exp::Undef => rhs,
            _ => unreachable!(),
        });
        self
    }
}

impl Exp {
    pub fn make_bexpl(&mut self, op: BinOp) -> Self {
        match self {
            Exp::Bin(ref mut b) => {
                if b.op >= op {
                    Exp::Bin(BinaryExp::new(self.clone(), op))
                } else {
                    b.rhs = Box::new(b.rhs.make_bexpl(op));
                    self.clone()
                }
            }
            Exp::Assign(subst) => Exp::Assign(Assignment {
                ident: subst.ident.clone(),
                rhs: Box::new(subst.rhs.make_bexpl(op)),
            }),
            Exp::Num(s) => Exp::Bin(BinaryExp::new(Exp::Num(s.to_string()), op)),
            Exp::Ident(s) => Exp::Bin(BinaryExp::new(Exp::Ident(s.to_string()), op)),
            Exp::Return(r) => Exp::Return(Box::new(r.as_mut().make_bexpl(op))),
            Exp::Undef => unreachable!(),
        }
    }

    pub fn extend(&self, exp: Exp) -> Exp {
        match self {
            Exp::Bin(ref b) => Exp::Bin(b.clone().set_rhs(exp)),
            Exp::Assign(subst) => Exp::Assign(Assignment {
                ident: subst.ident.clone(),
                rhs: Box::new(match subst.rhs.as_ref() {
                    Exp::Undef => exp,
                    Exp::Bin(b) => Exp::Bin(b.clone().set_rhs(exp)),
                    _ => unreachable!(),
                }),
            }),
            Exp::Return(boxed) => Exp::Return(Box::new(boxed.clone().as_mut().extend(exp))),
            Exp::Undef => exp,
            _ => panic!("Unknown Syntax"),
        }
    }
}
