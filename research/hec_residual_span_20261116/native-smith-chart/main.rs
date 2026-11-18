use hec_native_ic::{counted,fp::Fp,poly::Poly,instance::pick_instance,oracle::{split,OracleStats}};
fn mul(a:&Poly,b:&Poly,f:&Fp)->Poly{a.mul(b,f)}
fn square(a:&Poly,f:&Fp)->Poly{a.mul(a,f)}
fn scale(a:&Poly,k:i64,f:&Fp)->Poly{a.scale(f.from_i64(k),f)}
fn det(m:[[Poly;3];3],f:&Fp)->Poly{let mut out=Poly::zero();for p in [[0,1,2],[1,2,0],[2,0,1]]{out=out.add(&m[0][p[0]].mul(&m[1][p[1]],f).mul(&m[2][p[2]],f),f);}for p in [[0,2,1],[2,1,0],[1,0,2]]{out=out.sub(&m[0][p[0]].mul(&m[1][p[1]],f).mul(&m[2][p[2]],f),f);}out}
fn remainder(source:&Poly,g:&[Poly;3],f:&Fp)->[Poly;3]{let mut a=[Poly::zero();3];for i in(0..source.n).rev(){a=[scale(&a[2].mul(&g[0],f),-1,f),a[0].sub(&a[2].mul(&g[1],f),f),a[1].sub(&a[2].mul(&g[2],f),f)];a[0]=a[0].add(&Poly::constant(source.c[i]),f);}a}
fn norm(a:&[Poly;3],g:&[Poly;3],f:&Fp)->Poly{let c0=*a;let c1=[scale(&a[2].mul(&g[0],f),-1,f),a[0].sub(&a[2].mul(&g[1],f),f),a[1].sub(&a[2].mul(&g[2],f),f)];let c2=[scale(&c1[2].mul(&g[0],f),-1,f),c1[0].sub(&c1[2].mul(&g[1],f),f),c1[1].sub(&c1[2].mul(&g[2],f),f)];det([[c0[0],c1[0],c2[0]],[c0[1],c1[1],c2[1]],[c0[2],c1[2],c2[2]]],f)}
fn sqrt_monic(s:&Poly,f:&Fp)->Poly{assert!(!s.is_zero());assert_eq!(s.deg()%2,0);let n=s.deg()as usize/2;let mut h=Poly::zero();h.n=n+1;h.c[n]=1;let inv2=f.inv(2);for i in(0..n).rev(){let known=h.mul(&h,f).c[n+i];h.c[i]=f.mul(f.sub(s.c[n+i],known),inv2);}assert_eq!(h.mul(&h,f),*s);h}
fn key<'a>(l:&'a str,k:&str)->Option<&'a str>{l.split_whitespace().find_map(|s|s.strip_prefix(&format!("{k}=")))}
fn array(l:&str,k:&str)->Poly{let text=l.split(&format!("{k}=[")).nth(1).unwrap().split(']').next().unwrap();Poly::from_coeffs(&text.split(',').map(|s|s.trim().parse().unwrap()).collect::<Vec<u64>>())}
fn coeff(p:&Poly)->Vec<u64>{p.c[..p.n].to_vec()}
fn main(){counted::disable();let raw=include_str!("../native-smith-affine/CONTROL.txt");let mut maps=0;for (line_index,l) in raw.lines().enumerate().filter(|(_,l)|l.starts_with("MAP p=61 ")){
let p=61;let inst=pick_instance(p,3).unwrap();let f=&inst.curve.fp;let n=array(l,"normalized_n");let d=array(l,"normalized_d");let norm_s=array(l,"norm_s");let root=array(l,"monic_square_root").scale(f.sqrt(norm_s.lc()).unwrap(),f);assert_eq!(root.mul(&root,f),norm_s);
let delta_line=raw.lines().nth(line_index+1).unwrap();assert!(delta_line.starts_with("AFFINE p=61 "));let delta0=array(delta_line,"delta0");let delta2=array(delta_line,"delta2");let delta4=array(delta_line,"delta4");let g=[Poly::from_coeffs(&[n.c[0],f.neg(d.c[0])]),Poly::from_coeffs(&[n.c[1],f.neg(d.c[1])]),Poly::from_coeffs(&[0,f.neg(1)])];let a=remainder(&inst.curve.f,&g,f);
let mut excluded=0;let mut singular_projection=0;let mut eligible=0;let mut points=0;let mut nonsplit_points=0;let mut split_points=0;let mut incidences=0;let mut image=std::collections::BTreeSet::new();
for t in 0..p{let dv=[delta0.eval(t,f),f.mul(8,root.eval(t,f)),delta2.eval(t,f),delta4.eval(t,f)];if dv.iter().any(|&x|x==0){excluded+=1;continue;}
let q=Poly::from_coeffs(&[dv[0],f.neg(dv[1]),dv[2],0,dv[3]]);if Poly::gcd(&q,&q.deriv(f),f).deg()>0{singular_projection+=1;continue;}eligible+=1;
let gt=n.sub(&d.scale(t,f),f);assert_eq!(gt.deg(),3);assert_ne!(gt.disc_monic(f),0);let av=[a[0].eval(t,f),a[1].eval(t,f),a[2].eval(t,f)];let gv=[g[0].eval(t,f),g[1].eval(t,f),g[2].eval(t,f)];let is_split=split(&gt,f,&mut OracleStats::default()).is_some();let mut at_t=0;
for rho in 1..p{let is_root=q.eval(rho,f)==0;let mut solutions=Vec::new();let rr=f.mul(rho,rho);let inv2rho=f.inv(f.mul(2,rho));for sigma in 0..p{
// The x^2 equation uniquely determines tau when rho != 0.
let tau=f.mul(f.sub(f.add(f.sub(av[2],f.mul(sigma,sigma)),f.mul(f.mul(2,gv[2]),f.mul(sigma,rho))),f.mul(f.sub(f.mul(gv[2],gv[2]),gv[1]),rr)),inv2rho);
let bp=Poly::from_coeffs(&[tau,sigma,rho]);if bp.mulmod(&bp,&gt,f)!=Poly::from_coeffs(&av){continue;}
let top=f.add(f.add(f.mul(dv[3],f.mul(rr,rr)),f.mul(dv[2],rr)),dv[0]);if top!=f.mul(dv[1],rho){continue;}solutions.push(bp);
}
assert_eq!(solutions.len(),if is_root{1}else{0},"chart bijection t={t} rho={rho}");if !is_root{continue;}let bp=solutions[0];let z=[f.mul(bp.c[0],bp.c[0]),f.mul(bp.c[0],bp.c[1]),f.mul(bp.c[0],rho),f.mul(bp.c[1],bp.c[1]),f.mul(bp.c[1],rho),rr];assert!(image.insert((t,z)));let inverse=Poly::from_coeffs(&[f.mul(z[2],f.inv(rho)),f.mul(z[4],f.inv(rho)),f.mul(z[5],f.inv(rho))]);assert_eq!(inverse,bp);assert_ne!(bp, bp.add(&Poly::constant(1),f));assert_ne!(bp.add(&Poly::constant(1),f).mulmod(&bp.add(&Poly::constant(1),f),&gt,f),Poly::from_coeffs(&av));
for x in 0..p{if gt.eval(x,f)==0{let y=bp.eval(x,f);assert_eq!(f.mul(y,y),inst.curve.f.eval(x,f));assert_eq!(n.eval(x,f),f.mul(t,d.eval(x,f)));incidences+=1;}}
println!("POINT map={maps} t={t} rho={rho} b={:?} trigonal_split={is_split}",coeff(&bp));at_t+=1;points+=1;if is_split{split_points+=1}else{nonsplit_points+=1}
}assert!(at_t<=4);
}
assert_eq!(eligible+excluded+singular_projection,p);assert_eq!(points,split_points+nonsplit_points);assert_eq!(image.len(),points);assert!(points>0);println!("MAP map={maps} p={p} eligible_t={eligible} excluded_delta_t={excluded} excluded_singular_projection_t={singular_projection} quotient_points={points} split_points={split_points} nonsplit_points={nonsplit_points} rational_source_incidences={incidences} n={:?} d={:?}",coeff(&n),coeff(&d));maps+=1;
}assert_eq!(maps,2);eprintln!("PASS maps=2 all_eligible_rational_rho_and_sigma=true exact_chart_bijection=true native_incidence_checks=true no_global_normalization_or_target_transport_claim=true");}
