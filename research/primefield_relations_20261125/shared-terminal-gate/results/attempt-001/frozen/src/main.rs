//! Deterministic M31 correctness gate. No relation search or timing experiment.
mod archive_source;
mod reused_native;

use archive_source::Labelled;
use reused_native::{check_curve, Arithmetic, Base, Curve, Point, PRIME31};
use std::collections::{BTreeMap, BTreeSet};

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const Q: (u64, u64) = (1_477_799_919, 824_532_935);
const ARCHIVE_SEED: u64 = 0x4150_535f_5052_3331;
const ARCHIVE_INDICES: [usize; 3] = [437, 1247, 879];
const ARCHIVE_SIGNS: [i8; 3] = [-1, 1, -1];
const EXPECTED_ALPHA: u64 = 1_144_570_523;
const EXPECTED_BETA: u64 = 2_011_873_483;
const EXPECTED_LOG: u64 = 1_038_579_171;
type Poly = Vec<u64>;

fn add(a: u64, b: u64, p: u64) -> u64 { (a + b) % p }
fn sub(a: u64, b: u64, p: u64) -> u64 { (a + p - b) % p }
fn mul(a: u64, b: u64, p: u64) -> u64 { ((a as u128 * b as u128) % p as u128) as u64 }
fn pow(mut a: u64, mut k: u64, p: u64) -> u64 {
    let mut r = 1;
    while k != 0 {
        if k & 1 != 0 { r = mul(r, a, p); }
        k >>= 1;
        a = mul(a, a, p);
    }
    r
}
fn trim(mut f: Poly) -> Poly {
    while f.len() > 1 && f.last() == Some(&0) { f.pop(); }
    if f.is_empty() { f.push(0); }
    f
}
fn zero(f: &[u64]) -> bool { f == [0] }
fn pmul(a: &[u64], b: &[u64]) -> Poly {
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() { c[i+j] = add(c[i+j], mul(x, y, P), P); }
    }
    trim(c)
}
fn psub(a: &[u64], b: &[u64]) -> Poly {
    trim((0..a.len().max(b.len())).map(|i| sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0), P)).collect())
}
fn prem(a: &[u64], b: &[u64]) -> Poly {
    assert!(!zero(b));
    let mut r = trim(a.to_vec());
    let inv = pow(*b.last().unwrap(), P - 2, P);
    while !zero(&r) && r.len() >= b.len() {
        let offset = r.len() - b.len();
        let q = mul(*r.last().unwrap(), inv, P);
        for (j, &x) in b.iter().enumerate() { r[offset+j] = sub(r[offset+j], mul(q, x, P), P); }
        r = trim(r);
    }
    r
}
fn pgcd(mut a: Poly, mut b: Poly) -> Poly {
    while !zero(&b) { let r = prem(&a, &b); a = b; b = r; }
    if zero(&a) { return a; }
    let inv = pow(*a.last().unwrap(), P - 2, P);
    a.into_iter().map(|x| mul(x, inv, P)).collect()
}
fn ppowmod(mut a: Poly, mut n: u64, modulus: &[u64]) -> Poly {
    let mut r = vec![1];
    a = prem(&a, modulus);
    while n != 0 {
        if n & 1 != 0 { r = prem(&pmul(&r, &a), modulus); }
        n >>= 1;
        if n != 0 { a = prem(&pmul(&a, &a), modulus); }
    }
    r
}
fn peval(f: &[u64], x: u64) -> u64 { f.iter().rev().fold(0, |a, &c| add(mul(a, x, P), c, P)) }

