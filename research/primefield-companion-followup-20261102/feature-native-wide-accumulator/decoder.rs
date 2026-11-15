//! Exact bitplane evaluation with one final modular reduction.
const _: () = assert!(usize::BITS <= 64);
#[derive(Debug, PartialEq)]
pub enum Error { Modulus, Width, Coefficient, Offset, Sign, Size }
pub struct Program {
    modulus: u64,
    width: usize,
    words: usize,
    bits: usize,
    // support rows, odd-multiplicity rows, then coefficient-bit rows.
    planes: Vec<u64>,
}
impl Program {
    pub fn new(modulus: u64, width: usize, terms: &[(u64, Vec<usize>)]) -> Result<Self, Error> {
        if modulus < 3 || modulus > u64::MAX / 2 { return Err(Error::Modulus); }
        if width > 64 { return Err(Error::Width); }
        let mut largest = 0;
        for (coefficient, offsets) in terms {
            if *coefficient >= modulus { return Err(Error::Coefficient); }
            if offsets.iter().any(|&a| a >= width) { return Err(Error::Offset); }
            largest = largest.max(*coefficient);
        }
        let bits = (64 - largest.leading_zeros()) as usize;
        let words = terms.len() / 64 + usize::from(terms.len() % 64 != 0);
        let len = (2 * width + bits).checked_mul(words).ok_or(Error::Size)?;
        let mut planes = vec![0; len];
        for (i, (coefficient, offsets)) in terms.iter().enumerate() {
            let word = i / 64;
            let flag = 1u64 << (i % 64);
            for &a in offsets {
                planes[a * words + word] |= flag;
                planes[(width + a) * words + word] ^= flag;
            }
            for b in 0..bits {
                if coefficient & (1u64 << b) != 0 {
                    planes[(2 * width + b) * words + word] |= flag;
                }
            }
        }
        Ok(Self { modulus, width, words, bits, planes })
    }

    /// Query scratch stays on the stack; oracle order/error behavior is preserved.
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
        if self.bits == 0 { return Ok(0); }
        let mut digits = [0i128; 63];
        for word in 0..self.words {
            let (mut zeros, mut negatives) = (0u64, 0u64);
            for a in 0..self.width {
                if zero & (1u64 << a) != 0 { zeros |= self.planes[a * self.words + word]; }
                if negative & (1u64 << a) != 0 { negatives ^= self.planes[(self.width + a) * self.words + word]; }
            }
            for (b, digit) in digits[..self.bits].iter_mut().enumerate() {
                let active = self.planes[(2 * self.width + b) * self.words + word] & !zeros;
                *digit += active.count_ones() as i128 - 2 * (active & negatives).count_ones() as i128;
            }
        }
        Ok(fold_digits(&digits[..self.bits]).rem_euclid(self.modulus as i128) as u64)
    }
}

// Internal contract: <=63 digits, each with absolute value <=u64::MAX.
// After h steps |value| <= (2^64-1)(2^h-1); see DERIVATION.md.
fn fold_digits(digits: &[i128]) -> i128 {
    let mut value = 0i128;
    for &digit in digits.iter().rev() { value = 2 * value + digit; }
    value
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
        include_str!("../feature-native-signmask/program.tsv").lines().map(|l| {
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
        assert_eq!((p.words,p.bits,p.planes.len()),(8,9,328));
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
    #[test] fn word_boundaries_and_oracle_failure_order() {
        for n in [0usize,1,63,64,65,127,128,129,477] {
            let terms:Vec<_>=(0..n).map(|i| (((17*i+3)%479) as u64,
                if i%5==0 {vec![]} else {vec![i%4,(i/4)%4,i%4]})).collect();
            let program=Program::new(479,4,&terms).unwrap();
            for mut code in 0..81 {let mut signs=[0i8;4];for s in &mut signs{*s=(code%3) as i8-1;code/=3;}
                assert_eq!(program.evaluate(|a|signs[a]).unwrap(),reference(479,&terms,&signs));
            }
        }
        let p=Program::new(479,8,&[(0,vec![2,2])]).unwrap();
        let mut calls=vec![];
        assert_eq!(p.evaluate(|a|{calls.push(a);if a==3{2}else{1}}),Err(Error::Sign));
        assert_eq!(calls,vec![0,1,2,3]);
    }

    #[test] fn extreme_digits_fit_and_match_modular_fold() {
        let nmax = u64::MAX as i128;
        let limit = nmax.checked_mul((1i128 << 63)-1).unwrap();
        assert!(limit < i128::MAX);
        assert_eq!(fold_digits(&[nmax;63]), limit);
        assert_eq!(fold_digits(&[-nmax;63]), -limit);
        let mut cases = 0;
        for n in [0i128,1,63,nmax] { for bits in 0..=63 { for mode in 0..4 {
            let digits:Vec<i128>=(0..bits).map(|b| match mode {
                0=>n, 1=>-n, 2=>if b%2==0{n}else{-n}, _=>if b%3==0{-n}else{0}
            }).collect();
            // Independent low-to-high weighted sum, checked at every step.
            let mut exact=0i128;
            for (b,&digit) in digits.iter().enumerate() {
                exact=exact.checked_add(digit.checked_mul(1i128 << b).unwrap()).unwrap();
            }
            assert_eq!(fold_digits(&digits),exact);
            for r in [3u64,7,479,u64::MAX/2] {
                let mut modular=0i128;
                for &digit in digits.iter().rev() {
                    modular=(2*modular+digit).rem_euclid(r as i128);
                }
                assert_eq!(fold_digits(&digits).rem_euclid(r as i128),modular);cases+=1;
            }
        }}}
        assert_eq!(cases,4096);
    }

}
