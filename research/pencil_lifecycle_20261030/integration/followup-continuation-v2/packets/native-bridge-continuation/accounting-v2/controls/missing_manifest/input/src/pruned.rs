#![allow(dead_code)]
use five_leads_study::three_leads::{Divisor, Field, Jacobian, Ring, Rng, SignedGraph};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};
fn parse_vec4(v: &Value) -> [u64; 4] {
    std::array::from_fn(|i| v[i].as_u64().unwrap())
}
fn parse_matrix(v: &Value) -> [[u64; 4]; 4] {
    std::array::from_fn(|i| parse_vec4(&v[i]))
}
fn sub_mod(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn mul_mod(x: u64, y: u64, p: u64) -> u64 {
    x * y % p
}
fn inv_mod(x: u64, p: u64) -> u64 {
    let mut base = x;
    let mut exp = p - 2;
    let mut z = 1;
    while exp > 0 {
        if exp & 1 != 0 {
            z = mul_mod(z, base, p);
        }
        base = mul_mod(base, base, p);
        exp >>= 1;
    }
    z
}

mod walk {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/walk_impl_uncached.rs"
    ));
    type Pair = ([u64; 4], [u64; 4]);
    type StepResult = (Option<Pair>, u64, [Option<u64>; 3], [bool;3], [bool;3]);
    pub fn step(
        k: [u64; 4],
        kp: [u64; 4],
        kg: [u64; 4],
        k0: [u64; 4],
        t: [u64; 4],
        acts: &[(u64, [[u64; 4]; 4])],
        p: u64,
    ) -> StepResult {
        let choice = branch(k, p);
        // Proposed arm: only the selected double is evaluated.
        // None in the unused repair slot means not evaluated, not repair failure.
        if choice == 0 {
            let (kd, r0) = xadd_repaired(k, k, k0, t, acts, p);
            let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
            (kd.zip(km), choice, [r0, r1, None], [true,true,false], [true,true,false])
        } else {
            let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
            let (kpp, r2) = xadd_repaired(kp, kp, k0, t, acts, p);
            (km.zip(kpp), choice, [None, r1, r2], [false,true,true], [false,true,true])
        }
    }

    pub fn repair_probe(x:[u64;4],y:[u64;4],diff:[u64;4],t:[u64;4],acts:&[(u64,[[u64;4];4])],p:u64)->(Option<[u64;4]>,Option<[u64;4]>,Option<u64>){
       let plain=xadd(x,y,diff,t,p);let(repaired,root)=xadd_repaired(x,y,diff,t,acts,p);(plain,repaired,root)
    }
    pub fn project_divisor(d: Divisor, t: &serde_json::Value, p: u64) -> Option<[u64; 4]> {
        project(d, t, p)
    }
    pub fn project_any(
        d: Divisor,
        j: &Jacobian,
        t: &serde_json::Value,
        _acts: &[(u64, [[u64; 4]; 4])],
        p: u64,
    ) -> Option<[u64; 4]> {
        // CCS §6.3 and §7.4 give the classical Kummer vector q followed by
        // the invertible C1 fast-theta map. This handles degree 0/1/2,
        // including the a0=0 zero-numerator locus of the older generic law.
        let q = match d.degree {
            0 => [0, 0, 0, 1],
            1 => {
                let x = sub(0, d.u[0], p);
                [0, 1, x, mul(x, x, p)]
            }
            2 => {
                let (a0, a1, b1) = (d.u[0], d.u[1], d.v[1]);
                let f = &j.polynomial;
                let z = sub(mul(a1, a1, p), a0, p);
                let xi4 = sub(
                    add(
                        add(mul(b1, b1, p), mul(z, sub(mul(f[5], a1, p), 0, p), p), p),
                        mul(a1, sub(f[3], mul(f[4], a1, p), p), p),
                        p,
                    ),
                    f[2],
                    p,
                );
                [1, sub(0, a1, p), a0, xi4]
            }
            _ => return None,
        };
        let (l, mu, nu) = (
            t["lambda"].as_u64().unwrap(),
            t["mu"].as_u64().unwrap(),
            t["nu"].as_u64().unwrap(),
        );
        let m0 = [
            mul(mu, add(l, nu, p), p),
            mul(mul(nu, l, p), add(1, mu, p), p),
            mul(nu, add(l, mu, p), p),
            mul(mul(mu, l, p), add(1, nu, p), p),
        ];
        let m1 = [
            sub(0, mu, p),
            sub(0, mul(nu, l, p), p),
            sub(0, nu, p),
            sub(0, mul(mu, l, p), p),
        ];
        let m2 = [add(mu, 1, p), add(l, nu, p), add(nu, 1, p), add(l, mu, p)];
        let scale = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
        let out = std::array::from_fn(|i| {
            let row = [m0[i], m1[i], m2[i], p - 1];
            mul(
                scale[i],
                (0..4).fold(0, |v, k| add(v, mul(row[k], q[k], p), p)),
                p,
            )
        });
        (out != [0; 4]).then_some(out)
    }
    pub fn same_k(x: [u64; 4], y: [u64; 4], p: u64) -> bool {
        same(x, y, p)
    }
}
mod signed {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/recover_boundary_impl.rs"
    ));
    pub fn curve_coeff(t: &Value, p: u64) -> Vec<u64> {
        curve(t, p)
    }
    pub fn general_coords(k: [u64; 4], t: &Value, p: u64) -> Option<[u64; 4]> {
        general(k, t, p)
    }
    #[allow(clippy::too_many_arguments)] // The unchanged N11 Recover wrapper.


    pub fn delta_lift(anchor:Divisor,kq:[u64;4],kplus:[u64;4],t:&Value,j:&Jacobian,p:u64)->(Option<Divisor>,&'static str,usize){
      if kq==[0;4]||kplus==[0;4]{return(None,"delta_zero_vector",0);}
      let Some(q)=general(kq,t,p)else{return(None,"delta_nonordinary_chart",0);};let f=curve(t,p);
      let(a0,a1)=(q[2],s(0,q[1],p));let z=s(m(a1,a1,p),a0,p);let b1sq=a(s(s(q[3],m(z,s(m(f[5],a1,p),m(f[6],z,p),p),p),p),m(a1,s(f[3],m(f[4],a1,p),p),p),p),f[2],p);
      let Some(root)=j.f.sqrt(b1sq)else{return(None,"delta_nonsquare",0);};
      let mut power=[1,0];let mut rem=[0,0];for &coeff in &f{rem[0]=a(rem[0],m(coeff,power[0],p),p);rem[1]=a(rem[1],m(coeff,power[1],p),p);power=[s(0,m(a0,power[1],p),p),s(power[0],m(a1,power[1],p),p)];}
      let mut candidates=Vec::new();for b1 in[root,j.f.neg(root)]{let b0s=if b1!=0{vec![m(a(rem[1],m(a1,b1sq,p),p),inv(m(2,b1,p),p),p)]}else{match j.f.sqrt(rem[0]){Some(b0)=>vec![b0,j.f.neg(b0)],None=>vec![]}};
        for b0 in b0s{let d=Divisor{degree:2,u:[a0,a1,1],v:[b0,b1]};if !candidates.contains(&d){candidates.push(d);}}}
      let tested=candidates.len();assert!(tested<=4);let mut matches=Vec::new();
      for d in candidates{if !j.valid(d){continue;}let Some(kd)=super::walk::project_any(d,j,t,&[],p)else{continue;};if !super::walk::same_k(kd,kq,p){continue;}let sum=j.add(anchor,d);let Some(ks)=super::walk::project_any(sum,j,t,&[],p)else{continue;};if super::walk::same_k(ks,kplus,p){matches.push(d);}}
      match matches.len(){1=>(Some(matches[0]),"delta_unique_signed_lift",tested),0=>(None,"delta_no_signed_lift",tested),_=>(None,"delta_ambiguous_signed_lift",tested)}
    }
    pub fn boundary_lift(anchor:Divisor,kq:[u64;4],kplus:[u64;4],t:&Value,j:&Jacobian,p:u64)->(Option<Divisor>,&'static str,usize){
      if kq==[0;4]||kplus==[0;4]{return(None,"zero_projective_vector",0);}
      let Some(q)=general_homogeneous(kq,t,p)else{return(None,"singular_coordinate_map",0);};
      if q[0]!=0{return(None,"not_boundary_chart",0);}
      if q[1]==0{
        if q[2]!=0||q[3]==0{return(None,"invalid_boundary_chart",0);}
        let Some(ka)=super::walk::project_any(anchor,j,t,&[],p)else{return(None,"anchor_projection_failure",0);};
        return if super::walk::same_k(ka,kplus,p){(Some(Divisor::O),"identity_recovered",1)}else{(None,"identity_neighbour_mismatch",1)};
      }
      if m(q[1],q[3],p)!=m(q[2],q[2],p){return(None,"invalid_degree1_conic",0);}
      let x=m(q[2],inv(q[1],p),p);let value=j.polynomial.iter().rev().fold(0,|z,&c|j.f.add(j.f.mul(z,x),c));
      let Some(y)=j.f.sqrt(value)else{return(None,"degree1_nonsquare",0);};
      let mut matches=Vec::new();let mut tested=0;
      for yy in [y,j.f.neg(y)]{
        let candidate=Divisor{degree:1,u:[j.f.neg(x),1,0],v:[yy,0]};if matches.contains(&candidate){continue;}tested+=1;
        if !j.valid(candidate){continue;}let Some(kc)=super::walk::project_any(candidate,j,t,&[],p)else{continue;};
        if !super::walk::same_k(kc,kq,p){continue;}let sum=j.add(anchor,candidate);let Some(ks)=super::walk::project_any(sum,j,t,&[],p)else{continue;};
        if super::walk::same_k(ks,kplus,p){matches.push(candidate);}
      }
      match matches.len(){1=>(Some(matches[0]),"degree1_unique_signed_lift",tested),0=>(None,"degree1_no_signed_lift",tested),_=>(None,"degree1_ambiguous_signed_lift",tested)}
    }
    pub fn lift(
        anchor: Divisor,
        kq: [u64; 4],
        kplus: [u64; 4],
        kp: [u64; 4],
        t: &Value,
        acts: &[(u64, [[u64; 4]; 4])],
        j: &Jacobian,
        p: u64,
    ) -> (Option<Divisor>, &'static str, usize) {
        let constants = ["A", "B", "C", "D"].map(|z| t[z].as_u64().unwrap());
        let (km, _) = xadd_repaired(kq, kp, kplus, constants, acts, p);
        let (q, plus, minus) = (
            general(kq, t, p),
            general(kplus, t, p),
            km.and_then(|x| general(x, t, p)),
        );
        if let (Some(q), Some(plus), Some(minus)) = (q, plus, minus) {
            let f = curve(t, p);
            let (mut out, mut status) = recover(anchor, q, plus, minus, &f, p);
            let mut tested = 0;
            if out.is_none() && status == "D_zero" {
                let (candidate, n) = exceptional_lift(anchor, q, plus, &f, j, p);
                out = candidate;
                tested = n;
                if out.is_some() {
                    status = "enumerated_exception_lift";
                }
            }
            if out.is_none() && status == "Delta_zero" {
                return delta_lift(anchor,kq,kplus,t,j,p);
            }
            (out, status, tested)
        } else {
            boundary_lift(anchor,kq,kplus,t,j,p)
        }
    }
}

