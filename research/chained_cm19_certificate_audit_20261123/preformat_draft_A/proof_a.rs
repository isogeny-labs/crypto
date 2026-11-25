#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct O(i128, i128);
const ZERO: O = O(0,0);
const ONE: O = O(1,0);
type Matrix = [[O;3];3];
impl O {
    fn sum(self,b:Self)->Self { Self(self.0+b.0,self.1+b.1) }
    fn opposite(self)->Self { Self(-self.0,-self.1) }
    fn product(self,b:Self)->Self { Self(self.0*b.0-5*self.1*b.1,self.0*b.1+self.1*b.0+self.1*b.1) }
    fn conjugate(self)->Self { Self(self.0+self.1,-self.1) }
}
fn product(a: Matrix,b:Matrix)->Matrix {
    a.map(|row| std::array::from_fn(|j|row.into_iter().enumerate().fold(ZERO,|s,(k,x)|s.sum(x.product(b[k][j])))))
}
fn adjoint(a:Matrix)->Matrix {
    std::array::from_fn(|i|std::array::from_fn(|j|a[j][i].conjugate()))
}
fn scaled(a:Matrix,c:i128)->Matrix {a.map(|row|row.map(|x|x.product(O(c,0))))}
fn identity()->Matrix {std::array::from_fn(|i|std::array::from_fn(|j|if i==j{ONE}else{ZERO}))}
fn minor(a:Matrix,i:usize,j:usize)->O {
    let r:Vec<_>=(0..3).filter(|&k|k!=i).collect();
    let c:Vec<_>=(0..3).filter(|&k|k!=j).collect();
    a[r[0]][c[0]].product(a[r[1]][c[1]]).sum(a[r[0]][c[1]].product(a[r[1]][c[0]]).opposite())
}
pub fn audit() {
    let pi=O(-2,1);
    assert_eq!(pi.sum(pi.conjugate()),O(-3,0));
    assert_eq!(pi.product(pi.conjugate()),O(7,0));
    let g=[[O(2,0),O(-1,0),ZERO],[O(-1,0),O(2,0),ZERO],[ZERO,ZERO,O(3,0)]];
    let h=[[O(2,0),ONE,O(-1,0)],[ONE,O(3,0),pi],[O(-1,0),pi.conjugate(),O(3,0)]];
    let n=[[ZERO,O(-1,0),ONE],[O(3,0),ONE,O(-1,0)],[ZERO,pi,pi.opposite().sum(O(-3,0))]];
    let t=[[ONE,ONE,ZERO],[O(-1,-1),ZERO,O(-1,0)],[O(2,-1),ZERO,O(-1,0)]];
    assert_eq!(product(product(adjoint(n),g),n),scaled(h,9));
    assert_eq!(product(n,t),scaled(identity(),3));
    assert_eq!(product(product(adjoint(t),h),t),g);
    let cof:Matrix=std::array::from_fn(|i|std::array::from_fn(|j|{
        let x=minor(h,j,i);
        if (i+j)%2==0{x}else{x.opposite()}
    }));
    assert_eq!(product(h,cof),identity());
    let det=(0..3).fold(ZERO,|s,j|s.sum(h[0][j].product(cof[j][0])));
    assert_eq!(det,ONE);
    let mut wrong_h=h;
    wrong_h[0][0]=wrong_h[0][0].sum(ONE);
    assert_ne!(product(product(adjoint(n),g),n),scaled(wrong_h,9));
    let mut wrong_t=t;
    wrong_t[0][0]=wrong_t[0][0].sum(ONE);
    assert_ne!(product(n,wrong_t),scaled(identity(),3));
    assert_eq!(product(product(adjoint(n),g),n),scaled(h,9));
    println!("AUDIT_WRITTEN_PROOF\tpi={:?}\tN={:?}\tTminus={:?}\tH={:?}\tH_inverse={:?}\tNdagGN=9H\tNTminus=3I\tTminusdagHTminus=G\tdetH=1\tactual_H_T_mutations_rejected_and_restored\tstandard_geometry_dependencies_remain_written\tPASS",pi,n,t,h,cof);
}
