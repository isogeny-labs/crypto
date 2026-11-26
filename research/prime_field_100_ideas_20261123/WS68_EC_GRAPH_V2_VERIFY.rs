// Independent native verifier for EC_GRAPH_RANK_GP_V2.gp / stdout.
// Fixed E/F_1009: y^2=x^3+x+14, P=(0,425), order 1013.
// Checks six EC row identities and ranks over F_1013, without GP imports.
const P:i64=1009;
const R:i64=1013;
#[derive(Clone,Copy,Debug,Eq,PartialEq)] enum Pt { O, A(i64,i64) }
fn m(x:i64,n:i64)->i64 { x.rem_euclid(n) }
fn inv(x:i64,n:i64)->i64 {
    let (mut a,mut b,mut u,mut v)=(m(x,n),n,1,0);
    while b!=0 { let q=a/b; (a,b)=(b,a-q*b); (u,v)=(v,u-q*v); }
    assert_eq!(a,1);m(u,n)
}
fn valid(q:Pt)->bool { match q {Pt::O=>true,Pt::A(x,y)=>m(y*y-x*x*x-x-14,P)==0} }
fn neg(q:Pt)->Pt {match q {Pt::O=>q,Pt::A(x,y)=>Pt::A(x,m(-y,P))}}
fn plus(q:Pt,t:Pt)->Pt {
    match(q,t) {
        (Pt::O,x)|(x,Pt::O)=>x,
        (Pt::A(x,y),Pt::A(u,v))=>{
            if x==u && m(y+v,P)==0{return Pt::O;}
            let s=if x==u {m((3*x*x+1)*inv(2*y,P),P)} else {m((v-y)*inv(u-x,P),P)};
            let xx=m(s*s-x-u,P);Pt::A(xx,m(s*(x-xx)-y,P))
        }
    }
}
fn times(mut n:i64,mut q:Pt)->Pt {
    if n<0 {n=-n;q=neg(q);} let mut out=Pt::O;
    while n!=0 {if n&1!=0 {out=plus(out,q);} q=plus(q,q);n>>=1;}out
}
fn rank(a:[[i64;3];3])->usize {
    let mut b=a;let mut h=0;
    for j in 0..3 {
        if let Some(k)=(h..3).find(|&k|m(b[k][j],R)!=0) {
            b.swap(h,k);let z=inv(b[h][j],R);
            for c in j..3 {b[h][c]=m(b[h][c]*z,R);}
            let pivot=b[h];
            for i in 0..3 {if i!=h {let z=b[i][j];for c in j..3 {b[i][c]=m(b[i][c]-z*pivot[c],R);}}}
            h+=1;
        }
    }h
}
fn det(a:[[i64;3];3])->i64 {
    a[0][0]*(a[1][1]*a[2][2]-a[1][2]*a[2][1])
    -a[0][1]*(a[1][0]*a[2][2]-a[1][2]*a[2][0])
    +a[0][2]*(a[1][0]*a[2][1]-a[1][1]*a[2][0])
}
fn main(){
    let g=Pt::A(0,425);assert!(valid(g));assert_eq!(times(R,g),Pt::O);
    let cardinality=1+(0..P).flat_map(|x|(0..P).map(move|y|Pt::A(x,y))).filter(|&q|valid(q)).count();
    assert_eq!(cardinality,1013);
    let points=[g,times(2,g),times(3,g)];assert!(points.iter().all(|&q|valid(q)));
    let a=[[1,1,0],[0,1,1],[1,0,1]];
    let b=[[1,1,0],[0,1,1],[1,0,-1]];
    let ra=[3,5,4];let rb=[3,5,-2];
    for (name,rows,rhs) in [("A",a,ra),("B",b,rb)] {
        for (i,row) in rows.iter().enumerate(){
            let lhs=(0..3).fold(Pt::O,|acc,j|plus(acc,times(row[j],points[j])));
            assert_eq!(lhs,times(rhs[i],g),"{name} row {i}");
        }
    }
    for i in 0..3{for j in 0..3{assert_eq!(a[i][j]!=0,b[i][j]!=0);}}
    let (da,db)=(det(a),det(b));let (rka,rkb)=(rank(a),rank(b));
    assert_eq!((da,db,rka,rkb),(2,0,3,2));
    assert_eq!(points,[Pt::A(0,425),Pt::A(991,641),Pt::A(162,510)]);
    println!("curve_p={P} group_order={R} group_count={cardinality}");
    println!("factor_base={points:?} support_equal=true graph_vertices=3 graph_edges=3 cycle_rank=1");
    println!("A_rows_valid=3 B_rows_valid=3 det_A={da} rank_A={rka} det_B={db} rank_B={rkb}");
    println!("A_rhs={ra:?} B_rhs={rb:?}");
}
