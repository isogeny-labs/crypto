use std::collections::BTreeMap;
mod tensor {include!("../generic-kummer/terms.rs");}
type P=BTreeMap<([u8;4],u8,u8),i64>;
fn mul(a:&P,b:&P)->P {let mut o=P::new();for ((k,c,r),v) in a {for ((l,d,s),w) in b {let mut n=[0;4];for i in 0..4{n[i]=k[i]+l[i]}*o.entry((n,c+d,r+s)).or_default()+=v*w;}}o.retain(|_,v|*v!=0);o}
fn atom(i:usize,r:u8,v:i64)->P {let mut k=[0;4];k[i]=1;P::from([((k,0,r),v)])}
fn main(){
 let mut shift=atom(2,0,1);shift.extend(atom(1,1,1));shift.extend(atom(0,2,-1));
 println!("form\tcoefficient\tc_power\tr_power\t[K1,K2,U,K4]");
 for j in 0..4 {let mut out=P::new();for &(v,c,k,l) in tensor::FORMS[j] {if l[0]!=0{continue}let mut term=P::from([(([0;4],c,l[2]+2*l[3]),v)]);for i in 0..4{for _ in 0..k[i]{term=mul(&term,&if i==2 {shift.clone()}else{atom(i,0,1)});}}for (k,v) in term{*out.entry(k).or_default()+=v;}}
 out.retain(|_,v|*v!=0);
 // Independent inverse substitution U=K3-r*K2+r²*K1 must restore the input exactly.
 let mut inv=atom(2,0,1);inv.extend(atom(1,1,-1));inv.extend(atom(0,2,1));
 let mut back=P::new();for ((k,c,r),v) in &out{let mut t=P::from([(([0;4],*c,*r),*v)]);for i in 0..4{for _ in 0..k[i]{t=mul(&t,&if i==2 {inv.clone()}else{atom(i,0,1)});}}for (k,v) in t{*back.entry(k).or_default()+=v;}}back.retain(|_,v|*v!=0);
 let mut original=P::new();for &(v,c,k,l) in tensor::FORMS[j]{if l[0]==0{*original.entry((k,c,l[2]+2*l[3])).or_default()+=v;}}original.retain(|_,v|*v!=0);assert_eq!(back,original);
 for ((k,c,r),v) in out{println!("{j}\t{v}\t{c}\t{r}\t{k:?}")}}
 eprintln!("exact_inverse_checks=4 passed=true");
}
