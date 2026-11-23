//! Exploratory MLP for the retained prime m=2 radix census.
//! It fits exact Hilbert degrees and treats other records only as lower bounds.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path};

const HIDDEN: usize = 4;
const INPUT: usize = 6;
const EPOCHS: usize = 160;

#[derive(Clone)]
struct Row { id: usize, parts: Vec<usize>, x: [f64; INPUT], exact: Option<f64>, lower: f64 }
fn features(parts: &[usize]) -> [f64; INPUT] {
    let d: usize = parts.iter().map(|p| (1usize << p) - 1).sum();
    let max = *parts.iter().max().unwrap() as f64;
    let min = *parts.iter().min().unwrap() as f64;
    let one = parts.iter().filter(|&&p| p == 1).count() as f64;
    let wide = parts.iter().filter(|&&p| p >= 5).count() as f64;
    [parts.len() as f64, d as f64, max, min, one, wide]
}
fn load(dir: &Path) -> Vec<Row> {
    let mut out = Vec::new();
    for id in 0..3010 {
        let p = dir.join(format!("{id:04}.json"));
        let v: Value = serde_json::from_slice(&fs::read(p).expect("read case")).expect("case JSON");
        let parts: Vec<usize> = serde_json::from_value(v["parts"].clone()).expect("parts");
        let exact = v["first_vanishing_degree"].as_u64().map(|x| x as f64);
        let lower = v["lower_bound"].as_u64().expect("certified lower_bound") as f64;
        let status = v["status"].as_str().expect("status");
        assert_eq!(exact.is_some(), status == "exact_hilbert_under_large_characteristic_theorem");
        out.push(Row { id, x: features(&parts), parts, exact, lower });
    }
    let ne = out.iter().filter(|r| r.exact.is_some()).count();
    let nc = out.len() - ne;
    assert_eq!((out.len(), ne, nc), (3010, 35, 2975));
    out
}
fn fold(r: &Row) -> usize {
    let b = serde_json::to_vec(&r.parts).unwrap();
    let mut h = Sha256::new(); h.update(b"n83-radix-neural-fold-v1"); h.update(b);
    u64::from_be_bytes(h.finalize()[..8].try_into().unwrap()) as usize % 5
}
#[derive(Clone)]
struct Net { mean: [f64; INPUT], scale: [f64; INPUT], w1: [[f64; HIDDEN]; INPUT], b1: [f64; HIDDEN], w2: [f64; HIDDEN], b2: f64, ymean: f64, yscale: f64 }
fn predict(n: &Net, raw: &[f64; INPUT]) -> f64 {
    let mut y = n.b2;
    for j in 0..HIDDEN {
        let mut z = n.b1[j];
        for i in 0..INPUT { z += ((raw[i]-n.mean[i])/n.scale[i])*n.w1[i][j]; }
        y += n.w2[j] * z.tanh();
    }
    y*n.yscale+n.ymean
}
fn train(rows: &[Row], seed: u64) -> (Net, f64) {
    let exact: Vec<_> = rows.iter().filter(|r| r.exact.is_some()).collect();
    let cens: Vec<_> = rows.iter().filter(|r| r.exact.is_none()).collect();
    assert!(!exact.is_empty() && !cens.is_empty());
    let ymean = exact.iter().map(|r| r.exact.unwrap()).sum::<f64>()/exact.len() as f64;
    let yscale = (exact.iter().map(|r| (r.exact.unwrap()-ymean).powi(2)).sum::<f64>()/exact.len() as f64).sqrt().max(1.0);
    let mut mean=[0.0;INPUT];
    for r in rows { for i in 0..INPUT { mean[i]+=r.x[i]/rows.len() as f64; } }
    let mut scale=[0.0;INPUT];
    for r in rows { for i in 0..INPUT { scale[i]+=(r.x[i]-mean[i]).powi(2)/rows.len() as f64; } }
    for x in &mut scale { *x=x.sqrt().max(1.0); }
    let mut s=seed|1; let mut rnd=|| { s^=s<<13; s^=s>>7; s^=s<<17; (s as i64 as f64)/(i64::MAX as f64)*0.08 };
    let mut n=Net { mean,scale,w1:[[0.0;HIDDEN];INPUT],b1:[0.0;HIDDEN],w2:[0.0;HIDDEN],b2:0.0,ymean,yscale };
    for i in 0..INPUT { for j in 0..HIDDEN { n.w1[i][j]=rnd(); } }
    for j in 0..HIDDEN { n.w2[j]=rnd(); }
    // Full-batch Adam; equally weight the exact and censored populations.
    let mut params=pack(&n); let mut m=vec![0.0;params.len()]; let mut v=vec![0.0;params.len()];
    for epoch in 1..=EPOCHS {
        let mut grad=vec![0.0;params.len()];
        for r in &exact {
            let (pred,cache)=forward(&n,&r.x); let y=(r.exact.unwrap()-ymean)/yscale;
            backprop(&n,&cache,pred-y,0.5/exact.len() as f64,&mut grad);
        }
        for r in &cens {
            let bound=(r.lower-ymean)/yscale; let (pred,cache)=forward(&n,&r.x);
            if pred < bound { backprop(&n,&cache,pred-bound,0.5/cens.len() as f64,&mut grad); }
        }
        for i in 0..params.len() {
            m[i]=0.9*m[i]+0.1*grad[i]; v[i]=0.999*v[i]+0.001*grad[i]*grad[i];
            let mh=m[i]/(1.0-0.9f64.powi(epoch as i32)); let vh=v[i]/(1.0-0.999f64.powi(epoch as i32));
            params[i]-=0.01*mh/(vh.sqrt()+1e-8);
        }
        unpack(&params,&mut n);
    }
    let loss_exact=exact.iter().map(|r| (predict(&n,&r.x)-r.exact.unwrap()).powi(2)).sum::<f64>()/exact.len() as f64;
    let loss_cens=cens.iter().map(|r| (r.lower-predict(&n,&r.x)).max(0.0).powi(2)).sum::<f64>()/cens.len() as f64;
    (n,0.5*(loss_exact/(yscale*yscale)+loss_cens/(yscale*yscale)))
}
fn forward(n:&Net, raw:&[f64;INPUT])->(f64,([f64;INPUT],[f64;HIDDEN],[f64;HIDDEN])) {
    let x=std::array::from_fn(|i|(raw[i]-n.mean[i])/n.scale[i]); let mut z=[0.0;HIDDEN]; let mut a=[0.0;HIDDEN]; let mut y=n.b2;
    for j in 0..HIDDEN { z[j]=n.b1[j]; for i in 0..INPUT { z[j]+=x[i]*n.w1[i][j]; } a[j]=z[j].tanh(); y+=a[j]*n.w2[j]; }
    (y,(x,z,a))
}
fn pack(n:&Net)->Vec<f64> { let mut p=Vec::new(); for i in 0..INPUT { for j in 0..HIDDEN { p.push(n.w1[i][j]); } } p.extend(n.b1); p.extend(n.w2); p.push(n.b2); p }
fn unpack(p:&[f64],n:&mut Net) { let mut k=0; for i in 0..INPUT { for j in 0..HIDDEN { n.w1[i][j]=p[k];k+=1; } } for j in 0..HIDDEN {n.b1[j]=p[k];k+=1;} for j in 0..HIDDEN {n.w2[j]=p[k];k+=1;} n.b2=p[k]; }
fn backprop(n:&Net,c:&([f64;INPUT],[f64;HIDDEN],[f64;HIDDEN]),d:f64,w:f64,g:&mut[f64]) {
    let (x,_z,a)=c; let mut k=0; let mut da=[0.0;HIDDEN];
    for j in 0..HIDDEN { da[j]=d*n.w2[j]*(1.0-a[j]*a[j]); }
    for i in 0..INPUT { for j in 0..HIDDEN { g[k]+=w*da[j]*x[i];k+=1; } }
    for j in 0..HIDDEN { g[k]+=w*da[j];k+=1; }
    for j in 0..HIDDEN { g[k]+=w*d*a[j];k+=1; }
    g[k]+=w*d;
}
fn baseline_score(mid:f64,exact:&[&Row],cens:&[&Row])->f64 {
    0.5*exact.iter().map(|r|mid-r.exact.unwrap()).sum::<f64>()/exact.len() as f64
      +0.5*cens.iter().map(|r|(mid-r.lower).min(0.0)).sum::<f64>()/cens.len() as f64
}
fn baseline(exact:&[&Row],cens:&[&Row])->f64 {
    assert!(!exact.is_empty() && !cens.is_empty());
    let lo0=exact.iter().map(|r|r.exact.unwrap()).chain(cens.iter().map(|r|r.lower)).fold(f64::INFINITY,f64::min);
    let hi0=exact.iter().map(|r|r.exact.unwrap()).chain(cens.iter().map(|r|r.lower)).fold(f64::NEG_INFINITY,f64::max);
    // The derivative is nonpositive below every observation and positive above
    // every one-sided bound. Bracket from the observed range, not a fixed cap.
    let (mut lo,mut hi)=(lo0-1.0,hi0+1.0);
    assert!(baseline_score(lo,exact,cens)<=0.0);
    assert!(baseline_score(hi,exact,cens)>=0.0);
    for _ in 0..128 { let mid=(lo+hi)/2.0; if baseline_score(mid,exact,cens)>0.0 {hi=mid}else{lo=mid} }
    (lo+hi)/2.0
}
fn main() {
    let mut args=env::args().skip(1); let input=args.next().expect("CASE_DIR"); let output=args.next().expect("OUTPUT_DIR");
    let input=Path::new(&input); let out=Path::new(&output); fs::create_dir_all(out).unwrap(); let rows=load(input);
    let mut cv=Vec::new();
    for f in 0..5 {
        let tr:Vec<_>=rows.iter().filter(|r|r.exact.is_none()||fold(r)!=f).cloned().collect();
        let te:Vec<_>=rows.iter().filter(|r|r.exact.is_some()&&fold(r)==f).collect();
        assert!(!te.is_empty()); let (net,loss)=train(&tr,20261030+f as u64);
        let ex:Vec<_>=tr.iter().filter(|r|r.exact.is_some()).collect(); let ce:Vec<_>=tr.iter().filter(|r|r.exact.is_none()).collect(); let b=baseline(&ex,&ce);
        for r in te { cv.push(json!({"case":r.id,"fold":f,"exact_degree":r.exact,"mlp_prediction":predict(&net,&r.x),"censored_aware_constant":b,"training_loss":loss})); }
    }
    let (_,cv_model_mae)= { let (a,b)=cv.iter().fold((0.0,0.0),|(a,b),v| {let y=v["exact_degree"].as_f64().unwrap();(a+(v["mlp_prediction"].as_f64().unwrap()-y).abs(),b+(v["censored_aware_constant"].as_f64().unwrap()-y).abs())}); ((),json!({"mlp_mae":a/cv.len() as f64,"constant_mae":b/cv.len() as f64})) };
    let metrics=&cv_model_mae;
    let (model,loss)=train(&rows,20261030); let mut preds=Vec::new(); let mut lead=0usize; let mut contradicted=0usize; let mut bound_violations=0usize;
    for r in &rows { let p=predict(&model,&r.x); let below=p<42.0; let contradiction=below&&r.exact.is_none()&&r.lower>=43.0; if below&&!r.exact.is_some(){lead+=1;} if contradiction{contradicted+=1;} if r.exact.is_none()&&p+1e-9<r.lower{bound_violations+=1;}
        preds.push(json!({"case":r.id,"parts":r.parts,"status":if r.exact.is_some(){"exact"}else{"lower_bound_only"},"exact_degree":r.exact,"lower_bound":r.lower,"prediction":p,"model_flags_below_42":below,"certified_contradiction":contradiction}));
    }
    fs::write(out.join("cv.json"),serde_json::to_vec_pretty(&json!({"folds":5,"held_out_exact":cv.len(),"metrics":metrics,"predictions":cv})).unwrap()).unwrap();
    fs::write(out.join("predictions.json"),serde_json::to_vec_pretty(&preds).unwrap()).unwrap();
    fs::write(out.join("model.json"),serde_json::to_vec_pretty(&json!({"architecture":{"inputs":INPUT,"hidden":HIDDEN,"activation":"tanh","output":"linear"},"epochs":EPOCHS,"seed":20261030,"features":["part_count","socle_degree","largest_part","smallest_part","unit_parts","parts_ge_5"],"parameters":pack(&model),"feature_mean":model.mean,"feature_scale":model.scale,"target_mean":model.ymean,"target_scale":model.yscale})).unwrap()).unwrap();
    fs::write(out.join("metrics.json"),serde_json::to_vec_pretty(&json!({"schema":"n83.radix-neural-results/v1","rows":rows.len(),"exact":35,"lower_bound_only":2975,"cv":metrics,"full_training_loss":loss,"model_flagged_censored_below_42":lead,"flagged_candidates_contradicted_by_certified_lower_bound":contradicted,"censored_bound_violations":bound_violations,"lowest_exact_degree":42,"interpretation":"Exploratory model only. Any flagged row is checked against the exact retained certificate. No new factor base or point set is searched."})).unwrap()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    fn row(exact:Option<f64>,lower:f64)->Row { Row{id:0,parts:vec![1],x:[0.0;INPUT],exact,lower} }
    #[test]
    fn censored_constant_bracket_covers_large_certified_bounds() {
        let a=row(Some(42.0),42.0); let b=row(Some(44.0),44.0);
        let c=row(None,43.0); let d=row(None,134_217_729.0);
        let ex=vec![&a,&b]; let ce=vec![&c,&d];
        let x=baseline(&ex,&ce);
        assert!(x>200.0 && x<134_217_729.0);
        assert!(baseline_score(x,&ex,&ce).abs()<1e-6);
    }
}
