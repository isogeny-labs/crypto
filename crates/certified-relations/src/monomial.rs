//! Operations shared by the public Vec representation and the bounded private key.
use std::cmp::Ordering;
pub(crate) trait MonomialOps: Sized {
    const PACKED: bool = false;
    fn zero(n: usize) -> Self;
    fn set_exp(&mut self, index: usize, value: usize);
    fn swap_exp(&mut self, a: usize, b: usize);
    fn total_degree(&self) -> usize;
    fn reverse_lex_cmp(&self, other: &Self) -> Ordering;
    fn grevlex_cmp(&self, other: &Self) -> Ordering {
        self.total_degree()
            .cmp(&other.total_degree())
            .then_with(|| other.reverse_lex_cmp(self))
    }
    fn add_exp(&self, other: &Self) -> Self;
    fn sub_exp(&self, other: &Self) -> Self;
    fn divides_exp(&self, other: &Self) -> bool;
}
impl MonomialOps for Vec<usize> {
    fn zero(n: usize) -> Self {
        vec![0; n]
    }
    fn set_exp(&mut self, i: usize, value: usize) {
        self[i] = value;
    }
    fn swap_exp(&mut self, a: usize, b: usize) {
        self.swap(a, b);
    }
    fn total_degree(&self) -> usize {
        self.iter().sum()
    }
    fn reverse_lex_cmp(&self, other: &Self) -> Ordering {
        self.iter().rev().cmp(other.iter().rev())
    }
    fn add_exp(&self, other: &Self) -> Self {
        self.iter()
            .zip(other)
            .map(|(x, y)| x.checked_add(*y).expect("exponent overflow"))
            .collect()
    }
    fn sub_exp(&self, other: &Self) -> Self {
        self.iter().zip(other).map(|(x, y)| x - y).collect()
    }
    fn divides_exp(&self, other: &Self) -> bool {
        self.iter().zip(other).all(|(x, y)| x <= y)
    }
}
