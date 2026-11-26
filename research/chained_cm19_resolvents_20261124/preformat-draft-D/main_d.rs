#[path="field_a.rs"]
mod field_d;
mod poly_d;
mod cubic_d;
mod cm_d;
use cubic_d::{k,K};
use field_d::*;
use poly_d::*;
fn on_e(x:u8,y:u8)->bool {mul(y,y)==add(add(mul(mul(x,x),x),mul(2,x)),6)}
fn weil_two(a:usize,b:usize)->u8 {if a!=0&&b!=0&&a!=b{6}else{1}}
fn actual_e2(roots:[K;3])->[usize;4] {
    let points=[None,Some(roots[0]),Some(roots[1]),Some(roots[2])];
    let mut table=[[0usize;4];4];
    for(i,&p)in points.iter().enumerate(){for(j,&q)in points.iter().enumerate(){
        let sum=match(p,q){(None,q)=>q,(p,None)=>p,(Some(x),Some(y))if x==y=>None,(Some(x),Some(y))=>{
            let slope=k(0).times(y.minus(x).inverse());let xx=slope.pow(2).minus(x).minus(y);let yy=slope.times(x.minus(xx));assert_eq!(yy,k(0));assert_eq!(xx.pow(3).plus(xx.scaled(2)).plus(k(6)),k(0));Some(xx)
        }};let label=points.iter().position(|&v|v==sum).unwrap();assert_eq!(label,i^j);table[i][j]=label;
    }}
    let frob=std::array::from_fn(|i|points.iter().position(|&v|v==points[i].map(|x|x.pow(7))).unwrap());
    println!("FULL_ACTUAL_E2_GROUP\tO-root-labels={points:?}\tactual-addition-table={table:?}\tFrob7={frob:?}\tWeil2=unique-nondegenerate-alternating-on-E2-written-bridge");frob
}
fn main(){
    println!("PROFILE\tactual-CM19-principal-Prym-F7-genus2-glue-D\tfixed-known-object\tall-scientific-standing-unsigned\tno-search-or-runtime-advantage-claim");
    let j=mul(mul(6,4),inv(add(4,mul(6,mul(6,6)))));assert!(![0,6].contains(&j));
    println!("ACTUAL_ELLIPTIC_AUTOMORPHISM_HYPOTHESIS\tE=y2-x3-2x-6\tj={j}\t1728mod7=6\tAut=plus-minus1-standard-odd-characteristic-bridge");
    let roots=cubic_d::actual_roots();let frob=actual_e2(roots);let graph=cm_d::qualify(frob);
    let pairing:Vec<Vec<u8>>=graph.iter().map(|a|graph.iter().map(|b|mul(weil_two(a[0],b[0]),weil_two(a[1],b[1]))).collect()).collect();assert!(pairing.iter().flatten().all(|&v|v==1));
    println!("FULL_ACTUAL_ORDERED_GLUE_ISOTROPY\tgraph={graph:?}\tall16-product-Weil2={pairing:?}\torder4-maximal-in-E2xE2-order16\tFrobenius-equivariant");
    let [a,b,c,d]=cubic_d::mobius(roots);let determinant=sub(mul(a,d),mul(b,c));
    let numerator=vec![b,a];let denominator=vec![d,c];let f=vec![6,2,0,1];
    let transformed=addp(&addp(&pow(&numerator,3),&scale(&times(&numerator,&pow(&denominator,2)),2)),&scale(&pow(&denominator,3),6));
    assert_eq!(transformed.len(),4);let lambda=transformed[3];assert_ne!(lambda,0);assert_eq!(transformed,scale(&f,lambda));
    println!("COMPLETE_MOBIUS_ELLIPTIC_CUBIC_IDENTITY\tA-B-C-D={:?}\tdeterminant={determinant}\tlambda={lambda}\tfull-LHS={transformed:?}\tfull-RHS={:?}",[a,b,c,d],scale(&f,lambda));
    let x=vec![sub(0,mul(d,inv(c))),0,inv(mul(lambda,c))];let h=addp(&addp(&pow(&x,3),&scale(&x,2)),&[6]);assert_eq!(h.len(),7);assert_ne!(h[0],0);
    let h_derivative=derivative(&h);let(gcd,s,t)=bezout(&h,&h_derivative);assert_eq!(gcd,vec![1]);
    let a2=mul(a,inv(c));let b2=sub(0,mul(mul(lambda,determinant),inv(c)));let l2=mul(lambda,lambda);assert_ne!(b2,0);
    let n2=vec![b2,0,a2];let second_rhs=addp(&addp(&pow(&n2,3),&scale(&times(&n2,&[0,0,0,0,1]),2)),&[0,0,0,0,0,0,6]);let l4=mul(l2,l2);assert_eq!(scale(&h,l4),second_rhs);
    println!("ACTUAL_GENUS_TWO_MODEL\tY2=H(W)\tfull-H-ascending={h:?}\tp1-x-ascending={x:?}\tp1-y=Y\tp2-x={a2}+{b2}/W2\tp2-y={l2}*Y/W3");
    println!("COMPLETE_SQUAREFREE_BEZOUT\tH={h:?}\tHprime={h_derivative:?}\ts={s:?}\tt={t:?}\tgcd={gcd:?}\tfull-sH-plus-tHprime={:?}",addp(&times(&s,&h),&times(&t,&h_derivative)));
    println!("COMPLETE_TWO_COVER_IDENTITIES\tp1-full-RHS={:?}\tp1-H={h:?}\tp2-LHS={:?}\tp2-RHS={second_rhs:?}",addp(&addp(&pow(&x,3),&scale(&x,2)),&[6]),scale(&h,l4));
    assert!(h.iter().enumerate().all(|(i,&v)|i.is_multiple_of(2)||v==0));
    for(i,alpha)in roots.into_iter().enumerate(){let r2=alpha.scaled(c).plus(k(d)).scaled(lambda);assert_ne!(r2,k(0));let image1=k(x[0]).plus(r2.scaled(x[2]));let image2=k(a2).plus(r2.inverse().scaled(b2));assert_eq!(image1,alpha);assert_eq!(image2,alpha.pow(49));let residue=h.iter().enumerate().filter(|(i,_)|i.is_multiple_of(2)).fold(k(0),|r,(i,&v)|r.plus(r2.pow(i/2).scaled(v)));assert_eq!(residue,k(0));println!("ALL_ACTUAL_WEIERSTRASS_GLUE_PACKETS\tindex={i}\tE1-root={alpha:?}\tE2-root={image2:?}\tu=W2-minus={r2:?}\tv=0\tp1-x={image1:?}\tp2-x={image2:?}\tfull-H-remainder={residue:?}\tshared-effective-pair\tbase-divisors-D0-minus-Dinfinity=divW");}
    let lead=h[6];let infinity_y:Vec<u8>=(0..7).filter(|&y|mul(y,y)==lead).collect();assert_eq!(mul(l4,lead),add(add(mul(mul(a2,a2),a2),mul(2,a2)),6));
    println!("ALL_INFINITY_COVER_EXTENSION\teta2={lead}\tF7-eta-roots={infinity_y:?}\tFrob7-eta-multiplier={}\tp1=O\tp1-pole-orders=2,3\tp2=({a2},{l2}eta)\tfull-E-equation-residual=0\tp1-unramified-local-tE-leading=-x2/eta",(0..3).fold(1,|v,_|mul(v,lead)));
    let mut finite=0;
    for w in 0..7{let value=evaluate(&h,w);let ys:Vec<u8>=(0..7).filter(|&y|mul(y,y)==value).collect();println!("COMPLETE_PROJECTIVE_BASE_FIBRE\tW={w}\tH={value}\tall-Y={ys:?}");for y in ys{finite+=1;let image1=(evaluate(&x,w),y);assert!(on_e(image1.0,image1.1));let image2=if w==0{assert_ne!(y,0);None}else{let wi=inv(w);let image=(add(a2,mul(b2,mul(wi,wi))),mul(mul(l2,y),mul(mul(wi,wi),wi)));assert!(on_e(image.0,image.1));Some(image)};println!("ALL_ACTUAL_GENUS2_RATIONAL_MAP_POINTS\tD=({w},{y})\tp1={image1:?}\tp2={image2:?}\tW0-p2-extension=proven-2,3-pole-orders");}}
    let points=finite+infinity_y.len();println!("COMPLETE_GENUS2_POINT_COUNT\tfinite={finite}\tinfinity={}\ttotal={points}\tE-square-isogeny-trace-prediction=14",infinity_y.len());assert_eq!(points,14);
    let mut mutant=h.clone();mutant[0]=add(mutant[0],1);let residual1=subp(&mutant,&addp(&addp(&pow(&x,3),&scale(&x,2)),&[6]));let residual2=subp(&scale(&mutant,l4),&second_rhs);assert!(!residual1.is_empty());assert!(!residual2.is_empty());
    println!("REAL_GENUS2_CONSTANT_MUTANT\tH0+1\tfull-p1-residual={residual1:?}\tfull-p2-residual={residual2:?}\tboth-actual-map-identities-reject\tpristine-restored");
    assert_eq!(scale(&h,l4),second_rhs);assert_eq!(addp(&times(&s,&h),&times(&t,&h_derivative)),vec![1]);
    println!("RESTORED_PRISTINE\tactual-principal-Prym-ordered-glue\texplicit-genus2-and-two-degree2-covers\tglobal-quotient-Jacobian-bridge-written-separately\tfull-Jacobian-group-engine-next");
}