struct Advance {
    next: Option<([u64; 4], [u64; 4])>,
    choice: u64,
    roots: [Option<u64>; 3],
    evaluated: [bool;3],
    selected: [bool;3],
    used_fallback: bool,
    fallback_validity_checks: u64,
    transition_checks: u64,
    validation_failed: bool,
}

#[allow(clippy::too_many_arguments)]
fn signed_fallback(
    j: &Jacobian,
    t: &Value,
    acts: &[(u64, [[u64; 4]; 4])],
    p: u64,
    r: u64,
    g: Divisor,
    q: Divisor,
    a: u64,
    b: u64,
    choice: u64,
) -> Option<([u64; 4], [u64; 4])> {
    let aa = (2 * a + choice) % r;
    let bb = (2 * b) % r;
    let d = j.add(j.mul(g, aa), j.mul(q, bb));
    let dplus = j.add(d, g);
    let d_valid = j.valid(d);
    let plus_valid = j.valid(dplus);
    if !d_valid || !plus_valid {
        return None;
    }
    walk::project_any(d, j, t, acts, p).zip(walk::project_any(dplus, j, t, acts, p))
}

#[allow(clippy::too_many_arguments)]
fn advance(
    k: [u64; 4],
    kp: [u64; 4],
    kg: [u64; 4],
    k0: [u64; 4],
    constants: [u64; 4],
    acts: &[(u64, [[u64; 4]; 4])],
    j: &Jacobian,
    t: &Value,
    p: u64,
    r: u64,
    g: Divisor,
    q: Divisor,
    a: u64,
    b: u64,
    validate_transition: bool,
) -> Advance {
    let (next, choice, roots, evaluated, selected) = walk::step(k, kp, kg, k0, constants, acts, p);
    if let Some((nk, nkp)) = next {
        if validate_transition {
            let aa = (2 * a + choice) % r;
            let bb = (2 * b) % r;
            let d = j.add(j.mul(g, aa), j.mul(q, bb));
            let dplus = j.add(d, g);
            let expected =
                walk::project_any(d, j, t, acts, p).zip(walk::project_any(dplus, j, t, acts, p));
            let valid = j.valid(d)
                && j.valid(dplus)
                && expected
                    .is_some_and(|(e, ep)| walk::same_k(nk, e, p) && walk::same_k(nkp, ep, p));
            return Advance {
                next: valid.then_some((nk, nkp)),
                choice,
                roots, evaluated, selected,
                used_fallback: false,
                fallback_validity_checks: 0,
                transition_checks: 2,
                validation_failed: !valid,
            };
        }
        return Advance {
            next,
            choice,
            roots, evaluated, selected,
            used_fallback: false,
            fallback_validity_checks: 0,
            transition_checks: 0,
            validation_failed: false,
        };
    }
    // The labels determine the signed successor without a discrete-log
    // oracle. Arithmetic here is native Jacobian arithmetic and is charged.
    let next = signed_fallback(j, t, acts, p, r, g, q, a, b, choice);
    Advance {
        next,
        choice,
        roots, evaluated, selected,
        used_fallback: true,
        fallback_validity_checks: 2,
        transition_checks: 0,
        validation_failed: false,
    }
}

