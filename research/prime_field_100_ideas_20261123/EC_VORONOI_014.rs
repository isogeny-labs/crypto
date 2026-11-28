// Frozen deterministic Voronoi boundary selection over the exact S4 grid.
use std::{collections::HashMap, env, fs};

#[derive(Clone)] struct Witness { i: usize, j: usize, k: usize, row: [i64;17] }
fn between<'a>(s: &'a str, key: &str) -> &'a str {
    let start=s.find(key).expect("missing key")+key.len();
    let end=s[start..].find(']').expect("missing bracket")+start;
    &s[start..end]
}
fn nums(s:&str)->Vec<i64> { s.split(',').map(|x|x.trim().parse().expect("integer")).collect() }
fn modp(a:i64)->i64 { a.rem_euclid(1013) }
fn inv(a:i64)->i64 { let (mut t,mut nt,mut r,mut nr)=(0,1,1013,modp(a)); while nr!=0 { let q=r/nr;(t,nt)=(nt,t-q*nt);(r,nr)=(nr,r-q*nr); } assert_eq!(r,1);modp(t) }
fn rank(rows:&[[i64;17]])->usize {
    let mut a=rows.to_vec();let mut r=0;
    for c in 0..17 { let pivot=(r..a.len()).find(|&i|modp(a[i][c])!=0); if let Some(k)=pivot {
        a.swap(r,k);let d=inv(a[r][c]);for j in c..17 {a[r][j]=modp(a[r][j]*d)}
        for i in 0..a.len() { if i!=r {let f=a[i][c];for j in c..17 {a[i][j]=modp(a[i][j]-f*a[r][j])}}}
        r+=1;if r==a.len(){break}
    }}r
}
fn hash(mut x:u64)->u64 { x=x.wrapping_add(0x9e3779b97f4a7c15);x=(x^(x>>30)).wrapping_mul(0xbf58476d1ce4e5b9);x=(x^(x>>27)).wrapping_mul(0x94d049bb133111eb);x^(x>>31) }
fn id(i:usize,j:usize,k:usize)->usize { (i-1)*256+(j-1)*16+(k-1) }
fn main(){
    let path=env::args().nth(1).expect("input path");let raw=fs::read_to_string(path).expect("read input");
    assert!(raw.contains("header p=1009 a=1 b=14 order=1013 B=16 triples_per_target=4096"));
    assert!(raw.contains("complete matched exact comparison"));
    let mut by_z:HashMap<i64,Vec<Witness>>=HashMap::new();
    for line in raw.lines().filter(|s|s.starts_with("witness z=")) {
        let z=line[10..].split_whitespace().next().unwrap().parse::<i64>().unwrap();
        let v=nums(between(line,"indices=["));let s=nums(between(line,"signs=["));let r=nums(between(line,"row=["));
        assert_eq!(v.len(),3);assert_eq!(s.len(),3);assert_eq!(r.len(),17);
        let (i,j,k)=(v[0] as usize,v[1] as usize,v[2] as usize);assert!((1..=16).contains(&i)&& (1..=16).contains(&j)&& (1..=16).contains(&k));
        let mut reconstructed=[0_i64;17];reconstructed[i-1]+=s[0];reconstructed[j-1]+=s[1];reconstructed[k-1]+=s[2];reconstructed[16]=-1;
        let row:[i64;17]=r.try_into().unwrap();assert_eq!(row,reconstructed);
        by_z.entry(z).or_default().push(Witness{i,j,k,row});
    }
    assert_eq!(by_z.get(&0).unwrap().len(),24);assert_eq!(by_z.get(&320).unwrap().len(),33);
    let mut boundary=Vec::new();let mut keyed=Vec::new();
    for i in 1..=16 {for j in 1..=16 {for k in 1..=16 {
        let n=id(i,j,k);let sum=i+j+k;
        if (24..=27).contains(&sum){boundary.push(n)}
        keyed.push((hash((n as u64)^0x14_2026_1124),n));
    }}}
    keyed.sort_unstable();let n=boundary.len();let mut in_boundary=[false;4096];let mut in_hash=[false;4096];
    for &v in &boundary{in_boundary[v]=true}for &(_,v) in keyed.iter().take(n){in_hash[v]=true}
    println!("protocol grid=16^3 sites=(1,1,1),(16,16,16) boundary_sum=24..27 boundary_count={} baseline_hash_seed=0x1420261124 baseline_count={} distance_proxies_per_candidate=6",n,n);
    for z in [0,320] {let w=by_z.get(&z).unwrap();let mut seen=[false;4096];
        for v in w {let q=id(v.i,v.j,v.k);assert!(!seen[q],"duplicate witness triple");seen[q]=true}
        for (name,mask) in [("boundary",&in_boundary),("hash",&in_hash)] {
            let rows:Vec<[i64;17]>=w.iter().filter(|v|mask[id(v.i,v.j,v.k)]).map(|v|v.row).collect();
            let selected_triples=(0..4096).filter(|&q|mask[q]).count();assert_eq!(selected_triples,n);
            println!("target_z={} arm={} candidates={} hit_triples={} signed_rows={} rank_over_1013={}",z,name,n,rows.len(),rows.len(),rank(&rows));
        }
        println!("target_z={} full_grid_candidates=4096 full_hits={} full_rank={}",z,w.len(),rank(&w.iter().map(|v|v.row).collect::<Vec<_>>()));
    }
    println!("complete Voronoi EC candidate control");
}
