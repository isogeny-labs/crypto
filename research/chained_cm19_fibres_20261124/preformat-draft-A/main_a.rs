use std::collections::BTreeSet;

type Point = [u8; 3];
type EPoint = Option<(u8, u8)>;
type JPoint = [EPoint; 3];
type Matrix = [[u8; 3]; 3];
const FIXTURE: &str = include_str!("curve-map-fixture.stdout");
const R: Matrix = [[1, 5, 2], [0, 5, 0], [2, 1, 1]];
const T: EPoint = Some((1, 3));
const MINUS: Point = [1, 0, 6];
const PLUS: Point = [1, 0, 1];
const H_INVERSE: Matrix = [[2, 1, 4], [7, 5, 8], [10, 7, 5]];
const N_DAGGER: Matrix = [[0, 3, 0], [10, 1, 7], [1, 10, 1]];
const PULLBACK: [u8; 3] = [8, 2, 9];

fn m(x: i32) -> u8 { u8::try_from(x.rem_euclid(7)).unwrap() }
fn add(a: u8,b: u8) -> u8 { m(i32::from(a)+i32::from(b)) }
fn sub(a: u8,b: u8) -> u8 { m(i32::from(a)-i32::from(b)) }
fn mul(a: u8,b: u8) -> u8 { m(i32::from(a)*i32::from(b)) }
fn square(a: u8) -> u8 { mul(a,a) }
fn cube(a: u8) -> u8 { mul(square(a),a) }
fn inv(a: u8) -> u8 { (1..7u8).find(|x|mul(a,*x)==1).unwrap() }
fn valid(p: EPoint) -> bool { p.is_none_or(|(x,y)|square(y)==add(add(cube(x),mul(2,x)),6)) }
fn eneg(p: EPoint) -> EPoint { p.map(|(x,y)|(x,sub(0,y))) }
fn eadd(a: EPoint,b: EPoint) -> EPoint {
    assert!(valid(a) && valid(b));
    let Some((x,y))=a else {return b;};
    let Some((xx,yy))=b else {return a;};
    if x==xx && add(y,yy)==0 {return None;}
    let slope=if x==xx {mul(add(mul(3,square(x)),2),inv(mul(2,y)))} else {mul(sub(yy,y),inv(sub(xx,x)))};
    let nx=sub(sub(square(slope),x),xx);
    let ny=sub(mul(slope,sub(x,nx)),y);
    let result=Some((nx,ny));assert!(valid(result));result
}
fn emul(mut n: usize,mut p: EPoint) -> EPoint {
    let mut result=None;
    while n!=0 {if n&1!=0 {result=eadd(result,p);}p=eadd(p,p);n>>=1;}
    result
}
fn jadd(a: JPoint,b: JPoint) -> JPoint {std::array::from_fn(|i|eadd(a[i],b[i]))}
fn jsub(a: JPoint,b: JPoint) -> JPoint {jadd(a,b.map(eneg))}
fn jmul(n: usize,a: JPoint) -> JPoint {a.map(|x|emul(n,x))}
fn elliptic_linear(row: [u8;3],p: JPoint) -> EPoint {
    row.into_iter().zip(p).fold(None,|a,(n,b)|eadd(a,emul(usize::from(n),b)))
}
fn pullback(p: EPoint,column: [u8;3]) -> JPoint {column.map(|n|emul(usize::from(n),p))}
fn split_row(p: JPoint) -> EPoint {eneg(eadd(p[0],p[1]))}
fn push(p: JPoint) -> EPoint {eadd(emul(3,p[0]),p[1])}
fn mmul(a: Matrix,b: Matrix) -> Matrix {
    std::array::from_fn(|i|std::array::from_fn(|j|{
        let n:u32=(0..3).map(|k|u32::from(a[i][k])*u32::from(b[k][j])).sum();u8::try_from(n%11).unwrap()
    }))
}
fn matrix_point(a: Matrix,p: JPoint) -> JPoint {a.map(|row|elliptic_linear(row,p))}
fn curve(p: Point) -> u8 {
    let [x,y,z]=p;let g=[square(z),mul(6,square(x)),mul(3,mul(x,y)),square(y)].into_iter().fold(0,add);
    sub(square(g),mul(y,add(add(cube(x),mul(2,mul(x,square(y)))),mul(6,cube(y)))))
}
fn transform(p: Point,a: Matrix) -> Point {
    a.map(|row|row.into_iter().zip(p).map(|(x,y)|mul(x,y)).fold(0,add))
}
fn f0(p: Point) -> EPoint {
    let [x,y,z]=p;if y==0 {return None;}
    let g=[square(z),mul(6,square(x)),mul(3,mul(x,y)),square(y)].into_iter().fold(0,add);
    let result=Some((mul(x,inv(y)),mul(g,inv(square(y)))));assert!(valid(result));result
}
fn f3(p: Point,flip_infinity: bool) -> EPoint {
    let [a,b,c]=p;let x=mul(3,b);let y=mul(4,add(sub(a,mul(3,b)),c));let z=mul(4,sub(sub(a,mul(3,b)),c));
    if x==0 {assert!(y==0 || z==0);return if (y==0)!=flip_infinity {None} else {T};}
    let u=mul(mul(y,z),inv(square(x)));let v=mul(cube(y),inv(cube(x)));
    let w=[mul(4,v),1,mul(3,u),mul(2,square(u))].into_iter().fold(0,add);
    let delta=[w,mul(2,square(u)),mul(6,u),3].into_iter().fold(0,add);
    let result=Some((add(delta,1),add(mul(add(mul(5,u),4),delta),4)));assert!(valid(result));result
}
fn point_digits(text: &str) -> Point {
    let d:Vec<_>=text.bytes().filter(u8::is_ascii_digit).map(|x|x-b'0').collect();d.try_into().unwrap()
}
fn parse_e(text: &str) -> EPoint {
    if text=="None" {return None;}
    assert!(text.starts_with("Some((") && text.ends_with("))"));
    let d:Vec<_>=text.bytes().filter(u8::is_ascii_digit).map(|x|x-b'0').collect();assert_eq!(d.len(),2);Some((d[0],d[1]))
}
fn original_image(p: Point,prefix: &str) -> EPoint {
    let rows:Vec<_>=FIXTURE.lines().filter(|line|line.starts_with(prefix)).filter(|line|{
        let value=line.split("source=[").nth(1).unwrap().split(']').next().unwrap();point_digits(value)==p
    }).collect();assert_eq!(rows.len(),1);
    parse_e(rows[0].split("\tE7=").nth(1).unwrap().split('\t').next().unwrap())
}
fn source_points() -> Vec<Point> {
    let line=FIXTURE.lines().find_map(|s|s.strip_prefix("FROZEN_SOURCE_POINTS\t")).unwrap();
    let digits:Vec<_>=line.bytes().filter(u8::is_ascii_digit).map(|x|x-b'0').collect();assert_eq!(digits.len(),51);
    let result:Vec<Point>=digits.as_chunks::<3>().0.to_vec();assert_eq!(result.iter().copied().collect::<BTreeSet<_>>().len(),17);
    for &p in &result {assert_eq!(curve(p),0);assert_eq!(p[0],1);}
    result
}
fn line_through(a: Point,b: Point) -> Point {
    let mut line=[sub(mul(a[1],b[2]),mul(a[2],b[1])),sub(mul(a[2],b[0]),mul(a[0],b[2])),sub(mul(a[0],b[1]),mul(a[1],b[0]))];
    let scale=inv(*line.iter().find(|x|**x!=0).unwrap());line=line.map(|x|mul(x,scale));line
}
fn on_line(p: Point,line: Point) -> bool {p.into_iter().zip(line).map(|(a,b)|mul(a,b)).fold(0,add)==0}
fn f4(cover: JPoint,coefficient: usize) -> EPoint {
    emul(4,eadd(eadd(cover[0],emul(coefficient,cover[1])),eneg(cover[2])))
}
fn main() {
    println!("PROFILE\tactual-CM19-degree4-fibre-certificate-A\texact-fixed-object\tunsigned\tno-performance-claim");
    let points=source_points();let target:Vec<_>=(0..11).map(|i|emul(i,T)).collect();
    assert_eq!(emul(11,T),None);assert_eq!(target.iter().copied().collect::<BTreeSet<_>>().len(),11);assert!(target.iter().all(|p|valid(*p)));
    println!("ACTUAL_TARGET_BINDING\tT={T:?}\t3T={:?}\tall11={target:?}",emul(3,T));
    let mut alpha=mmul(H_INVERSE,N_DAGGER);for row in &mut alpha {for a in row {*a=u8::try_from((u32::from(*a)*4)%11).unwrap();}}
    assert_eq!(alpha,[[1,1,0],[1,6,7],[3,7,7]]);
    let numerator_row:[u8;3]=std::array::from_fn(|i|u8::try_from((3*u32::from(alpha[0][i])+u32::from(alpha[1][i]))%11).unwrap());
    assert_eq!(numerator_row,[4,9,7]);
    println!("ACTUAL_ALPHA_BINDING\tHinv={H_INVERSE:?}\tNdagger={N_DAGGER:?}\tinv3=4\talpha={alpha:?}\tell-alpha={numerator_row:?}");
    assert_eq!(f0(MINUS),None);assert_eq!(f0(PLUS),None);assert_eq!(f3(MINUS,false),None);assert_eq!(f3(PLUS,false),T);
    let mut classes=Vec::new();let mut images=Vec::new();let mut covers=Vec::new();
    for (index,&p) in points.iter().enumerate() {
        let cover=[f0(p),f0(transform(p,R)),f3(p,false)];
        assert_eq!(cover[0],original_image(p,"DEGREE2_POINT\tinvolution=1\t"));
        assert_eq!(cover[2],original_image(p,"DEGREE3_POINT\t"));
        let class=matrix_point(alpha,cover);let image=f4(cover,5);assert_eq!(image,push(class));
        let scalar=target.iter().position(|q|*q==image).unwrap();
        println!("ACTUAL_POINT\t{index}\tC={p:?}\tf0-f1-f3={cover:?}\talpha-Abel={class:?}\tf4={image:?}\tscalar-T={scalar}");
        classes.push(class);images.push(image);covers.push(cover);
    }
    let iminus=points.iter().position(|p|*p==MINUS).unwrap();let iplus=points.iter().position(|p|*p==PLUS).unwrap();
    assert_eq!(classes[iminus],[None;3]);assert_eq!(classes[iplus],pullback(T,[0,7,7]));assert_eq!(images[iplus],emul(7,T));
    let canonical=jmul(2,classes[iplus]);assert_eq!(canonical,pullback(T,[0,3,3]));assert_eq!(push(canonical),emul(3,T));
    for &q in &target {assert_eq!(split_row(pullback(q,PULLBACK)),q);assert_eq!(push(pullback(q,PULLBACK)),emul(4,q));}
    println!("GLOBAL_INPUT_RATIONAL_BINDINGS\tKminus4Pminus={canonical:?}\tL-K={:?}\tLdagger={PULLBACK:?}\tsplit-row=(-1,-1,0)\tfull-global-bridges=written",push(canonical));
    let mut full=Vec::new();let mut total=0;
    for (scalar,&q) in target.iter().enumerate() {
        let members:Vec<_>=images.iter().enumerate().filter_map(|(i,p)|(*p==q).then_some(i)).collect();assert!(members.len()<=4);total+=members.len();
        println!("ACTUAL_FIBRE\tscalar-T={scalar}\tE={q:?}\tsize={}\tC-indices={members:?}",members.len());
        if members.len()==4 {full.push((q,members));}
    }
    assert_eq!(total,17);
    if full.is_empty() {println!("FULL_RATIONAL_FIBRE_NEGATIVE\tcount=0\tfull-pole-divisor-not-obtained\tRR-input-branch-inconclusive");}
    else {
        let mut origin=None;
        for (q,members) in &full {
            let class=members.iter().fold([None;3],|a,i|jadd(a,classes[*i]));let a0=jsub(class,pullback(*q,PULLBACK));
            if let Some(expected)=origin {assert_eq!(a0,expected);} else {origin=Some(a0);}
            let line=line_through(points[members[0]],points[members[1]]);let collinear=members.iter().all(|i|on_line(points[*i],line));
            assert_eq!(class==canonical,collinear);
            println!("FULL_GEOMETRIC_FIBRE_CERTIFICATE\tR={q:?}\tindices={members:?}\tactual-points={:?}\tAbel-class={class:?}\torigin-fibre-class={a0:?}\tcanonical={collinear}\tline={line:?}\tsaturation=global-degree4",members.iter().map(|i|points[*i]).collect::<Vec<_>>());
        }
        let a0=origin.unwrap();let mut canonical_parameters=Vec::new();
        for (scalar,&q) in target.iter().enumerate() {
            let class=jadd(a0,pullback(q,PULLBACK));let equal=class==canonical;if equal {canonical_parameters.push(q);}
            println!("TRANSLATED_CANONICAL_DECISION\tscalar-T={scalar}\tQ={q:?}\tclass={class:?}\tK={canonical:?}\tequal={equal}");
        }
        assert!(canonical_parameters.len()<=1);let candidate=split_row(jsub(canonical,a0));
        assert!(target.contains(&candidate));
        println!("CANONICAL_CLASS_RESULT\tfull-rational-fibres={}\tall11-decisions\tparameters={canonical_parameters:?}\tsplit-row-candidate={candidate:?}\tgeometric-completeness=global-injective-F7-Ldagger-and-Frobenius-written",full.len());
    }
    let flipped=points.iter().filter(|p|f3(**p,true)!=original_image(**p,"DEGREE3_POINT\t")).count();assert_eq!(flipped,2);
    let changed=covers.iter().filter(|p|f4(**p,4)!=f4(**p,5)).count();assert!(changed>0);
    let mut bad_column=PULLBACK;bad_column[0]=7;assert_ne!(split_row(pullback(T,bad_column)),T);assert_eq!(split_row(pullback(T,PULLBACK)),T);
    assert_ne!(curve([0,1,0]),0);
    for (i,&p) in points.iter().enumerate() {assert_eq!(f3(p,false),original_image(p,"DEGREE3_POINT\t"));assert_eq!(f4(covers[i],5),images[i]);}
    println!("ACTUAL_MUTANTS\tinfinity-normalization-rejected={flipped}\tnumerator-coefficient-rejected={changed}\tLdagger-column-rejected\tsource-point-rejected\tpristine-restored");
    println!("SUMMARY\tactual-C-points=17\ttarget-points=11\tfull-rational-fibres={}\tglobal-degree-input=4\tactual-function-extraction=NEXT\tEclipse-performance=UNMEASURED\tstanding=unsigned",full.len());
}
