mod elliptic_a;
mod geometry_a;
use elliptic_a::*;
use geometry_a::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

const FIXTURE: &str = include_str!("../chained_genus3_quartic_20261123/cm19_f7/first-B-native/quartics.stdout");

fn original_points(f: &Form) -> Vec<Point> {
    let line = FIXTURE.lines().find(|line| line.starts_with("COUNT\tq=7\tkappa=1\t")).unwrap();
    let encoded = line.split("pointsF7=").nth(1).unwrap();
    let coordinates: Vec<u8> = encoded.bytes().filter(u8::is_ascii_digit).map(|x| x - b'0').collect();
    assert_eq!(coordinates.len(), 51);
    let mut points = Vec::new();
    for row in coordinates.as_chunks::<3>().0 {
        assert!(row.iter().all(|&x| x < 7));
        let point = canonical(*row);
        assert_eq!(evaluate(f, point), 0);
        assert!(!points.contains(&point));
        points.push(point);
    }
    points.sort();
    assert_eq!(points.len(), 17);
    println!("FROZEN_SOURCE_POINTS\t{points:?}");
    points
}

fn frame(points: &[Point]) -> [Point; 4] {
    for &a in points {
        for &b in points {
            if a == b { continue; }
            for &c in points {
                if a == c || b == c { continue; }
                for &d in points {
                    if a == d || b == d || c == d { continue; }
                    let candidate = [a,b,c,d];
                    if frame_basis(candidate).is_some() {
                        return candidate;
                    }
                }
            }
        }
    }
    panic!("fixed17-point population contains no projective frame")
}

fn permutation(a: Matrix, points: &[Point]) -> Option<Vec<usize>> {
    let mut out = Vec::new();
    for &point in points {
        let image = canonical(mv(a, point));
        let index = points.binary_search(&image).ok()?;
        out.push(index);
    }
    assert_eq!(out.iter().copied().collect::<BTreeSet<_>>().len(), points.len());
    Some(out)
}

fn enumerate(f: &Form, points: &[Point]) -> Vec<Matrix> {
    let source = frame(points);
    let source_inverse = inverse(frame_basis(source).unwrap()).unwrap();
    println!("FIXED_FRAME\t{source:?}\tbasis_inverse={source_inverse:?}");
    let mut codec = vec![0u8; 14280];
    let mut counts = [0usize; 4];
    let mut index = 0;
    let mut accepted = Vec::new();
    for (i, &a) in points.iter().enumerate() {
        for (j, &b) in points.iter().enumerate() {
            if i == j { continue; }
            for (k, &c) in points.iter().enumerate() {
                if k == i || k == j { continue; }
                for (l, &d) in points.iter().enumerate() {
                    if l == i || l == j || l == k { continue; }
                    let images = [a,b,c,d];
                    let code = if let Some(target) = frame_basis(images) {
                        let matrix = canonical_matrix(mm(target, source_inverse));
                        assert_ne!(det(matrix), 0);
                        for (&p, &q) in source.iter().zip(&images) {
                            assert_eq!(canonical(mv(matrix, p)), q);
                        }
                        if permutation(matrix, points).is_some() {
                            let transformed = transform(f, matrix);
                            if let Some(scale) = proportional(&transformed, f) {
                                println!("AUTOMORPHISM_CANDIDATE\tM={matrix:?}\tlambda={scale}\tcoefficients={:?}\tpermutation={:?}", coefficients(&transformed), permutation(matrix, points).unwrap());
                                accepted.push(matrix);
                                3
                            } else {
                                println!("POINT_SET_ONLY_REJECTION\tM={matrix:?}\tcoefficients={:?}", coefficients(&transformed));
                                2
                            }
                        } else { 1 }
                    } else { 0 };
                    assert!(index < 57120);
                    codec[index / 4] |= code << (2 * (index % 4));
                    counts[code as usize] += 1;
                    index += 1;
                }
            }
        }
    }
    assert_eq!(index, 57120);
    assert_eq!(counts[2], 0, "17-point Bezout consistency");
    accepted.sort();
    let count = accepted.len();
    accepted.dedup();
    assert_eq!(count, accepted.len());
    assert!(accepted.contains(&ID));
    assert!(accepted.contains(&DECK));
    let mut hex = String::with_capacity(28560);
    for byte in codec {
        write!(&mut hex, "{byte:02x}").unwrap();
    }
    assert_eq!(hex.len(), 28560);
    println!("FRAME_CODEC_HEX\t{hex}");
    println!("FRAME_SUMMARY\tdecisions={index}\tcodes0_1_2_3={counts:?}\taccepted={}", accepted.len());
    accepted
}

