use crate::field_a::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly(pub BTreeMap<(usize, usize), u8>);
impl Poly {
    pub fn zero() -> Self { Self(BTreeMap::new()) }
    pub fn term(y: usize, z: usize, c: u8) -> Self {
        assert!(c < 7);
        if c == 0 { Self::zero() } else { Self(BTreeMap::from([((y,z),c)])) }
    }
    pub fn from_coefficients(basis: &[(usize,usize)], coefficients: &[u8]) -> Self {
        assert_eq!(basis.len(),coefficients.len());
        basis.iter().zip(coefficients).fold(Self::zero(),|p, (&(y,z),&c)| p.plus(&Self::term(y,z,c)))
    }
    pub fn plus(&self, other: &Self) -> Self {
        let mut out=self.0.clone();
        for (&key,&c) in &other.0 {
            let value=add(out.get(&key).copied().unwrap_or(0),c);
            if value==0 { out.remove(&key); } else { out.insert(key,value); }
        }
        Self(out)
    }
    pub fn scaled(&self,c:u8)->Self {
        Self(self.0.iter().filter_map(|(&key,&value)| {
            let value=mul(value,c); (value!=0).then_some((key,value))
        }).collect())
    }
    pub fn minus(&self,other:&Self)->Self { self.plus(&other.scaled(6)) }
    pub fn times(&self,other:&Self)->Self {
        let mut out=BTreeMap::new();
        for (&(y,z),&c) in &self.0 {
            for (&(yy,zz),&d) in &other.0 {
                let key=(y+yy,z+zz);
                let value=add(out.get(&key).copied().unwrap_or(0),mul(c,d));
                if value==0 { out.remove(&key); } else { out.insert(key,value); }
            }
        }
        Self(out)
    }
    pub fn power(&self,n:usize)->Self { (0..n).fold(Self::term(0,0,1),|p,_| p.times(self)) }
    pub fn divide_curve(&self,curve:&Self)->(Self,Self) {
        assert_eq!(curve.0.get(&(0,4)),Some(&1));
        assert!(curve.0.keys().all(|&(y,z)| z<4 || (y,z)==(0,4)));
        let mut remainder=self.clone(); let mut quotient=Self::zero();
        loop {
            let next=remainder.0.iter().filter(|((_,z),_)| *z>=4)
                .max_by_key(|((y,z),_)| (*z,*y)).map(|(&(y,z),&c)|(y,z,c));
            let Some((y,z,c))=next else { break; };
            let factor=Self::term(y,z-4,c);
            quotient=quotient.plus(&factor);
            remainder=remainder.minus(&curve.times(&factor));
        }
        assert_eq!(curve.times(&quotient).plus(&remainder),*self);
        (quotient,remainder)
    }
}
pub fn normal_basis(degree:usize)->Vec<(usize,usize)> {
    (0..=degree).flat_map(|y| (0..=degree-y).filter(|&z|z<4).map(move |z|(y,z))).collect()
}
pub fn actual_curve()->Poly {
    [(0,0,1),(3,0,4),(4,0,2),(0,2,5),(1,2,6),(2,2,2),(0,4,1)]
        .into_iter().fold(Poly::zero(),|p,(y,z,c)|p.plus(&Poly::term(y,z,c)))
}
