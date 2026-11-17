//! Evaluated integral diamond-2 graph correspondence on the level77 incidence chart.
use super::*;
use serde_json::Value;
fn diamond(f: Field, v: [u32; 4]) -> ([u32; 4], Value) {
    let [b, c, l, n] = v;
    let a = 1 ^ c;
    let k = b ^ f.mul(a, c);
    assert_ne!(c, 0);
    assert_ne!(k, 0);
    let s = f.mul(k, f.inv(c));
    let u = f.mul(f.pow(c, 3), f.inv(k));
    let ui = f.inv(u);
    let bp = f.mul(f.mul(b, c), f.pow(ui, 3));
    let ap = f.mul(a, ui);
    assert_eq!(f.mul(u, f.sq(s) ^ f.mul(a, s)), f.mul(b, c));
    let lp = f.mul(l ^ s, ui);
    let np = f.mul(n ^ f.mul(l, b) ^ f.mul(b, c), f.pow(ui, 3));
    (
        [bp, 1 ^ ap, lp, np],
        json!({"u":u,"s":s,"K":k,"r":b,"t":f.mul(b,c)}),
    )
}
pub fn run(dir: &Path) {
    let old = crate::round04::read(&dir.join("cm17-explicit-level77-incidence.json"));
    let row = &old["finite_field_records"][4];
    let f = Field {
        m: 5,
        modulus: row["modulus"].as_u64().unwrap(),
    };
    assert!(f.irreducible());
    let pts = row["untwisted_incidence_points"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| std::array::from_fn(|i| p[i].as_u64().unwrap() as u32))
        .collect::<Vec<[u32; 4]>>();
    let (circuit, _, _) = circuit(false);
    let mut records = vec![];
    let mut identity_checks = 0;
    let mut point_checks = 0;
    for input in &pts {
        let mut v = *input;
        let mut orbit = vec![];
        for j in 1..=10 {
            let (w, change) = diamond(f, v);
            assert!(eval(&circuit, f, w).is_some());
            assert!(pts.contains(&w));
            orbit.push(json!({"power":j,"image":w,"change":change}));
            v = w;
            if j == 5 {
                assert_eq!(
                    v,
                    [
                        input[0],
                        input[1],
                        input[2] ^ 1 ^ input[1],
                        input[3] ^ input[0]
                    ]
                );
            }
        }
        assert_eq!(v, *input);
        let (w, ch) = diamond(f, *input);
        let b = input[0];
        let cc = input[1];
        let a = 1 ^ cc;
        let u = ch["u"].as_u64().unwrap() as u32;
        let s = ch["s"].as_u64().unwrap() as u32;
        let ce = Curve {
            f,
            b,
            c: cc,
            twist: false,
        };
        let cd = Curve {
            f,
            b: w[0],
            c: w[1],
            twist: false,
        };
        let g = |x: u32, y: u32| {
            f.sq(y) ^ f.mul(a, f.mul(x, y)) ^ f.mul(b, y) ^ f.mul(x, f.sq(x)) ^ f.mul(b, f.sq(x))
        };
        for x in 0..32 {
            for y in 0..32 {
                let xx = f.mul(f.sq(u), x) ^ b;
                let yy = f.mul(f.pow(u, 3), y) ^ f.mul(s, f.mul(f.sq(u), x)) ^ f.mul(b, cc);
                let dest = f.sq(y)
                    ^ f.mul(cd.a(), f.mul(x, y))
                    ^ f.mul(cd.b, y)
                    ^ f.mul(x, f.sq(x))
                    ^ f.mul(cd.b, f.sq(x));
                assert_eq!(g(xx, yy), f.mul(f.pow(u, 6), dest));
                identity_checks += 1;
                if cd.valid(Pt::A(x, y)) {
                    assert!(ce.valid(Pt::A(xx, yy)));
                    point_checks += 1;
                }
            }
        }
        let two = ce.times(Pt::A(0, 0), 2);
        assert_eq!(two, Pt::A(b, f.mul(b, cc)));
        assert_eq!(ce.times(two, 11), Pt::O);
        records.push(json!({"input":input,"orbit":orbit,"fifth_is_diamond_involution":true,"tenth_is_identity":true}));
    }
    println!("CM19:diamond2 evaluated on10 actual F32 incidence points;fifth involution/tenth identity;{identity_checks} coordinate residual and{point_checks} curve-point controls");
    crate::round02::save(
        dir,
        "cm19-explicit-diamond2-correspondence.json",
        &json!({"schema":"koblitz.round18/cm19-v1","field_degree":5,"modulus":f.modulus,"formula":{"A":"1+c","K":"b+A*c","u":"c^3/K","s":"K/c","b_prime":"b*c/u^3","c_prime":"1+A/u","lambda_prime":"(lambda+s)/u","nu_prime":"(nu+lambda*b+b*c)/u^3","original_coordinates":"x=u^2*X+b;y=u^3*Y+s*u^2*X+b*c"},"records":records,"coordinate_identity_controls":identity_checks,"curve_point_controls":point_checks,"integral_graph_correspondence":"degree-one diamond2 automorphism on smooth modular normalization under written moduli bridge","Koblitz_factor_inclusion_evaluated":false,"scope":"actual deck correspondence and universal Tate-coordinate residual;not an A5-to-J factor map"}),
    );
}
