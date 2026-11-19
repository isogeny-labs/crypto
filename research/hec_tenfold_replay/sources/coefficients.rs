use hec_native_ic::{fp::Fp, jac::{Curve, Div}, poly::Poly};

pub struct Leading {
    genus: usize,
    first: u64,
    second: u64,
    twice_v: u64,
    u: u64,
}

impl Leading {
    pub fn new(c: &Curve, anchor: &Div, quadratic: bool) -> Self {
        let fp = &c.fp;
        let g = c.g;
        let u = anchor.u.c[g - 1];
        let v = anchor.v.c[g - 1];
        let first = fp.sub(c.f.c[2*g], u);
        let second = if quadratic {
            fp.sub(fp.sub(c.f.c[2*g - 1], anchor.u.c[g-2]), fp.mul(u,first))
        } else { 0 };
        Self { genus:g, first, second, twice_v:fp.pre(fp.add(v,v)), u:fp.pre(u) }
    }

    pub fn residual(&self, fp:&Fp, t:u64, known:&[(u64,u64)], squares:&[u64]) -> Poly {
        if known.len()==self.genus+1 {return Poly::one();}
        let quadratic=known.len()+1==self.genus;
        let mut sum=0;
        let mut pairs=0;
        for (i,&(x,_)) in known.iter().enumerate() {
            if quadratic && i>0 {pairs=fp.add(pairs,fp.mul(sum,x));}
            sum=fp.add(sum,x);
        }
        let square=if squares.is_empty(){fp.mul(t,t)}else{squares[t as usize]};
        let a=fp.add(fp.sub(self.first,square),sum);
        if !quadratic {return Poly::from_coeffs(&[a,1]);}
        let j=fp.sub(fp.sub(self.second,fp.mul_pre(t,self.twice_v)),fp.mul_pre(square,self.u));
        let b=fp.sub(fp.add(j,fp.mul(a,sum)),pairs);
        Poly::from_coeffs(&[b,a,1])
    }
}

pub fn squares(fp:&Fp)->Vec<u64>{
    let mut table=Vec::with_capacity(fp.p as usize);
    let (mut square,mut difference)=(0,1);
    for _ in 0..fp.p {
        table.push(square);
        square=fp.add(square,difference);
        difference=fp.add(difference,2);
    }
    table
}
