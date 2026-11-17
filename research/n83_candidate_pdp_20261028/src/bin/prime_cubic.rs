//! Exact degree-seven coefficient maps; see PRIME_CUBIC_DESIGN.md.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"]
mod donor;
#[path = "../prime_terminal_math.rs"]
mod math;
#[path = "../prime_linear_source_donor.rs"]
mod source_donor;
#[path = "../prime_cubic_bounded.rs"]
mod bounded;
use math::{add, mul, neg};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};
type Polynomial = BTreeMap<Vec<u8>, u128>;
type Row = BTreeMap<usize, u128>;

fn put<K: Ord>(a: &mut BTreeMap<K, u128>, k: K, c: u128, p: u128) {
    let c = add(*a.get(&k).unwrap_or(&0), c, p);
    if c == 0 { a.remove(&k); } else { a.insert(k, c); }
}
fn product(a: &Polynomial, b: &Polynomial, nb: usize, p: u128) -> Polynomial {
    let mut out = Polynomial::new();
    for (x, &c) in a { for (y, &d) in b {
        let mut z=x.clone();z.extend(y);z.sort_unstable();
        if z.windows(2).any(|w| w[0]==w[1] && (w[0] as usize)<nb) {continue;}
        put(&mut out,z,mul(c,d,p),p);
    }}
    out
}
fn sum(a: &Polynomial,b: &Polynomial,c:u128,p:u128)->Polynomial {
    let mut out=a.clone();for (k,&v) in b {put(&mut out,k.clone(),mul(c,v,p),p);}out
}
fn linear(indices: impl IntoIterator<Item=usize>)->Polynomial {
    indices.into_iter().map(|i|{assert!(i<256);(vec![i as u8],1)}).collect()
}
fn quartics(m:usize,ell:usize,p:u128)->Vec<Polynomial> {
    assert!((3..=9).contains(&m) && (1..=9).contains(&ell) && p>2);
    let nb=m*ell;
    let xs:Vec<_>=(0..m).map(|i|linear(i*ell..(i+1)*ell)).collect();
    (0..m-1).map(|k|{
        if k==m-2 {
            let u=linear([nb+k-1]);
            return product(&product(&u,&u,nb,p),&product(&xs[k+1],&xs[k+1],nb,p),nb,p);
        }
        let x=if k==0 {xs[0].clone()} else {linear([nb+k-1])};
        let y=&xs[k+1];let z=linear([nb+k]);
        let d=sum(&x,y,neg(1,p),p);let s=sum(&x,y,1,p);
        let xy=product(&x,y,nb,p);
        let first=product(&product(&d,&d,nb,p),&product(&z,&z,nb,p),nb,p);
        let middle=product(&product(&s,&xy,nb,p),&z,nb,p);
        sum(&sum(&first,&middle,neg(2,p),p),&product(&xy,&xy,nb,p),1,p)
    }).collect()
}
fn cubic_multipliers(nb:usize,nv:usize)->Vec<Vec<u8>> {
    assert!(nb<=nv && nv<256);let mut out=Vec::new();
    for i in 0..nv {for j in i..nv {for k in j..nv {
        if (i==j && i<nb) || (j==k && j<nb) {continue;}
        out.push(vec![i as u8,j as u8,k as u8]);
    }}}
    assert_eq!(out.len(),nv*(nv+1)*(nv+2)/6-nb*nv);out
}
fn matrix(m:usize,ell:usize,p:u128)->(usize,BTreeMap<Vec<u8>,Row>) {
    let nb=m*ell;let nv=nb+m-2;let fs=quartics(m,ell,p);
    let multipliers=cubic_multipliers(nb,nv);
    let mut rows=BTreeMap::<Vec<u8>,Row>::new();
    for (j,f) in fs.iter().enumerate() {for (q,monomial) in multipliers.iter().enumerate() {
        let factor=[(monomial.clone(),1)].into();
        for (mon,c) in product(f,&factor,nb,p) {
            assert_eq!(mon.len(),7);
            put(rows.entry(mon).or_default(),j*multipliers.len()+q,c,p);
        }
    }}
    ((m-1)*multipliers.len(),rows)
}
fn power(mut a:u128,mut e:u128,p:u128)->u128 {
    let mut r=1;while e>0 {if e&1==1{r=mul(r,a,p);}a=mul(a,a,p);e>>=1;}r
}
fn certificate(m:usize,ell:usize,p:u128)->Value {
    let (ncols,rows)=matrix(m,ell,p);
    let mut basis=BTreeMap::<usize,Row>::new();let mut pivots=Vec::new();
    for (mon,raw) in &rows {
        let mut row=raw.clone();
        loop {
            let Some((&lead,&c))=row.first_key_value() else {break};
            if let Some(old)=basis.get(&lead) {
                for (&i,&v) in old {put(&mut row,i,neg(mul(c,v,p),p),p);}
            } else {
                let inverse=power(c,p-2,p);assert_eq!(mul(c,inverse,p),1);
                pivots.push(json!({"monomial":mon,"column":lead,"pivot":c.to_string(),"original_row":raw.iter().map(|(&i,c)|json!([i,c.to_string()])).collect::<Vec<_>>()}));
                for v in row.values_mut(){*v=mul(*v,inverse,p);}
                basis.insert(lead,row);break;
            }
        }
        if basis.len()==ncols {break;}
    }
    json!({"schema":"n83.prime-cubic-template/v1","m":m,"ell_projected":ell,"multiplier_degree":3,"p":p.to_string(),"columns":ncols,"nonzero_coefficient_rows":rows.len(),"rank":basis.len(),"injective":basis.len()==ncols,"pivot_rows":pivots})
}
fn coverage()->Vec<Vec<usize>> {
    let mut sets=std::collections::BTreeSet::new();
    for i in 0..27 {for j in i..27 {for k in j..27 {
        let mut set=std::collections::BTreeSet::from([i,j,k]);
        for h in 0..27 {if set.len()==9 {break;}set.insert(h);}
        sets.insert(set.into_iter().collect::<Vec<_>>());
    }}}
    sets.into_iter().collect()
}
fn validate_coverage(c:&[Vec<usize>])->bool {
    let mut seen=vec![false;27*27*27];
    for set in c {
        if set.len()!=9 || set.windows(2).any(|w|w[0]>=w[1]) || set.iter().any(|&i|i>=27) {return false;}
        for &i in set {for &j in set {for &k in set {seen[(i*27+j)*27+k]=true;}}}
    }
    seen.iter().all(|&x|x)
}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    assert_eq!(args.len(),6,"development M ELL PRIME OUTPUT; n83 production is not enabled");
    assert_eq!(args[1],"development");
    let m=args[2].parse().unwrap();let ell=args[3].parse().unwrap();let p=args[4].parse::<u128>().unwrap();
    assert!(p<1000 && p>2 && (2..p).all(|d|p%d!=0));
    let out=Path::new(&args[5]);assert!(!out.exists());
    assert!(validate_coverage(&coverage()));
    let c=certificate(m,ell,p);fs::write(out,serde_json::to_string_pretty(&c).unwrap()+"\n").unwrap();
    println!("{}",json!({"development_only":true,"m":m,"ell":ell,"p":p.to_string(),"rank":c["rank"],"columns":c["columns"],"injective":c["injective"]}));
}
#[cfg(test)]
mod tests {
    use super::*;
    fn retain(name:&str,value:&Value){
        let root=std::env::var_os("N83_CUBIC_TEST_ARTIFACTS").map(std::path::PathBuf::from).unwrap_or_else(||std::env::temp_dir().join("n83-cubic-cap-tests"));
        let dir=root.join(format!("{}-algebra",std::process::id()));fs::create_dir_all(&dir).unwrap();
        let path=dir.join(name);assert!(!path.exists());math::save(&path,value);
    }

