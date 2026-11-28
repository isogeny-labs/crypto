use std::collections::BTreeMap;
mod tensor {include!("../generic-kummer/terms.rs");}
type P=BTreeMap<[u8;6],i64>;
fn a(i:usize)->P{let mut e=[0;6];e[i]=1;P::from([(e,1)])}
fn add(a:&P,b:&P)->P{let mut o=a.clone();for(k,v)in b{*o.entry(*k).or_default()+=v}o.retain(|_,v|*v!=0);o}
fn scale(a:&P,n:i64)->P{a.iter().filter_map(|(k,v)|if v*n==0{None}else{Some((*k,v*n))}).collect()}
fn sub(a:&P,b:&P)->P{add(a,&scale(b,-1))}
fn mul(a:&P,b:&P)->P{let mut o=P::new();for(k,v)in a{for(l,w)in b{let mut e=[0;6];for i in 0..6{e[i]=k[i]+l[i]}*o.entry(e).or_default()+=v*w}}o.retain(|_,v|*v!=0);o}
fn pow(a:&P,n:u8)->P{let mut o=P::from([([0;6],1)]);for _ in 0..n{o=mul(&o,a)}o}
fn main(){let(x,y,k,z,r,c)=(a(0),a(1),a(2),a(3),a(4),a(5));
let h=sub(&y,&mul(&r,&x));let u=add(&sub(&k,&mul(&r,&y)),&mul(&pow(&r,2),&x));let s=add(&pow(&r,5),&c);
let b0=pow(&u,2);
let b1=add(&mul(&u,&sub(&sub(&z,&mul(&pow(&r,2),&y)),&scale(&mul(&pow(&r,3),&x),2))),&scale(&mul(&s,&pow(&x,2)),2));
let l2=add(&sub(&sub(&mul(&r,&z),&mul(&pow(&r,2),&u)),&mul(&pow(&r,3),&y)),&scale(&mul(&pow(&r,4),&x),3));
let b2=add(&mul(&u,&l2),&scale(&mul(&s,&mul(&x,&h)),2));
let l3=sub(&add(&add(&mul(&pow(&r,2),&z),&scale(&mul(&pow(&r,3),&u),2)),&scale(&mul(&pow(&r,4),&y),4)),&mul(&add(&scale(&pow(&r,5),4),&scale(&c,2)),&x));
let b3=add(&mul(&u,&l3),&scale(&mul(&s,&pow(&h,2)),2));
for(j,b)in [b0,b1,b2,b3].iter().enumerate(){let mut target=P::new();for &(v,cp,k,l)in tensor::FORMS[j]{if l[0]==0{let e=[k[0],k[1],k[2],k[3],l[2]+2*l[3],cp];*target.entry(e).or_default()+=v;}}target.retain(|_,v|*v!=0);assert_eq!(*b,target,"form {j}");println!("form={j} exact_original_tensor_identity=true terms={}",b.len());}
}