fn order(a: Matrix, limit: usize) -> usize {
    let mut power = ID;
    for n in 1..=limit {
        power = canonical_matrix(mm(power, a));
        if power == ID {
            return n;
        }
    }
    panic!("accepted matrix group is not closed")
}

fn group(automorphisms: &[Matrix]) -> Vec<usize> {
    let mut orders = Vec::new();
    for (i, &a) in automorphisms.iter().enumerate() {
        let inverse = canonical_matrix(inverse(a).unwrap());
        assert!(automorphisms.contains(&inverse));
        let row: Vec<_> = automorphisms.iter().map(|&b| {
            let product = canonical_matrix(mm(a,b));
            automorphisms.binary_search(&product).unwrap()
        }).collect();
        let ord = order(a, automorphisms.len());
        orders.push(ord);
        println!("AUT_GROUP\tid={i}\tM={a:?}\torder={ord}\tmultiplication={row:?}");
    }
    orders
}

fn linear(row: Point) -> Form {
    row.into_iter().enumerate().fold(Form::new(), |out, (i, value)| {
        let mut key = [0;3];key[i]=1;
        fadd(&out,&term(key,value))
    })
}

fn cover_identity(f: &Form, a: Matrix) {
    let x=linear(a[0]);let y=linear(a[1]);let z=linear(a[2]);
    let g=fadd(&fadd(&fscale(&fpow(&x,2),6),&fscale(&fmul(&x,&y),3)),&fpow(&y,2));
    let v=fadd(&fpow(&z,2),&g);
    let d=fmul(&y,&fadd(&fadd(&fpow(&x,3),&fscale(&fmul(&x,&fpow(&y,2)),2)),&fscale(&fpow(&y,3),6)));
    assert_eq!(fsub(&fpow(&v,2),&d),transform(f,a));
    assert!(proportional(&transform(f,a),f).is_some());
}

fn quadratic_covers(f: &Form, points: &[Point], automorphisms: &[Matrix], orders: &[usize]) -> usize {
    let mut count=0;
    for (index,&involution) in automorphisms.iter().enumerate() {
        if orders[index]!=2 {continue;}
        let conjugator=automorphisms.iter().copied().find(|&a| canonical_matrix(mm(mm(inverse(a).unwrap(),DECK),a))==involution);
        let Some(a)=conjugator else {
            println!("NONCONJUGATE_INVOLUTION\tid={index}\t{involution:?}");continue;
        };
        cover_identity(f,a);
        let mut fibres=vec![Vec::new();11];
        for (pindex,&point) in points.iter().enumerate() {
            let image=cover(mv(a,point));
            let changed=cover(mv(a,mv(involution,point)));
            assert_eq!(image,changed);
            let target=E7_POINTS.iter().position(|&e| e==image).unwrap();
            fibres[target].push(pindex);
            println!("DEGREE2_POINT\tinvolution={index}\tsource={point:?}\tE7={image:?}");
        }
        let sizes:Vec<_>=fibres.iter().map(Vec::len).collect();
        assert_eq!(sizes.iter().sum::<usize>(),17);
        assert_eq!(sizes.iter().filter(|&&n|n==0).count(),2);
        assert_eq!(sizes.iter().filter(|&&n|n==1).count(),1);
        assert_eq!(sizes.iter().filter(|&&n|n==2).count(),8);
        println!("DEGREE2_COVER\tinvolution={index}\tconjugator={a:?}\tfibres={fibres:?}\tfull15coeff=PASS");
        count+=1;
    }
    for point in [[1,0,1],[1,0,6]] {
        assert!(points.contains(&point));
        assert_eq!(cover(point),None);
    }
    assert!(count>0);
    count
}

