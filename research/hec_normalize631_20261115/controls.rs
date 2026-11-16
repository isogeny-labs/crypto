fn validate_functions(c:&Curve,refc:&jac::Curve,ks:&[Vec<Div>;2],hs:&[Vec<H>;2]){
 let mut principal=0;let mut corrupt=0;let mut wrong_half=0;
 for k in 0..2{for(i,d)in ks[k].iter().enumerate(){if c.is_zero(d){continue;}assert!(valid_h(c,d,&hs[k][i]));principal+=1;if lift_with_half(c,d,64).is_none(){wrong_half+=1;}let mut bad=hs[k][i].clone();bad.a=bad.a.add(&Pol::one(),&c.field);if !valid_h(c,d,&bad){corrupt+=1;}}}
 assert_eq!(principal,52);assert!(corrupt>0&&wrong_half>0);
 let fb=hec_native_ic::instance::factor_base(refc);let mut comparisons=0;let mut excluded=0;
 for offset in 0..8{let pp=&fb[offset..offset+3];let mut d=refc.zero();let pts:Vec<_>=pp.iter().map(|&(x,y)|(base(x),base(y))).collect();for &(x,y)in pp{d=refc.add(&d,&refc.place(x,y));}let x=Div{u:Pol::embed(&d.u),v:Pol::embed(&d.v)};assert!(c.valid(&x));
 for k in 0..2{for i in 0..27{let a=evaluate(c,&ks[k][i],&hs[k][i],&x);let b=direct(c,&ks[k][i],&hs[k][i],&pts);match(a,b){(Some(a),Some(b))=>{assert_eq!(a,b,"independent_split_evaluation");comparisons+=1;},_=>{excluded+=1;println!("SPLIT_CONTROL_EXCLUDE offset={offset} kernel={k} point={i}");}}}}
 }
 assert!(comparisons>=200);
 println!("FUNCTION_CONTROL principal_identities={principal} corrupted_functions_rejected={corrupt} stale_half64_failures={wrong_half} independent_split_evaluations={comparisons} exclusions={excluded} inverse_two={}",(P+1)/2);
}