pub fn run(output_dir: &str) {
    let theta_rows =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let mut theta = BTreeMap::new();
    let mut normal = BTreeMap::new();
    for line in theta_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        theta.insert(p, v["selected"]["constants"].clone());
        normal.insert(
            p,
            (
                v["selected"]["r0"].as_u64().unwrap(),
                v["selected"]["delta"].as_u64().unwrap(),
            ),
        );
    }
    let model_rows =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap();
    let mut actions = BTreeMap::new();
    for line in model_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let (r0, delta) = normal[&p];
        let acts: Vec<_> = v["torsion_actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|z| {
                (
                    mul_mod(
                        sub_mod(z["root"].as_u64().unwrap(), r0, p),
                        inv_mod(delta, p),
                        p,
                    ),
                    parse_matrix(&z["action"]["matrix"]),
                )
            })
            .collect();
        actions.insert(p, acts);
    }
    let max_targets = std::env::var("KUMMER_TARGET_CAP")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(1260);
    let start_index = std::env::var("KUMMER_START_INDEX")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(0);
    let mut completed = 0;
    let mut solved = 0;
    let mut seen_targets = 0;
    let validate_transitions =
        std::env::var("KUMMER_VALIDATE_TRANSITIONS").ok().as_deref() == Some("1");
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let input = fs::read_to_string(format!(
            "research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
        ))
        .unwrap();
        let mut out = String::new();
        for line in input.lines() {
            let global_index = seen_targets;
            seen_targets += 1;
            if global_index < start_index {
                continue;
            }
            if completed == max_targets {
                break;
            }
            let v: Value = serde_json::from_str(line).unwrap();
            let p = v["p"].as_u64().unwrap();
            let r = v["r"].as_u64().unwrap();
            walk::reset_ops();
            signed::reset_ops();
            let t = &theta[&p];
            let acts = &actions[&p];
            let f = signed::curve_coeff(t, p);
            let j = Jacobian {
                f: Field::new(p, 1, 0),
                polynomial: f[..6].to_vec(),
            };
            let cases = v["cases"].as_array().unwrap();
            let g: Divisor =
                serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone())
                    .unwrap();
            let q: Divisor =
                serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone())
                    .unwrap();
            let kg = parse_vec4(&cases[1]["coordinate_witness"]["k_expected"]);
            let mut k = parse_vec4(&cases[2]["coordinate_witness"]["k_expected"]);
            let mut kp = parse_vec4(&cases[3]["coordinate_witness"]["k_expected"]);
            let k0 = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
            let consts = ["A", "B", "C", "D"].map(|key| t[key].as_u64().unwrap());
            let ring = Ring::new(r);
            let mut graph = SignedGraph::new(ring.clone());
            let mut nodes = HashMap::new();
            let mut relation_seen = HashSet::new();
            let mut walk_seen = HashSet::new();
            let mut rng = Rng(v["algorithm_seed"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap()
                ^ 0x6b756d6d6572);
            let (mut a, mut b) = (0u64, 1u64);
            let (
                mut attempts,
                mut splits,
                mut recovery_ok,
                mut recovery_fail,
                mut repairs,
                mut restarts,
                mut general_skips,
                mut fallback_steps,
                mut fallback_failures,
                mut fallback_checks,
                mut transition_checks,
            ) = (
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
            );
            let mut recovered = None;
            let mut status = "candidate_cap";
            let cap = 100 * p + 10000;
            while attempts < cap {
                let Some(general) = signed::general_coords(k, t, p) else {
                    general_skips += 1;
                    let step = advance(
                        k,
                        kp,
                        kg,
                        k0,
                        consts,
                        acts,
                        &j,
                        t,
                        p,
                        r,
                        g,
                        q,
                        a,
                        b,
                        validate_transitions,
                    );
                    repairs += step.roots.iter().filter(|x| x.is_some()).count() as u64;
                    attempts += 1;
                    if step.used_fallback {
                        fallback_steps += 1;
                        fallback_checks += step.fallback_validity_checks;
                    }
                    transition_checks += step.transition_checks;
                    let Some((nk, nkp)) = step.next else {
                        if step.validation_failed {
                            status = "transition_mismatch";
                        } else {
                            fallback_failures += 1;
                            status = "fallback_projection_fail";
                        }
                        break;
                    };
                    a = (2 * a + step.choice) % r;
                    b = 2 * b % r;
                    k = nk;
                    kp = nkp;
                    continue;
                };
                let key = (general[1], general[2], general[3]);
                if !walk_seen.insert(key) {
                    restarts += 1;
                    walk_seen.clear();
                    a = rng.next() % r;
                    b = rng.next() % r;
                    let d = j.add(j.mul(g, a), j.mul(q, b));
                    let dplus = j.add(d, g);
                    let Some(kk) = walk::project_any(d, &j, t, acts, p) else {
                        status = "restart_projection_fail";
                        break;
                    };
                    let Some(kpp) = walk::project_any(dplus, &j, t, acts, p) else {
                        status = "restart_projection_fail";
                        break;
                    };
                    k = kk;
                    kp = kpp;
                    continue;
                }
                let a0 = general[2];
                let a1 = sub_mod(0, general[1], p);
                let disc = j.f.sub(j.f.sq(a1), j.f.mul(4, a0));
                if let Some(root) = j.f.sqrt(disc) {
                    let x0 = j.f.mul(j.f.sub(root, a1), p.div_ceil(2));
                    let x1 = j.f.mul(j.f.sub(j.f.neg(root), a1), p.div_ceil(2));
                    if x0 != x1 {
                        let (signed_d, rec_status, _tested) =
                            signed::lift(g, k, kp, kg, t, acts, &j, p);
                        if let Some(d) = signed_d {
                            recovery_ok += 1;
                            if !relation_seen.insert(d.min(j.neg(d))) {
                                // Preserve one relation per signed divisor, as in baseline.
                            } else {
                                let y0 = j.f.add(d.v[0], j.f.mul(d.v[1], x0));
                                let y1 = j.f.add(d.v[0], j.f.mul(d.v[1], x1));
                                let n0 = *nodes.entry(x0).or_insert_with(|| graph.node());
                                let n1 = *nodes.entry(x1).or_insert_with(|| graph.node());
                                splits += 1;
                                recovered = graph.edge(
                                    n0,
                                    if y0 <= p / 2 { 1 } else { -1 },
                                    n1,
                                    if y1 <= p / 2 { 1 } else { -1 },
                                    a,
                                    b,
                                );
                                if let Some(ans) = recovered {
                                    if j.mul(g, ans) == q && ans == v["planted"].as_u64().unwrap() {
                                        status = "verified";
                                    } else {
                                        status = "wrong_scalar";
                                    }
                                    break;
                                }
                            }
                        } else {
                            recovery_fail += 1;
                            if rec_status == "D_zero" {
                                status = "recover_D_zero";
                            }
                        }
                    }
                }
                let step = advance(
                    k,
                    kp,
                    kg,
                    k0,
                    consts,
                    acts,
                    &j,
                    t,
                    p,
                    r,
                    g,
                    q,
                    a,
                    b,
                    validate_transitions,
                );
                repairs += step.roots.iter().filter(|x| x.is_some()).count() as u64;
                attempts += 1;
                if step.used_fallback {
                    fallback_steps += 1;
                    fallback_checks += step.fallback_validity_checks;
                }
                transition_checks += step.transition_checks;
                let Some((nk, nkp)) = step.next else {
                    if step.validation_failed {
                        status = "transition_mismatch";
                    } else {
                        fallback_failures += 1;
                        status = "fallback_projection_fail";
                    }
                    break;
                };
                a = (2 * a + step.choice) % r;
                b = 2 * b % r;
                k = nk;
                kp = nkp;
            }
            if status == "verified" {
                solved += 1;
            }
            completed += 1;
            out.push_str(&format!("{}\n",json!({"schema":"kummer-labelled-fallback/v1","bits":bits,"p":p,"r":r,"target_index":v["target_index"],"workload_id":v["workload_id"],"planted":v["planted"],"source_scalar_verified":v["source_scalar_verified"],"status":status,"recovered":recovered,"source_verified":status=="verified","attempts":attempts,"split_relations":splits,"recover_success":recovery_ok,"recover_fail":recovery_fail,"general_conversion_skips":general_skips,"xadd_repairs":repairs,"walk_restarts":restarts,"fallback_steps":fallback_steps,"fallback_failures":fallback_failures,"fallback_validity_checks":fallback_checks,"transition_checks":transition_checks,"transition_validation_enabled":validate_transitions,"graph_cycles":graph.cycles,"graph_operations":graph.operations,"walk_ops":walk::read_ops(),"recover_ops":signed::read_ops(),"field_counts":j.f.counts()})));
        }
        fs::create_dir_all(output_dir).unwrap();
        fs::write(format!("{output_dir}/collector-{bits}.jsonl"), out).unwrap();
        if completed == max_targets {
            break;
        }
    }
    println!("completed={completed} solved={solved}");
}
fn main() {
    run("research/kummer_differential_collector_20261022/n19-labelled-fallback/results/collector");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c2_oracle(d: Divisor, t: &Value, f: &[u64], p: u64) -> [u64; 4] {
        assert_eq!(d.degree, 2);
        let mm = |x: u64, y: u64| x * y % p;
        let aa = |x: u64, y: u64| (x + y) % p;
        let ss = |x: u64, y: u64| (x + p - y) % p;
        let (a0, a1, b1) = (d.u[0], d.u[1], d.v[1]);
        let (l, mu, nu) = (
            t["lambda"].as_u64().unwrap(),
            t["mu"].as_u64().unwrap(),
            t["nu"].as_u64().unwrap(),
        );
        let mut c = 1;
        let mut lin = 0;
        let mut prev_lin = 0;
        let mut rem0_over_a0 = 0;
        for (n, &coef) in f.iter().enumerate() {
            if n >= 2 {
                rem0_over_a0 = ss(rem0_over_a0, mm(coef, prev_lin));
            }
            let nc = ss(0, mm(a0, lin));
            let nl = ss(c, mm(a1, lin));
            prev_lin = lin;
            (c, lin) = (nc, nl);
        }
        let terms = [
            mm(ss(mu, a0), aa(aa(l, a1), nu)),
            mm(ss(mm(nu, l), a0), aa(aa(1, a1), mu)),
            mm(ss(nu, a0), aa(aa(l, a1), mu)),
            mm(ss(mm(mu, l), a0), aa(aa(1, a1), nu)),
        ];
        let scale = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
        std::array::from_fn(|i| mm(scale[i], ss(ss(terms[i], mm(b1, b1)), rem0_over_a0)))
    }

    #[test]
    fn all_models_force_zero_xadd_and_reconstruct_signed_successor() {
        std::env::set_current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .unwrap();
        let theta_rows =
            fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl")
                .unwrap();
        let theta: BTreeMap<u64, Value> = theta_rows
            .lines()
            .map(|line| {
                let v: Value = serde_json::from_str(line).unwrap();
                (v["p"].as_u64().unwrap(), v["selected"]["constants"].clone())
            })
            .collect();
        let mut seen = HashSet::new();
        for bits in [16, 20, 24, 28, 32, 36, 40] {
            let input = fs::read_to_string(format!(
                "research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
            ))
            .unwrap();
            for line in input.lines() {
                let v: Value = serde_json::from_str(line).unwrap();
                let p = v["p"].as_u64().unwrap();
                if !seen.insert(p) {
                    continue;
                }
                let t = &theta[&p];
                let f = signed::curve_coeff(t, p);
                let j = Jacobian {
                    f: Field::new(p, 1, 0),
                    polynomial: f[..6].to_vec(),
                };
                let cases = v["cases"].as_array().unwrap();
                let g: Divisor = serde_json::from_value(
                    cases[1]["coordinate_witness"]["jacobian_expected"].clone(),
                )
                .unwrap();
                let q: Divisor = serde_json::from_value(
                    cases[2]["coordinate_witness"]["jacobian_expected"].clone(),
                )
                .unwrap();
                let kg = parse_vec4(&cases[1]["coordinate_witness"]["k_expected"]);
                let k = parse_vec4(&cases[2]["coordinate_witness"]["k_expected"]);
                let kp = parse_vec4(&cases[3]["coordinate_witness"]["k_expected"]);
                let constants = ["A", "B", "C", "D"].map(|key| t[key].as_u64().unwrap());
                let r = v["r"].as_u64().unwrap();
                // A zero difference forces the cleared xADD vector to zero
                // in both doubling calls, hence at least one chosen output.
                let forced = advance(
                    k,
                    kp,
                    kg,
                    [0; 4],
                    constants,
                    &[],
                    &j,
                    t,
                    p,
                    r,
                    g,
                    q,
                    0,
                    1,
                    true,
                );
                assert!(forced.used_fallback, "p={p}");
                let (out, out_plus) = forced.next.expect("signed fallback projection");
                let aa = forced.choice % r;
                let bb = 2 % r;
                let d = j.add(j.mul(g, aa), j.mul(q, bb));
                let dplus = j.add(d, g);
                let independent_d = if forced.choice == 0 {
                    j.add(q, q)
                } else {
                    j.add(j.add(q, q), g)
                };
                assert_eq!(d, independent_d, "signed label invariant p={p}");
                assert!(j.valid(d) && j.valid(dplus));
                if d.degree == 2 {
                    assert!(walk::same_k(out, c2_oracle(d, t, &f, p), p));
                }
                if dplus.degree == 2 {
                    assert!(walk::same_k(out_plus, c2_oracle(dplus, t, &f, p), p));
                }
                assert!(walk::same_k(
                    walk::project_any(g, &j, t, &[], p).unwrap(),
                    kg,
                    p
                ));
                assert!(walk::same_k(
                    walk::project_any(q, &j, t, &[], p).unwrap(),
                    k,
                    p
                ));
                assert!(walk::same_k(
                    walk::project_any(j.add(q, g), &j, t, &[], p).unwrap(),
                    kp,
                    p
                ));
                let k0 = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
                let (mut regular_k, mut regular_kp) = (k, kp);
                let (mut label_a, mut label_b) = (0, 1);
                for _ in 0..3 {
                    let step = advance(
                        regular_k,
                        regular_kp,
                        kg,
                        k0,
                        constants,
                        &[],
                        &j,
                        t,
                        p,
                        r,
                        g,
                        q,
                        label_a,
                        label_b,
                        true,
                    );
                    assert!(!step.validation_failed, "regular transition p={p}");
                    assert_eq!(
                        step.transition_checks,
                        if step.used_fallback { 0 } else { 2 }
                    );
                    (regular_k, regular_kp) = step.next.expect("validated regular pair");
                    label_a = (2 * label_a + step.choice) % r;
                    label_b = 2 * label_b % r;
                }
                assert!(j.f.counts().jac_adds > 0);
                assert!(walk::project_any(Divisor::O, &j, t, &[], p).is_some());
                let zero_numerator = Divisor {
                    degree: 2,
                    u: [0, p - 1, 1],
                    v: [0, 0],
                };
                assert!(j.valid(zero_numerator));
                assert!(walk::project_divisor(zero_numerator, t, p).is_none());
                let repaired = walk::project_any(zero_numerator, &j, t, &[], p).unwrap();
                assert!(walk::same_k(
                    repaired,
                    c2_oracle(zero_numerator, t, &f, p),
                    p
                ));
                for x in [
                    0,
                    1,
                    t["lambda"].as_u64().unwrap(),
                    t["mu"].as_u64().unwrap(),
                    t["nu"].as_u64().unwrap(),
                ] {
                    let branch = Divisor {
                        degree: 1,
                        u: [sub_mod(0, x, p), 1, 0],
                        v: [0, 0],
                    };
                    assert!(j.valid(branch));
                    assert!(walk::project_any(branch, &j, t, &[], p).is_some());
                }
            }
        }
        assert_eq!(seen.len(), 21);
    }
}

