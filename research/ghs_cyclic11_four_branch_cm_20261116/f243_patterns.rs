use std::collections::BTreeMap;
const Q: usize = 243;
fn digits(mut a: usize) -> [i32;5] {
    let mut d=[0;5]; for x in &mut d { *x=(a%3) as i32; a/=3; } d
}
fn pack(d:[i32;5])->usize {
    let (mut n,mut k)=(0,1);
    for x in d { n+=x.rem_euclid(3) as usize*k; k*=3; } n
}
fn add(a:usize,b:usize)->usize {
    let x=digits(a); let y=digits(b); pack(std::array::from_fn(|i|x[i]+y[i]))
}
fn sub(a:usize,b:usize)->usize {
    let x=digits(a); let y=digits(b); pack(std::array::from_fn(|i|x[i]-y[i]))
}
fn mul(a:usize,b:usize)->usize {
    let x=digits(a); let y=digits(b); let mut d=[0_i32;9];
    for i in 0..5 { for j in 0..5 { d[i+j]+=x[i]*y[j]; } }
    for k in (5..=8).rev() { let c=d[k].rem_euclid(3); d[k-4]+=c; d[k-5]+=2*c; }
    pack([d[0],d[1],d[2],d[3],d[4]])
}
fn pow(mut a:usize,mut n:usize)->usize {
    let mut b=1; while n>0 { if n&1==1 {b=mul(b,a);} a=mul(a,a); n>>=1; } b
}
fn fourth(a:i32,b:i32,c:i32)->i32 { (-(a+b+c)).rem_euclid(11) }
fn paired(a:i32,b:i32,c:i32,d:i32)->bool {
    ((a+b)%11==0&&(c+d)%11==0) || ((a+c)%11==0&&(b+d)%11==0) ||
    ((a+d)%11==0&&(b+c)%11==0)
}
fn fifth_trace(s:i64)->i64 {s.pow(5)-15*s.pow(3)+45*s}
fn main() {
    assert_eq!(mul(3,81),add(3,2));
    for a in 1..Q { assert_eq!(pow(a,242),1); }
    let zeta=(2..Q).map(|g|pow(g,22)).find(|&z|z!=1).expect("order-11 root");
    assert_eq!(pow(zeta,11),1);
    let mut log=BTreeMap::new(); let mut z=1;
    for e in 0..11 { assert!(log.insert(z,e).is_none()); z=mul(z,zeta); }
    assert_eq!(z,1);
    let mut chars=Vec::new();
    for x in 0..Q {
        if x==0||x==1||x==2 {continue;}
        let e0=log[&pow(x,22)];
        let e1=log[&pow(sub(x,1),22)];
        let e2=log[&pow(sub(x,2),22)];
        chars.push((e0,e1,e2));
    }
    assert_eq!(chars.len(),240);
    let mut patterns:BTreeMap<[i32;4],Vec<(i32,i32,i32,i32,usize)>>=BTreeMap::new();
    let mut rows=0;
    for a in 1..=10 {for b in 1..=10 {for c in 1..=10 {
        let d=fourth(a,b,c); if d==0||!paired(a,b,c,d) {continue;}
        let n=chars.iter().filter(|&&(e0,e1,e2)|
            (a*e0+b*e1+c*e2)%11==0).count();
        let mut key=[a,b,c,d]; key.sort();
        patterns.entry(key).or_default().push((a,b,c,d,n)); rows+=1;
    }}}
    assert_eq!(rows,270); assert_eq!(patterns.len(),15);
    let n_control=patterns[&[1,1,10,10]].iter().find(|x|x.0==1&&x.1==1&&x.2==10).unwrap().4;
    assert_eq!(n_control,0);
    let mut traces=BTreeMap::<i64,Vec<[i32;4]>>::new();
    println!("schema=ghs.cyclic11-f243-patterns/v1");
    println!("field=F3[z]/(z^5+2z+1); checked_nonzero={}; character_root={zeta}",Q-1);
    println!("ordered_rows={rows}; sorted_patterns={}",patterns.len());
    println!("exact_control_N={n_control}");
    for (key,values) in &patterns {
        let n=values[0].4;
        let agrees=values.iter().all(|v|v.4==n);
        let t_num=240_i64-11*n as i64;
        assert!(agrees,"ordered realizations disagree for {key:?}");
        assert_eq!(t_num%10,0,"nonintegral trace for {key:?}");
        let t=t_num/10;
        assert!(t*t<=972,"Hasse failure for {key:?}");
        let descended=(-3..=3).any(|s|fifth_trace(s)==t);
        println!("pattern={key:?}; rows={}; N={n}; t={t}; ordinary={}; descended={descended}",values.len(),t%3!=0);
        traces.entry(t).or_default().push(*key);
    }
    println!("distinct_traces={:?}",traces.keys().collect::<Vec<_>>());
    for (t,keys) in traces { println!("trace={t}; patterns={keys:?}"); }
}