fn chain(x: u64, depth: usize) -> Result<Vec<u64>, &'static str> {
    if x >= P { return Err("noncanonical field input"); }
    if !(1..=10).contains(&depth) { return Err("unsupported chain depth"); }
    let mut out = vec![x];
    for _ in 0..depth { let a = *out.last().unwrap(); out.push(sub(mul(a, a, P), 2, P)); }
    Ok(out)
}
fn terminal(x: u64, depth: usize) -> u64 { *chain(x, depth).unwrap().last().unwrap() }
fn terminals(raw: &[u64]) -> Result<Vec<u64>, &'static str> {
    if raw.iter().any(|&c| c >= P) { return Err("noncanonical terminal"); }
    Ok(raw.iter().copied().collect::<BTreeSet<_>>().into_iter().collect())
}
fn check_chain(path: &[u64], cs: &[u64]) -> bool {
    path.len() >= 2 && path.iter().all(|&x| x < P)
        && path.windows(2).all(|w| sub(mul(w[0], w[0], P), 2, P) == w[1])
        && cs.binary_search(path.last().unwrap()).is_ok()
}
fn inverse_fiber(c: u64, depth: usize, ar: &mut Arithmetic) -> Vec<u64> {
    let mut roots = BTreeSet::from([c]);
    for _ in 0..depth {
        let mut next = BTreeSet::new();
        for y in roots {
            if let Some(x) = ar.sqrt(add(y, 2, P), P) {
                next.insert(x);
                next.insert(sub(0, x, P));
            }
        }
        roots = next;
    }
    roots.into_iter().collect()
}
fn shared_polynomial(cs: &[u64], depth: usize) -> Poly {
    let mut d = vec![0, 1];
    for _ in 0..depth { d = psub(&pmul(&d, &d), &[2]); }
    cs.iter().fold(vec![1], |h, &c| pmul(&h, &psub(&d, &[c])))
}
fn root_polynomial(roots: &[u64]) -> Poly {
    roots.iter().fold(vec![1], |h, &x| pmul(&h, &[sub(0, x, P), 1]))
}
fn field_support_polynomial(h: &[u64]) -> Poly {
    if h.len() == 1 { return vec![1]; }
    pgcd(h.to_vec(), psub(&ppowmod(vec![0, 1], P, h), &[0, 1]))
}

