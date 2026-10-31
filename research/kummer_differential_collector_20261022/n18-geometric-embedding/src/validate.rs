use serde_json::Value;
use std::{collections::{BTreeMap,BTreeSet},fs};

fn fmod(x:i128,p:i128)->i128{x.rem_euclid(p)}
fn one_case(degree:usize,s:i128,t:i128)->i128{
    let f0=match degree{0=>2,1=>s,2=>2*t,3=>s*t,4=>2*t*t,5=>s*t*t,_=>unreachable!()};
    let mut r0=0;let mut r1=0;let(mut c,mut l)=(1,0);
    for n in 0..=5{if n==degree{r0=c;r1=l} (c,l)=(-t*l,c+s*l)}
    let p=match degree{0|1=>0,2=>-1,3=>-s,4=>-s*s,5=>-s*(s*s-t),_=>unreachable!()};
    f0-s*r1-2*r0-(s*s-4*t)*p
}
fn main(){
    // Each coefficient-basis residual has degree <= 5 in s and <= 3 in t.
    // Vanishing on this 6x4 integer grid therefore proves the polynomial
    // residual identically zero by repeated univariate interpolation.
    for degree in 0..=5{for s in 0..=5{for t in 0..=3{
        assert_eq!(one_case(degree,s,t),0,"coefficient {degree}, s={s}, t={t}");
    }}}
    let input="research/kummer_differential_collector_20261022/n16-action-certificate/results/exceptional-ideal.json";
    let data:Value=serde_json::from_str(&fs::read_to_string(input).unwrap()).unwrap();
    let mut models=BTreeMap::new();for row in data["rows"].as_array().unwrap(){models.entry(row["p"].as_u64().unwrap()).or_insert_with(||row.clone());}
    let path="research/kummer_differential_collector_20261022/n18-geometric-embedding/results/certificate.csv";
    let csv=fs::read_to_string(path).unwrap();let mut lines=csv.lines();let header=lines.next().unwrap();assert_eq!(header.split(',').count(),17);
    let mut seen=BTreeSet::new();let mut count=0;
    for line in lines{let c:Vec<_>=line.split(',').collect();assert_eq!(c.len(),17);assert_eq!(c[0],"kummer-n18-geometric/v1");
        let p:u64=c[1].parse().unwrap();let lambda:u64=c[2].parse().unwrap();assert!(seen.insert(p));let row=&models[&p];
        let f:Vec<i128>=row["f"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap() as i128).collect();
        let s=1+lambda as i128;let t=lambda as i128;let modulus=p as i128;
        let delta=fmod(s*s-4*t,modulus);assert_ne!(delta,0);
        let numer=fmod(f[1]*s+2*f[2]*t+f[3]*s*t+2*f[4]*t*t+f[5]*s*t*t,modulus);
        let inverse=(1..modulus).find(|&x|fmod(x*delta,modulus)==1).unwrap();
        let k4=fmod(numer*inverse,modulus);
        assert_eq!(c[7].parse::<i128>().unwrap(),k4);assert_eq!(c[8].parse::<i128>().unwrap(),k4);
        assert_eq!(c[5],"6");assert_eq!(c[6],"true");assert_eq!(c[9],"true");assert_eq!(c[10],"true");
        assert_eq!(c[11],"written_limit");assert_eq!(c[12],"written_limit");
        assert_eq!(c[13],"true");assert_eq!(c[14],"");assert_eq!(c[15],"0");assert_eq!(c[16],"");
        assert_eq!(c[3].parse::<u64>().unwrap(),row["degree_one_conversion_determinant"].as_u64().unwrap());
        assert_eq!(c[4].parse::<u64>().unwrap(),row["degree_two_conversion_determinant"].as_u64().unwrap());
        count+=1;
    }
    assert_eq!(count,21);assert_eq!(seen.len(),21);
    println!("validated 6 universal residuals on the proof grid and 21 independent finite-field CSV rows");
}
