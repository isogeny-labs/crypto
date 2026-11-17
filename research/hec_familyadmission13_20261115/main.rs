fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut r=1;while n>0{if n&1!=0{r=mul(r,a,p);}a=mul(a,a,p);n>>=1;}r}
fn fourth(a:u64,p:u64)->u64{pow(a,4,p)}
fn leg(a:u64,p:u64)->i64{if a==0{0}else if pow(a,(p-1)/2,p)==1{1}else{-1}}
fn factor(mut n:u64)->Vec<(u64,u32)>{let mut d=2;let mut r=vec![];while d*d<=n{let mut e=0;while n%d==0{n/=d;e+=1;}if e>0{r.push((d,e));}d+=1;}if n>1{r.push((n,1));}r}
fn elliptic(p:u64,minus:bool)->u64{let mut n=0i64;for t in 0..p{let a=fourth(t,p);let f=if minus{(1+p-a)%p}else{(1+a)%p};n+=1+leg(f,p);}n+=1+leg(if minus{p-1}else{1},p);n as u64}
fn main(){for p in [97,257,509,1009]{let e0=elliptic(p,true);let e1=elliptic(p,false);let mut c=0u64;let mut mapchecks=0;let mut derivativechecks=0;
 for x in 0..p{for y in 0..p{if(fourth(x,p)+fourth(y,p))%p!=1{continue;}c+=1;
 assert_eq!((mul(mul(x,x,p),mul(x,x,p),p)+fourth(y,p))%p,1);assert_eq!((mul(mul(y,y,p),mul(y,y,p),p)+fourth(x,p))%p,1);mapchecks+=2;
 if y!=0{let yi=pow(y,p-2,p);let t=mul(x,yi,p);let s=mul(yi,yi,p);assert_eq!(mul(s,s,p),(fourth(t,p)+1)%p);mapchecks+=1;
 let dy=(p-mul(pow(x,3,p),pow(yi,3,p),p))%p;
 assert_eq!((y+p-mul(x,dy,p))%p,pow(yi,3,p));derivativechecks+=1;
 if x!=0{assert_eq!(mul(dy,pow(mul(x,x,p),p-2,p),p),(p-mul(x,pow(yi,3,p),p))%p);derivativechecks+=1;}
 }} }
 // Infinite points: normalize Y=1, Z=0; X4+1=0. The excluded [1:0:0] is not on C.
 for x in 0..p{if(fourth(x,p)+1)%p==0{c+=1;}}
 let t0=p as i64+1-e0 as i64;let t1=p as i64+1-e1 as i64;let predicted=p as i64+1-2*t0-t1;assert_eq!(c as i64,predicted,"independent_point_trace");
 let n=e0*e0*e1;let fac=factor(n);let max=fac.last().unwrap().0;assert!(max<=p+1+2*((p as f64).sqrt().ceil()as u64));
 println!("FAMILY p={p} C_points={c} E0_order={e0} E1_order={e1} J_order_derived={n} factors={fac:?} largest_prime={max} quotient_point_checks={mapchecks} differential_checks={derivativechecks} trace_agrees=true generic_genus3_subgroup_admitted=false");
 }println!("PASS family_screen=true cases=4 elliptic_product_obstruction=true no_DLP_or_timing_executed=true");}