pub fn original_gstep_fixture(input:&Value)->Value{
 let v=&input["target"];let th=&input["theta"]["selected"];let t=&th["constants"];let p=v["p"].as_u64().unwrap();let r=v["r"].as_u64().unwrap();assert!((2..=(1u64<<63)).contains(&r));let r0=th["r0"].as_u64().unwrap();let delta=th["delta"].as_u64().unwrap();let acts:Vec<_>=input["model"]["torsion_actions"].as_array().unwrap().iter().map(|z|(mul_mod(sub_mod(z["root"].as_u64().unwrap(),r0,p),inv_mod(delta,p),p),parse_matrix(&z["action"]["matrix"]))).collect();let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};let cases=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();assert!(j.valid(g)&&j.valid(q));assert_ne!(j.add(g,g),Divisor::O);assert_eq!(j.mul(g,r),Divisor::O);assert_eq!(j.mul(q,r),Divisor::O);
 let before=j.f.counts();walk::reset_ops();signed::reset_ops();let qg=j.add(q,g);let(mut k,mut kp)=(walk::project_any(q,&j,t,&acts,p).unwrap(),walk::project_any(qg,&j,t,&acts,p).unwrap());let kg=walk::project_any(g,&j,t,&acts,p).unwrap();let k0=walk::project_any(Divisor::O,&j,t,&acts,p).unwrap();let initialization=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});let constants=["A","B","C","D"].map(|z|t[z].as_u64().unwrap());let(mut a,mut b)=(0,1);let mut rows=vec![];let mut adapter=crate::graph::Adapter::new(r);
 for index in 0..3{
  assert!(a<r&&b<r);walk::reset_ops();signed::reset_ops();let before=j.f.counts();let step=advance(k,kp,kg,k0,constants,&acts,&j,t,p,r,g,q,a,b,false);let transition=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});a=(2*a+step.choice)%r;b=2*b%r;
  let Some((nk,nkp))=step.next else{rows.push(json!({"index":index,"status":"step_failed","choice":step.choice,"labels":[a,b],"transition":transition,"fallback":step.used_fallback,"pass":false}));break;};
  // No expected signed point is computed until after recovery consumes propagated pair.
  walk::reset_ops();signed::reset_ops();let before=j.f.counts();let(recovered,status,tested)=signed::lift(g,nk,nkp,kg,t,&acts,&j,p);let recovery=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});
  let graph_row=adapter.consume(&j,recovered,a,b);let terminal=graph_row["answer"].as_u64();
  walk::reset_ops();signed::reset_ops();let before=j.f.counts();let expected=j.add(j.mul(g,a),j.mul(q,b));let ep=j.add(expected,g);let expected_pair=[expected,ep].map(|d|walk::project_any(d,&j,t,&acts,p).unwrap());let corners=[walk::same_k(nk,expected_pair[0],p),walk::same_k(nkp,expected_pair[1],p)];let signed_ok=recovered==Some(expected);let verification=json!({"field":j.f.counts()-before,"walk":walk::read_ops(),"signed":signed::read_ops()});
  rows.push(json!({"index":index,"status":"checked","choice":step.choice,"labels":[a,b],"pair":[nk,nkp],"roots":step.roots,"evaluated":step.evaluated,"selected":step.selected,"fallback":step.used_fallback,"fallback_checks":step.fallback_validity_checks,"internal_transition_checks":step.transition_checks,"corner_matches":corners,"recovered":recovered,"expected":expected,"recovery_status":status,"recovery_tested":tested,"signed_ok":signed_ok,"pass":corners.iter().all(|&x|x)&&signed_ok,"graph":graph_row,"counts":{"transition":transition,"recovery":recovery,"verification":verification}}));k=nk;kp=nkp;if let Some(ans)=terminal{assert_eq!(j.mul(g,ans),q);assert_eq!(Some(ans),v["planted"].as_u64());break;}
 }
 json!({"p":p,"r":r,"workload_id":v["workload_id"],"initialization":initialization,"rows":rows})
}
pub fn duplicate_fixture(input:&Value)->Value{
 let v=&input["target"];let t=&input["theta"]["selected"]["constants"];let p=v["p"].as_u64().unwrap();let r=v["r"].as_u64().unwrap();let k=v["planted"].as_u64().unwrap();assert_eq!((p,r,k),(739,34147,31124));let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};let cases=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let d:Divisor=serde_json::from_value(input["retained_divisor"].clone()).unwrap();let(a,b)=(2u64,8u64);let before=j.f.counts();assert!(j.valid(d));assert_eq!(j.mul(g,k),q);assert_eq!(j.add(j.mul(g,a),j.mul(q,b)),d);let input_validation=j.f.counts()-before;
 let alt_a=(a+k)%r;let alt_b=(b+r-1)%r;let mut out=vec![];
 for name in["informative_same","informative_opposite","redundant_same","redundant_opposite","invalid_zero_target","invalid_wrong_candidate","noncanonical","changed_target"]{
  let actual_q=if name=="changed_target"{j.mul(g,2)}else{q};let before=j.f.counts();let mut labels=crate::duplicate::Labels::new(&j,g,actual_q,r);let initialization=j.f.counts()-before;let mut graph=crate::graph::Adapter::new(r);
  let first=labels.ingest(d,a,b);assert_eq!(first["emit_graph_edge"],true);
  // Crucial: normalized labels belong ONLY to the table's canonical key.
  // First graph edge receives the original signed divisor and original labels.
  let edge=graph.consume(&j,Some(d),a,b);assert_eq!(edge["status"],"inserted");let before_graph=graph.snapshot();
  let(dd,aa,bb,expected)=match name{
   "informative_same"=>(d,alt_a,alt_b,"candidate_verified"),
   "informative_opposite"=>(j.neg(d),(r-alt_a)%r,(r-alt_b)%r,"candidate_verified"),
   "redundant_same"=>(d,a,b,"redundant"),
   "redundant_opposite"=>(j.neg(d),r-a,r-b,"redundant"),
   "invalid_zero_target"=>(d,a+1,b,"invariant_error"),
   "invalid_wrong_candidate"=>(d,(alt_a+1)%r,alt_b,"candidate_rejected"),
   "noncanonical"=>(d,r,b,"noncanonical_label"),
   _=>(d,alt_a,alt_b,"candidate_rejected")};
  let before=j.f.counts();let valid_label=aa<r&&bb<r&&j.add(j.mul(g,aa),j.mul(actual_q,bb))==dd;let label_reference=j.f.counts()-before;
  let second=labels.ingest(dd,aa,bb);assert_eq!(second["status"],expected);assert_eq!(second["emit_graph_edge"],false);let after_graph=graph.snapshot();assert_eq!(before_graph,after_graph);assert_eq!(after_graph["equations"],1);assert_eq!(after_graph["cycles"],0);
  if expected=="candidate_verified"{assert_eq!(second["answer"],k);assert!(valid_label)}else{assert!(second["answer"].is_null())}
  if name.starts_with("redundant"){assert!(valid_label)}
  out.push(json!({"name":name,"first":first,"first_graph_edge":edge,"second_input":{"divisor":dd,"a":aa,"b":bb},"second":second,"label_valid":valid_label,"before_graph":before_graph,"after_graph":after_graph,"initialization_field":initialization,"label_reference_field":label_reference,"pass":true}));
 }
 json!({"schema":"native-signed-duplicate-adapter/v1","p":p,"r":r,"planted":k,"public_known_answer":true,"alternate_labels_deliberately_planted":true,"new_trajectory_executed":false,"input_validation_field":input_validation,"controls":out,"all_pass":true})
}