#[derive(Clone, Debug)]
struct Tag {
    id: usize,
    point: (u64, u64),
    a: u64,
    b: u64,
    original: Labelled,
    original_orientation: i8,
}
fn negate_scalar(x: u64) -> u64 { if x == 0 { 0 } else { N-x } }
fn signed_scalar(x: u64, s: i8) -> u64 { if s == 1 { x } else { negate_scalar(x) } }
fn canonical_tag(id: usize, original: Labelled) -> Tag {
    let flip = original.pt.1 > P-original.pt.1;
    Tag {
        id,
        point: (original.pt.0, original.pt.1.min(P-original.pt.1)),
        a: if flip { negate_scalar(original.a) } else { original.a },
        b: if flip { negate_scalar(original.b) } else { original.b },
        original,
        original_orientation: if flip { -1 } else { 1 },
    }
}
fn same_curve(c: Curve) -> bool {
    c.p == P && c.a == PRIME31.a && c.b == PRIME31.b && c.n == N && c.g == PRIME31.g
}
fn validate_tag(c: Curve, q: (u64,u64), t: &Tag, expected: &Tag, ar: &mut Arithmetic) -> Result<(), &'static str> {
    if !same_curve(c) { return Err("wrong curve identity"); }
    if q != Q { return Err("wrong public target identity"); }
    if t.id != expected.id || t.point != expected.point { return Err("wrong original point identity"); }
    if t.point.0 >= P || t.point.1 >= P || t.point.1 > P-t.point.1 || t.a >= N || t.b >= N { return Err("noncanonical point or scalar"); }
    if !ar.on_curve(c, Some(t.point)) { return Err("point does not lift to curve"); }
    let ag = ar.scalar(c, t.a, Some(c.g));
    let bq = ar.scalar(c, t.b, Some(q));
    if ar.add(c, ag, bq) != Some(t.point) { return Err("public tag equation fails"); }
    if t.a != expected.a || t.b != expected.b { return Err("public tag differs from original record"); }
    if t.original.pt != expected.original.pt || t.original.a != expected.original.a || t.original.b != expected.original.b
        || t.original_orientation != expected.original_orientation { return Err("original source record differs"); }
    if t.original.a >= N || t.original.b >= N || t.original.pt.0 >= P || t.original.pt.1 >= P
        || (t.original_orientation != 1 && t.original_orientation != -1) { return Err("invalid original source record"); }
    if t.a != signed_scalar(t.original.a,t.original_orientation) || t.b != signed_scalar(t.original.b,t.original_orientation) {
        return Err("canonical public tag orientation mismatch");
    }
    let orig = if t.original_orientation == 1 { Some(t.original.pt) } else { ar.neg(c, Some(t.original.pt)) };
    if orig != Some(t.point) { return Err("original orientation mismatch"); }
    Ok(())
}
#[derive(Clone, Copy, Debug)]
struct Witness { id: usize, point: (u64,u64), sign: i8 }
#[derive(Debug, PartialEq, Eq)]
struct RelationReplay { alpha: u64, beta: u64, logarithm: u64 }
fn replay_relation(c: Curve, q: (u64,u64), roster: &BTreeMap<usize, Tag>, ws: &[Witness], cs: &[u64], depth: usize, ar: &mut Arithmetic) -> Result<RelationReplay, &'static str> {
    // Membership is a separate gate from signed group/tag replay.
    for w in ws {
        if !check_chain(&chain(w.point.0, depth)?, cs) { return Err("point outside terminal union"); }
    }
    replay_tagged_sum(c,q,roster,ws,ar)
}
fn replay_tagged_sum(c: Curve, q: (u64,u64), roster: &BTreeMap<usize, Tag>, ws: &[Witness], ar: &mut Arithmetic) -> Result<RelationReplay, &'static str> {
    if !same_curve(c) { return Err("wrong curve identity"); }
    if q != Q { return Err("wrong public target identity"); }
    let mut sum: Point = None;
    let (mut alpha, mut beta) = (0, 0);
    for w in ws {
        let t = roster.get(&w.id).ok_or("unlabeled sibling or unknown original column")?;
        validate_tag(c, q, t, t, ar)?;
        if w.point != t.point { return Err("wrong original point identity"); }
        if w.sign != 1 && w.sign != -1 { return Err("invalid sign"); }
        let point = if w.sign == 1 { Some(w.point) } else { ar.neg(c, Some(w.point)) };
        sum = ar.add(c, sum, point);
        alpha = add(alpha, signed_scalar(t.a, w.sign), N);
        beta = add(beta, signed_scalar(t.b, w.sign), N);
    }
    if sum.is_some() { return Err("signed point sum is nonzero"); }
    if beta == 0 { return Err("zero aggregate Q coefficient"); }
    let logarithm = mul(negate_scalar(alpha), pow(beta, N-2, N), N);
    if ar.scalar(c, logarithm, Some(c.g)) != Some(q) { return Err("final scalar replay failed"); }
    Ok(RelationReplay { alpha, beta, logarithm })
}

