//! Actual theta-class and finite character descent data; not section coordinates.
use super::*;
type V2 = [usize; 2];
fn mv2(m: [[usize; 2]; 2], v: V2) -> V2 {
    [
        (m[0][0] * v[0] + m[0][1] * v[1]) % 19,
        (m[1][0] * v[0] + m[1][1] * v[1]) % 19,
    ]
}
fn inv2(m: [[usize; 2]; 2]) -> Option<[[usize; 2]; 2]> {
    let d = (m[0][0] * m[1][1] + 361 - m[0][1] * m[1][0]) % 19;
    let k = (1..19).find(|i| i * d % 19 == 1)?;
    Some([
        [m[1][1] * k % 19, (19 - m[0][1]) * k % 19],
        [(19 - m[1][0]) * k % 19, m[0][0] * k % 19],
    ])
}
fn pm2(m: &[Vec<usize>], code: usize) -> usize {
    m.iter().enumerate().fold(0, |out, (i, r)| {
        out | ((r
            .iter()
            .enumerate()
            .map(|(j, a)| a * ((code >> j) & 1))
            .sum::<usize>()
            % 2)
            << i)
    })
}
fn delta(a: O) -> i64 {
    (a.a * a.b).rem_euclid(2)
}
fn bias_controls() -> Value {
    let f = Field::find(4);
    let mut records = vec![];
    for trace in [-1, 1] {
        let c = Curve {
            f,
            a: if trace == 1 { 1 } else { 0 },
            b: 1,
        };
        let mut pts = vec![Pt::O];
        for x in 0..16 {
            for y in 0..16 {
                let p = Pt::A(x, y);
                if c.valid(p) {
                    pts.push(p);
                }
            }
        }
        assert_eq!(pts.len(), 16);
        let t = Pt::A(0, 1);
        assert_eq!(c.times(t, 2), Pt::O);
        assert_ne!(t, Pt::O);
        assert_eq!(
            pts.iter()
                .filter(|p| **p != Pt::O && c.times(**p, 2) == Pt::O)
                .count(),
            1
        );
        let mut cases = vec![];
        for a in -5..=5 {
            for b in -5..=5 {
                let alpha = O::n(a, b);
                let n = alpha.mul(alpha.bar());
                assert_eq!(n.b, 0);
                if ![1, 2, 4, 8, 16].contains(&n.a) {
                    continue;
                }
                let mut z = alpha;
                let mut inseparable = 1;
                while z.a % 2 == 0 {
                    z = O::n(z.b - z.a / 2, -z.a / 2);
                    inseparable *= 2;
                    assert!(inseparable <= 16);
                }
                let kernel = pts
                    .iter()
                    .copied()
                    .filter(|p| {
                        let wp = if trace == -1 {
                            c.frob(*p, 1)
                        } else {
                            c.neg(c.frob(*p, 1))
                        };
                        c.add(negp(c, *p, a), negp(c, wp, b)) == Pt::O
                    })
                    .collect::<Vec<_>>();
                assert_eq!(kernel.len() as i64 * inseparable, n.a);
                let sum = kernel.iter().fold(Pt::O, |v, p| c.add(v, *p));
                let weighted = c.times(sum, inseparable as u64);
                assert_eq!(weighted, if delta(alpha) == 1 { t } else { Pt::O });
                cases.push(json!({"alpha":alpha,"degree":n.a,"inseparable_degree":inseparable,"separable_kernel_points":kernel.len(),"weighted_kernel_sum_is_T":weighted==t}));
            }
        }
        records.push(json!({"trace":trace,"F16_modulus":f.modulus,"rational_points":pts.len(),"cases":cases}));
    }
    json!({"records":records,"scope":"all selected2-power CM kernels exhaust their separable degree;weighted sum checks the proposed Picard correction"})
}
pub fn run(dir: &Path) {
    let old = read(&dir.join("g14-canonical-polarized-target.json"));
    let forms = read(&dir.join("gl2b-integral-polarizations.json"));
    let r: M = vec![
        vec![O::n(-1, 0); 4],
        vec![O::n(1, 0), O::default(), O::default(), O::default()],
        vec![O::default(), O::n(1, 0), O::default(), O::default()],
        vec![O::default(), O::default(), O::n(1, 0), O::default()],
    ];
    assert_eq!((0..5).fold(ident(4), |x, _| mm(&x, &r)), ident(4));
    let pic: Vec<Vec<usize>> = ct(&r)
        .iter()
        .map(|row| row.iter().map(|v| v.a.rem_euclid(2) as usize).collect())
        .collect();
    let mut records = vec![];
    for g in old["records"].as_array().unwrap() {
        let h: M = serde_json::from_value(g["H"].clone()).unwrap();
        assert_eq!(mm(&mm(&ct(&r), &h), &r), h);
        let row = forms["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| {
                v["hermitian_matrix"] == g["H"]
                    && v["target_trace"] == g["target_trace"]
                    && v["coefficients"] == g["coefficients"]
                    && v["congruence_coset"] == g["coset"]
            })
            .unwrap();
        let trace = g["target_trace"].as_i64().unwrap();
        let w = if trace == -1 {
            rep(O::n(0, 1))
        } else {
            rep(O::n(0, -1))
        };
        let mut integral = vec![vec![0; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                integral[i][j] = r[i / 2][j / 2].a * w[i % 2][j % 2];
            }
        }
        assert_eq!(json!(integral), row["frobenius"]);
        let mut drift = 0;
        for j in 0..4 {
            let mut d = 0;
            for i in 0..4 {
                d += h[i][i].a * delta(r[i][j]);
            }
            for i in 0..4 {
                for k in i + 1..4 {
                    let a = r[i][j];
                    let b = h[i][k].mul(r[k][j]);
                    d += delta(a.add(b)) - delta(a) - delta(b);
                }
            }
            drift |= (d.rem_euclid(2) as usize) << j;
        }
        let corrections = (0..16)
            .map(|d| {
                let fixed = (0..16)
                    .filter(|a| pm2(&pic, *a) ^ d == *a)
                    .collect::<Vec<_>>();
                assert_eq!(fixed.len(), 1);
                fixed[0]
            })
            .collect::<Vec<_>>();
        let c: [[usize; 2]; 2] =
            serde_json::from_value(g["Cartier_character_Frobenius_matrix"].clone()).unwrap();
        let ic = [
            [(20 - c[0][0]) % 19, (19 - c[0][1]) % 19],
            [(19 - c[1][0]) % 19, (20 - c[1][1]) % 19],
        ];
        let inverse = inv2(ic).expect("nontrivial arithmetic theta obstruction");
        let mut character_corrections = vec![];
        for a in 0..19 {
            for b in 0..19 {
                let d = [a, b];
                let answer = mv2(inverse, d);
                let mut count = 0;
                for x in 0..19 {
                    for y in 0..19 {
                        let v = [x, y];
                        let cv = mv2(c, v);
                        if [(cv[0] + a) % 19, (cv[1] + b) % 19] == v {
                            count += 1;
                            assert_eq!(v, answer);
                        }
                    }
                }
                assert_eq!(count, 1);
                character_corrections.push(answer);
            }
        }
        println!("G15:graph{},theta drift{drift},unique correction{},all361 character drifts uniquely corrected",g["graph"],corrections[drift]);
        let mut pull = vec![vec![O::default(); 5]; 5];
        for i in 0..4 {
            for j in 0..4 {
                pull[i][j] = h[i][j].mul(O::n(3, 0));
            }
        }
        pull[4][4] = O::n(57, 0);
        assert_eq!(det(&pull), O::n(3i64.pow(5) * 361, 0));
        records.push(json!({"graph":g["graph"],"target_trace":trace,"H":h,"rotation":r,"Picard_Frobenius_mod2":pic,"actual_theta_drift_bits":drift,"actual_theta_correction_bits":corrections[drift],"all16_drift_corrections":corrections,"Cartier_character_Frobenius":c,"I_minus_C":ic,"I_minus_C_inverse":inverse,"all361_character_drift_corrections":character_corrections,"character_fixed_point_checks":361*361,"pullback_3lambda_matrix":pull,"global_sections_constructed":false,"projective_equations_constructed":false}));
    }
    assert!(inv2([[0, 0], [0, 0]]).is_none());
    let singular_counts = [[0, 0], [1, 0]].map(|d| {
        (0..19)
            .flat_map(|x| (0..19).map(move |y| [x, y]))
            .filter(|v| [(v[0] + d[0]) % 19, (v[1] + d[1]) % 19] == *v)
            .count()
    });
    assert_eq!(singular_counts, [361, 0]);
    let controls = bias_controls();
    round02::save(
        dir,
        "g15-theta-class-descent.json",
        &json!({"schema":"koblitz.round15/g15-v1","records":records,"CM_divisor_bias_controls":controls,"singular_character_control":{"C":"identity","zero_drift_fixed_points":singular_counts[0],"nonzero_drift_fixed_points":singular_counts[1]},"dimension":5,"sections_3lambda":3usize.pow(5),"sections_pullback_3lambda":361*3usize.pow(5),"ambient_projective_dimension":242,"degree_3lambda":120*3usize.pow(5),"canonical6_to3_section_ratio":32,"bridge":"explicit symmetric split line-bundle class,2-torsion/Frobenius correction,theta-group19-descent,ample RR and third-power very-ampleness;written scheme proof","actual_section_basis":false,"full_h_saturation":false,"IC_efficiency_verdict":null}),
    );
}
