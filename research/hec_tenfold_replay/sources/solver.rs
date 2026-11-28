use crate::linear::{put, Linear};
use crate::table_oracle::{split, OracleStats};
use hec_native_ic::{
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
};
use std::collections::BTreeMap;
#[path="inverse.rs"] mod inverse;

#[derive(Clone, Copy)]
pub struct Options {pub quarters:usize,pub admission:usize,pub initial_pencils:usize,pub coefficient_mode:usize,pub small_mode:usize,pub priority:usize,pub balance:usize,pub reciprocal:bool,pub step_choice:usize,
    pub normalize: bool,
    pub finish: bool,
    pub seed: u64,
    pub certificate: bool,
    pub rolling: bool,
    pub carry: bool,
    pub indexed: bool,
    pub mode: usize,
    pub eager: bool,
    pub delay: bool,
    pub layout: usize,
    pub integrated: bool,
    pub defer: usize,
    pub cutoff: usize,
    pub substitute: bool,
    pub fused: bool,
}
#[derive(Debug, Default)]
pub struct Report {
    pub k: Option<u64>,
    pub multiple: usize,
    pub anchors: usize,
    pub unready: usize,
    pub expanded: usize,
    pub direct: usize,
    pub degree_skips: usize,
    pub scanned: usize,
    pub u_zero: usize,
    pub buckets: usize,
    pub nonsplit: usize,
    pub accepted: usize,
    pub repeated: usize,
    pub overlaps: usize,
    pub dense_audits: usize,
    pub rows: usize,
    pub pivots: usize,
    pub small: usize,
    pub saved: usize,
    pub replayed: usize,
    pub catalogue: usize,
    pub zero_anchor: bool,
    pub final_rejected: bool,
    pub public_split: usize,
    pub public_nonsplit: usize,
    pub public_inserted: usize,
    pub public_pending: usize,
    pub public_probes: usize,
    pub branch_rows: usize,
    pub finish: crate::finish::Stats,
}
fn brute(mut f: Poly, c: &Curve) -> (bool, BTreeMap<u64, u32>) {
    let mut roots = BTreeMap::new();
    for x in 0..c.fp.p {
        while f.deg() > 0 && f.eval(x, &c.fp) == 0 {
            *roots.entry(x).or_insert(0) += 1;
            f = f.div_exact(&Poly::x_minus(x, &c.fp), &c.fp);
        }
    }
    (f.deg() == 0, roots)
}
pub fn solve<const HARVEST: bool, const EXTRA:u8,const SHIFT:bool>(
    c: &Curve,
    d: &Div,
    target: &Div,
    ell: u64,
    limit: usize,
    audit: bool,
    prepared: &crate::prepared::Prepared,
    powers: Option<&crate::powers::Powers>,
    signed: bool,
    shared:Option<&[u64]>,
    options: Options,
) -> Report {
    let recover = if true { crate::centered::recover } else { split };
    let product = |k| {let _product=crate::phases::timer(2);let result=match powers {Some(p) => p.product(c,k,signed), None => c.mul(d,k)};result};
    assert_eq!(prepared.roots.p, c.fp.p);
    let fb = prepared.fb.clone();
    let len = fb.len();
    let mut m = 1usize;
    while (m as u128).pow(c.g as u32) < (len as u128).pow((c.g - 1) as u32) {
        m += 1;
    }
    let width = match options.mode {
        1 => m,
        2 | 3 => (2 * m).min(len),
        4 => len,
        _ => unreachable!(),
    };
    let width=if options.integrated && c.g==4 {(2*width).min(len)} else {width};
    let step_index = fb[..m].iter().position(|&(_, y)| y != 0).unwrap();
    let m = match options.cutoff {
        0 => m,
        1 => (m + 1) / 2,
        2 => (2 * m).min(len),
        3 => step_index + 1,
        _ => unreachable!(),
    }
    .max(step_index + 1);
    let m=if options.integrated && c.g==4 {width} else {m};
    assert_eq!(m,width);
    let width=(width*options.quarters).div_ceil(4).min(len);
    let m=width;
    if audit {println!("BASIS {} {} {} {} {}",c.g,c.fp.p,options.quarters,width,len);}

    let threshold = if EXTRA>0 {c.g-1} else {match options.mode {
        1 | 2 => c.g - 1,
        3 => c.g,
        4 => c.g + 1,
        _ => unreachable!(),
    }};
    let mut index = vec![usize::MAX; c.fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        index[x as usize] = j;
    }
    let mut linear = crate::dag::Dag::new(m, len, ell, audit, options.eager);
    // Each entry starts below ell and receives at most m+1 unreduced products.
    let largest = ell - 1;
    linear.delay = options.delay && largest <= (u64::MAX - largest) / largest / (m + 1) as u64;
    if options.reciprocal && linear.delay {linear.reciprocal=u64::MAX/ell;}
    linear.substitute = options.substitute;

    let mut out = Report {
        small: m,
        catalogue: len,
        ..Report::default()
    };
    if options.normalize {
        for (j, &(_, y)) in fb.iter().enumerate() {
            if y == 0 {
                let mut row = BTreeMap::new();
                put(&mut row, if j < m { j } else { j + 1 }, 1, &linear.f);
                assert_eq!({let _push=crate::phases::timer(3);linear.push(row, 0, audit)}, None);
                out.branch_rows += 1;
            }
        }
    }
    let (mut pending, nonsplit) = if options.normalize {
        crate::public::rows(c, d, target, m, &fb, &index, ell, audit, prepared)
    } else {
        (vec![], 0)
    };
    out.public_split = pending.len();
    out.public_nonsplit = nonsplit;
    if options.integrated && (c.g==2 || (HARVEST && c.g==3)) {
        let table=powers.expect("checked source table");
        let mut admitted=0;
        for (rhs,point,partial) in table.multiples().iter().enumerate().skip(1)
            .map(|(i,d)|(1u64<<i,d,false))
            .chain(table.partial.iter().map(|(k,d)|(*k,d,true))) {

            let roots = if c.g==3 {
                let found = crate::harvest::recover(&point.u,&c.fp,&fb[..m],&prepared.roots);
                if audit {
                    let expected = split(&point.u,&c.fp,&mut OracleStats::default(),&prepared.roots)
                        .filter(|roots| point.u.n<4 || roots.as_slice().iter().any(|&(x,_)| index[x as usize]<m));
                    assert_eq!(found, expected);
                }
                found
            } else {
                recover(&point.u,&c.fp,&mut OracleStats::default(),&prepared.roots)
            };
            let Some(roots)=roots else {continue;};

            let mut row = BTreeMap::new();
            let mut check = c.zero();
            for &(px,count) in roots.as_slice() {
                let i = index[px as usize];
                let py = point.v.eval(px,&c.fp);
                let sign = if py == fb[i].1 {1} else {assert_eq!(py,c.fp.neg(fb[i].1)); -1};
                put(&mut row,if i<m {i} else {i+1},linear.f.from_i64(sign*count as i64),&linear.f);
                if audit {
                    for _ in 0..count {check=c.add_place(&check,px,py);}
                }
            }
            if partial {admitted+=1;}
            if audit {
                assert_eq!(check,*point);
                assert_eq!(check,c.mul(d,rhs));
            }
            pending.push(Some((row,rhs)));

        }
        if audit {println!("HARVEST {} {} {} {}",c.g,c.fp.p,table.partial.len(),admitted); }
    }
    let defer=options.defer!=0 && (c.g==2 || c.g==4);
    const USED:usize=usize::MAX;
    let mut state=vec![0usize;if defer {len} else {0}];
    let mut recipes=Vec::<Option<crate::recipe::Recipe>>::new();
    if defer {
        assert!(width<=m);
        for (i,&(_,y)) in fb.iter().enumerate() {if y==0 {state[i]=USED;}}
        for (row,_) in pending.iter().flatten() {
            for (&col,_) in row.range(m+1..) {state[col-1]=USED;}
        }
    }
    let j=match options.step_choice {
        0=>step_index,
        1=>(0..m).map(|offset|(m/2+offset)%m).find(|&i|fb[i].1!=0).unwrap(),
        2=>(0..m).rev().find(|&i|fb[i].1!=0).unwrap(),
        _=>unreachable!()
    };
    if audit {println!("STEP {} {} {} {} {}",c.g,c.fp.p,options.step_choice,j,fb[j].0);}
    let (x, y) = fb[j];
    if options.certificate {
        let mut trial = c.zero();
        for n in 1..=32 {
            trial = c.add_place(&trial, x, y);
            if trial == *d {
                let mut row = BTreeMap::new();
                put(&mut row, j, n, &linear.f);
                assert_eq!({let _push=crate::phases::timer(3);linear.push(row, 1, audit)}, None);
                out.multiple = n as usize;
                break;
            }
        }
    }
    crate::phases::mark("walk_setup");
    let mut streams = [*target, *d];
    let mut cofactors = streams.map(|s| {
        if options.carry {
            crate::carried::cofactor(c, &s)
        } else {
            Poly::zero()
        }
    });
    let fp = &c.fp;
    let p = fp.p as usize;
    let mut packed = crate::compact::Compact::new(if true {p} else {0});
    let mut bounded = crate::bounded::Bounded::new(if !true && options.layout>0 {p} else {0},options.layout==2);
    let mut indexed = crate::buckets::Buckets::new(if options.indexed && ((!true && options.layout==0) || audit) { p } else { 0 });
    let owned;let inverse=if let Some(values)=shared{values}else{
    let mut inverse = vec![0; p];
    inverse[1] = 1;
    for i in 2..p {
        inverse[i] = fp.neg(fp.mul((p / i) as u64, inverse[p % i]));
    }
    owned=inverse; &owned[..]};
    let dx_mont: Vec<_> = if true {
        fb[..width].iter().map(|&(px, _)| fp.pre(fp.sub(px, x))).collect()
    } else { Vec::new() };
    let mut biases=[0u64;2];
    let mut caches = [vec![None::<[u64; 2]>; width], vec![None::<[u64; 2]>; width]];

    let mut slots=vec![0u16;if EXTRA==3{p*p}else{0}];
    let mut stored=Vec::<([i16;3],usize,usize,bool)>::new();
    let mut pairs=
BTreeMap::<(u64,u64),(BTreeMap<usize,u64>,u64)>::new();
    let squares=if options.coefficient_mode==2 {crate::coefficients::squares(fp)} else {Vec::new()};
    let mut startup_sides=[false;2];
    let mut translations=[0usize;2];
    'anchors: for a in 1..=limit {
        crate::phases::mark("pending");
        for entry in &mut pending {
            let Some((row, _)) = entry.as_ref() else {
                continue;
            };
            out.public_probes += 1;
            if !linear.ready(row) {
                continue;
            }
            let (row, rhs) = entry.take().unwrap();
            out.public_inserted += 1;
            if options.balance!=0 && a==1 {startup_sides[usize::from(rhs!=0)]=true;}
            crate::phases::mark("linear");
            let mut answer = {let _push=crate::phases::timer(3);linear.push(row, rhs, audit)};

            if let Some(k) = answer {
                if product(k) == *target {
                    out.k = Some(k);
                } else {
                    out.final_rejected = true;
                }
                break 'anchors;
            }
        }
        crate::phases::mark("anchor");
        let anchor_timer=crate::phases::timer(6);out.anchors = a;
        let prefix = if out.multiple==0 {options.initial_pencils} else {0};
        let (stream,n) = if a<=prefix {
            (a-1,0)
        } else {
            let translated=a-prefix;
            if out.multiple!=0 {(0,translated)} else if options.balance!=0 && startup_sides[0]!=startup_sides[1] {
                let missing=usize::from(startup_sides[0]);
                let period=if options.balance==1 {4} else {8};
                let side=if translated%period==0 {1-missing} else {missing};
                translations[side]+=1;
                (side,translations[side])
            } else {((translated-1)%2,(translated+1)/2)}

        };
        let previous = streams[stream];
        let anchor = if n==0 {previous} else if options.carry {
            let cached = if previous.u.n == c.g + 1 {
                caches[stream][j].map(|q| if SHIFT {fp.sub(q[0],biases[stream])}else{q[0]})
            } else {
                None
            };
            let next =
                crate::carried::step(c, &previous, &mut cofactors[stream], x, y, &inverse, cached);
            if audit {
                assert_eq!(next, c.add_place(&previous, x, y));
                assert_eq!(cofactors[stream], crate::carried::cofactor(c, &next));
            }
            next
        } else {
            c.add_place(&previous, x, y)
        };

        drop(anchor_timer);streams[stream] = anchor;
        if audit {
            let start = if stream == 0 { target } else { d };
            assert_eq!(anchor, c.add(start, &c.mul(&c.place(x, y), n as u64)));
        }
        let rhs = if stream == 0 { 0 } else { linear.f.neg(1) };
        if c.is_zero(&anchor) {
            let mut row = BTreeMap::new();
            put(&mut row, j, n as u64, &linear.f);
            if stream == 0 {
                put(&mut row, m, 1, &linear.f);
            }
            crate::phases::mark("linear");
            if let Some(k) = {let _push=crate::phases::timer(3);linear.push(row, rhs, audit)} {
                if product(k) == *target {
                    out.k = Some(k)
                } else {
                    out.final_rejected = true
                }
                break;
            }
            continue;
        }
        if anchor.u.deg() != c.g as isize {
            out.degree_skips += 1;
            continue;
        }
        let scan=crate::phases::timer(4);
        let cache = &mut caches[stream];
        let slope = if options.rolling && previous.u.deg() == c.g as isize {
            cache[j].map(|s| s[0])
        } else {
            None
        };
        let bias=if SHIFT {slope.map_or(0,|k|fp.sub(k,biases[stream]))}else{0}; biases[stream]=bias;
        let mut buckets: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
        indexed.clear();
        bounded.clear();
        packed.clear(); let mut put_point = |t, p, tag| {
            if true {packed.push(t,tag,threshold); if audit {indexed.push(t,p,threshold);}} else if options.layout>0 {
                bounded.push(t,p,threshold);
                if audit {indexed.push(t,p,threshold);}
            } else if options.indexed {
                indexed.push(t, p, threshold)
            } else {
                buckets.entry(t).or_default().push(p);
            }
        };
        for (i, &(px, py)) in fb[..width].iter().enumerate() {
            out.scanned += 1;
            let direct = || {
                let u = anchor.u.eval(px, fp);
                if u == 0 {
                    return None;
                }
                let v = anchor.v.eval(px, fp);
                Some([py, fp.neg(py)].map(|yy| fp.mul(fp.sub(yy, v), inverse[u as usize])))
            };
            let values = match (slope, cache[i]) {
                (Some(k), Some(old)) if px != x && !old.contains(&k) => {
                    let dx = if true { dx_mont[i] } else { fp.sub(px, x) };
                    Some(old.map(|t| {
                        let q=fp.mul_pre(inverse[fp.sub(t,k) as usize],dx);
                        if SHIFT {q} else {fp.sub(q,k)}
                    }))
                }
                _ => direct().map(|pair|if SHIFT {pair.map(|t|fp.add(t,bias))}else{pair}),
            };
            if audit {
                assert_eq!(values.map(|pair|pair.map(|t|fp.sub(t,bias))), direct(), "slope state versus Mumford evaluation");
            }
            cache[i] = values;
            if let Some(t) = values {
                put_point(t[0], (px, py), 2*i as u32);
                if py != 0 {
                    put_point(t[1], (px, fp.neg(py)), 2*i as u32+1);
                }
            } else {
                out.u_zero += 1;
            }
        }
        if true { packed.finish(&fb[..width],fp.p,options.priority);
            if audit {
                let mut actual:Vec<_>=packed.qualifying().collect();
                if options.priority==2 {assert!(actual.windows(2).all(|v|v[0].1.len()>=v[1].1.len()));}
                actual.sort_unstable_by_key(|v|v.0);
                assert!(actual.into_iter().eq(indexed.qualifying().map(|(t,v)|(t,v.as_slice()))));
            } } else if options.layout>0 {
            bounded.finish(cache,&fb[..width],fp.p,threshold);
            if audit {
                assert!(bounded.qualifying().eq(indexed.qualifying().map(|(t,v)|(t,v.as_slice()))));
            }
        }
        drop(scan);let leading=(options.coefficient_mode!=0).then(||crate::coefficients::Leading::new(c,&anchor,threshold<c.g));for (t,known) in buckets.iter().map(|(t,k)|(*t,k.as_slice()))
            .chain(indexed.qualifying().take(if !true && options.layout==0 {usize::MAX} else {0}).map(|(t,v)|(t,v.as_slice())))
            .chain(bounded.qualifying()).chain(packed.qualifying()) {
            let t=if SHIFT {fp.sub(t,bias)}else{t};
            if known.len() < threshold {
                continue;
            }
            crate::phases::mark("row");
            let _row=crate::phases::timer(5);out.buckets += 1;
            let remainder=match &leading {Some(h)=>h.residual(fp,t,known,&squares),None=>crate::tail::extract(c,&anchor,t,known)}; if audit {assert_eq!(remainder,crate::tail::extract(c,&anchor,t,known));}
            let h = if audit {
                let vt = anchor.v.add(&anchor.u.scale(t, &c.fp), &c.fp);
                let norm = c.f.sub(&vt.mul(&vt, &c.fp), &c.fp);
                let h = norm.div_exact(&anchor.u, &c.fp);
                assert_eq!(h.deg(), (c.g + 1) as isize);
                assert_eq!(h.lc(), 1);
                let mut reference = h;
                let mut known_poly = Poly::one();
                for &(x, y) in known {
                    assert_eq!(vt.eval(x, &c.fp), y);
                    let factor = Poly::x_minus(x, &c.fp);
                    reference = reference.div_exact(&factor, &c.fp);
                    known_poly = known_poly.mul(&factor, &c.fp);
                }
                assert_eq!(reference, remainder);
                assert_eq!(known_poly.mul(&remainder, &c.fp), h);
                assert_eq!(anchor.u.mul(&h, &c.fp), norm);
                Some(h)
            } else {
                None
            };
            let residual = recover(
                &remainder,
                &c.fp,
                &mut OracleStats::default(),
                &prepared.roots,
            );
            let truth = if audit && options.mode != 4 {
                Some(brute(h.unwrap(), c))
            } else {
                None
            };
            if let Some((all, _)) = &truth {
                assert_eq!(residual.is_some(), *all);
            }
            let Some(residual) = residual else {
                out.nonsplit += 1;
                if EXTRA==3 && remainder.n==3{
                    assert_eq!(known.len(),3);
                    let sum=fp.neg(remainder.c[1]);let prod=remainder.c[0];let(mut a,mut b)=(t,0);
                    for i in (0..c.g).rev(){let h=fp.add(anchor.v.c[i],fp.mul(t,anchor.u.c[i]));let next=fp.add(a,fp.mul(b,sum));a=fp.sub(h,fp.mul(b,prod));b=next;}
                    let z=if a==0{b}else{a};let reverse=z>fp.neg(z);
                    let mut tags=[0i16;3];let mut check=anchor;
                    for(k,&(px,py))in known.iter().enumerate(){let i=index[px as usize];assert!(i<m && i<32767);tags[k]=if py==fb[i].1{(i+1)as i16}else{-((i+1)as i16)};if audit{check=c.add_place(&check,px,py);}}
                    if audit{let v=anchor.v.add(&anchor.u.scale(t,fp),fp).rem(&remainder,fp);assert_eq!((v.c[0],v.c[1]),(a,b));assert!(c.is_zero(&c.add(&check,&Div{u:remainder,v})));}
                    let rec=(tags,n,stream,reverse);
                    let slot=(sum as usize)*p+prod as usize;
                    if z!=0 && slots[slot]==0{stored.push(rec);slots[slot]=u16::try_from(stored.len()).unwrap();continue;}
                    let mut row=BTreeMap::new();let mut right=0;
                    let mut emit=|(ids,step,side,rev):([i16;3],usize,usize,bool),subtract:bool|{
                        let neg=rev^subtract;
                        let sign=|value:u64,flip:bool|if neg^flip{linear.f.neg(value)}else{value};
                        for tag in ids{put(&mut row,tag.unsigned_abs()as usize-1,sign(1,tag<0),&linear.f);}
                        put(&mut row,j,sign(step as u64,false),&linear.f);
                        if side==0{put(&mut row,m,sign(1,false),&linear.f);}else{right=linear.f.add(right,sign(1,true));}
                    };
                    emit(rec,false);if z!=0{emit(stored[slots[slot]as usize-1],true);}
                    if let Some(k)={let _push=crate::phases::timer(3);linear.push(row,right,audit)}{assert_eq!(product(k),*target);out.k=Some(k);break 'anchors;}
                }
                if EXTRA==2 && remainder.n==3{

                    let sum=fp.neg(remainder.c[1]);let prod=remainder.c[0];let(mut a,mut b)=(t,0);
                    for i in (0..c.g).rev(){let h=fp.add(anchor.v.c[i],fp.mul(t,anchor.u.c[i]));let next=fp.add(a,fp.mul(b,sum));a=fp.sub(h,fp.mul(b,prod));b=next;}
                    let z=if a==0{b}else{a};let reverse=z>fp.neg(z);
                    let mut row=BTreeMap::new();let mut check=anchor;
                    for &(px,py)in known{let i=index[px as usize];assert!(i<m);let v=if py==fb[i].1{1}else{linear.f.neg(1)};put(&mut row,i,v,&linear.f);if audit{check=c.add_place(&check,px,py);}}
                    if audit{let v=anchor.v.add(&anchor.u.scale(t,fp),fp).rem(&remainder,fp);assert_eq!((v.c[0],v.c[1]),(a,b));assert!(c.is_zero(&c.add(&check,&Div{u:remainder,v})));}
                    put(&mut row,j,n as u64,&linear.f);if stream==0{put(&mut row,m,1,&linear.f);}
                    let mut right=rhs;
                    if z!=0{if reverse{for v in row.values_mut(){*v=linear.f.neg(*v);}right=linear.f.neg(right);}
                        if let Some((old,h))=pairs.get(&(prod,sum)){for(&i,&v)in old{put(&mut row,i,linear.f.neg(v),&linear.f);}right=linear.f.sub(right,*h);}
                        else{pairs.insert((prod,sum),(row,right));continue;}
                    }
                    if let Some(k)={let _push=crate::phases::timer(3);linear.push(row,right,audit)}{assert_eq!(product(k),*target);out.k=Some(k);break 'anchors;}
                }

                continue;
            };
            let mut audit_recipe=None;
            if defer && remainder.n==2 {
                let px=fp.neg(remainder.c[0]);
                let i=index[px as usize];
                if i>=m && state[i]!=USED {
                    assert_eq!(known.len(),c.g);
                    if state[i]==0 {
                        let rec=crate::recipe::Recipe::new(known,i,&fb,&index,&anchor,t,n as u64,stream==0,options.defer==2,fp);
                        if audit {audit_recipe=Some(rec.build(&fb,j,m,fp,&linear.f));}
                        recipes.push(Some(rec));
                        state[i]=recipes.len();
                        out.saved+=1;
                        if !audit {out.accepted+=1;continue;}
                    } else {
                        let old=recipes[state[i]-1].take().unwrap();
                        let (saved,srhs)=old.build(&fb,j,m,fp,&linear.f);
                        crate::phases::mark("linear");
                        out.replayed += 1;
                        state[i] = USED;
                        if let Some(k) = {let _push=crate::phases::timer(3);linear.push(saved, srhs, audit)} {
                            if product(k) == *target {
                                out.k = Some(k);
                            } else {
                                out.final_rejected = true;
                            }
                            break 'anchors;
                        }
                        crate::phases::mark("row");
                    }
                }
            }
            let mut row = BTreeMap::new();
            let small=options.small_mode!=0 && options.fused && options.substitute && width<=m &&
                (residual.as_slice().is_empty() || (options.small_mode==2 && residual.as_slice().iter().all(|&(px,_)|index[px as usize]<m)));
            let mut dense=small.then(||vec![0;m+2]);
            if options.fused {
                let mut add = |px: u64, py: u64, count: u32| {
                    let i = index[px as usize];
                    let sign = if py == fb[i].1 {
                        1
                    } else {
                        assert_eq!(py, fp.neg(fb[i].1));
                        -1
                    };
                    let value=linear.f.from_i64(sign*count as i64);
                    if let Some(v)=dense.as_mut(){v[i]=linear.f.add(v[i],value);}
                    if !small || audit {put(&mut row,if i<m{i}else{i+1},value,&linear.f);}
                };
                for &(px, py) in known {
                    add(px, py, 1);
                }
                for &(px, count) in residual.as_slice() {
                    let py = known
                        .iter()
                        .find(|&&(x, _)| x == px)
                        .map(|&(_, y)| y)
                        .unwrap_or_else(|| {
                            fp.add(anchor.v.eval(px, fp), fp.mul(t, anchor.u.eval(px, fp)))
                        });
                    add(px, py, count);
                }
            }
            if audit || !options.fused {
                let mut roots = BTreeMap::<u64, u32>::new();
                for &(x, _) in known {
                    *roots.entry(x).or_default() += 1;
                }
                for &(x, mult) in residual.as_slice() {
                    *roots.entry(x).or_default() += mult;
                }
                if let Some((_, expected)) = truth {
                    assert_eq!(roots, expected);
                }
                assert_eq!(roots.values().copied().sum::<u32>() as usize, c.g + 1);
                out.repeated += roots.values().filter(|&&v| v > 1).count();
                let unknown: Vec<u64> = roots
                    .keys()
                    .copied()
                    .filter(|x| !known.iter().any(|&(xx, _)| xx == *x))
                    .collect();
                let images = crate::images::recover(c, &anchor, t, &remainder, &unknown, audit);
                out.overlaps += images.values().filter(|(_, u)| *u == 0).count();
                let mut reference_row = BTreeMap::new();
                let mut native = c.zero();
                for (&x, &mult) in &roots {
                    let j = index[x as usize];
                    assert_ne!(j, usize::MAX);
                    let y = known
                        .iter()
                        .find(|&&(xx, _)| xx == x)
                        .map(|&(_, yy)| yy)
                        .unwrap_or_else(|| images[&x].0);
                    let sign = if y == fb[j].1 {
                        1
                    } else {
                        assert_eq!(y, c.fp.neg(fb[j].1));
                        -1
                    };
                    let col = if j < m { j } else { j + 1 };
                    put(
                        &mut reference_row,
                        col,
                        linear.f.from_i64(sign * mult as i64),
                        &linear.f,
                    );
                    if audit {
                        for _ in 0..mult {
                            native = c.add_place(&native, x, y);
                        }
                    }
                }
                if audit {
                    assert!(
                        c.is_zero(&c.add(&native, &anchor)),
                        "principal divisor identity"
                    );
                }
                if options.fused {
                    assert_eq!(row, reference_row);
                } else {
                    row = reference_row;
                }
            }
            if !small || audit {
                put(&mut row,j,n as u64,&linear.f);
                if stream==0 {put(&mut row,m,1,&linear.f);}
            }
            if let Some(mut v)=dense {
                v[j]=linear.f.add(v[j],n as u64);
                if stream==0 {v[m]=linear.f.add(v[m],1);}
                v[m+1]=linear.f.neg(rhs);
                out.accepted+=1;
                if let Some(k)=linear.push_small(v,row,rhs) {
                    assert_eq!(product(k),*target);
                    out.k=Some(k);
                    break 'anchors;
                }
                continue;
            }
            if let Some(expected)=audit_recipe {assert_eq!((row,rhs),expected);out.accepted+=1;continue;}
            let certified=options.admission==2 && width<=m && remainder.n<=3; if certified && audit {assert!(linear.ready(&row));} if !certified && !linear.ready(&row) {
                out.unready += 1;
                continue;
            }
            out.accepted += 1;
            crate::phases::mark("linear");
            let mut answer = {let _push=crate::phases::timer(3);if options.admission==0 {linear.push(row,rhs,audit)}else{linear.push_admitted(row,rhs,audit)}};

            if let Some(k) = answer {
                if product(k) == *target {
                    out.k = Some(k);
                } else {
                    out.final_rejected = true;
                }
                break 'anchors;
            }
        }
        if out.k.is_some() {
            break;
        }
    }
    if audit && options.balance!=0 {println!("BIAS {} {} {} {:?} {:?}",c.g,c.fp.p,options.balance,startup_sides,translations);}
    out.public_pending = pending.iter().filter(|r| r.is_some()).count();
    out.direct = linear.direct;
    out.rows = linear.constraints;
    out.pivots = linear.joins;
    out.expanded = linear.expanded_links();
    out
}