    fn dense_rank(mut a:Vec<Vec<u128>>,p:u128)->usize {
        let cols=a.first().map_or(0,|r|r.len());let mut rank=0;
        for j in 0..cols {
            let Some(k)=(rank..a.len()).find(|&i|a[i][j]!=0) else{continue};
            a.swap(rank,k);let inv=power(a[rank][j],p-2,p);
            for c in j..cols {a[rank][c]=mul(a[rank][c],inv,p);}
            for i in rank+1..a.len(){let s=a[i][j];if s==0{continue;}for c in j..cols{a[i][c]=(a[i][c]+p-s*a[rank][c]%p)%p;}}
            rank+=1;if rank==a.len(){break;}
        }rank
    }
    #[test]fn multiplication_rules(){
        let x=linear([0]);let u=linear([1]);
        assert!(product(&x,&x,1,101).is_empty());
        assert_eq!(product(&u,&u,1,101),[(vec![1,1],1)].into());
        assert_eq!(product(&linear([0,1]),&linear([0,1]),2,101),[(vec![0,1],2)].into());
    }
    #[test]fn covering_all_selectors(){
        assert!(validate_coverage(&coverage()));let mut c=coverage();c.pop();assert!(!validate_coverage(&c));
        let mut c=coverage();c[0][1]=0;assert!(!validate_coverage(&c));
    }
    #[test]fn small_dense_rank_agrees(){
        for ell in [1,2,3] {for p in [11,101] {
            let (nc,rows)=matrix(3,ell,p);
            let dense=rows.values().map(|r|{let mut v=vec![0;nc];for(&j,&c)in r{v[j]=c;}v}).collect();
            let c=certificate(3,ell,p);let independent_rank=dense_rank(dense,p);
            retain(&format!("dense-m3-ell{ell}-p{p}.json"),&json!({"development_only":true,"m":3,"ell":ell,"p":p.to_string(),"columns":nc,"sparse_certificate":c,"independent_dense_rank":independent_rank,"matrix_rows":rows.iter().map(|(mon,row)|json!({"monomial":mon,"entries":row.iter().map(|(&j,c)|json!([j,c.to_string()])).collect::<Vec<_>>() })).collect::<Vec<_>>()}));
            assert_eq!(c["rank"].as_u64().unwrap() as usize,independent_rank);
            assert_eq!(c["injective"],false);
        }}
    }
    #[test]fn three_selector_annihilator(){
        let fs=quartics(3,3,101);let h=sum(&linear([6]),&linear([7]),neg(1,101),101);
        assert!(product(&fs[1],&h,9,101).is_empty());
    }
    #[test]fn cubic_domain_and_triple_coverage(){
        for m in 3..=9 {
            let nb=9*m;let nv=nb+m-2;let domain=cubic_multipliers(nb,nv);
            assert_eq!(domain.len(),nv*(nv+1)*(nv+2)/6-nb*nv);
            assert!(domain.windows(2).all(|w|w[0]<w[1]));
            assert!(!domain.contains(&vec![0,0,nb as u8]));
            assert!(domain.contains(&vec![nb as u8,nb as u8,nb as u8]));
            assert!(domain.contains(&vec![0,nb as u8,nb as u8]));
        }
        // Every pair is covered, but no subset contains all of 24,25,26.
        let mut bad=std::collections::BTreeSet::new();
        for i in 0..27 {for j in i..27 {
            let mut s=std::collections::BTreeSet::from([i,j]);
            for k in 0..24 {if s.len()==9{break;}s.insert(k);}
            bad.insert(s.into_iter().collect::<Vec<_>>());
        }}
        let bad:Vec<_>=bad.into_iter().collect();
        assert!((0..27).all(|i|(0..27).all(|j|bad.iter().any(|s|s.contains(&i)&&s.contains(&j)))));
        assert!(!validate_coverage(&bad));assert!(validate_coverage(&coverage()));
    }
    #[test]fn seven_selector_cubic_annihilator(){
        let fs=quartics(3,7,101);let mut h=Polynomial::from([(vec![],1)]);
        for i in [14,16,18] {
            let d=sum(&linear([i]),&linear([i+1]),neg(1,101),101);h=product(&h,&d,21,101);
        }
        let result=product(&fs[1],&h,21,101);
        retain("seven-selector-annihilator.json",&json!({"development_only":true,"p":"101","terms":h.iter().map(|(mon,c)|json!([mon,c.to_string()])).collect::<Vec<_>>(),"product_zero":result.is_empty()}));
        assert_eq!(h.len(),8);assert!(result.is_empty());
    }
    #[test]fn independent_terminal_positive_control(){
        // Direct squarefree union and coefficient2, independent of quartics/product.
        for p in [11,101] {
            let columns=cubic_multipliers(8,8);assert_eq!(columns.len(),56);
            let mut rows=BTreeMap::<Vec<u8>,Vec<u128>>::new();
            for (j,q) in columns.iter().enumerate(){for a in 0u8..8{for b in a+1..8{
                if q.contains(&a)||q.contains(&b){continue;}
                let mut mon=q.clone();mon.extend([a,b]);mon.sort_unstable();
                rows.entry(mon).or_insert_with(||vec![0;56])[j]=2;
            }}}
            let rank=dense_rank(rows.values().cloned().collect(),p);
            retain(&format!("terminal-ell8-p{p}.json"),&json!({"development_only":true,"p":p.to_string(),"columns":columns,"rows":rows.iter().map(|(mon,row)|json!({"monomial":mon,"coefficients":row})).collect::<Vec<_>>(),"rank":rank}));
            assert_eq!(rows.len(),56);assert_eq!(rank,56);
        }
    }
    #[test]fn five_selector_quadratic_annihilator(){
        let fs=quartics(3,5,101);
        let a=sum(&linear([10]),&linear([11]),neg(1,101),101);
        let b=sum(&linear([12]),&linear([13]),neg(1,101),101);
        let h=product(&a,&b,15,101);assert_eq!(h.len(),4);
        assert!(product(&fs[1],&h,15,101).is_empty());
    }
    #[test]fn independent_six_term_expansion(){
        // Direct four-fold enumeration, using the six symmetric terms. This
        // path does not call product(), sum(), or the factored S3 builder.
        for m in [3,4] {for ell in [3,7,9] {for p in [11,101] {
            let nb=m*ell;let got=quartics(m,ell,p);
            for k in 0..m-1 {
                let x:Vec<_>=if k==0 {(0..ell).collect()}else{vec![nb+k-1]};
                let y:Vec<_>=((k+1)*ell..(k+2)*ell).collect();
                let z=vec![nb+k];let mut expected=Polynomial::new();
                let terms=if k==m-2 {vec![(&x,&x,&y,&y,1)]}else{
                    vec![(&x,&x,&y,&y,1),(&x,&x,&z,&z,1),(&y,&y,&z,&z,1),
                         (&x,&x,&y,&z,p-2),(&x,&y,&y,&z,p-2),(&x,&y,&z,&z,p-2)]};
                for (a,b,c,d,coefficient) in terms {
                    for &i in a {for &j in b {for &h in c {for &l in d {
                        let mut mon=vec![i as u8,j as u8,h as u8,l as u8];mon.sort_unstable();
                        if mon.windows(2).any(|v|v[0]==v[1] && (v[0] as usize)<nb){continue;}
                        let value=(expected.get(&mon).copied().unwrap_or(0)+coefficient)%p;
                        if value==0{expected.remove(&mon);}else{expected.insert(mon,value);}
                    }}}}
                }
                assert_eq!(got[k],expected,"m={m},ell={ell},p={p},k={k}");
            }
        }}}
    }
}
