use hec_native_ic::jac::{Curve,Div};
pub fn coordinate<const P:u64>(c:&Curve,d:&Div,l:u64,ys:&[i32],values:&[i32],xs:&[u64],inv:&[u64],roots:&[i32],audit:bool)->Option<u64>{
    if d.u.n!=5{return None;}
    let eval=|a:&[u64],x:u64|a.iter().rev().fold(0,|s,v|(s*x+v)%P);
    let mut count=[0usize;67];
    let mut seen=[[0u64;3];67];
    for &x in xs{
        let u=eval(&d.u.c[..5],x);
        if u==0{continue;}
        let v=eval(&d.v.c[..4],x);
        let y=ys[x as usize] as u64;
        for side in 0..if y==0{1}else{2}{
            let yy=if side==0{y}else{P-y};
            let t=(yy+P-v)*inv[u as usize]%P;
            let i=t as usize;
            if count[i]==3{continue;}
            seen[i][count[i]]=x;
            count[i]+=1;
            if count[i]!=3{continue;}
            let [a,b,e]=seen[i];
            let s=(a+b+e)%P;
            let known_pairs=(a*b+b*e+a*e)%P;
            let u3=d.u.c[3];
            let total=(t*t+u3)%P;
            let pairs=(u3*total+2*P-((2*t*(d.v.c[3]+t*u3))%P)-d.u.c[2])%P;
            let rem_sum=(total+P-s)%P;
            let rem_prod=(pairs+2*P-known_pairs-s*rem_sum%P)%P;
            let disc=(rem_sum*rem_sum+4*P-4*rem_prod)%P;
            let r=roots[disc as usize];
            if r<0{continue;}
            let first=(rem_sum+r as u64)*inv[2]%P;
            let second=(rem_sum+P-first)%P;
            let mut log=0;
            let mut sum=*d;
            for xx in [a,b,e,first,second]{
                let actual=(eval(&d.v.c[..4],xx)+t*eval(&d.u.c[..5],xx))%P;
                let canon=ys[xx as usize];
                assert!(canon>=0 && (actual==canon as u64 || actual==(P-canon as u64)%P));
                let k=if canon==0{0}else{values[xx as usize] as u64};
                log+=if actual==canon as u64{k}else{l-k};
                if audit{sum=c.add(&sum,&c.place(xx,actual));}
            }
            if audit{assert!(c.is_zero(&sum));}
            return Some((l-log%l)%l);
        }
    }
    None
}
