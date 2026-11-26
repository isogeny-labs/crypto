//! Deterministic exact-integer certificates for shared quadratic-chain selectors.
//! No sampling, field experiment, timing claim, or external dependency.
use std::{cmp::Ordering, fmt::Write as _, fs};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Nat(Vec<u32>); // normalized little-endian base 2^32

impl Nat {
    fn zero() -> Self { Self(Vec::new()) }
    fn one() -> Self { Self(vec![1]) }
    fn from_u64(x: u64) -> Self {
        let mut v = vec![x as u32, (x >> 32) as u32];
        while v.last() == Some(&0) { v.pop(); }
        Self(v)
    }
    fn normalize(&mut self) { while self.0.last() == Some(&0) { self.0.pop(); } }
    fn add(&self, b: &Self) -> Self {
        let mut v = Vec::with_capacity(self.0.len().max(b.0.len()) + 1);
        let mut carry = 0u64;
        for i in 0..self.0.len().max(b.0.len()) {
            let t = self.0.get(i).copied().unwrap_or(0) as u64
                + b.0.get(i).copied().unwrap_or(0) as u64 + carry;
            v.push(t as u32); carry = t >> 32;
        }
        if carry != 0 { v.push(carry as u32); }
        Self(v)
    }
    fn mul(&self, b: &Self) -> Self {
        if self.0.is_empty() || b.0.is_empty() { return Self::zero(); }
        let mut v = vec![0u32; self.0.len() + b.0.len()];
        for (i, &a) in self.0.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &bb) in b.0.iter().enumerate() {
                let t = a as u64 * bb as u64 + v[i+j] as u64 + carry;
                v[i+j] = t as u32; carry = t >> 32;
            }
            let mut k = i + b.0.len();
            while carry != 0 {
                if k == v.len() { v.push(0); }
                let t = v[k] as u64 + carry;
                v[k] = t as u32; carry = t >> 32; k += 1;
            }
        }
        let mut result = Self(v); result.normalize(); result
    }
    fn mul_u64(&self, b: u64) -> Self {
        if b == 0 { return Self::zero(); }
        let mut v = Vec::with_capacity(self.0.len()+2);
        let mut carry = 0u128;
        for &a in &self.0 {
            let t = a as u128 * b as u128 + carry;
            v.push(t as u32); carry = t >> 32;
        }
        while carry != 0 { v.push(carry as u32); carry >>= 32; }
        Self(v)
    }
    fn div_exact_u64(&self, b: u64) -> Self {
        assert!(b != 0);
        let mut v = vec![0; self.0.len()];
        let mut rem = 0u128;
        for i in (0..self.0.len()).rev() {
            let t = (rem << 32) + self.0[i] as u128;
            v[i] = (t / b as u128) as u32; rem = t % b as u128;
        }
        assert_eq!(rem, 0, "nonintegral division");
        let mut result = Self(v); result.normalize(); result
    }
    fn pow(base: u64, exponent: usize) -> Self {
        let mut out = Self::one();
        for _ in 0..exponent { out = out.mul_u64(base); }
        out
    }
    fn shl(&self, bits: usize) -> Self {
        if self.0.is_empty() { return Self::zero(); }
        let mut v = vec![0; bits / 32];
        let mut carry = 0u64;
        for &a in &self.0 {
            let t = ((a as u64) << (bits % 32)) | carry;
            v.push(t as u32); carry = t >> 32;
        }
        if carry != 0 { v.push(carry as u32); }
        Self(v)
    }
    fn bits(&self) -> usize {
        self.0.last().map(|last| 32*(self.0.len()-1) + (32-last.leading_zeros()) as usize).unwrap_or(0)
    }
    fn hex(&self) -> String {
        if self.0.is_empty() { return "0".into(); }
        let mut out = format!("{:x}", self.0.last().unwrap());
        for &word in self.0[..self.0.len()-1].iter().rev() { write!(&mut out, "{word:08x}").unwrap(); }
        out
    }
}
impl Ord for Nat {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.len().cmp(&rhs.0.len()).then_with(|| self.0.iter().rev().cmp(rhs.0.iter().rev()))
    }
}
impl PartialOrd for Nat { fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> { Some(self.cmp(rhs)) } }

