include!("../hec_inverse631_20261116/verify_poly.rs");
#[allow(dead_code)]mod exact{include!("../hec_recover631_20261116/verify.rs");include!("../hec_recover631_20261116/exact_api.rs");}
#[allow(dead_code)]mod direct{include!("direct_check.rs");}
include!("../hec_symmetric631_20261116/symmetric.rs");
fn main(){let(e,x)=exact::checked("../hec_inverse631_20261116/generated-D1/target.ms","../hec_inverse631_20261116/solve-D1.res");let uv=ucoeff(&x,&e).unwrap();direct::check("generated-D1/target.ms",uv,e);}