struct Report {
    cs: Vec<u64>, roots: Vec<u64>, rational_x: Vec<u64>, signed_lifts: usize,
    h: Poly, gcd: Poly, tags: Vec<Tag>, signs: Vec<i8>, checks: Vec<(&'static str, bool)>,
    relation: RelationReplay, source_count: usize, identity_rejections: u64, duplicate_rejections: u64,
    reflection: Reflection,
}
struct Reflection { a: Tag, r: Tag, sibling: Tag, half: (u64,u64), half_tag: (u64,u64), map_value: u64 }
fn require(checks: &mut Vec<(&'static str,bool)>, name: &'static str, ok: bool) -> Result<(), String> {
    checks.push((name,ok));
    if ok { Ok(()) } else { Err(format!("correctness gate failed: {name}")) }
}
fn run_gate() -> Result<Report, String> {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let mut checks = Vec::new();
    require(&mut checks,"prime_p_and_r_native_trial_division_and_nonsingular_curve",true)?;
    require(&mut checks,"nonzero_generator_and_target_prime_order", ar.on_curve(PRIME31,Some(PRIME31.g))
        && ar.on_curve(PRIME31,Some(Q)) && ar.scalar(PRIME31,N,Some(PRIME31.g)).is_none()
        && ar.scalar(PRIME31,N,Some(Q)).is_none())?;
    let mut integer_root=0_u64;
    while (integer_root+1)*(integer_root+1)<=P { integer_root+=1; }
    require(&mut checks,"integer_Hasse_total_group_order_certificate",integer_root*integer_root<=P
        && (integer_root+1)*(integer_root+1)>P && 2*N>P+1+2*(integer_root+1))?;
    // Archival source reconstruction only. No pair table or relation search runs.
    let (parts, identities, duplicates) = archive_source::source(Q,ARCHIVE_SEED,&mut ar)?;
    require(&mut checks,"archive_source_population_and_rejections", parts.iter().all(|v|v.len()==1536) && identities==0 && duplicates==0)?;
    let tags: Vec<_> = (0..3).map(|i|canonical_tag(i*1536+ARCHIVE_INDICES[i],parts[i][ARCHIVE_INDICES[i]])).collect();
    let signs: Vec<_> = (0..3).map(|i|ARCHIVE_SIGNS[i]*tags[i].original_orientation).collect();
    for tag in &tags { validate_tag(PRIME31,Q,tag,tag,&mut ar).map_err(str::to_owned)?; }
    require(&mut checks,"all_original_public_tags_and_canonical_signs",true)?;
    let roster: BTreeMap<_,_> = tags.iter().cloned().map(|t|(t.id,t)).collect();
    let depth = 3;
    let base = Base::build(PRIME31,depth,&mut ar)?;
    require(&mut checks,"reused_complete_dickson_fiber",base.values.len()==1<<depth && base.values.iter().all(|&x|terminal(x,depth)==base.terminal))?;
    let mut raw_cs: Vec<_> = tags.iter().map(|t|terminal(t.point.0,depth)).collect();
    raw_cs.extend([base.terminal,2,P-2,base.terminal]);
    let cs = terminals(&raw_cs).map_err(str::to_owned)?;
    let mut all = BTreeSet::new();
    for &c in &cs { all.extend(inverse_fiber(c,depth,&mut ar)); }
    let roots: Vec<_> = all.into_iter().collect();
    let h = shared_polynomial(&cs,depth);
    let gcd = field_support_polynomial(&h);
    require(&mut checks,"exact_all_Fp_support_certificate",gcd==root_polynomial(&roots))?;
    require(&mut checks,"root_chain_and_polynomial_membership",roots.iter().all(|&x|check_chain(&chain(x,depth).unwrap(),&cs)&&peval(&h,x)==0))?;
    require(&mut checks,"duplicate_terminals_preserve_support",field_support_polynomial(&shared_polynomial(&raw_cs,depth))==gcd)?;
    let mut missing = roots.clone(); missing.pop();
    require(&mut checks,"missing_root_certificate_rejected",root_polynomial(&missing)!=gcd)?;
    let mut bad_path = chain(roots[0],depth).unwrap(); bad_path[1]=add(bad_path[1],1,P);
    require(&mut checks,"mutated_chain_rejected",!check_chain(&bad_path,&cs))?;
    require(&mut checks,"noncanonical_input_rejected",chain(P,depth).is_err()&&terminals(&[P]).is_err())?;
    require(&mut checks,"empty_terminal_union",shared_polynomial(&[],depth)==[1]&&field_support_polynomial(&[1])==[1])?;
    let removed=terminals(&cs[1..]).unwrap();
    let first_fiber=inverse_fiber(cs[0],depth,&mut ar);
    require(&mut checks,"wrong_terminal_membership_rejected",!first_fiber.is_empty()
        && first_fiber.iter().all(|&x|!check_chain(&chain(x,depth).unwrap(),&removed)))?;
    let mut rational_x=Vec::new();
    let mut signed_lifts=0;
    let mut nonlift=0;
    for &x in &roots {
        let rhs=ar.rhs(PRIME31,x);
        if let Some(y)=ar.sqrt(rhs,P) {
            let canonical=y.min(P-y);
            require(&mut checks,"individual_curve_lift",ar.on_curve(PRIME31,Some((x,canonical)))&&ar.on_curve(PRIME31,Some((x,sub(0,canonical,P)))))?;
            rational_x.push(x); signed_lifts+=if y==0 {1}else{2};
        } else { nonlift+=1; }
    }
    require(&mut checks,"selector_and_curve_lift_sets_distinguished",nonlift>0&&rational_x.len()>tags.len())?;
    let witnesses: Vec<_>=tags.iter().zip(&signs).map(|(t,&sign)|Witness{id:t.id,point:t.point,sign}).collect();
    let relation=replay_relation(PRIME31,Q,&roster,&witnesses,&cs,depth,&mut ar).map_err(str::to_owned)?;
    require(&mut checks,"historical_useful_relation_replay",relation.alpha==EXPECTED_ALPHA&&relation.beta==EXPECTED_BETA&&relation.logarithm==EXPECTED_LOG)?;
    let sibling_x=*rational_x.iter().find(|x|tags.iter().all(|t|t.point.0!=**x)).ok_or("missing unlabeled sibling fixture")?;
    let sibling_rhs=ar.rhs(PRIME31,sibling_x);
    let sy=ar.sqrt(sibling_rhs,P).ok_or("bad sibling fixture")?;
    let sibling=Witness{id:usize::MAX,point:(sibling_x,sy.min(P-sy)),sign:1};
    require(&mut checks,"unlabeled_sibling_rejected",replay_relation(PRIME31,Q,&roster,&[sibling],&cs,depth,&mut ar)==Err("unlabeled sibling or unknown original column"))?;
    let mut changed=tags[0].clone();changed.a=add(changed.a,1,N);
    require(&mut checks,"mutated_public_tag_rejected",validate_tag(PRIME31,Q,&changed,&tags[0],&mut ar)==Err("public tag equation fails"))?;
    let mut wrongsign=witnesses.clone();wrongsign[0].sign*=-1;
    require(&mut checks,"wrong_relation_sign_rejected",replay_relation(PRIME31,Q,&roster,&wrongsign,&cs,depth,&mut ar)==Err("signed point sum is nonzero"))?;
    let mut wrongpoint=witnesses.clone();wrongpoint[0].point.1=sub(0,wrongpoint[0].point.1,P);
    require(&mut checks,"noncanonical_point_orientation_rejected",replay_relation(PRIME31,Q,&roster,&wrongpoint,&cs,depth,&mut ar)==Err("wrong original point identity"))?;
    let mut wrongcurve=PRIME31;wrongcurve.b=add(wrongcurve.b,1,P);
    require(&mut checks,"wrong_curve_rejected",replay_relation(wrongcurve,Q,&roster,&witnesses,&cs,depth,&mut ar)==Err("wrong curve identity"))?;
    require(&mut checks,"wrong_public_target_rejected",replay_relation(PRIME31,PRIME31.g,&roster,&witnesses,&cs,depth,&mut ar)==Err("wrong public target identity"))?;
    let mut badsign=witnesses.clone();badsign[0].sign=0;
    require(&mut checks,"invalid_sign_rejected",replay_relation(PRIME31,Q,&roster,&badsign,&cs,depth,&mut ar)==Err("invalid sign"))?;
    let canceled=[Witness{sign:1,..witnesses[0]},Witness{sign:-1,..witnesses[0]}];
    require(&mut checks,"zero_aggregate_Q_coefficient_rejected",replay_relation(PRIME31,Q,&roster,&canceled,&cs,depth,&mut ar)==Err("zero aggregate Q coefficient"))?;
    let mut swapped=witnesses.clone();swapped[0].id=tags[1].id;
    require(&mut checks,"wrong_original_column_rejected",replay_relation(PRIME31,Q,&roster,&swapped,&cs,depth,&mut ar)==Err("wrong original point identity"))?;
    let reflection=reflection_control(&tags[0],&tags[1],&mut ar,&mut checks)?;
    Ok(Report{cs,roots,rational_x,signed_lifts,h,gcd,tags,signs,checks,relation,source_count:parts.iter().map(Vec::len).sum(),identity_rejections:identities,duplicate_rejections:duplicates,reflection})
}

fn reflection_control(a: &Tag, r: &Tag, ar: &mut Arithmetic, checks: &mut Vec<(&'static str,bool)>) -> Result<Reflection,String> {
    // A deterministic degree-two map ON E: h_A(R)=x(R-[1/2]A).
    // It is not an original-x Dickson selector or a newly discovered useful relation.
    let half=ar.scalar(PRIME31,(N+1)/2,Some(a.point)).ok_or("reflection half is infinity")?;
    let half_tag=(mul((N+1)/2,a.a,N),mul((N+1)/2,a.b,N));
    let half_record=canonical_tag(4609,Labelled{pt:half,a:half_tag.0,b:half_tag.1});
    validate_tag(PRIME31,Q,&half_record,&half_record,ar).map_err(str::to_owned)?;
    require(checks,"reflection_half_is_tagged_half_of_A",ar.add(PRIME31,Some(half),Some(half))==Some(a.point))?;
    let neg_r=ar.neg(PRIME31,Some(r.point));
    let sibling_point=ar.add(PRIME31,Some(a.point),neg_r).ok_or("reflection sibling is infinity")?;
    let sibling_original=Labelled{pt:sibling_point,a:sub(a.a,r.a,N),b:sub(a.b,r.b,N)};
    let sibling=canonical_tag(4608,sibling_original);
    validate_tag(PRIME31,Q,&sibling,&sibling,ar).map_err(str::to_owned)?;
    let neg_half=ar.neg(PRIME31,Some(half));
    let translated=ar.add(PRIME31,Some(r.point),neg_half).ok_or("reflection fixed point")?;
    let reflected=ar.add(PRIME31,Some(sibling_point),neg_half).ok_or("reflection fixed sibling")?;
    require(checks,"reflection_two_distinct_points_in_one_complete_fiber",r.point!=sibling_point
        && translated.0==reflected.0 && ar.neg(PRIME31,Some(translated))==Some(reflected))?;
    let lifted_rhs=ar.rhs(PRIME31,translated.0);
    let y=ar.sqrt(lifted_rhs,P).ok_or("reflection image fails to lift")?;
    let preimages:BTreeSet<_>=[y,sub(0,y,P)].into_iter().map(|lift_y|ar.add(PRIME31,Some((translated.0,lift_y)),Some(half)).unwrap()).collect();
    require(checks,"reflection_exact_rational_fiber_preimages",preimages==BTreeSet::from([r.point,sibling_point]))?;
    let reflection_roster=BTreeMap::from([(a.id,a.clone()),(r.id,r.clone()),(sibling.id,sibling.clone())]);
    let pair_identity=[Witness{id:r.id,point:r.point,sign:1},Witness{id:sibling.id,point:sibling.point,sign:sibling.original_orientation},Witness{id:a.id,point:a.point,sign:-1}];
    let alpha=add(add(r.a,signed_scalar(sibling.a,sibling.original_orientation),N),negate_scalar(a.a),N);
    let beta=add(add(r.b,signed_scalar(sibling.b,sibling.original_orientation),N),negate_scalar(a.b),N);
    require(checks,"reflection_planted_identity_has_zero_alpha_and_beta",alpha==0&&beta==0)?;
    require(checks,"reflection_planted_identity_rejected_as_useless",replay_tagged_sum(PRIME31,Q,&reflection_roster,&pair_identity,ar)==Err("zero aggregate Q coefficient"))?;
    Ok(Reflection{a:a.clone(),r:r.clone(),sibling,half,half_tag,map_value:translated.0})
}

fn print_report(r: &Report) {
    let checks=r.checks.iter().map(|(n,b)|format!("{{\"name\":\"{n}\",\"pass\":{b}}}")).collect::<Vec<_>>().join(",");
    let tags=r.tags.iter().map(|t|format!("{{\"id\":{},\"canonical_point\":[{},{}],\"a\":{},\"b\":{},\"original_point\":[{},{}],\"original_a\":{},\"original_b\":{},\"original_orientation\":{}}}",t.id,t.point.0,t.point.1,t.a,t.b,t.original.pt.0,t.original.pt.1,t.original.a,t.original.b,t.original_orientation)).collect::<Vec<_>>().join(",");
    let f=&r.reflection;
    let reflection=format!("{{\"map\":\"h_A(R)=x(R-[1/2]A), a degree-two map on E\",\"A\":[{},{}],\"R\":[{},{}],\"A_minus_R\":[{},{}],\"half_A\":[{},{}],\"half_A_public_tag\":[{},{}],\"common_image\":{},\"sibling_public_tag\":[{},{}],\"planted_identity_alpha\":0,\"planted_identity_beta\":0,\"useful_relation\":false}}",f.a.point.0,f.a.point.1,f.r.point.0,f.r.point.1,f.sibling.original.pt.0,f.sibling.original.pt.1,f.half.0,f.half.1,f.half_tag.0,f.half_tag.1,f.map_value,f.sibling.original.a,f.sibling.original.b);
    println!("{{\n\"schema\":\"isogeny.shared-terminal-correctness/v1\",\n\"status\":\"pass\",\n\"classification\":\"deterministic archived-fixture correctness replay; no relation search, randomized experiment, timing or speed claim\",\n\"p\":{P},\"n\":{N},\"curve\":[{},{}],\"generator\":[{},{}],\"public_target\":[{},{}],\n\"depth\":3,\"terminals\":{:?},\"selector_roots\":{:?},\"rational_curve_abscissae\":{:?},\"signed_curve_lifts\":{},\"labeled_original_columns\":{},\n\"shared_selector_coefficients_low_to_high\":{:?},\"gcd_with_Xp_minus_X_coefficients_low_to_high\":{:?},\n\"public_tags\":[{}],\"canonical_relation_signs\":{:?},\n\"archival_source_replay\":{{\"checked_commit\":\"90a0314f5be63d5ca56465a884091f4be0f114fd\",\"seed_hex\":\"4150535f50523331\",\"points_reconstructed\":{},\"identity_rejections\":{},\"duplicate_x_rejections\":{},\"relation_search_calls\":0}},\n\"historical_relation\":{{\"alpha\":{},\"beta\":{},\"replayed_scalar\":{}}},\n\"reflection_control\":{reflection},\n\"checks\":[{}],\"check_count\":{},\"all_checks_pass\":true,\n\"standing\":\"producer correctness replay, not independently signed or kernel-verified\"\n}}",PRIME31.a,PRIME31.b,PRIME31.g.0,PRIME31.g.1,Q.0,Q.1,r.cs,r.roots,r.rational_x,r.signed_lifts,r.tags.len(),r.h,r.gcd,tags,r.signs,r.source_count,r.identity_rejections,r.duplicate_rejections,r.relation.alpha,r.relation.beta,r.relation.logarithm,checks,r.checks.len());
}
fn main() {
    match run_gate() {
        Ok(r)=>print_report(&r),
        Err(e)=>{eprintln!("{e}");std::process::exit(1);}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polynomial_reference_handles_repeated_roots_at_m31() {
        let f=pmul(&[P-1,1],&[P-1,1]);
        assert_eq!(field_support_polynomial(&f),vec![P-1,1]);
        assert_eq!(prem(&f,&[P-1,1]),vec![0]);
        assert_eq!(field_support_polynomial(&[1]),vec![1]);
    }
    #[test]
    fn complete_archived_fixture_and_mutation_gates() {
        let r=run_gate().unwrap();
        assert!(r.checks.iter().all(|x|x.1));
        assert_eq!(r.relation.logarithm,EXPECTED_LOG);
        assert_eq!(r.source_count,4608);
    }
}
