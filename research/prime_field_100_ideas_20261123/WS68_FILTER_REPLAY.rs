// Frozen native EC relation replay for ideas 75, 76, and 84.
// Compile and execute only after the parent confirms the S3 checkpoint gate.
// Input: E/F_17: y^2=x^3+2x+2, G=(5,1), subgroup order 19.
// This is a planted exactness and rank control, not a performance benchmark.
use std::fmt::Write;

const P: i64 = 17;
const N: i64 = 19;
const COLS: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Point { Inf, Aff(i64, i64) }

fn red(x: i64, m: i64) -> i64 { x.rem_euclid(m) }
fn inv(x: i64, m: i64) -> i64 {
    let (mut a, mut b, mut u, mut v) = (red(x,m), m, 1, 0);
    while b != 0 { let q=a/b; (a,b)=(b,a-q*b); (u,v)=(v,u-q*v); }
    assert_eq!(a,1); red(u,m)
}
fn on_curve(q: Point) -> bool {
    match q { Point::Inf => true, Point::Aff(x,y) => red(y*y-x*x*x-2*x-2,P)==0 }
}
fn neg(q: Point) -> Point { match q { Point::Inf=>q, Point::Aff(x,y)=>Point::Aff(x,red(-y,P)) } }
fn add(a: Point, b: Point) -> Point {
    match (a,b) {
        (Point::Inf,q)|(q,Point::Inf) => q,
        (Point::Aff(x,y),Point::Aff(u,v)) => {
            if x==u && red(y+v,P)==0 { return Point::Inf; }
            let s=if x==u { red((3*x*x+2)*inv(2*y,P),P) }
                  else { red((v-y)*inv(u-x,P),P) };
            let xx=red(s*s-x-u,P); let yy=red(s*(x-xx)-y,P);
            Point::Aff(xx,yy)
        }
    }
}
fn mul(mut k: i64, mut q: Point) -> Point {
    if k<0 { k=-k; q=neg(q); }
    let mut out=Point::Inf;
    while k>0 { if k&1==1 { out=add(out,q); } q=add(q,q); k>>=1; }
    out
}
fn rank(mut a: Vec<Vec<i64>>) -> usize {
    let mut r=0;
    for c in 0..a[0].len() {
        if let Some(j)=(r..a.len()).find(|&j| red(a[j][c],N)!=0) {
            a.swap(r,j); let z=inv(a[r][c],N);
            for v in &mut a[r] { *v=red(*v*z,N); }
            let pivot=a[r].clone();
            for i in 0..a.len() { if i!=r {
                let z=a[i][c];
                for k in c..a[i].len() { a[i][k]=red(a[i][k]-z*pivot[k],N); }
            }}
            r+=1; if r==a.len() { break; }
        }
    }
    r
}
fn group_sum(row: &[i64], points: &[Point]) -> Point {
    row.iter().zip(points).fold(Point::Inf, |acc,(&c,&q)| add(acc,mul(c,q)))
}
fn main() {
    let g=Point::Aff(5,1);
    assert!(on_curve(g)); assert_eq!(mul(N,g),Point::Inf);
    for k in 1..N { assert_ne!(mul(k,g),Point::Inf); }
    let group_count=1+(0..P).flat_map(|x| (0..P).map(move |y| (x,y)))
        .filter(|&(x,y)| on_curve(Point::Aff(x,y))).count();
    assert_eq!(group_count,N as usize);

    // Columns: x0,x1,x2,L10,L11,L12. Their points are actual EC multiples.
    let logs=[3,5,12,10,9,7];
    let points: Vec<Point>=logs.iter().map(|&k| mul(k,g)).collect();
    assert!(points.iter().all(|&q|on_curve(q)));
    let rows: [[i64;COLS];3]=[
        [1,0,0,1,-1,0], [0,1,0,0,1,-1], [0,0,1,-1,0,1]
    ];
    let rhs=[4,7,9];
    for (row,&b) in rows.iter().zip(rhs.iter()) {
        assert_eq!(group_sum(row,&points),mul(b,g));
    }
    let mut closed=[0;COLS];
    for row in &rows { for j in 0..COLS { closed[j]=red(closed[j]+row[j],N); } }
    assert_eq!(closed,[1,1,1,0,0,0]);
    let closed_rhs=red(rhs.iter().sum(),N);
    assert_eq!(closed_rhs,1); // 20 modulo subgroup order 19.
    assert_eq!(group_sum(&closed,&points),mul(closed_rhs,g));
    let source_rank=rank(rows.iter().map(|r|r.to_vec()).collect());
    let small_rows=vec![vec![1,0,0],vec![0,1,0],closed[..3].to_vec()];
    let small_rank=rank(small_rows.clone());
    let missing_rank=rank(vec![small_rows[0].clone(),small_rows[2].clone()]);
    assert_eq!((source_rank,small_rank,missing_rank),(3,3,2));
    let target_log=red(closed_rhs-3-5,N); assert_eq!(target_log,12);
    assert_eq!(mul(target_log,g),points[2]);

    // A deliberately disconnected block: z0=[2]G, z1=[4]G.
    let z=[mul(2,g),mul(4,g)];
    assert_eq!(group_sum(&[1,1],&z),mul(6,g));
    assert_eq!(group_sum(&[1,-1],&z),mul(17,g));
    let block=vec![
        vec![1,0,0,0,0],vec![0,1,0,0,0],vec![1,1,1,0,0],
        vec![0,0,0,1,1],vec![0,0,0,1,-1],
    ];
    let block_rank=rank(block); assert_eq!(block_rank,5);
    let mut out=String::new();
    writeln!(out,"curve_p={P} subgroup_n={N} group_count={group_count}").unwrap();
    writeln!(out,"generator={g:?} points={points:?}").unwrap();
    writeln!(out,"source_rows_valid=3 source_rank={source_rank}").unwrap();
    writeln!(out,"closed_coefficients={closed:?} closed_rhs={closed_rhs} closed_identity_valid=true").unwrap();
    writeln!(out,"small_rank={small_rank} rank_without_x1={missing_rank} target_x2_log={target_log}").unwrap();
    writeln!(out,"disconnected_component_rank=2 combined_block_rank={block_rank}").unwrap();
    print!("{out}");
}
