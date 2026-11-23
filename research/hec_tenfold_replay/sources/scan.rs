use hec_native_ic::{oracle::Roots,poly::Poly};

pub fn split<const P:u64>(u:&Poly,xs:&[u64])->Option<Roots>{
    let mut a=[0;5];
    let mut degree=u.n-1;
    a[..u.n].copy_from_slice(&u.c[..u.n]);
    let mut roots=Roots::default();
    for &x in xs{
        if degree==0{break;}
        let mut multiplicity=0;
        while degree>0{
            let mut q=[0;5];
            let mut carry=a[degree];
            for k in (0..degree).rev(){
                q[k]=carry;
                carry=(a[k]+x*carry)%P;
            }
            if carry!=0{break;}
            a=q; degree-=1; multiplicity+=1;
        }
        if multiplicity>0{
            roots.r[roots.len]=(x,multiplicity);
            roots.len+=1;
        }
    }
    (degree==0).then_some(roots)
}
