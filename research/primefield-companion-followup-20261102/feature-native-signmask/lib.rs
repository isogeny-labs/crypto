//! Native signed-character evaluation. No primality or representation claim is
//! made by construction: the caller supplies the modulus and certified program.
#[derive(Debug, PartialEq)]
pub enum Error { Modulus, Width, Coefficient, Offset, Sign }
struct Term { coefficient: u64, support: u64, odd: u64 }
pub struct Program { modulus: u64, width: usize, terms: Vec<Term> }
impl Program {
    /// Supports repeated offsets: presence controls zero, multiplicity parity sign.
    /// Validate and compile once, outside target-dependent evaluation.
    pub fn new(modulus: u64, width: usize, terms: &[(u64, Vec<usize>)]) -> Result<Self, Error> {
        if modulus < 3 || modulus > u64::MAX / 2 { return Err(Error::Modulus); }
        if width > 64 { return Err(Error::Width); }
        let mut compiled = Vec::with_capacity(terms.len());
        for (coefficient, offsets) in terms {
            if *coefficient >= modulus { return Err(Error::Coefficient); }
            let (mut support, mut odd) = (0u64, 0u64);
            for &a in offsets {
                if a >= width { return Err(Error::Offset); }
                support |= 1u64 << a;
                odd ^= 1u64 << a;
            }
            compiled.push(Term { coefficient: *coefficient, support, odd });
        }
        Ok(Self { modulus, width, terms: compiled })
    }
    /// Calls every prefix offset once in ascending order on valid input.
    /// A malformed sign fails at that offset; oracle errors remain caller-owned.
    pub fn evaluate(&self, mut oracle: impl FnMut(usize) -> i8) -> Result<u64, Error> {
        let (mut zero, mut negative) = (0u64, 0u64);
        for a in 0..self.width {
            match oracle(a) {
                0 => zero |= 1u64 << a,
                -1 => negative |= 1u64 << a,
                1 => (),
                _ => return Err(Error::Sign),
            }
        }
        let mut value = 0u64;
        for t in &self.terms {
            if t.support & zero != 0 { continue; }
            let c = if (t.odd & negative).count_ones() & 1 == 0 || t.coefficient == 0 {
                t.coefficient
            } else { self.modulus - t.coefficient };
            value += c; // Both summands < modulus <= MAX/2.
            if value >= self.modulus { value -= self.modulus; }
        }
        Ok(value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn reference(r:u64, terms:&[(u64,Vec<usize>)], signs:&[i8])->u64 {
        terms.iter().fold(0,|sum,(c,offsets)| {
            let v=offsets.iter().fold(*c,|v,&a| ((v as u128 * (signs[a] as i128).rem_euclid(r as i128) as u128) % r as u128) as u64);
            ((sum as u128 + v as u128) % r as u128) as u64
        })
    }
    fn fixture()->Vec<(u64,Vec<usize>)> {
        include_str!("program.tsv").lines().map(|l| {
            let mut s=l.split('\t');
            let c=s.next().unwrap().parse().unwrap();
            let offsets=s.next().unwrap().split(',').filter(|v| !v.is_empty()).map(|v|v.parse().unwrap()).collect();
            assert!(s.next().is_none()); (c,offsets)
        }).collect()
    }
    #[test] fn all_ternary_inputs_and_repeated_offsets() {
        // Every monomial of length 0..4 on three coordinates, with repetitions.
        let mut terms=vec![(6,vec![])];
        for n in 1..=4 { for mut code in 0..3usize.pow(n) {
            let mut offsets=vec![];
            for _ in 0..n { offsets.push(code%3);code/=3; }
            terms.push(((terms.len()%7) as u64,offsets));
        }}
        let p=Program::new(7,3,&terms).unwrap();
        for mut code in 0..27 { let mut signs=vec![];
            for _ in 0..3 { signs.push((code%3) as i8-1);code/=3; }
            assert_eq!(p.evaluate(|a|signs[a]).unwrap(),reference(7,&terms,&signs));
        }
    }
    #[test] fn repaired_program_full_field() {
        let terms=fixture(); assert_eq!(terms.len(),477);
        let p=Program::new(479,16,&terms).unwrap();
        let mut squares=[false;479];for x in 1..479 {squares[x*x%479]=true;}
        for x in 0..479 { let signs:Vec<i8>=(0..16).map(|a| {
            let v=(x+a)%479; if v==0 {0} else if squares[v] {1} else {-1}
        }).collect();
            let mut calls=vec![];
            let v=p.evaluate(|a|{calls.push(a);signs[a]}).unwrap();
            assert_eq!(calls,(0..16).collect::<Vec<_>>());
            assert_eq!(v,reference(479,&terms,&signs));assert_eq!(v,x as u64);
        }
    }
    #[test] fn boundaries_and_rejections() {
        assert!(matches!(Program::new(2,0,&[]),Err(Error::Modulus)));
        assert!(matches!(Program::new(u64::MAX,0,&[]),Err(Error::Modulus)));
        assert!(matches!(Program::new(7,65,&[]),Err(Error::Width)));
        assert!(matches!(Program::new(7,1,&[(7,vec![])]),Err(Error::Coefficient)));
        assert!(matches!(Program::new(7,1,&[(1,vec![1])]),Err(Error::Offset)));
        let r=u64::MAX/2;let terms=vec![(r-1,vec![63]),(r-1,vec![]),(0,vec![63,63])];
        let p=Program::new(r,64,&terms).unwrap();
        for s in [-1,0,1] {let signs=vec![s;64];assert_eq!(p.evaluate(|a|signs[a]).unwrap(),reference(r,&terms,&signs));}
        assert_eq!(p.evaluate(|_|2),Err(Error::Sign));
        assert_eq!(Program::new(7,0,&[(6,vec![])]).unwrap().evaluate(|_|panic!("no calls")),Ok(6));
    }
}
