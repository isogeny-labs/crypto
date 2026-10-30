//! Exact degree-six coefficient maps; see PRIME_QUADRATIC_DESIGN.md.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"]
mod donor;
#[path = "../prime_terminal_math.rs"]
mod math;
#[path = "../prime_linear_source_donor.rs"]
mod source_donor;
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
    assert!((3..=9).contains(&m) && (1..=7).contains(&ell) && p>2);
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
fn quadratic_multipliers(nb:usize,nv:usize)->Vec<Vec<u8>> {
    let mut out=Vec::new();
    for i in 0..nv {for j in i..nv {
        if i==j && i<nb {continue;}
        out.push(vec![i as u8,j as u8]);
    }}
    assert_eq!(out.len(),nv*(nv+1)/2-nb);out
}
fn matrix(m:usize,ell:usize,p:u128)->(usize,BTreeMap<Vec<u8>,Row>) {
    let nb=m*ell;let nv=nb+m-2;let fs=quartics(m,ell,p);
    let multipliers=quadratic_multipliers(nb,nv);
    let mut rows=BTreeMap::<Vec<u8>,Row>::new();
    for (j,f) in fs.iter().enumerate() {for (q,monomial) in multipliers.iter().enumerate() {
        let factor=[(monomial.clone(),1)].into();
        for (mon,c) in product(f,&factor,nb,p) {
            assert_eq!(mon.len(),6);
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
    json!({"schema":"n83.prime-quadratic-template/v1","m":m,"ell_projected":ell,"multiplier_degree":2,"p":p.to_string(),"columns":ncols,"nonzero_coefficient_rows":rows.len(),"rank":basis.len(),"injective":basis.len()==ncols,"pivot_rows":pivots})
}
fn coverage()->Vec<Vec<usize>> {
    let mut sets=std::collections::BTreeSet::new();
    for i in 0..27 {for j in i..27 {
        let mut set=std::collections::BTreeSet::from([i,j]);
        for k in 0..27 {if set.len()==7 {break;}set.insert(k);}
        sets.insert(set.into_iter().collect::<Vec<_>>());
    }}
    sets.into_iter().collect()
}
fn validate_coverage(c:&[Vec<usize>])->bool {
    let mut seen=[[false;27];27];
    for set in c {
        if set.len()!=7 || set.windows(2).any(|w|w[0]>=w[1]) || set.iter().any(|&i|i>=27) {return false;}
        for &i in set {for &j in set {seen[i][j]=true;}}
    }
    seen.iter().flatten().all(|&x|x)
}
fn panel(root:&Path,protocol:&Path,out:&Path,replay:bool){
    let protocol=math::read(protocol);
    let cases=protocol["extensions"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(),42);
    let mut expected=std::collections::BTreeSet::new();
    for arm in ["original","baseline","spike40"]{for m in 3..=9{for target in ["planted","natural"]{
        expected.insert(format!("{arm}-prime-m{m}-{target}"));
    }}}
    let mut keys=std::collections::BTreeSet::new();
    for case in cases {assert!(keys.insert(case["key"].as_str().unwrap().to_owned()));}
    assert_eq!(keys,expected);
    if replay{assert!(out.is_dir());}else{assert!(!out.exists());fs::create_dir_all(out).unwrap();}
    let persist=|name:&str,value:&Value|{
        let path=out.join(name);
        if replay {assert_eq!(&math::read(&path),value,"replay {name}");}else{math::save(&path,value);}
    };
    let pc=donor::primality();persist("primality.json",&pc);
    let p=donor::field();let cover=coverage();assert!(validate_coverage(&cover));
    let mut templates=BTreeMap::new();
    for m in 3..=9{
        let c=certificate(m,7,p);persist(&format!("template-m{m}.json"),&c);templates.insert(m,c);
    }
    let mut rows=Vec::new();
    for case in cases {
        let key=case["key"].as_str().unwrap();let arm=case["arm"].as_str().unwrap();
        let dir=root.join("inputs").join(key);
        let bytes=fs::read(dir.join("input.json")).unwrap();
        assert_eq!(case["input_sha256"],math::sha(&bytes));
        let input:Value=serde_json::from_slice(&bytes).unwrap();
        let source=fs::read_to_string(dir.join("system.sing")).unwrap();
        assert_eq!(case["source_sha256"],math::sha(source.as_bytes()));
        assert_eq!(case["factor_base_manifest_sha256"],input["factor_base_manifest_sha256"]);
        let checked=source_donor::source(&input,&source,arm);
        let m=input["m"].as_u64().unwrap() as usize;
        assert_eq!(key,format!("{arm}-prime-m{m}-{}",input["target_kind"].as_str().unwrap()));
        let weights=checked["weights"].as_array().unwrap();
        let inverses:Vec<_>=weights.iter().map(|w|{
            let w=w.as_str().unwrap().parse::<u128>().unwrap();let inverse=power(w,p-2,p);
            assert!(w>0 && w<p);assert_eq!(mul(w,inverse,p),1);inverse.to_string()
        }).collect();
        let c=&templates[&m];let injective=c["injective"].as_bool().unwrap();
        let bound=if injective{Some(7)}else{None::<u64>};
        let cert=json!({"schema":"n83.prime-quadratic-case/v1","key":key,"m":m,"arm":arm,"source":checked,"source_sha256":case["source_sha256"],"input_sha256":case["input_sha256"],"factor_base_manifest_sha256":case["factor_base_manifest_sha256"],"normalization_inverses":inverses,"covering_subsets":cover,"template":format!("template-m{m}.json"),"template_sha256":math::sha(&fs::read(out.join(format!("template-m{m}.json"))).unwrap()),"first_fall_lower_bound":bound,"last_fall_lower_bound":null,"exact_first_fall":null,"interpretation":if injective{"degree-six quadratic injectivity plus pair-covering projection argument"}else{"projection inconclusive; no original-system kernel claimed"}});
        persist(&format!("{key}.json"),&cert);
        rows.push(json!({"key":key,"m":m,"arm":arm,"target_kind":input["target_kind"],"first_fall_lower_bound":bound,"projected_rank":c["rank"],"projected_columns":c["columns"],"source_sha256":case["source_sha256"],"factor_base_manifest_sha256":case["factor_base_manifest_sha256"]}));
    }
    persist("summary.json",&json!({"schema":"n83.prime-quadratic-summary/v1","rows":rows,"shared_template_count":7,"coverage":cover,"new_lfd_claim":false,"independent_review":"pending"}));
    println!("{}",json!({"cases_verified":rows.len(),"shared_templates":7,"replay":replay}));
}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    if args.get(1).map(String::as_str)==Some("run") || args.get(1).map(String::as_str)==Some("replay"){
        assert!(std::env::var("N83_QUADRATIC_FROZEN_PRODUCTION").as_deref()==Ok("1"),"Production not enabled until a new protocol is frozen");
        assert_eq!(args.len(),5,"run|replay INPUT_ROOT PROTOCOL OUTPUT");
        panel(Path::new(&args[2]),Path::new(&args[3]),Path::new(&args[4]),args[1]=="replay");return;
    }
    // Explicit small-prime development mode cannot impersonate n83 results.
    assert_eq!(args.len(),6,"development M ELL PRIME OUTPUT");
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
        for ell in [3,5,7] {for p in [11,101] {
            let (nc,rows)=matrix(3,ell,p);
            let dense=rows.values().map(|r|{let mut v=vec![0;nc];for(&j,&c)in r{v[j]=c;}v}).collect();
            let c=certificate(3,ell,p);assert!(p<=101);assert_eq!(c["rank"].as_u64().unwrap() as usize,dense_rank(dense,p));
            if ell<=5 {assert_eq!(c["injective"],false);}else{assert_eq!(c["injective"],true);}
        }}
    }
    #[test]fn three_selector_annihilator(){
        let fs=quartics(3,3,101);let h=sum(&linear([6]),&linear([7]),neg(1,101),101);
        assert!(product(&fs[1],&h,9,101).is_empty());
    }
    #[test]fn quadratic_domain_and_pair_coverage(){
        let expected=[464,1311,2824,5195,8616,13279,19376];
        for m in 3..=9 {
            let nb=7*m;let nv=nb+m-2;let domain=quadratic_multipliers(nb,nv);
            assert_eq!((m-1)*domain.len(),expected[m-3]);
            assert!(domain.windows(2).all(|w|w[0]<w[1]));
            assert!(!domain.contains(&vec![0,0]));
            assert!(domain.contains(&vec![nb as u8,nb as u8]));
        }
        // All individual indices occur, but remote pairs do not. A simple
        // variable-cover check would wrongly admit this family.
        let bad:Vec<Vec<usize>>=(0..27).map(|i|{let mut s:Vec<_>=(0..7).map(|j|(i+j)%27).collect();s.sort_unstable();s}).collect();
        assert!((0..27).all(|i|bad.iter().any(|s|s.contains(&i))));
        assert!(!validate_coverage(&bad));
        assert!(validate_coverage(&coverage()));
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
        for m in [3,4] {for ell in [3,5,7] {for p in [11,101] {
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
