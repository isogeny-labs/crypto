//! Bounded two-box incidence encodings, with a matched ordinary mask.
use crate::{digest, toy_field::*, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

fn lagrange(p: u64, n: usize, i: usize) -> Vec<u64> {
    let mut a = vec![1];
    let mut den = 1;
    for j in 0..n {
        if j != i {
            let mut b = vec![0; a.len() + 1];
            for (d, c) in a.iter().enumerate() {
                b[d] = (b[d] + c * (p - j as u64)) % p;
                b[d + 1] = (b[d + 1] + c) % p;
            }
            a = b;
            den = den * (p + i as u64 - j as u64) % p;
        }
    }
    let z = inv(den, p);
    a.iter_mut().for_each(|c| *c = *c * z % p);
    a
}
fn interpolate(p: u64, grid: &[Vec<bool>]) -> Vec<Vec<u64>> {
    let h = grid.len();
    let k = grid[0].len();
    let lr = (0..h).map(|i| lagrange(p, h, i)).collect::<Vec<_>>();
    let lu = (0..k).map(|i| lagrange(p, k, i)).collect::<Vec<_>>();
    let mut c = vec![vec![0; k]; h];
    for i in 0..h {
        for j in 0..k {
            if grid[i][j] {
                for a in 0..h {
                    for b in 0..k {
                        c[a][b] = (c[a][b] + lr[i][a] * lu[j][b]) % p;
                    }
                }
            }
        }
    }
    c
}
fn eval(p: u64, c: &[Vec<u64>], r: u64, u: u64) -> u64 {
    c.iter().rev().fold(0, |a, row| {
        (a * r + row.iter().rev().fold(0, |b, x| (b * u + x) % p)) % p
    })
}
fn degree(c: &[Vec<u64>]) -> usize {
    c.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, x)| (*x != 0).then_some(i + j))
        })
        .max()
        .unwrap_or(0)
}
fn gp_matrix(c: &[Vec<u64>]) -> String {
    format!(
        "[{}]",
        c.iter()
            .map(|row| format!(
                "[{}]",
                row.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}
pub fn run(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);\n");
    gp.push_str("twist_inc_eval(C,r,u,p)={sum(i=1,#C,sum(j=1,#C[i],C[i][j]*Mod(r,p)^(i-1)*Mod(u,p)^(j-1)))};\n");
    gp.push_str("twist_inc_check(p,a,b,d,l,h,k,C,D,B)={my(E=ellinit([Mod(a,p),Mod(b,p)]),ET=ellinit([Mod(d^2*a,p),Mod(d^3*b,p)]),A=List(),G=List());for(r=0,h-1,for(u=0,k-1,my(x=(r-l*u)%p,yes=(#ellordinate(E,Mod(x,p))>0)&&(#ellordinate(ET,Mod(d*u,p))>0),ordinary=(setsearch(B,x)>0));if(twist_inc_eval(C,r,u,p)!=yes,error(\"twist mask mismatch\"));if(twist_inc_eval(D,r,u,p)!=ordinary,error(\"ordinary mask mismatch\"));if(yes,listput(A,x));if(ordinary,listput(G,x))));if(Set(A)!=B||Set(G)!=B,error(\"projected incidence set mismatch\"));my(bad=C);bad[1][1]+=1;if(twist_inc_eval(bad,0,0,p)==twist_inc_eval(C,0,0,p),error(\"false mask not rejected\"));};\n");
    for f in fixtures
        .iter()
        .filter(|f| prime(f.order) && f.order % 2 == 1)
    {
        let e = f.curve();
        let nt = 2 * (f.p + 1) - f.order;
        assert_eq!(nt % 2, 1);
        let rational = e
            .rational()
            .into_iter()
            .filter_map(|p| p.map(|(x, y)| (x.0, Some((x, y)))))
            .collect::<BTreeMap<_, _>>();
        let et = Quad::new(
            f.p,
            f.a * f.d % f.p * f.d % f.p,
            f.b * f.d % f.p * f.d % f.p * f.d % f.p,
        );
        let twist = et
            .rational()
            .into_iter()
            .filter_map(|p| {
                p.map(|(x, y)| {
                    let u = x.0 * inv(f.d, f.p) % f.p;
                    (
                        u,
                        Some((Q(u, 0), Q(0, y.0 * inv(f.d * f.d % f.p, f.p) % f.p))),
                    )
                })
            })
            .collect::<BTreeMap<_, _>>();
        for lambda in [1u64, 2, 3] {
            for h in [2usize, 4, 8] {
                for k in [2usize, 4, 8] {
                    let mut grid = vec![vec![false; k]; h];
                    let mut base = BTreeSet::new();
                    let mut witnesses = vec![];
                    let mut pairs = 0;
                    for (r, row) in grid.iter_mut().enumerate() {
                        for (u, yes) in row.iter_mut().enumerate() {
                            let x = (r as u64 + f.p - lambda * u as u64 % f.p) % f.p;
                            if let (Some(t), Some(v)) = (rational.get(&x), twist.get(&(u as u64))) {
                                *yes = true;
                                pairs += 1;
                                base.insert(x);
                                let s = e.pa(
                                    e.pm(*t, ((f.order + 1) / 2) as u128),
                                    e.pm(*v, ((nt + 1) / 2) as u128),
                                );
                                assert!(e.on(s));
                                assert_eq!(e.trace(s), *t);
                                assert_eq!(e.pa(s, e.pn(e.frob(s))), *v);
                                witnesses.push(json!({"r":r,"u":u,"source_point":t,"anti_point":v,"extension_point":s}));
                            }
                        }
                    }
                    let ordinary = (0..h)
                        .map(|r| {
                            (0..k)
                                .map(|u| {
                                    base.contains(
                                        &((r as u64 + f.p - lambda * u as u64 % f.p) % f.p),
                                    )
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    let c = interpolate(f.p, &grid);
                    let d = interpolate(f.p, &ordinary);
                    let mut direct_image = BTreeSet::new();
                    let mut ordinary_image = BTreeSet::new();
                    for r in 0..h {
                        for u in 0..k {
                            assert_eq!(eval(f.p, &c, r as u64, u as u64), u64::from(grid[r][u]));
                            assert_eq!(
                                eval(f.p, &d, r as u64, u as u64),
                                u64::from(ordinary[r][u])
                            );
                            let x = (r as u64 + f.p - lambda * u as u64 % f.p) % f.p;
                            if grid[r][u] {
                                direct_image.insert(x);
                            }
                            if ordinary[r][u] {
                                ordinary_image.insert(x);
                            }
                        }
                    }
                    assert_eq!(direct_image, base);
                    assert_eq!(ordinary_image, base);
                    let mut bad = c.clone();
                    bad[0][0] = (bad[0][0] + 1) % f.p;
                    assert_ne!(eval(f.p, &bad, 0, 0), eval(f.p, &c, 0, 0));
                    let id = format!("{}-l{lambda}-h{h}-k{k}", f.id);
                    let record = json!({"experiment":"E30","fixture":f.id,"p":f.p,"lambda":lambda,"h":h,"k":k,
                "base_x":base,"accepted_twist_pairs":pairs,"ordinary_mask_pairs":ordinary.iter().flatten().filter(|x|**x).count(),
                "mask":c,"ordinary_mask":d,"mask_total_degree":degree(&c),"ordinary_mask_total_degree":degree(&d),
                "direct_membership_degree":base.len(),"incidence_max_generator_degree":h.max(k).max(degree(&c)),
                "ordinary_incidence_max_generator_degree":h.max(k).max(degree(&d)),"witnesses":witnesses,
                "same_projected_set":true,"all_grid_values_verified":true,"false_constant_shift_rejected":true,
                "generic_ordinary_encoding_uses_no_twist_after_base_construction":true,"performance_claim":false,
                "full_experiment_complete":false,"status":"finite-incidence-and-matched-ordinary-mask-verified"});
                    write_json(out.join(format!("{id}.json")), &record)?;
                    gp.push_str(&format!(
                        "twist_inc_check({},{},{},{},{lambda},{h},{k},{},{},[{}]);\n",
                        f.p,
                        f.a,
                        f.b,
                        f.d,
                        gp_matrix(&c),
                        gp_matrix(&d),
                        base.iter()
                            .map(u64::to_string)
                            .collect::<Vec<_>>()
                            .join(",")
                    ));
                    rows.push(json!({"experiment":"E30","fixture":f.id,"p":f.p,"lambda":lambda,"h":h,"k":k,
                "B":base.len(),"pairs":pairs,"direct_degree":base.len(),"twist_degree":h.max(k).max(degree(&c)),
                "ordinary_degree":h.max(k).max(degree(&d)),"record":format!("{id}.json"),"status":"finite-incidence-and-matched-ordinary-mask-verified"}));
                }
            }
        }
    }
    gp.push_str("print(\"ALL_INCIDENCE_REPLAY_OK\");quit;\n");
    fs::write(out.join("verify.gp"), &gp)?;
    let lower = rows
        .iter()
        .filter(|r| r["twist_degree"].as_u64() < r["direct_degree"].as_u64())
        .count();
    let ordinary_lower = rows
        .iter()
        .filter(|r| r["ordinary_degree"].as_u64() < r["twist_degree"].as_u64())
        .count();
    write_json(
        out.join("INVENTORY.json"),
        &json!({"cells":rows.len(),"twist_lower_max_generator_degree_than_direct":lower,
        "ordinary_lower_max_generator_degree_than_twist":ordinary_lower,"same_projected_sets_verified":rows.len(),
        "gp_script_sha256":digest(gp.as_bytes()),"performance_claim":false,"novelty_established":false,
        "interpretation":"Polynomial degree is an encoding descriptor. More variables, construction and solving costs remain unmeasured."}),
    )?;
    Ok(rows)
}
