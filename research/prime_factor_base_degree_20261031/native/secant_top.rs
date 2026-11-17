//! Exact upper identities and a lower dual witness for the quadratic top ideal.
//! Fixed F17 proof certificate, not an F4/refutation-degree measurement.
use super::{add, mul};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
type M = [u32; 3];
type P = BTreeMap<M, u64>;
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Term {
    m: M,
    c: u64,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Recipe {
    generator: usize,
    m: M,
    c: u64,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Upper {
    target: M,
    recipe: Vec<Recipe>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Branch {
    signs: [i8; 2],
    generators: Vec<Vec<Term>>,
    upper: Vec<Upper>,
    lower: Vec<Term>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Certificate {
    schema: String,
    p: u64,
    metric: String,
    degree: u32,
    branches: Vec<Branch>,
}
fn inv(c: u64) -> u64 {
    (0..15).fold(1, |out, _| mul(out, c))
}
fn generators(signs: [i8; 2]) -> Vec<Vec<Term>> {
    let c = if signs[0] == 1 { 4 } else { 13 };
    let d = if signs[1] == 1 { 4 } else { 13 };
    vec![
        vec![Term { m: [0, 0, 2], c: 1 }],
        vec![Term { m: [1, 1, 0], c: 1 }],
        vec![
            Term { m: [2, 0, 0], c },
            Term {
                m: [1, 0, 1],
                c: 16,
            },
        ],
        vec![
            Term { m: [0, 2, 0], c: d },
            Term {
                m: [0, 1, 1],
                c: 16,
            },
        ],
    ]
}
fn insert(p: &mut P, m: M, c: u64) {
    let v = add(p.get(&m).copied().unwrap_or(0), c);
    if v == 0 {
        p.remove(&m);
    } else {
        p.insert(m, v);
    }
}
fn polynomial(terms: &[Term]) -> Result<P, String> {
    let mut p = P::new();
    for t in terms {
        if !(1..17).contains(&t.c) || t.m.iter().any(|e| *e > 3) || p.contains_key(&t.m) {
            return Err("noncanonical polynomial/dual".into());
        }
        p.insert(t.m, t.c);
    }
    Ok(p)
}
pub(super) fn make() -> Certificate {
    let mut branches = Vec::new();
    for signs in [[1, 1], [1, -1], [-1, 1], [-1, -1]] {
        let gens = generators(signs);
        let c = inv(gens[2][0].c);
        let d = inv(gens[3][0].c);
        let r = |generator, m, c| Recipe { generator, m, c };
        let upper = vec![
            Upper {
                target: [3, 0, 0],
                recipe: vec![
                    r(2, [1, 0, 0], c),
                    r(2, [0, 0, 1], mul(c, c)),
                    r(0, [1, 0, 0], mul(c, c)),
                ],
            },
            Upper {
                target: [2, 1, 0],
                recipe: vec![r(1, [1, 0, 0], 1)],
            },
            Upper {
                target: [2, 0, 1],
                recipe: vec![r(2, [0, 0, 1], c), r(0, [1, 0, 0], c)],
            },
            Upper {
                target: [1, 2, 0],
                recipe: vec![r(1, [0, 1, 0], 1)],
            },
            Upper {
                target: [1, 1, 1],
                recipe: vec![r(1, [0, 0, 1], 1)],
            },
            Upper {
                target: [1, 0, 2],
                recipe: vec![r(0, [1, 0, 0], 1)],
            },
            Upper {
                target: [0, 3, 0],
                recipe: vec![
                    r(3, [0, 1, 0], d),
                    r(3, [0, 0, 1], mul(d, d)),
                    r(0, [0, 1, 0], mul(d, d)),
                ],
            },
            Upper {
                target: [0, 2, 1],
                recipe: vec![r(3, [0, 0, 1], d), r(0, [0, 1, 0], d)],
            },
            Upper {
                target: [0, 1, 2],
                recipe: vec![r(0, [0, 1, 0], 1)],
            },
            Upper {
                target: [0, 0, 3],
                recipe: vec![r(0, [0, 0, 1], 1)],
            },
        ];
        branches.push(Branch {
            signs,
            generators: gens,
            upper,
            lower: vec![Term { m: [2, 0, 0], c }, Term { m: [1, 0, 1], c: 1 }],
        });
    }
    Certificate {
        schema: "secant-graph-homogeneous-certificate/v1".into(),
        p: 17,
        metric: "first_zero_component_of_top_ideal".into(),
        degree: 3,
        branches,
    }
}
pub(super) fn check(cert: &Certificate) -> Result<(), String> {
    if cert.schema != "secant-graph-homogeneous-certificate/v1"
        || cert.p != 17
        || cert.degree != 3
        || cert.metric != "first_zero_component_of_top_ideal"
        || cert.branches.len() != 4
    {
        return Err("wrong certificate scope".into());
    }
    let mut signs = BTreeSet::new();
    for b in &cert.branches {
        if b.signs.iter().any(|s| ![-1, 1].contains(s)) || !signs.insert(b.signs) {
            return Err("missing or duplicate sign branch".into());
        }
        let actual = b
            .generators
            .iter()
            .map(|g| polynomial(g))
            .collect::<Result<Vec<_>, _>>()?;
        let expected = generators(b.signs)
            .iter()
            .map(|g| polynomial(g))
            .collect::<Result<Vec<_>, _>>()?;
        if actual != expected || b.upper.len() != 10 {
            return Err("wrong top ideal or incomplete upper coverage".into());
        }
        let mut targets = BTreeSet::new();
        for u in &b.upper {
            if u.target.iter().any(|e| *e > 3)
                || u.target.iter().sum::<u32>() != 3
                || !targets.insert(u.target)
            {
                return Err("bad target monomial".into());
            }
            let mut out = P::new();
            for r in &u.recipe {
                if r.generator >= 4
                    || r.m.iter().any(|e| *e > 1)
                    || r.m.iter().sum::<u32>() != 1
                    || !(1..17).contains(&r.c)
                {
                    return Err("bad upper recipe".into());
                }
                for (m, c) in &actual[r.generator] {
                    insert(
                        &mut out,
                        [m[0] + r.m[0], m[1] + r.m[1], m[2] + r.m[2]],
                        mul(*c, r.c),
                    );
                }
            }
            if out != BTreeMap::from([(u.target, 1)]) {
                return Err("upper polynomial identity fails".into());
            }
        }
        let dual = polynomial(&b.lower)?;
        if dual.keys().any(|m| m.iter().sum::<u32>() != 2) || dual.get(&[1, 0, 1]) != Some(&1) {
            return Err("bad lower witness".into());
        }
        for generator in &actual {
            if generator.iter().fold(0, |s, (m, c)| {
                add(s, mul(*c, dual.get(m).copied().unwrap_or(0)))
            }) != 0
            {
                return Err("lower witness fails to annihilate top generator".into());
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_degree_three_has_upper_and_lower_proof_in_every_branch() {
        check(&make()).unwrap();
        let committed: Certificate =
            serde_json::from_str(include_str!("../secant/homogeneous-degree.json")).unwrap();
        check(&committed).unwrap();
    }
    #[test]
    fn corruption_or_discarded_branch_is_rejected() {
        let mut c = make();
        c.branches[0].upper[0].recipe[0].c = 1;
        assert!(check(&c).is_err());
        let mut c = make();
        c.branches.pop();
        assert!(check(&c).is_err());
        let mut c = make();
        c.branches[0].lower.clear();
        assert!(check(&c).is_err());
    }
}
