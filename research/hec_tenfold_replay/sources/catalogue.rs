use crate::prepared::{Prepared,RootTable};
use hec_native_ic::jac::Curve;

pub fn build(c:&Curve, reflected:bool, audit:bool)->Prepared {
    match c.f.deg() {
        5=>scan::<6>(c,reflected,audit),
        7=>scan::<8>(c,reflected,audit),
        9=>scan::<10>(c,reflected,audit),
        _=>crate::prepared::prepare(c,audit)
    }
}

fn scan<const N:usize>(c:&Curve,reflected:bool,audit:bool)->Prepared {
    let fp=&c.fp;
    let mut values:[u64;N]=std::array::from_fn(|i|c.f.eval(i as u64%fp.p,fp));
    let mut diff=[0;N];
    for order in 0..N {
        diff[order]=values[0];
        for i in 0..N-1-order {values[i]=fp.sub(values[i+1],values[i]);}
    }
    let roots=RootTable::build(fp);
    let mirror=reflected && (4..N).step_by(2).all(|i|c.f.c[i]==0);
    let stop=if mirror {(fp.p-1)/2} else {fp.p-1};
    let mut fb=Vec::with_capacity((fp.p/2+1) as usize);
    let mut back=Vec::new();
    let mut even=fp.add(c.f.c[0],c.f.c[0]);
    let twice_c2=fp.add(c.f.c[2],c.f.c[2]);
    let mut delta=twice_c2;
    let second=fp.add(delta,delta);
    for x in 0..=stop {
        if let Some(y)=roots.sqrt(diff[0],fp){fb.push((x,y));}
        if mirror {
            if x!=0 {
                let reflected=fp.sub(even,diff[0]);
                if audit {assert_eq!(reflected,c.f.eval(fp.p-x,fp));}
                if let Some(y)=roots.sqrt(reflected,fp){back.push((fp.p-x,y));}
            }
            if twice_c2!=0 {even=fp.add(even,delta);delta=fp.add(delta,second);}
        }
        for i in 0..N-1 {diff[i]=fp.add(diff[i],diff[i+1]);}
    }
    fb.extend(back.into_iter().rev());
    if audit {assert_eq!(fb,crate::prepared::prepare(c,true).fb);}
    Prepared{roots,fb}
}
