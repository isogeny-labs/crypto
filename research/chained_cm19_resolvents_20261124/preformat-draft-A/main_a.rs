mod field_a;
mod poly_a;
use field_a::*;
use poly_a::*;

const INPUT:&str=include_str!("../chained_cm19_fibres_20261124/first-E-native/actual-functions.stdout");
fn field<'a>(line:&'a str,prefix:&str)->&'a str {
    line.split('\t').find_map(|s|s.strip_prefix(prefix)).unwrap()
}
fn parse_poly(raw:&str,degree:usize)->Poly {
    let numbers:Vec<usize>=raw.split(|c:char|!c.is_ascii_digit()).filter(|s|!s.is_empty()).map(|s|s.parse().unwrap()).collect();
    assert_eq!(numbers.len()%3,0);
    let mut p=Poly::zero();
    for chunk in numbers.as_chunks::<3>().0 {
        assert!(chunk[0]+chunk[1]<=degree && chunk[2]>0 && chunk[2]<7);
        assert!(p.0.insert((chunk[0],chunk[1]),u8::try_from(chunk[2]).unwrap()).is_none());
    }
    assert!(!p.0.is_empty()); p
}
fn qualified_functions()->(Poly,Poly,Poly) {
    let lookup=|prefix:&str| INPUT.lines().find(|s|s.starts_with(prefix)).unwrap();
    let q=parse_poly(field(lookup("ACTUAL_DENOMINATOR_CONIC\t"),"terms-Y-Z="),2);
    let p=parse_poly(field(lookup("ACTUAL_NUMERATOR\tdegree=4\t"),"terms-Y-Z="),4);
    let r=parse_poly(field(lookup("ACTUAL_NUMERATOR\tdegree=6\t"),"terms-Y-Z="),6);
    let identity=lookup("COMPLETE_ELLIPTIC_POLYNOMIAL_IDENTITY\t");
    assert_eq!(field(identity,"remainder="),"{}");
    assert_eq!(parse_poly(field(identity,"curve="),4),actual_curve());
    (q,p,r)
}
fn coordinates(p:&Poly,basis:&[(usize,usize)])->Vec<u8> {
    assert!(p.0.keys().all(|key|basis.contains(key)));
    basis.iter().map(|key|p.0.get(key).copied().unwrap_or(0)).collect()
}
fn matrix(columns:&[Poly],rhs:&Poly,basis:&[(usize,usize)])->Vec<Vec<u8>> {
    let columns:Vec<Vec<u8>>=columns.iter().map(|p|coordinates(p,basis)).collect();
    let rhs=coordinates(rhs,basis);
    basis.iter().enumerate().map(|(row,_)|columns.iter().map(|c|c[row]).chain(std::iter::once(rhs[row])).collect()).collect()
}
fn solve(name:&str,original:Vec<Vec<u8>>,variables:usize)->(Vec<u8>,Vec<Vec<u8>>,usize) {
    assert!(original.iter().all(|r|r.len()==variables+1));
    let mut m=original.clone(); let mut pivots=Vec::new(); let mut row=0;
    for column in 0..variables {
        let Some(pivot)=(row..m.len()).find(|&r|m[r][column]!=0) else { continue; };
        m.swap(row,pivot);
        let scale=inv(m[row][column]);
        for value in &mut m[row] { *value=mul(*value,scale); }
        let normalized=m[row].clone();
        for (i,other) in m.iter_mut().enumerate() {
            if i!=row {
                let c=other[column];
                for (value,&p) in other.iter_mut().zip(&normalized) { *value=sub(*value,mul(c,p)); }
            }
        }
        pivots.push(column); row+=1;
    }
    let consistent=m.iter().all(|r|r[..variables].iter().any(|&c|c!=0)||r[variables]==0);
    println!("EXACT_LINEAR_CERTIFICATE\t{name}\tvariables={variables}\tinput={original:?}\trank={}\tpivots={pivots:?}\trref={m:?}\tconsistent={consistent}",pivots.len());
    assert!(consistent);
    let mut solution=vec![0;variables];
    for (row,&column) in pivots.iter().enumerate() { solution[column]=m[row][variables]; }
    let kernel:Vec<Vec<u8>>=(0..variables).filter(|c|!pivots.contains(c)).map(|free| {
        let mut v=vec![0;variables]; v[free]=1;
        for (row,&column) in pivots.iter().enumerate() { v[column]=sub(0,m[row][free]); } v
    }).collect();
    let residual:Vec<u8>=original.iter().map(|r|sub(r[..variables].iter().zip(&solution).fold(0,|v,(&a,&b)|add(v,mul(a,b))),r[variables])).collect();
    assert!(residual.iter().all(|&c|c==0));
    for v in &kernel {
        assert!(original.iter().all(|r|r[..variables].iter().zip(v).fold(0,|s,(&a,&b)|add(s,mul(a,b)))==0));
    }
    println!("LINEAR_SOLUTION\t{name}\tparticular={solution:?}\tkernel={kernel:?}\tall-row-residuals={residual:?}");
    (solution,kernel,pivots.len())
}
fn full_identity(name:&str,p:&Poly,curve:&Poly) {
    let (quotient,remainder)=p.divide_curve(curve);
    println!("COMPLETE_CURVE_IDENTITY\t{name}\tpolynomial={:?}\tquotient={:?}\tremainder={:?}",p.0,quotient.0,remainder.0);
    assert!(remainder.0.is_empty());
}
fn main() {
    println!("PROFILE\tactual-CM19-norm-primitive-quartic-A\tfixed-known-object\tall-claims-unsigned\tno-performance-claim");
    let (q,p,r)=qualified_functions(); let curve=actual_curve();
    let b=p.minus(&q.power(2).scaled(3));
    let a=p.power(2).scaled(3).plus(&p.times(&q.power(2)).scaled(4))
        .plus(&q.power(4).scaled(4)).plus(&r.times(&q).scaled(4));
    let n18=r.minus(&q.power(3).scaled(5)).times(&p.times(&b.power(2)).minus(&a.times(&q.power(2))))
        .minus(&r.times(&b.power(3)));
    println!("ACTUAL_TRANSLATION_NUMERATORS\tb-degree4={:?}\tA-degree8={:?}\tN-degree18={:?}",b.0,a.0,n18.0);
    let basis12=normal_basis(12); let basis18=normal_basis(18);
    assert_eq!((basis12.len(),basis18.len()),(46,70));
    let columns:Vec<Poly>=basis12.iter().map(|&(y,z)|q.power(3).times(&Poly::term(y,z,1)).divide_curve(&curve).1).collect();
    let rhs=n18.divide_curve(&curve).1;
    let (coeff,kernel,rank)=solve("translation-B12",matrix(&columns,&rhs,&basis18),46);
    assert_eq!(rank,46); assert!(kernel.is_empty());
    let b12=Poly::from_coefficients(&basis12,&coeff);
    println!("ACTUAL_UNTRANSLATED_V_NUMERATOR\tdegree=12\tterms-Y-Z={:?}\tdenominator=b^3",b12.0);
    full_identity("true-addition-v",&n18.minus(&q.power(3).times(&b12)),&curve);
    full_identity("true-addition-elliptic-equation",&b12.power(2).minus(&a.power(3))
        .minus(&a.times(&b.power(4)).scaled(2)).minus(&b.power(6).scaled(6)),&curve);
    let sections=[a.minus(&b.power(2).scaled(5)).times(&b.power(3)),
        b12.minus(&b.power(3).scaled(6)).times(&b.power(2)),
        a.power(2).minus(&b.power(4).scaled(4)).times(&b),
        a.times(&b12).minus(&b.power(5).scaled(2))];
    let columns:Vec<Poly>=(0..=4).flat_map(|i|sections.iter().map(move |s|s.times(&Poly::term(i,0,1)))).collect();
    let normal:Vec<Poly>=columns.iter().map(|p|p.divide_curve(&curve).1).collect();
    let basis24=normal_basis(24); assert_eq!(basis24.len(),94);
    let (particular,kernel,rank)=solve("actual-norm-quartic",matrix(&normal,&Poly::zero(),&basis24),20);
    assert!(particular.iter().all(|&c|c==0)); assert_eq!(rank,19); assert_eq!(kernel.len(),1);
    let scale=inv(*kernel[0].iter().find(|&&c|c!=0).unwrap());
    let coefficients:Vec<u8>=kernel[0].iter().map(|&c|mul(c,scale)).collect();
    assert!(coefficients[16..].iter().any(|&c|c!=0));
    for (i,s) in coefficients.as_chunks::<4>().0.iter().enumerate() {
        println!("ACTUAL_PRIMITIVE_QUARTIC_COEFFICIENT\tT-power={i}\tbasis=u-5,v-6,u^2-4,u*v-2\tvector={s:?}");
    }
    let identity=columns.iter().zip(&coefficients).fold(Poly::zero(),|p,(c,&s)|p.plus(&c.scaled(s)));
    full_identity("actual-primitive-quartic",&identity,&curve);
    let mut mutant=coefficients.clone(); mutant[0]=add(mutant[0],1);
    let bad=columns.iter().zip(&mutant).fold(Poly::zero(),|p,(c,&s)|p.plus(&c.scaled(s)));
    let bad_remainder=bad.divide_curve(&curve).1;
    println!("REAL_COEFFICIENT_MUTANT\tindex=0\told={}\tnew={}\tfull-remainder={:?}",coefficients[0],mutant[0],bad_remainder.0);
    assert!(!bad_remainder.0.is_empty());
    let restored=columns.iter().zip(&coefficients).fold(Poly::zero(),|p,(c,&s)|p.plus(&c.scaled(s)));
    assert_eq!(restored,identity); assert!(restored.divide_curve(&curve).1.0.is_empty());
    println!("RESTORED_PRISTINE\ttrue-translation-and-quartic-identities=0\tgeometry=separate-norm/primitive/normalization-proof\tunsigned");
}