fn choose(n: u64, k: usize) -> Nat {
    if k as u64 > n { return Nat::zero(); }
    let k = k.min((n-k as u64) as usize);
    let mut v = Nat::one();
    for i in 1..=k { v = v.mul_u64(n-k as u64+i as u64).div_exact_u64(i as u64); }
    v
}
fn falling(n: u64, k: usize) -> Nat {
    if k as u64 > n { return Nat::zero(); }
    (0..k).fold(Nat::one(), |v, i| v.mul_u64(n-i as u64))
}
fn stirling_row(n: usize) -> Vec<Nat> {
    let mut row = vec![Nat::zero(); n+1]; row[0] = Nat::one();
    for i in 1..=n {
        for j in (1..=i).rev() { row[j] = row[j].mul_u64(j as u64).add(&row[j-1]); }
        row[0] = Nat::zero();
    }
    row
}
fn partition_family(p: u64, s: usize) -> Nat {
    // Odd-p reduced separable degree-two maps, modulo final PGL2 output change.
    let mut out = Nat::zero(); let mut power = Nat::one();
    for _ in 0..=s { out = out.add(&power); power = power.mul_u64(p).mul_u64(p); }
    out
}
#[derive(Clone)]
struct Ratio { num: Nat, den: Nat }
impl Ratio {
    fn ceil_log2(&self) -> Option<i64> {
        assert!(self.den != Nat::zero());
        if self.num == Nat::zero() { return None; }
        let e = self.num.bits() as i64 - self.den.bits() as i64;
        let le = if e >= 0 { self.num <= self.den.shl(e as usize) }
            else { self.num.shl((-e) as usize) <= self.den };
        Some(if le { e } else { e+1 })
    }
}

fn adaptive_stirling(a: &Nat, m: u64, n: u64, h: usize, t: usize, d: u64) -> Ratio {
    assert!(1 <= t && t <= h && h as u64 <= n && n <= m && d >= 1);
    let q = m-h as u64+1;
    let ss = stirling_row(h);
    let mut sum = Nat::zero();
    for j in 1..=t {
        sum = sum.add(&ss[j].mul(&Nat::pow(d-1,h-j)).mul(&Nat::pow(q,j-1)));
    }
    Ratio { num: a.mul(&choose(n,h)).mul(&sum), den: Nat::pow(q,h-1) }
}
fn adaptive_anchors(a: &Nat, m: u64, n: u64, h: usize, t: usize, b: u64) -> Ratio {
    assert!(1 <= t && t <= h && h as u64 <= b && b <= m && h as u64 <= n && n <= m);
    Ratio { num: a.mul(&choose(n,h)).mul(&choose(h as u64,t)).mul(&falling(b-t as u64,h-t)),
        den: falling(m-t as u64,h-t) }
}
fn complete_fiber_probability(m: u64, n: u64, k: usize) -> Ratio {
    // Fixed selector, outputs selected adaptively; no family multiplier here.
    assert!(k >= 1 && k as u64 <= n && n <= m);
    Ratio { num: falling(n,k).mul_u64(m), den: falling(m,k).mul_u64(k as u64) }
}

