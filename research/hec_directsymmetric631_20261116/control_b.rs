include!("../hec_inverse631_20261116/verify_poly.rs");
#[allow(dead_code)]mod exact{include!("../hec_recover631_20261116/verify.rs");include!("../hec_recover631_20261116/exact_api.rs");}
#[allow(dead_code)]mod direct{include!("direct_check.rs");}
include!("../hec_symmetric631_20261116/symmetric.rs");
fn main(){let(e,x)=exact::checked("../hec_inverse631_20261116/generated-B-alt/target.ms","../hec_inverse631_20261116/solve-B-alt.res");let uv=ucoeff(&x,&e).unwrap();direct::check("generated-B/target.ms",uv,e);}
