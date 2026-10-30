//! Exact finite-field matrix identities and bounded exceptional-chart replay.
use five_leads_study::three_leads::{Divisor,Field,Jacobian};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs};

mod theta {
    include!(concat!(env!("CARGO_MANIFEST_DIR"),"/../src/walk_impl.rs"));
    pub fn project_one(d:Divisor,j:&Jacobian,t:&Value,acts:&[(u64,[[u64;4];4])],p:u64)->Option<[u64;4]>{project_full(d,j,t,acts,p)}
    pub fn generic(d:Divisor,t:&Value,p:u64)->Option<[u64;4]>{project(d,t,p)}
}
type Mat=[[u64;4];4];
fn mul(x:u64,y:u64,p:u64)->u64{((x as u128*y as u128)%p as u128) as u64}
fn sub(x:u64,y:u64,p:u64)->u64{(x+p-y)%p}
fn pow(mut x:u64,mut n:u64,p:u64)->u64{let mut a=1;while n>0{if n&1!=0{a=mul(a,x,p)}x=mul(x,x,p);n>>=1}a}
fn inv(x:u64,p:u64)->u64{assert_ne!(x,0);pow(x,p-2,p)}
fn linear(x:[u64;4],m:Mat,p:u64)->[u64;4]{std::array::from_fn(|i|(0..4).fold(0,|a,j|(a+mul(m[i][j],x[j],p))%p))}
fn mm(x:Mat,y:Mat,p:u64)->Mat{std::array::from_fn(|i|std::array::from_fn(|j|(0..4).fold(0,|a,k|(a+mul(x[i][k],y[k][j],p))%p)))}
fn id()->Mat{std::array::from_fn(|i|std::array::from_fn(|j|u64::from(i==j)))}
fn scalar_id(x:Mat,_p:u64)->Option<u64>{let c=x[0][0];(c!=0&&(0..4).all(|i|(0..4).all(|j|x[i][j]==if i==j{c}else{0}))).then_some(c)}
fn same_mat(x:Mat,y:Mat,p:u64)->bool{let a=x.into_iter().flatten().collect::<Vec<_>>();let b=y.into_iter().flatten().collect::<Vec<_>>();if a.iter().all(|&v|v==0)||b.iter().all(|&v|v==0){return false}let k=a.iter().position(|&v|v!=0).unwrap();(0..16).all(|i|mul(a[k],b[i],p)==mul(a[i],b[k],p))}
fn same(x:[u64;4],y:[u64;4],p:u64)->bool{if x==[0;4]||y==[0;4]{return false}let k=x.iter().position(|&v|v!=0).unwrap();(0..4).all(|i|mul(x[k],y[i],p)==mul(x[i],y[k],p))}
fn det(mut x:Mat,p:u64)->u64{let mut d=1;for c in 0..4{let Some(r)=(c..4).find(|&i|x[i][c]!=0) else{return 0};if r!=c{x.swap(r,c);d=sub(0,d,p)}let pivot=x[c][c];d=mul(d,pivot,p);let unit=inv(pivot,p);for i in c+1..4{let f=mul(x[i][c],unit,p);for j in c..4{x[i][j]=sub(x[i][j],mul(f,x[c][j],p),p)}}}d}
fn num(v:&Value,k:&str)->u64{v[k].as_u64().unwrap()}
fn vec4(v:&Value)->[u64;4]{std::array::from_fn(|i|v[i].as_u64().unwrap())}
fn matrix(v:&Value)->Mat{std::array::from_fn(|i|vec4(&v[i]))}
fn rows(path:&str)->Vec<Value>{fs::read_to_string(path).unwrap().lines().map(|s|serde_json::from_str(s).unwrap()).collect()}
fn digest(path:&str)->String{format!("{:x}",Sha256::digest(fs::read(path).unwrap()))}
fn curve(t:&Value,p:u64)->Vec<u64>{let mut f=vec![0;6];f[1]=1;for r in [1,num(t,"lambda"),num(t,"mu"),num(t,"nu")]{let mut g=vec![0;6];for i in 0..5{g[i+1]=(g[i+1]+f[i])%p;g[i]=sub(g[i],mul(r,f[i],p),p)}f=g}f}
fn eval(f:&[u64],x:u64,p:u64)->u64{f.iter().rev().fold(0,|a,&c|(mul(a,x,p)+c)%p)}
fn conversion_matrix(t:&Value,p:u64)->Mat{
    let (l,mu,nu)=(num(t,"lambda"),num(t,"mu"),num(t,"nu"));
    let diag=["a","b","c","d"].map(|s|num(t,s));
    let mat=[
        [mul(mu,(l+nu)%p,p),mul(mul(nu,l,p),(1+mu)%p,p),mul(nu,(l+mu)%p,p),mul(mul(mu,l,p),(1+nu)%p,p)],
        [sub(0,mu,p),sub(0,mul(nu,l,p),p),sub(0,nu,p),sub(0,mul(mu,l,p),p)],
        [(mu+1)%p,(l+nu)%p,(nu+1)%p,(l+mu)%p],
        [p-1,p-1,p-1,p-1],
    ];
    std::array::from_fn(|i|std::array::from_fn(|j|mul(mat[j][i],diag[i],p)))
}
fn degree_two_extended(d:Divisor,t:&Value,p:u64)->Option<[u64;4]>{
    let (a0,a1,b1)=(d.u[0],d.u[1],d.v[1]);
    let (l,mu,nu)=(num(t,"lambda"),num(t,"mu"),num(t,"nu"));
    let f=curve(t,p);
    // C_n+L_n*x = x^n mod (x²+a1*x+a0). For n>=2, C_n=-a0*L_{n-1}.
    let mut c=1;let mut lin=0;let mut prev_lin=0;let mut rem0_over_a0=0;
    for (n,&coef) in f.iter().enumerate(){
        if n>=2 {rem0_over_a0=sub(rem0_over_a0,mul(coef,prev_lin,p),p)}
        let next_c=sub(0,mul(a0,lin,p),p);
        let next_l=sub(c,mul(a1,lin,p),p);
        prev_lin=lin;
        (c,lin)=(next_c,next_l);
    }
    let t0=[
        mul(sub(mu,a0,p),(l+a1+nu)%p,p),
        mul(sub(mul(nu,l,p),a0,p),(1+a1+mu)%p,p),
        mul(sub(nu,a0,p),(l+a1+mu)%p,p),
        mul(sub(mul(mu,l,p),a0,p),(1+a1+nu)%p,p),
    ];
    let b1sq=mul(b1,b1,p);let scale=["a","b","c","d"].map(|s|num(t,s));
    let out=std::array::from_fn(|i|mul(scale[i],sub(sub(t0[i],b1sq,p),rem0_over_a0,p),p));
    (out!=[0;4]).then_some(out)
}
fn independent_project(d:Divisor,t:&Value,p:u64)->Option<[u64;4]>{
    match d.degree{
        0=>theta::generic(d,t,p),
        2=>degree_two_extended(d,t,p),
        1=>{let x=sub(0,d.u[0],p);let q=[0,1,x,mul(x,x,p)];let v=linear(q,conversion_matrix(t,p),p);(v!=[0;4]).then_some(v)},
        _=>None,
    }
}
fn main(){
 let base="research/kummer_differential_collector_20261022/n16-action-certificate";
 let thetas="research/kummer_theta_20261022/results/theta-models.jsonl";
 let coords="research/kummer_coordinates_20261022/results/generic-models.jsonl";
 let mut sources=vec![thetas.to_owned(),coords.to_owned(),format!("{base}/PROTOCOL.md"),format!("{base}/src/main.rs"),"research/kummer_differential_collector_20261022/src/walk_impl.rs".into()];
 let theta:BTreeMap<u64,Value>=rows(thetas).into_iter().map(|v|(num(&v,"p"),v)).collect();
 let actions:BTreeMap<u64,Value>=rows(coords).into_iter().map(|v|(num(&v,"p"),v)).collect();
 let mut raw=String::from("schema,bits,p,class,action_index,root,status\n");
 let mut models=Vec::new();
 for bits in [16,20,24,28,32,36,40]{
  let path=format!("research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl");sources.push(path.clone());
  for target in rows(&path).into_iter().filter(|v|num(v,"target_index")==0){
   let p=num(&target,"p");let selected=&theta[&p]["selected"];let t=&selected["constants"];
   let r0=num(selected,"r0");let delta=num(selected,"delta");let di=inv(delta,p);
   let acts:Vec<(u64,Mat)>=actions[&p]["torsion_actions"].as_array().unwrap().iter().map(|a|(mul(sub(num(a,"root"),r0,p),di,p),matrix(&a["action"]["matrix"]))).collect();
   assert_eq!(acts.len(),5);
   let mat:Vec<Mat>=acts.iter().map(|(_,m)|*m).collect();
   let determinant:Vec<u64>=mat.iter().map(|&m|det(m,p)).collect();
   let square_scalar:Vec<Option<u64>>=mat.iter().map(|&m|scalar_id(mm(m,m,p),p)).collect();
   let mut commute=Vec::new();for i in 0..5{for j in i+1..5{commute.push(json!({"i":i,"j":j,"projectively_equal":same_mat(mm(mat[i],mat[j],p),mm(mat[j],mat[i],p),p)}));}}
   let product=mat.iter().fold(id(),|v,&m|mm(v,m,p));let product_scalar=scalar_id(product,p);
   let conversion_det=det(conversion_matrix(t,p),p);
   let constants=["A","B","C","D"].map(|s|num(t,s));
   let f=curve(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f.clone()};
   let mut classes=vec![("O".to_owned(),Divisor::O)];
   for (i,(root,_)) in acts.iter().enumerate(){classes.push((format!("torsion_{i}"),Divisor{degree:1,u:[sub(0,*root,p),1,0],v:[0,0]}));}
   let mut degree_one=0;
   for x in 0..p.min(256){let fx=eval(&f,x,p);if let Some(y)=j.f.sqrt(fx){for yy in if y==0{vec![0]}else{vec![y,p-y]}{let d=Divisor{degree:1,u:[sub(0,x,p),1,0],v:[yy,0]};classes.push((format!("x{x}_y{yy}"),d));degree_one+=1;}}}
   let mut counts=BTreeMap::<String,u64>::new();let mut chart_counts=BTreeMap::<String,u64>::new();
   for (name,d) in &classes{
    assert!(j.valid(*d),"invalid class {name} p={p}");
    let k=independent_project(*d,t,p);
    for (i,(root,m)) in acts.iter().enumerate(){
      let translated=j.add(*d,Divisor{degree:1,u:[sub(0,*root,p),1,0],v:[0,0]});
      let expected=independent_project(translated,t,p);
      if translated.degree==2 {
       let source=theta::generic(translated,t,p);
       let key=match (source,expected){(Some(a),Some(b)) if same(a,b,p)=>"source_match",(None,Some(_))=>"source_zero_extended_nonzero",(None,None)=>"both_missing",_=>"source_mismatch"};
       *chart_counts.entry(key.into()).or_default()+=1;
      }
      let status=match (k,expected){(Some(x),Some(y)) if same(linear(x,*m,p),y,p)=>"match",(None,_)|(_,None)=>"missing",_=>"mismatch"};
      *counts.entry(status.into()).or_default()+=1;
      raw.push_str(&format!("kummer-n16-exception/v1,{bits},{p},{name},{i},{root},{status}\n"));
    }
   }
   models.push(json!({"bits":bits,"p":p,"action_count":5,"matrix_determinants":determinant,"square_scalars":square_scalar,"pairwise_commutation":commute,"five_product_scalar":product_scalar,"general_conversion_determinant":conversion_det,"theta_constants":constants,"degree_one_sample_classes":degree_one,"class_count":classes.len(),"check_counts":counts,"chart_counts":chart_counts}));
   eprintln!("model p={p} classes={} done",classes.len());
  }
 }
 fs::create_dir_all(format!("{base}/results")).unwrap();
 let csv_sha=format!("{:x}",Sha256::digest(raw.as_bytes()));
 fs::write(format!("{base}/results/exceptional.csv"),raw).unwrap();
 let source_digests:Vec<Value>=sources.iter().map(|path|json!({"path":path,"sha256":digest(path)})).collect();
 fs::write(format!("{base}/results/summary.json"),format!("{}\n",serde_json::to_string_pretty(&json!({"schema":"kummer-n16-action-certificate/v1","source_digests":source_digests,"exceptional_csv_sha256":csv_sha,"models":models,"geometric_all_state_action_identity":null,"rho_ratio":null})).unwrap())).unwrap();
}