struct Record { kind: &'static str, t: usize, support_cap: Option<u64>, ratio: Ratio }
fn main() {
    let p = 2_147_483_647u64;
    let r = 2_147_446_831u64;
    let m = (r-1)/2;
    let n = 32_768u64;
    let h = 128usize;
    let s = 10usize;
    let d = 1u64 << s;
    let a = partition_family(p,s);
    let mut records = Vec::new();
    for t in [1,2,4,8,10,12,13,14,15,16,32,64,128] {
        records.push(Record { kind:"adaptive_stirling", t, support_cap:None,
            ratio:adaptive_stirling(&a,m,n,h,t,d) });
    }
    for t in [1,4,8,12,14,16,32,64,128] {
        for b in [h as u64,2*h as u64,4*h as u64,1024,t as u64*d] {
            records.push(Record { kind:"adaptive_anchors", t, support_cap:Some(b),
                ratio:adaptive_anchors(&a,m,n,h,t,b) });
        }
    }
    for k in [2,3,4,8] {
        records.push(Record { kind:"fixed_chain_complete_fiber",t:k,support_cap:None,
            ratio:complete_fiber_probability(m,n,k) });
    }
    println!("kind\tt_or_k\tsupport_cap\tceil_log2_raw_bound\tprobability_at_most");
    let mut json = format!("{{\n  \"study\":\"m31-shared-chain-capacity-v1\",\n  \"randomness\":\"none\",\n  \"p\":{p},\n  \"r\":{r},\n  \"universe_m\":{m},\n  \"pool_n\":{n},\n  \"capture_h\":{h},\n  \"depth_s\":{s},\n  \"degree_cap_d\":{d},\n  \"family_partitions_hex\":\"{}\",\n  \"certificates\":[\n",a.hex());
    for (i,record) in records.iter().enumerate() {
        let e = record.ratio.ceil_log2();
        let e_text = e.map(|v|v.to_string()).unwrap_or("null".into());
        let cap_text = record.support_cap.map(|v|v.to_string()).unwrap_or("null".into());
        let probability = e.map(|v|format!("2^{}",v.min(0))).unwrap_or("0".into());
        println!("{}\t{}\t{}\t{}\t{}",record.kind,record.t,cap_text,e_text,probability);
        writeln!(&mut json,"    {{\"kind\":\"{}\",\"t_or_k\":{},\"support_cap\":{},\"ceil_log2_raw_bound\":{},\"probability_at_most\":\"{}\",\"numerator_hex\":\"{}\",\"denominator_hex\":\"{}\"}}{}",
            record.kind,record.t,cap_text,e_text,probability,record.ratio.num.hex(),record.ratio.den.hex(),if i+1==records.len(){""}else{","}).unwrap();
    }
    json.push_str("  ]\n}\n");
    let args:Vec<String> = std::env::args().collect();
    assert!(args.len()==1 || (args.len()==3 && args[1]=="--certificate"),"usage: exact_bounds [--certificate PATH]");
    if args.len()==3 { fs::write(&args[2],json).expect("write certificate"); }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn nat128(x:u128)->Nat {
        Nat::from_u64(x as u64).add(&Nat::from_u64((x>>64) as u64).shl(64))
    }
    #[test]
    fn arithmetic_against_machine_u128() {
        let xs = [0,1,2,3,17,0xffff_ffff,0x1_0000_0000,u64::MAX-1,u64::MAX];
        for &x in &xs { for &y in &xs {
            assert_eq!(Nat::from_u64(x).mul(&Nat::from_u64(y)),nat128(x as u128*y as u128));
            assert_eq!(Nat::from_u64(x).mul_u64(y),nat128(x as u128*y as u128));
            assert_eq!(Nat::from_u64(x).add(&Nat::from_u64(y)),nat128(x as u128+y as u128));
            if y>0 { assert_eq!(Nat::from_u64(x).mul_u64(y).div_exact_u64(y),Nat::from_u64(x)); }
        } }
        for x in xs { for k in 0..64 { assert_eq!(Nat::from_u64(x).shl(k),nat128((x as u128)<<k)); } }
    }
    #[test]
    fn combinatorics_independent_counting_identities() {
        for h in 0..=16 {
            let s = stirling_row(h);
            for m in 1..=12 {
                let sum=(0..=h).fold(Nat::zero(),|z,j|z.add(&s[j].mul(&falling(m,j))));
                assert_eq!(sum,Nat::pow(m,h));
            }
        }
        for k in 0..=24 {
            let sum=(0..=k).fold(Nat::zero(),|z,j|z.add(&choose(19,j).mul(&choose(23,k-j))));
            assert_eq!(sum,choose(42,k));
        }
    }
    #[test]
    fn exact_power_of_two_rounding() {
        for num in 1u64..100 { for den in 1u64..100 {
            let ratio=Ratio{num:Nat::from_u64(num),den:Nat::from_u64(den)};
            let e=ratio.ceil_log2().unwrap();
            for shift in [e,e-1] {
                let le=if shift>=0 { (num as u128)<=((den as u128)<<(shift as usize)) }
                    else { ((num as u128)<<((-shift) as usize))<=(den as u128) };
                assert_eq!(le,shift==e);
            }
        } }
    }
    #[test]
    fn actual_partition_events_below_bounds() {
        // Exhaustive finite SET fixture, not a small-field ECDLP experiment.
        // Partition [0,0,1,1,2,2], all 4-subsets, up to two chosen fibers.
        let blocks=[0,0,1,1,2,2]; let universe=6u64; let pool=4u64;
        for h in 2..=4 { for t in 1..=h.min(2) { for cap in (h as u64)..=6 {
            let mut successes=0u64; let mut total=0u64;
            for sample in 0u32..(1<<6) {
                if sample.count_ones()!=pool as u32 { continue; }
                total+=1;
                let mut found=false;
                for chosen in 1u32..(1<<3) {
                    if chosen.count_ones()>t as u32 || 2*chosen.count_ones() as u64>cap { continue; }
                    let hits=(0..6).filter(|&i|sample&(1<<i)!=0 && chosen&(1<<blocks[i])!=0).count();
                    found |= hits>=h;
                }
                successes+=found as u64;
            }
            let anchor=adaptive_anchors(&Nat::one(),universe,pool,h,t,cap);
            assert!(Nat::from_u64(successes).mul(&anchor.den)<=anchor.num.mul_u64(total));
            let degree=adaptive_stirling(&Nat::one(),universe,pool,h,t,2);
            assert!(Nat::from_u64(successes).mul(&degree.den)<=degree.num.mul_u64(total));
        } } }
    }
}
