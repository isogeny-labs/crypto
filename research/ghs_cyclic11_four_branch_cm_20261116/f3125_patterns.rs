use std::collections::{BTreeMap,BTreeSet};
const P: usize=5;
const Q: usize=3125;
#[derive(Clone,Copy)]
struct Field { c0:i32,c1:i32 }
fn modp(x:i32)->i32 {x.rem_euclid(P as i32)}
fn digits(mut a:usize)->[i32;5] {
    let mut d=[0;5]; for x in &mut d {*x=(a%P) as i32;a/=P;} d
}
fn pack(d:[i32;5])->usize {
    let (mut n,mut k)=(0,1);
    for x in d {n+=modp(x) as usize*k;k*=P;} n
}
impl Field {
    fn sub(self,a:usize,b:usize)->usize {
        let x=digits(a);let y=digits(b);pack(std::array::from_fn(|i|x[i]-y[i]))
    }
    fn mul(self,a:usize,b:usize)->usize {
        let x=digits(a);let y=digits(b);let mut d=[0_i32;9];
        for i in 0..5 {for j in 0..5 {d[i+j]+=x[i]*y[j];}}
        for k in (5..=8).rev() {
            let c=modp(d[k]);
            d[k-5]-=self.c0*c;d[k-4]-=self.c1*c;
        }
        pack([d[0],d[1],d[2],d[3],d[4]])
    }
    fn pow(self,mut a:usize,mut n:usize)->usize {
        let mut b=1;while n>0 {if n&1==1 {b=self.mul(b,a);}a=self.mul(a,a);n>>=1;}b
    }
}
fn irreducible(c0:i32,c1:i32)->bool {
    for r in 0_i32..5 {
        if modp(r.pow(5)+c1*r+c0)==0 {return false;}
    }
    for u in 0..5 {for v in 0..5 {
        let mut d=[c0,c1,0,0,0,1];
        for k in (2..=5).rev() {
            let c=modp(d[k]);d[k]-=c;d[k-1]-=u*c;d[k-2]-=v*c;
        }
        if modp(d[0])==0&&modp(d[1])==0 {return false;}
    }}
    true
}
fn fourth(a:i32,b:i32,c:i32)->i32 {(-(a+b+c)).rem_euclid(11)}
fn paired(a:i32,b:i32,c:i32,d:i32)->bool {
    ((a+b)%11==0&&(c+d)%11==0)||((a+c)%11==0&&(b+d)%11==0)||
    ((a+d)%11==0&&(b+c)%11==0)
}
fn fifth_trace(s:i64)->i64 {s.pow(5)-25*s.pow(3)+125*s}
fn main() {
    let field=(1..=4).flat_map(|c0|(0..=4).map(move |c1|(c0,c1)))
        .find(|&(c0,c1)|irreducible(c0,c1))
        .map(|(c0,c1)|Field{c0,c1}).expect("irreducible polynomial");
    for a in 1..Q {assert_eq!(field.pow(a,Q-1),1,"non-field element {a}");}
    let zeta=(2..Q).map(|g|field.pow(g,(Q-1)/11)).find(|&z|z!=1)
        .expect("order-11 root");
    assert_eq!(field.pow(zeta,11),1);
    let mut log=BTreeMap::new();let mut z=1;
    for e in 0..11 {assert!(log.insert(z,e).is_none());z=field.mul(z,zeta);}
    assert_eq!(z,1);
    println!("schema=ghs.cyclic11-f3125-patterns/v1");
    println!("field=F5[z]/(z^5+{}z+{}); checked_nonzero={}; character_root={zeta}",field.c1,field.c0,Q-1);
    let mut overall=BTreeMap::<i64,Vec<(i32,[i32;4])>>::new();
    for lambda in 2..=4 {
        let mut chars=Vec::new();
        for x in 0..Q {
            if x==0||x==1||x==lambda {continue;}
            chars.push((log[&field.pow(x,(Q-1)/11)],
                log[&field.pow(field.sub(x,1),(Q-1)/11)],
                log[&field.pow(field.sub(x,lambda),(Q-1)/11)]));
        }
        assert_eq!(chars.len(),Q-3);
        let mut classes=BTreeMap::<[i32;4],BTreeSet<(usize,i64)>>::new();
        let mut rows=0;
        for a in 1..=10 {for b in 1..=10 {for c in 1..=10 {
            let d=fourth(a,b,c);if d==0||!paired(a,b,c,d){continue;}
            let n=chars.iter().filter(|&&(e0,e1,e2)|
                (a*e0+b*e1+c*e2)%11==0).count();
            let t_num=3122_i64-11*n as i64;
            assert_eq!(t_num%10,0,"nonintegral trace lambda={lambda} exponents={a},{b},{c},{d}");
            let t=t_num/10;
            assert!(t*t<=4*3125,"Hasse failure lambda={} exponents={},{},{},{}",lambda,a,b,c,d);
            let mut key=[a,b,c,d];key.sort();
            classes.entry(key).or_default().insert((n,t));
            overall.entry(t).or_default().push((lambda as i32,key));
            rows+=1;
        }}}
        assert_eq!(rows,270);
        println!("lambda={lambda}; ordered_rows={rows}; sorted_patterns={}",classes.len());
        for (key,values) in classes {
            println!("lambda={lambda}; pattern={key:?}; N_and_trace={values:?}");
        }
    }
    for (t,rows) in overall {
        let descended=(-4..=4).any(|s|fifth_trace(s)==t);
        let unique:BTreeSet<_>=rows.into_iter().collect();
        println!("trace={t}; ordinary={}; descended={descended}; pattern_lambda_pairs={}",t%5!=0,unique.len());
        if t%5!=0&&!descended {println!("NON_DESCENDED_ORDINARY_TRACE={t}; exact_pairs={unique:?}");}
    }
}
