use super::*;
#[path="../feature-native-adaptive-extension/geometry.rs"]mod geometry;
#[path="../feature-native-division/lib.rs"]mod jets;
fn add(p:u64,a:u64,x:Point,y:Point)->Point{let(Some([x1,y1]),Some([x2,y2]))=(x,y)else{return x.or(y)};if x1==x2&&(y1+y2)%p==0{return None;}let(n,d)=if x1==x2{((3*mul(x1,x1,p)+a)%p,2*y1%p)}else{((y2+p-y1)%p,(x2+p-x1)%p)};let m=mul(n,pow(d,p-2,p),p);let xx=(mul(m,m,p)+2*p-x1-x2)%p;Some([xx,(mul(m,(x1+p-xx)%p,p)+p-y1)%p])}
fn point(p:u64,a:u64,base:[u64;2],n:usize)->Point{let mut q=None;for _ in 0..n{q=add(p,a,q,Some(base));}q}
fn chi(k:u64,r:u64)->i8{if k==0{0}else if pow(k,(r-1)/2,r)==1{1}else{-1}}
#[test]fn formula_matches_iterative_exponent_vectors(){
 let mut count=0;for r in (7..=1023).step_by(2){let[w,k,s,e]=partition(r).unwrap();
 let plus=|a:[i128;4],b:[i128;4]|std::array::from_fn(|i|a[i]+b[i]);let scale=|a:[i128;4],k:i128|a.map(|x|x*k);
 let update=|cur:[i128;4],old:[i128;4],square:[i128;4]|std::array::from_fn(|i|2*cur[i]-old[i]+square[i]);
 let u=[0,1,0,0];let(mut old,mut cur)=(u,u);let(mut bp,mut ap,mut prefix)=([0;4],[0;4],[0;4]);
 for j in 0..w{bp=plus(bp,cur);if j<s{prefix=plus(prefix,cur);}let next=update(cur,old,[2,0,0,0]);old=cur;cur=next;}
 for i in 0..k{ap=plus(ap,cur);if i+1<k{let next=update(cur,old,[0,0,2,0]);old=cur;cur=next;}}
 let total=plus(plus(scale(ap,2*w as i128),scale(bp,2*k as i128)),plus(scale(prefix,2),[0,e as i128,0,2*s as i128]));
 for p in [5,7,509,65537,2305843009213693951]{let expected=total.map(|v|v.rem_euclid((p-1)as i128)as u64);assert_eq!(exponents(p,r),Ok(expected),"p={p} r={r}");count+=1;}
 }assert_eq!(count,2545);println!("symbolic_exponent_cases={count}");
 assert!(exponents(3,479).is_err());assert!(exponents(u64::MAX,479).is_err());assert!(exponents(509,6).is_err());assert!(exponents(509,u64::MAX).is_ok());
}
#[test]fn arbitrary_scalar_inputs_and_homogeneous_scales(){
 let fixtures=[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])];let mut count=0;
 for(p,a,b,r,base)in fixtures{let part=partition(r).unwrap();let[w,k,s,e]=part;let curve=geometry::Curve::new(p,a,b).unwrap();let powers=exponents(p,r).unwrap();
 for scalar in [1,2,3,(r-1)as usize]{let q=point(p,a,base,scalar).unwrap();let grid=curve.grid(r,q,w,k,s,e).unwrap();let unit=residual::geometry_ratio(p,part,q,&grid.affine).unwrap();
 for n in 1..=8{let inputs=[1+(n%(p-1)),1+((3*n+1)%(p-1)),1+((5*n+2)%(p-1)),1+((7*n+3)%(p-1))];let expected=residual::reference_ratio(p,part,q,&grid.affine,inputs).unwrap();
 for scale in 1..=3{let pairs=std::array::from_fn(|i|{let z=1+((scale+i as u64)%(p-1));[mul(inputs[i],z,p),z]});let got=apply(p,unit,pairs,powers).unwrap();assert_eq!(mul(expected[0],got[1],p),mul(expected[1],got[0],p),"p={p} scalar={scalar} inputs={inputs:?}");count+=1;}
 }
 }}assert_eq!(count,288);println!("arbitrary_input_scaled_ratios={count}");
 for i in 0..4{for j in 0..2{let mut bad=[[1,1];4];bad[i][j]=0;assert_eq!(apply(509,[1,1],bad,[0;4]),Err(Error::Degenerate));bad[i][j]=509;assert_eq!(apply(509,[1,1],bad,[1;4]),Err(Error::Degenerate));}}
 assert_eq!(residual::geometry_ratio(509,partition(479).unwrap(),[3,68],&[]),Err(Error::Geometry));
}
#[test]fn complete_public_point_characters_and_raw_ratios(){
 let mut count=0;let mut rows=String::from("p\tr\tscalar\tx\ty\tsign\tnumerator\tdenominator\n");
 for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{let source=Source::new(p,a,b,r,base,128,500).unwrap();let prepared=source.prepare().unwrap();let before=transport::calls();let out=prepared.window(None,r as usize).unwrap();assert_eq!(transport::calls(),before);assert_eq!(out.direct_initializations,0);let part=partition(r).unwrap();let[w,k,s,e]=part;let curve=geometry::Curve::new(p,a,b).unwrap();let mut q=None;
 for scalar in 0..r{let i=scalar as usize;assert_eq!(out.points[i],q);assert_eq!(out.signs[i],chi(scalar,r));if let Some(pt)=q{let mut d=jets::Division::new(p,a,b,pt[0],pt[1]).unwrap();let c=d.phase(r).unwrap();let mut norm=|n:u64|mul(d.value(n),pow(c,((n as u128*n as u128)%(p-1)as u128)as u64,p),p);let inputs=[c,norm(r/2+1),norm(2*w),if s==0{1}else{norm(s+e)}];let grid=curve.grid(r,pt,w,k,s,e).unwrap();let expected=residual::reference_ratio(p,part,pt,&grid.affine,inputs).unwrap();let got=out.ratios[i];assert_eq!(mul(expected[0],got[1],p),mul(expected[1],got[0],p));}
 let(x,y)=q.map(|[x,y]|(x as i64,y as i64)).unwrap_or((-1,-1));rows.push_str(&format!("{p}\t{r}\t{scalar}\t{x}\t{y}\t{}\t{}\t{}\n",out.signs[i],out.ratios[i][0],out.ratios[i][1]));count+=1;q=add(p,a,q,Some(base));}assert_eq!(q,None);
 }assert_eq!(count,499);println!("complete_public_characters_and_ratios={count}");if let Ok(dir)=std::env::var("MULTISEED_OUT"){std::fs::write(format!("{dir}/characters.tsv"),rows).unwrap();}
}
#[test]fn shifted_windows_share_phase_value_and_geometry_work(){let s=Source::new(509,2,10,479,[3,68],128,16).unwrap();let ready=s.prepare().unwrap();let mut checked=0;for start in [0,1,2,3,25,173,292,440,476,477,478]{let q=point(509,2,[3,68],start);let before=transport::calls();let out=ready.window(q,16).unwrap();let after=transport::calls();assert_eq!(after.0-before.0,out.direct_initializations);assert_eq!(after.1-before.1,out.direct_initializations);assert!(out.direct_initializations<=2);assert_eq!(out.signs.len(),16);for i in 0..16{assert_eq!(out.signs[i],chi((start+i)as u64%479,479));checked+=1;}}
 assert_eq!(checked,176);println!("shifted_public_characters={checked}");assert!(ready.window(None,0).is_err());assert!(ready.window(None,17).is_err());assert!(ready.window(Some([0,0]),4).is_err());assert!(Source::new(509,2,10,479,[3,68],1,16).is_err());
}
