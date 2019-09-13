use crate::{
    expression::{Expression, Other, Term},
    polynomial::SparsePolynomial,
};
use primefield::FieldElement;
use std::{
    collections::BTreeSet,
    ops::{Add, AddAssign, DivAssign, Mul, MulAssign},
};
use u256::commutative_binop;

#[derive(Clone, Debug)]
pub struct Fraction {
    pub numerator:   Expression,
    pub denominator: BTreeSet<SparsePolynomial>,
}

impl Fraction {
    pub fn zero() -> Self {
        Fraction {
            numerator:   0.into(),
            denominator: BTreeSet::new(),
        }
    }

    pub fn one() -> Self {
        Fraction {
            numerator:   1.into(),
            denominator: BTreeSet::new(),
        }
    }
}

pub fn from(factors: BTreeSet<SparsePolynomial>) -> Expression {
    factors
        .iter()
        .fold(SparsePolynomial::new(&[(FieldElement::ONE, 0)]), Mul::mul)
        .into()
}

impl From<Other> for Fraction {
    fn from(value: Other) -> Self {
        Fraction {
            numerator:   Expression::Term(Term::Other(value)),
            denominator: BTreeSet::new(),
        }
    }
}

impl AddAssign<&Fraction> for Fraction {
    fn add_assign(&mut self, other: &Self) {
        self.numerator = self.numerator.clone() * from(&other.denominator - &self.denominator)
            + other.numerator.clone() * from(&self.denominator - &other.denominator);
        self.denominator = &self.denominator | &other.denominator;
    }
}

impl MulAssign<&Fraction> for Fraction {
    fn mul_assign(&mut self, other: &Self) {
        assert!((&self.denominator & &other.denominator).is_empty());
        self.numerator = self.numerator.clone() * other.numerator.clone();
        self.denominator = &self.denominator | &other.denominator;
    }
}

commutative_binop!(Fraction, Add, add, AddAssign, add_assign);
commutative_binop!(Fraction, Mul, mul, MulAssign, mul_assign);

impl DivAssign<SparsePolynomial> for Fraction {
    fn div_assign(&mut self, other: SparsePolynomial) {
        self.denominator.insert(other.clone());
    }
}