fn ambient_points() -> Vec<Point> {
    let mut points=BTreeSet::new();
    for x in 0..7 {
        for y in 0..7 {
            for z in 0..7 {
                if [x,y,z]!=[0,0,0] {points.insert(canonical([x,y,z]));}
            }
        }
    }
    assert_eq!(points.len(),57);
    points.into_iter().collect()
}

fn eigenbasis(f:&Form, a:Matrix)->(Matrix,Form,[u8;5]) {
    let cube=mm(mm(a,a),a);
    let scalar=cube[0][0];
    assert_ne!(scalar,0);
    assert_eq!(cube,mscale(ID,scalar));
    let scale=(1..7).find(|&t|mul(pow(t,3),scalar)==1).expect("F7 order3 lift");
    let lifted=mscale(a,scale);
    let mut eigen=BTreeMap::new();
    for lambda in [1,2,4] {
        let vectors:Vec<_>=ambient_points().into_iter().filter(|&p|mv(lifted,p)==p.map(|x|mul(x,lambda))).collect();
        assert_eq!(vectors.len(),1);
        eigen.insert(lambda,vectors[0]);
    }
    let off:Vec<_>=eigen.iter().filter(|(_,p)|evaluate(f,**p)!=0).collect();
    assert_eq!(off.len(),1);
    let lambda=*off[0].0;
    let columns=[eigen[&lambda],eigen[&mul(2,lambda)],eigen[&mul(4,lambda)]];
    let basis=std::array::from_fn(|i|[columns[0][i],columns[1][i],columns[2][i]]);
    let transformed=transform(f,basis);
    let leading=*transformed.get(&[4,0,0]).unwrap();
    let normalized=fscale(&transformed,inv(leading));
    let keys=[[4,0,0],[2,1,1],[1,3,0],[1,0,3],[0,2,2]];
    for key in normalized.keys() {assert!(keys.contains(key));}
    let abcde=keys.map(|key|*normalized.get(&key).unwrap_or(&0));
    assert_eq!(abcde[0],1);
    assert!(abcde[2..].iter().all(|&x|x!=0));
    assert_eq!(transform(&normalized,[[1,0,0],[0,2,0],[0,0,4]]),normalized);
    assert_ne!(det(basis),0);
    println!("ORDER3_NORMAL_FORM\tM={a:?}\tlifted={lifted:?}\teigenlines={eigen:?}\tbasis={basis:?}\tABCDEF={abcde:?}\tall15={:?}",coefficients(&normalized));
    (basis,normalized,abcde)
}

fn quotient(point:Point, abcde:[u8;5], basis_inverse:Matrix)->QPoint {
    let [x,y,z]=mv(basis_inverse,point);
    let [a,b,c,_d,e]=abcde;
    if x==0 {
        assert!(y==0 || z==0);
        if y!=0 {return QPoint::Infinity(sub(0,e));}
        return QPoint::Infinity(e);
    }
    let u=mul(mul(y,z),inv(pow(x,2)));
    let v=mul(pow(y,3),inv(pow(x,3)));
    let w=add(mul(mul(2,c),v),add(add(a,mul(b,u)),mul(e,pow(u,2))));
    QPoint::Finite(u,w)
}

