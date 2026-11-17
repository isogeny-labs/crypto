// Included inside `signed`: candidate construction uses no target scalar/oracle.
fn same_projective(x:[u64;4], y:[u64;4], p:u64)->bool {
    if y==[0;4] {return false}
    let Some(i)=x.iter().position(|&v|v!=0) else {return false};
    (0..4).all(|j| m(x[i],y[j],p)==m(x[j],y[i],p))
}
fn pow_counted(mut x:u64,mut n:u64,p:u64)->u64 {
    let mut z=1;
    while n>0 {if n&1!=0 {z=m(z,x,p)} x=m(x,x,p); n>>=1;}
    z
}
fn sqrt_counted(n:u64,p:u64)->Option<u64> {
    if n==0 {return Some(0)}
    if pow_counted(n,(p-1)/2,p)!=1 {return None}
    let mut q=p-1;let mut s=0u32;
    while q&1==0 {q>>=1;s+=1}
    let mut z=2;
    while pow_counted(z,(p-1)/2,p)!=p-1 {z+=1}
    let mut c=pow_counted(z,q,p);
    let mut t=pow_counted(n,q,p);
    let mut r=pow_counted(n,q.div_ceil(2),p);
    let mut level=s;
    while t!=1 {
        let mut i=0u32;let mut v=t;
        while v!=1 {v=m(v,v,p);i+=1;if i>=level {return None}}
        let b=pow_counted(c,1u64<<(level-i-1),p);
        level=i;c=m(b,b,p);t=m(t,c,p);r=m(r,b,p);
    }
    Some(r)
}
fn roots(x:u64,p:u64)->Vec<u64>{
    match sqrt_counted(x,p) {None=>vec![],Some(v) if v==0=>vec![0],Some(v)=>vec![v,p-v]}
}
fn degree_two_candidates(q:[u64;4],f:&[u64],p:u64)->Vec<Divisor>{
    let (a0,a1)=(q[2],s(0,q[1],p));
    let z=s(m(a1,a1,p),a0,p);
    let b1sq=a(s(s(q[3],m(z,s(m(f[5],a1,p),m(f[6],z,p),p),p),p),m(a1,s(f[3],m(f[4],a1,p),p),p),p),f[2],p);
    let mut power=[1,0];let mut rem=[0,0];
    for &coeff in f {
        rem[0]=a(rem[0],m(coeff,power[0],p),p);
        rem[1]=a(rem[1],m(coeff,power[1],p),p);
        power=[s(0,m(a0,power[1],p),p),s(power[0],m(a1,power[1],p),p)];
    }
    let mut out=Vec::new();
    for b1 in roots(b1sq,p){
        let b0s=if b1!=0 {
            vec![m(a(rem[1],m(a1,b1sq,p),p),inv(m(2,b1,p),p),p)]
        }else if rem[1]==0 {roots(rem[0],p)}else{vec![]};
        for b0 in b0s {out.push(Divisor{degree:2,u:[a0,a1,1],v:[b0,b1]});}
    }
    out
}
fn bounded_lift(g:Divisor,k:[u64;4],kp:[u64;4],t:&Value,acts:&[(u64,[[u64;4];4])],j:&Jacobian,p:u64)->(Option<Divisor>,usize,usize,&'static str){
    let f=curve(t,p);let mut tested=0usize;let mut zero_b1=0usize;
    let k0=["a","b","c","d"].map(|x|t[x].as_u64().unwrap());
    if same_projective(k,k0,p){
        tested+=1;
        if let Some(plus)=super::theta::projection(g,j,t,acts,p){
            if same_projective(plus,kp,p){return (Some(Divisor::O),tested,zero_b1,"identity")}
        }
    }
    // Each action is translation by a known nonzero 2-torsion point.
    for branch in 0..=acts.len(){
        let (translated,torsion)=if branch==0 {(k,None)} else {
            let (root,act)=acts[branch-1];
            (linear(k,act,p),Some(Divisor{degree:1,u:[s(0,root,p),1,0],v:[0,0]}))
        };
        let Some(q)=general(translated,t,p) else {continue};
        for mut candidate in degree_two_candidates(q,&f,p){
            tested+=1;
            zero_b1+=usize::from(candidate.v[1]==0);
            if !j.valid(candidate){continue}
            if let Some(torsion)=torsion {candidate=j.add(candidate,torsion)}
            let Some(check)=super::theta::projection(candidate,j,t,acts,p) else {continue};
            if !same_projective(check,k,p){continue}
            let plus=j.add(candidate,g);
            let Some(check_plus)=super::theta::projection(plus,j,t,acts,p) else {continue};
            if same_projective(check_plus,kp,p){
                let label=["direct","torsion_0","torsion_1","torsion_2","torsion_3","torsion_4"][branch];
                return (Some(candidate),tested,zero_b1,label)
            }
        }
    }
    (None,tested,zero_b1,"no_matching_candidate")
}
