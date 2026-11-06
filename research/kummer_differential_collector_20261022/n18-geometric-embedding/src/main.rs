use serde_json::Value;
use std::{collections::BTreeMap, fs};

// Exact Z[s,t] arithmetic. Six coefficient-basis checks prove the identity
// for every quintic, before any finite-field specialization.
type Poly = BTreeMap<(u8,u8),i64>;
fn mon(a:u8,b:u8,c:i64)->Poly { if c==0 {Poly::new()} else {BTreeMap::from([((a,b),c)])} }
fn add(x:&Poly,y:&Poly)->Poly { let mut z=x.clone(); for (&k,&v) in y { *z.entry(k).or_default()+=v; } z.retain(|_,v|*v!=0);z }
fn neg(x:&Poly)->Poly {x.iter().map(|(&k,&v)|(k,-v)).collect()}
fn sub(x:&Poly,y:&Poly)->Poly {add(x,&neg(y))}
fn mul(x:&Poly,y:&Poly)->Poly {let mut z=Poly::new();for (&(a,b),&u) in x {for (&(c,d),&v) in y {*z.entry((a+c,b+d)).or_default()+=u*v;}}z.retain(|_,v|*v!=0);z}
fn scale(x:&Poly,c:i64)->Poly {x.iter().filter_map(|(&k,&v)|(c*v!=0).then_some((k,c*v))).collect()}
fn pow(x:&Poly,n:usize)->Poly {let mut z=mon(0,0,1);for _ in 0..n{z=mul(&z,x)}z}
fn mp(a:u64,b:u64,p:u64)->u64 {((a as u128*b as u128)%p as u128) as u64}
fn sp(a:u64,b:u64,p:u64)->u64 {(a+p-b)%p}
fn qp(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n>0{if n&1!=0{z=mp(z,a,p)}a=mp(a,a,p);n>>=1}z}
fn mat(v:&Value)->[[u64;4];4]{std::array::from_fn(|i|std::array::from_fn(|j|v[i][j].as_u64().unwrap()))}
fn apply(c:&[[u64;4];4],w:&[u64;4],p:u64)->[u64;4]{std::array::from_fn(|i|(0..4).fold(0,|z,j|(z+mp(c[i][j],w[j],p))%p))}
fn solve(c:&[[u64;4];4],w:&[u64;4],p:u64)->[u64;4]{
    let mut a=[[0;5];4];for i in 0..4{for j in 0..4{a[i][j]=c[i][j]}a[i][4]=w[i]}
    for k in 0..4{let pivot=(k..4).find(|&i|a[i][k]!=0).unwrap();a.swap(k,pivot);
        let v=qp(a[k][k],p-2,p);for z in &mut a[k][k..]{*z=mp(*z,v,p)}let row=a[k];
        for (i,r) in a.iter_mut().enumerate(){if i!=k{let v=r[k];for(z,&base)in r[k..].iter_mut().zip(&row[k..]){*z=sp(*z,mp(v,base,p),p)}}}
    }std::array::from_fn(|i|a[i][4])
}
fn universal_identity()->[bool;6]{
    let s=mon(1,0,1);let t=mon(0,1,1);let delta=sub(&pow(&s,2),&scale(&t,4));
    let mut out=[false;6];
    for (degree,slot) in out.iter_mut().enumerate(){
        let mut rem0=Poly::new();let mut rem1=Poly::new();
        let mut x0=mon(0,0,1);let mut x1=Poly::new();
        for n in 0..=5{
            if n==degree{rem0=add(&rem0,&x0);rem1=add(&rem1,&x1)}
            let next0=neg(&mul(&t,&x1));let next1=add(&x0,&mul(&s,&x1));x0=next0;x1=next1;
        }
        let f0=match degree {0=>scale(&mon(0,0,1),2),1=>s.clone(),2=>scale(&t,2),3=>mul(&s,&t),4=>scale(&pow(&t,2),2),5=>mul(&s,&pow(&t,2)),_=>unreachable!()};
        let pterm=match degree {0|1=>Poly::new(),2=>mon(0,0,-1),3=>neg(&s),4=>neg(&pow(&s,2)),5=>neg(&mul(&s,&sub(&pow(&s,2),&t))),_=>unreachable!()};
        let lhs=sub(&sub(&f0,&mul(&s,&rem1)),&scale(&rem0,2));
        *slot=lhs==mul(&delta,&pterm);
    }
    out
}
fn main(){
    let basis=universal_identity();assert!(basis.iter().all(|&x|x));
    let input="research/kummer_differential_collector_20261022/n16-action-certificate/results/exceptional-ideal.json";
    let data:Value=serde_json::from_str(&fs::read_to_string(input).unwrap()).unwrap();
    let mut models=BTreeMap::new();for row in data["rows"].as_array().unwrap(){models.entry(row["p"].as_u64().unwrap()).or_insert_with(||row.clone());}
    assert_eq!(models.len(),21);
    let theta_path="research/kummer_theta_20261022/results/theta-models.jsonl";
    let mut thetas=BTreeMap::new();for line in fs::read_to_string(theta_path).unwrap().lines(){let v:Value=serde_json::from_str(line).unwrap();thetas.insert(v["p"].as_u64().unwrap(),v);}
    let mut csv=String::from("schema,p,lambda,det_c1,det_c2,universal_coefficient_identities,published_tau_matrix_match,classical_kappa_branch_pair,source_xi4,branch_pair_equal,c1_classical_equals_c2_direct,degree_one_limit,identity_limit,geometric_kummer_embedding,theta_xadd_all_state_total,new_collector_targets,rho_ratio\n");
    for (p,row) in models{
        let f:Vec<u64>=row["f"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()).collect();assert_eq!(f.len(),6);
        let roots:Vec<u64>=data["rows"].as_array().unwrap().iter().filter(|v|v["p"].as_u64()==Some(p)).map(|v|v["root"].as_u64().unwrap()).collect();
        let lambda=*roots.iter().find(|&&x|x!=0&&x!=1).unwrap();let s=(1+lambda)%p;let t=lambda;
        let d=sp(mp(s,s,p),mp(4,t,p),p);assert_ne!(d,0);
        let f0=(mp(f[1],s,p)+mp(2,mp(f[2],t,p),p)+mp(f[3],mp(s,t,p),p)+mp(2,mp(f[4],mp(t,t,p),p),p)+mp(f[5],mp(s,mp(t,t,p),p),p))%p;
        let k4=mp(f0,qp(d,p-2,p),p); // y1*y2=0 on the two branch points.
        let c1=mat(&row["conversion_matrix_degree_one"]);let c2=mat(&row["conversion_matrix_degree_two"]);
        let theta=&thetas[&p]["selected"]["constants"];
        let (l,mu,nu)=(theta["lambda"].as_u64().unwrap(),theta["mu"].as_u64().unwrap(),theta["nu"].as_u64().unwrap());
        let diag=["a","b","c","d"].map(|name|theta[name].as_u64().unwrap());
        assert!(diag.iter().all(|&v|v!=0));
        assert!(["A","B","C","D"].iter().all(|name|theta[name].as_u64().unwrap()!=0));
        let source_rows=[
            [mp(mu,(l+nu)%p,p),mp(mp(nu,l,p),(1+mu)%p,p),mp(nu,(l+mu)%p,p),mp(mp(mu,l,p),(1+nu)%p,p)],
            [sp(0,mu,p),sp(0,mp(nu,l,p),p),sp(0,nu,p),sp(0,mp(mu,l,p),p)],
            [(mu+1)%p,(l+nu)%p,(nu+1)%p,(l+mu)%p],
            [p-1,p-1,p-1,p-1],
        ];
        let published_tau: [[u64;4];4]=std::array::from_fn(|i|std::array::from_fn(|j|mp(source_rows[j][i],diag[i],p)));
        assert_eq!(published_tau,c1,"published tau matrix mismatch at p={p}");
        let a1=sp(0,s,p);let w2=[1,s,sp(0,t,p),mp(t,a1,p)];let k=apply(&c2,&w2,p);let q=solve(&c1,&k,p);
        assert_eq!(q,[1,s,t,k4],"classical Kummer disagreement at p={p}");
        // O is C1 column four, projectively the theta null.
        let det1=row["degree_one_conversion_determinant"].as_u64().unwrap();let det2=row["degree_two_conversion_determinant"].as_u64().unwrap();assert!(det1!=0&&det2!=0);
        csv.push_str(&format!("kummer-n18-geometric/v1,{p},{lambda},{det1},{det2},6,true,{k4},{},true,true,written_limit,written_limit,true,,0,\n",q[3]));
    }
    let output="research/kummer_differential_collector_20261022/n18-geometric-embedding/results/certificate.csv";
    fs::create_dir_all(std::path::Path::new(output).parent().unwrap()).unwrap();fs::write(output,csv).unwrap();
    println!("6 universal coefficient identities and 21 classical Kummer branch-pair checks passed");
}