fn cubic_quotient(f:&Form, points:&[Point], a:Matrix) {
    let (basis,normal,abcde)=eigenbasis(f,a);
    let basis_inverse=inverse(basis).unwrap();
    let model=QuarticModel::new(abcde[0],abcde[1],abcde[2],abcde[3],abcde[4]);
    let x=term([1,0,0],1);let y=term([0,1,0],1);let z=term([0,0,1],1);
    let numerator=fadd(&fadd(&fadd(&fpow(&x,4),&fscale(&fmul(&fpow(&x,2),&fmul(&y,&z)),abcde[1])),&fscale(&fmul(&fpow(&y,2),&fpow(&z,2)),abcde[4])),&fscale(&fmul(&x,&fpow(&y,3)),mul(2,abcde[2])));
    let homogenized=model.coefficients.into_iter().enumerate().fold(Form::new(),|out,(i,c)|fadd(&out,&fscale(&fmul(&fpow(&fmul(&y,&z),i as u8),&fpow(&x,8-2*i as u8)),c)));
    let expected=fscale(&fmul(&fmul(&x,&fpow(&y,3)),&normal),mul(4,abcde[2]));
    assert_eq!(fsub(&fpow(&numerator,2),&homogenized),expected);
    let altered=fadd(&homogenized,&fpow(&x,8));
    assert_ne!(fsub(&fpow(&numerator,2),&altered),expected);
    assert_eq!(fsub(&fpow(&numerator,2),&homogenized),expected);
    println!("DEGREE3_FUNCTION_FIELD\tfull homogeneous degree8 identity and actual constant mutation\tPASS");
    println!("QUOTIENT_MODEL_PARAMETERS\t{model:?}");
    model.check_all();
    assert!(!squarefree([0,0,1,0,0]));
    assert!(squarefree(model.coefficients));
    let mut fibres=vec![Vec::new();11];
    let mut fixed=0;
    for (index,&point) in points.iter().enumerate() {
        assert_eq!(evaluate(&normal,mv(basis_inverse,point)),0);
        let image=quotient(point,abcde,basis_inverse);
        assert!(model.valid(image));
        assert_eq!(image,quotient(canonical(mv(a,point)),abcde,basis_inverse));
        let is_fixed=canonical(mv(a,point))==point;
        fixed+=usize::from(is_fixed);
        assert_eq!(matches!(image,QPoint::Infinity(_)),is_fixed);
        let elliptic=model.to_e(image);
        assert_eq!(model.inverse_e(elliptic),image);
        let target=E7_POINTS.iter().position(|&p|p==elliptic).unwrap();
        fibres[target].push(index);
        println!("DEGREE3_POINT\tsource={point:?}\tquartic={image:?}\tE7={elliptic:?}\tfixed={is_fixed}\tinvariant/inverse=PASS");
    }
    let sizes:Vec<_>=fibres.iter().map(Vec::len).collect();
    assert_eq!(fixed,2);
    assert_eq!(sizes.iter().sum::<usize>(),17);
    assert_eq!(sizes.iter().filter(|&&n|n==0).count(),4);
    assert_eq!(sizes.iter().filter(|&&n|n==1).count(),2);
    assert_eq!(sizes.iter().filter(|&&n|n==3).count(),5);
    println!("DEGREE3_FIBRES\t{fibres:?}\ttwo actual ramification/infinity points\tPASS");
}

fn main() {
    println!("PROFILE\tactual-CM19-quartic-curve-maps-A\tunsigned\tno actual J4/cost claim");
    let f=curve();
    println!("SOURCE_FORM\t{f:?}\tfull15={:?}",coefficients(&f));
    let points=original_points(&f);
    check_group();
    assert!(proportional(&transform(&f,ID),&f).is_some());
    assert!(proportional(&transform(&f,DECK),&f).is_some());
    assert!(inverse([[1,0,0],[0,1,0],[0,0,0]]).is_none());
    assert!(proportional(&transform(&f,[[1,0,0],[0,1,0],[0,0,2]]),&f).is_none());
    assert!(proportional(&transform(&f,ID),&f).is_some());
    let automorphisms=enumerate(&f,&points);
    let orders=group(&automorphisms);
    let covers=quadratic_covers(&f,&points,&automorphisms,&orders);
    let cubic=automorphisms.iter().zip(&orders).find(|(_,order)|**order==3).map(|(&a,_)|a);
    if let Some(a)=cubic {
        cubic_quotient(&f,&points,a);
    } else {
        println!("ORDER3_QUOTIENT\tEMPTY_FIXED_AUTOMORPHISM_POPULATION\tno retuning");
    }
    println!("SUMMARY\tframe_decisions=57120\tautomorphisms={}\torders={orders:?}\tdegree2_covers={covers}\tdegree3_quotient={}\tactual_source_curve_maps=PASS\tCM19_alpha_divisor_S_maps=OPEN\tDLP_cost=UNMEASURED",automorphisms.len(),cubic.is_some());
}
