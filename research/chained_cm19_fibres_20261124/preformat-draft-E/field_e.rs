pub type Point = [u8; 3];
pub type EPoint = Option<(u8, u8)>;
pub fn m(n: i32) -> u8 { u8::try_from(n.rem_euclid(7)).unwrap() }
pub fn add(a: u8,b: u8) -> u8 { m(i32::from(a)+i32::from(b)) }
pub fn sub(a: u8,b: u8) -> u8 { m(i32::from(a)-i32::from(b)) }
pub fn mul(a: u8,b: u8) -> u8 { m(i32::from(a)*i32::from(b)) }
pub fn pow(a: u8,n: usize) -> u8 { (0..n).fold(1,|x,_|mul(x,a)) }
pub fn inv(a: u8) -> u8 { (1..7u8).find(|b|mul(a,*b)==1).unwrap() }
pub fn valid(p: EPoint) -> bool {p.is_none_or(|(x,y)|pow(y,2)==add(add(pow(x,3),mul(2,x)),6))}
pub fn eneg(p: EPoint) -> EPoint {p.map(|(x,y)|(x,sub(0,y)))}
pub fn eadd(a: EPoint,b: EPoint) -> EPoint {
    assert!(valid(a)&&valid(b));let Some((x,y))=a else{return b;};let Some((xx,yy))=b else{return a;};
    if x==xx && add(y,yy)==0 {return None;}
    let slope=if x==xx {mul(add(mul(3,pow(x,2)),2),inv(mul(2,y)))} else {mul(sub(yy,y),inv(sub(xx,x)))};
    let nx=sub(sub(pow(slope,2),x),xx);let ny=sub(mul(slope,sub(x,nx)),y);let p=Some((nx,ny));assert!(valid(p));p
}
pub fn emul(mut n: usize,mut p: EPoint) -> EPoint {let mut r=None;while n!=0 {if n&1!=0 {r=eadd(r,p);}p=eadd(p,p);n>>=1;}r}
pub type Jet = [u8;3];
pub fn jc(a: u8) -> Jet {[a,0,0]}
pub fn ja(a: Jet,b: Jet) -> Jet {std::array::from_fn(|i|add(a[i],b[i]))}
pub fn js(a: Jet,k: u8) -> Jet {a.map(|x|mul(x,k))}
pub fn jm(a: Jet,b: Jet) -> Jet {std::array::from_fn(|i|(0..=i).map(|j|mul(a[j],b[i-j])).fold(0,add))}
pub fn jp(a: Jet,n: usize) -> Jet {(0..n).fold(jc(1),|x,_|jm(x,a))}
pub fn jd(a: Jet,b: Jet) -> Jet {
    let d=inv(b[0]);let c0=mul(a[0],d);let c1=mul(sub(a[1],mul(b[1],c0)),d);let c2=mul(sub(sub(a[2],mul(b[1],c1)),mul(b[2],c0)),d);[c0,c1,c2]
}
pub struct Linear {pub rank: usize,pub pivots: Vec<usize>,pub reduced: Vec<Vec<u8>>,pub solution: Vec<u8>,pub kernel: Vec<Vec<u8>>}
pub fn solve(input: &[Vec<u8>],variables: usize) -> Linear {
    assert!(input.iter().all(|r|r.len()==variables+1));let mut a=input.to_vec();let mut pivots=Vec::new();let mut row=0;
    for column in 0..variables {
        let Some(found)=(row..a.len()).find(|i|a[*i][column]!=0) else {continue;};a.swap(row,found);let factor=inv(a[row][column]);for x in &mut a[row] {*x=mul(*x,factor);}
        let pivot=a[row].clone();for (i,r) in a.iter_mut().enumerate() {if i!=row {let c=r[column];for (x,&p) in r.iter_mut().zip(&pivot) {*x=sub(*x,mul(c,p));}}}
        pivots.push(column);row+=1;if row==a.len(){break;}
    }
    assert!(a.iter().all(|r|r[..variables].iter().any(|x|*x!=0)||r[variables]==0),"inconsistent exact system");
    let mut solution=vec![0;variables];for (i,&p) in pivots.iter().enumerate() {solution[p]=a[i][variables];}
    let mut kernel=Vec::new();for free in (0..variables).filter(|i|!pivots.contains(i)) {let mut v=vec![0;variables];v[free]=1;for (i,&p) in pivots.iter().enumerate() {v[p]=sub(0,a[i][free]);}kernel.push(v);}
    for r in input {assert_eq!(r[..variables].iter().zip(&solution).map(|(a,b)|mul(*a,*b)).fold(0,add),r[variables]);for v in &kernel {assert_eq!(r[..variables].iter().zip(v).map(|(a,b)|mul(*a,*b)).fold(0,add),0);}}
    Linear{rank:pivots.len(),pivots,reduced:a,solution,kernel}
}
