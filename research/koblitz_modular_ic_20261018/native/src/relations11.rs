//! Exact ambient correspondences for the remaining coupled-divisor tracks.
use crate::{
    round02,
    round04::{Curve, Field, Pt},
};
use serde_json::json;
use std::path::Path;
pub fn run(dir: &Path) {
    let mut fields = vec![];
    let mut total = 0u64;
    for m in 2..=8 {
        let f = Field::find(m);
        let e = Curve { f, a: 1, b: 1 };
        let pts = e.points();
        let t = Pt::A(0, 1);
        let mut trans = 0;
        let mut pair = 0;
        let mut cubic = 0;
        let mut asctrl = 0;
        for p in &pts {
            if let Pt::A(x, y) = *p {
                if x != 0 {
                    let q = e.add(*p, t);
                    assert!(matches!(q,Pt::A(u,_) if f.mul(x,u)==1));
                    trans += 1;
                }
                for q in &pts {
                    if let Pt::A(u, v) = *q {
                        let d = x ^ u;
                        let l = f.sq(x) ^ f.mul(x, u) ^ f.sq(u) ^ x ^ u;
                        // Difference of the two curve equations with y=v factors as d*(y+L).
                        assert_eq!(
                            f.mul(d, y ^ l),
                            f.sq(y) ^ f.mul(u, y) ^ f.pow(u, 3) ^ f.sq(u) ^ 1
                        );
                        cubic += 1;
                        if y == v {
                            assert_eq!(f.mul(d, y ^ l), 0);
                        }
                        if x != 0 && u != 0 {
                            let lhs = f.mul(x, u) ^ 1;
                            assert_eq!(lhs == 0, u == f.inv(x));
                            let sum = e.add(*p, *q);
                            let diff = e.add(*p, e.neg(*q));
                            assert_eq!(lhs == 0, sum == t || diff == t);
                            pair += 1;
                        }
                        if u == x ^ 1 && x != 0 && u != 0 {
                            let z = f.mul(y ^ 1, f.inv(x));
                            let w = f.mul(v ^ 1, f.inv(u));
                            assert_eq!(f.sq(z) ^ z, x ^ 1 ^ f.inv(x));
                            assert_eq!(f.sq(w) ^ w, x ^ f.inv(u));
                            assert_eq!(f.sq(z ^ w) ^ (z ^ w), 1 ^ f.inv(f.mul(x, u)));
                            asctrl += 1;
                        }
                    }
                }
            }
        }
        total += trans + pair + cubic + asctrl;
        fields.push(json!({"m":m,"modulus":f.modulus,"points":pts.len(),"translation_checks":trans,"product_graph_checks":pair,"equal_y_factorization_checks":cubic,"AS_correspondence_checks":asctrl}));
    }
    round02::save(
        dir,
        "m17-residual-correspondences.json",
        &json!({"schema":"koblitz.round11/m17-v1","fields":fields,"exact_point_controls":total,"translated_product_Cartier_identity":"div(x0*xinf+1)=(phi0-phiinf)^*(T)+(phi0+phiinf)^*(T)-2phi0^*(O)-2phiinf^*(O)","translated_fiber_degrees":[1280,1360],"translated_fibers_evaluated":false,"xsum_plus_one_normalization":{"base":"x!=0,1","equations":["z²+z=x+1+1/x","w²+w=x+1/(x+1)"],"recovery":["y=x*z+1","u=x+1","v=(x+1)*w+1"],"third_subcover":"(z+w)²+(z+w)=1+1/(x*(x+1))","AS_pole_supports":[["0","infinity"],["1","infinity"],["0","1"]],"each_pole_order":1,"elliptic_subcover_genera":[1,1,1],"normalized_correspondence_genus":3,"interpretation":"ambient zero correspondence,not a constructed curve cover or complete finite divisor on C"},"ysum_zero_decomposition":{"diagonal":"u=x,v=y","residual":"v=y, y=x²+x*u+u²+x+u","factorization":"E(u,y)+E(x,y)=(x+u)*(y+x²+x*u+u²+x+u)","Cartier_identity":"div(y0+yinf)=D_diagonal+pullback(R_y)-3phi0^*(O)-3phiinf^*(O)","diagonal_total_degree":1280,"residual_effective_degree":2680,"noncuspidal_diagonal_part_degree":860,"unresolved_residual_support":true},"seven_noncuspidal_divisors_complete":false,"new_evaluated_columns":0,"rank_promoted":false}),
    );
    println!("M17:{total} exact controls,translated product fibers,genus3 xsum+1 correspondence and equal-y residual factorization");
}
