const INPUT:&str=include_str!("first-C-native/kernel-basis-input.stdout");
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
struct O(i128,i128);
const ZERO:O=O(0,0);const ONE:O=O(1,0);
impl O {
    fn plus(self,b:Self)->Self {Self(self.0+b.0,self.1+b.1)}
    fn scaled(self,n:i128)->Self {Self(self.0*n,self.1*n)}
    fn times(self,b:Self)->Self {Self(self.0*b.0-5*self.1*b.1,self.0*b.1+self.1*b.0+self.1*b.1)}
    fn bar(self)->Self {Self(self.0+self.1,-self.1)}
    fn divided(self,n:i128)->Self {assert_eq!(self.0%n,0);assert_eq!(self.1%n,0);Self(self.0/n,self.1/n)}
}
type M=[[O;2];2];
fn product(a:M,b:M)->M {std::array::from_fn(|i|std::array::from_fn(|j|(0..2).fold(ZERO,|r,k|r.plus(a[i][k].times(b[k][j])))))}
fn dagger(a:M)->M {std::array::from_fn(|i|std::array::from_fn(|j|a[j][i].bar()))}
fn determinant(a:M)->O {a[0][0].times(a[1][1]).plus(a[0][1].times(a[1][0]).scaled(-1))}
fn field<'a>(line:&'a str,prefix:&str)->&'a str {line.split('\t').find_map(|s|s.strip_prefix(prefix)).unwrap()}
fn parse(s:&str)->Vec<O> {let n:Vec<i128>=s.split(|c:char|!c.is_ascii_digit()&&c!='-').filter(|s|!s.is_empty()).map(|s|s.parse().unwrap()).collect();assert!(n.len().is_multiple_of(2));n.as_chunks::<2>().0.iter().map(|p|O(p[0],p[1])).collect()}
fn act(c:O,p:usize,frob:[usize;4])->usize {let a=if c.0.rem_euclid(2)==1{p}else{0};let b=if c.1.rem_euclid(2)==1{frob[p]}else{0};a^b}
fn kernel(a:M,frob:[usize;4])->Vec<[usize;2]> {(0..4).flat_map(|p|(0..4).map(move |q|[p,q])).filter(|&[p,q]|(0..2).all(|i|(act(a[i][0],p,frob)^act(a[i][1],q,frob))==0)).collect()}
pub fn qualify(frob:[usize;4])->Vec<[usize;2]> {
    let line=INPUT.lines().find(|s|s.starts_with("FULL_ACTUAL_KERNEL_HERMITIAN\t")).unwrap();let flat=parse(field(line,"G="));assert_eq!(flat.len(),4);let g:M=std::array::from_fn(|i|std::array::from_fn(|j|flat[2*i+j]));assert_eq!(determinant(g),O(4,0));
    let h=[[g[0][0].divided(4),g[0][1].divided(2)],[g[1][0].divided(2),g[1][1]]];assert_eq!(h,dagger(h));assert_eq!(determinant(h),ONE);
    let u=[[ONE,ZERO],[ONE,ONE]];let hpp=product(product(dagger(u),h),u);let omega=O(0,1);let bar=omega.bar();assert_eq!(hpp,[[O(2,0),bar],[omega,O(3,0)]]);
    let t=[[ONE,bar.scaled(-1)],[ZERO,O(2,0)]];let swapped=[[bar.scaled(-1),ONE],[O(2,0),ZERO]];let twice=[[O(2,0),ZERO],[ZERO,O(2,0)]];
    assert_eq!(product(product(dagger(t),hpp),t),twice);assert_eq!(product(product(dagger(swapped),hpp),swapped),twice);
    let mod_two=g.map(|r|r.map(|c|O(c.0.rem_euclid(2),c.1.rem_euclid(2))));assert_eq!(mod_two,[[ZERO,ZERO],[ZERO,ONE]]);
    let first_kernel=kernel(t,frob);let graph=kernel(swapped,frob);let expected:Vec<[usize;2]>=(0..4).map(|p|[p,frob[frob[p]]]).collect();assert_eq!(graph,expected);assert_eq!(graph.len(),4);
    for p in 0..4{assert_eq!(act(bar,p,frob),frob[frob[p]]);}
    println!("FULL_ACTUAL_PRINCIPAL_PRYM_COORDINATES\tG={g:?}\tG-mod2={mod_two:?}\tHprime={h:?}\tchange-U={u:?}\tHpp={hpp:?}\tdeterminants={:?},{:?}\tactual-kernel-degree4-and-division2=separate-written-geometry",determinant(h),determinant(hpp));
    println!("FULL_ORDERED_GLUE_CERTIFICATE\tT-c-e={t:?}\tkernel-T={first_kernel:?}\tS-e-c={swapped:?}\tkernel-S={graph:?}\texpected-Graph-Frob49={expected:?}\tSdagger-Hpp-S={:?}\torientation-correction-ff1474",product(product(dagger(swapped),hpp),swapped));
    let mut bad=swapped;bad[0][1]=bad[0][1].plus(ONE);assert_ne!(product(product(dagger(bad),hpp),bad),twice);assert_eq!(product(product(dagger(swapped),hpp),swapped),twice);
    println!("REAL_ORDERED_GLUE_MUTANT\tS01+1={bad:?}\tfull-bad-Gram={:?}\tidentity-rejected\tpristine-restored",product(product(dagger(bad),hpp),bad));graph
}