// Archival reanalysis only: all comparisons retained, no recurrence run.
pub fn archive_target(base:&Value,v:&Value,row:usize){
 let cases=v["cases"].as_array().unwrap();
 let setup=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
 assert_eq!(v["p"],739);assert_eq!(v["r"],34147);assert_eq!(cases.len(),7);
 assert!(v["algorithm_seed"].is_string()&&v["workload_sha256"].as_str().unwrap().len()==64);
 let p=739;let r=34147;let f=signed::curve_coeff(&base["theta"]["selected"]["constants"],p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};
 let g:Divisor=serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();
 assert_eq!((cases[1]["a"].as_u64(),cases[1]["b"].as_u64()),(Some(1),Some(0)));assert_eq!((cases[2]["a"].as_u64(),cases[2]["b"].as_u64()),(Some(0),Some(1)));
 let before=j.f.counts();assert!(j.valid(g)&&j.valid(q));assert_ne!(g,Divisor::O);assert_eq!(j.mul(g,r),Divisor::O);assert_eq!(j.mul(q,r),Divisor::O);assert_eq!(j.mul(g,v["planted"].as_u64().unwrap()),q);let checks=j.f.counts()-before;
 (j,g,q,checks)
 }));
 let Ok((j,g,q,setup_counts))=setup else{for c in cases{println!("{}",json!({"archive_row":row,"target_index":v["target_index"],"case_index":c["case_index"],"status":"partition_binding_failed","native_evaluated":false}));}return};
 let r=34147;let before=j.f.counts();let mut table=crate::duplicate::Labels::new(&j,g,q,r);let constructor_counts=j.f.counts()-before;let mut graph=crate::graph::Adapter::new(r);let mut terminal=false;let mut partition_poisoned=false;let mut firsts=std::collections::HashMap::new();
 for c in cases{
 let common=json!({"archive_row":row,"target_index":v["target_index"],"workload_id":v["workload_id"],"workload_sha256":v["workload_sha256"],"algorithm_seed":v["algorithm_seed"],"case_index":c["case_index"],"provenance":"retained_coordinate_fixture","g":g,"q":q,"polynomial":j.polynomial,"p":739,"r":r});
 if partition_poisoned{println!("{}",json!({"identity":common,"status":"partition_poisoned","native_evaluated":false}));continue}
 let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
 let d:Divisor=serde_json::from_value(c["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let a=c["a"].as_u64().unwrap();let b=c["b"].as_u64().unwrap();let before=j.f.counts();
 let valid=a<r&&b<r&&j.valid(d)&&j.add(j.mul(g,a),j.mul(q,b))==d;let reference=j.f.counts()-before;
 if !valid{return json!({"status":"invalid_label_or_divisor","native_evaluated":true,"label_reference_field":reference,"a":a,"b":b,"divisor":d})}
 let before=j.f.counts();let eligible=if d.degree!=2{false}else{let disc=j.f.sub(j.f.sq(d.u[1]),j.f.mul(4,d.u[0]));j.f.sqrt(disc).map(|x|x!=0).unwrap_or(false)};let eligibility=j.f.counts()-before;
 if !eligible{return json!({"status":"collector_ineligible","native_evaluated":true,"label_reference_field":reference,"eligibility_field":eligibility,"a":a,"b":b,"divisor":d})}
 let before=j.f.counts();let key=d.min(j.neg(d));let key_reference=j.f.counts()-before;let first_case=*firsts.entry(key).or_insert(c["case_index"].as_u64().unwrap());let diagnostic=terminal;let result=table.ingest(d,a,b);let edge=if !terminal&&result["emit_graph_edge"]==true{graph.consume(&j,Some(d),a,b)}else{json!({"status":if terminal{"post_terminal_not_forwarded"}else{"duplicate_suppressed"}})};
 let verify_before=j.f.counts();for ans in [&result["answer"],&edge["answer"]]{if let Some(k)=ans.as_u64(){assert_eq!(j.mul(g,k),q);terminal=true}}
 json!({"status":"compared","native_evaluated":true,"a":a,"b":b,"divisor":d,"first_case_index":first_case,"label_reference_field":reference,"eligibility_field":eligibility,"key_reference_field":key_reference,"adapter":result,"graph":edge,"graph_state":graph.snapshot(),"post_terminal_diagnostic":diagnostic,"terminal_reference_field":j.f.counts()-verify_before})
 }));
 match outcome{Ok(z)=>println!("{}",json!({"identity":common,"result":z,"setup_checks_field":setup_counts,"constructor_field":constructor_counts,"setup_count_once_per_target":true,"setup_counter_scope":"nonadditive repeated metadata; aggregate unique physical archive_row only"})),Err(_)=>{partition_poisoned=true;println!("{}",json!({"identity":common,"status":"native_case_panic","native_evaluated":true}));}}
 }
}
