std::thread_local!{static ORBIT_POWERS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
fn character_residue(p:&mut polynomial::Poly,f:&[u64],a:u64,r:u64)->Result<Vec<u64>,Error>{
 let mut base=p.divrem(&[a,1],f).map_err(|_|Error::Arithmetic)?.1;let(mut n,mut out)=((r-1)/2,vec![1]);
 while n>0{if n&1!=0{let product=p.mul(&out,&base);out=p.divrem(&product,f).map_err(|_|Error::Arithmetic)?.1}n>>=1;if n>0{let product=p.mul(&base,&base);base=p.divrem(&product,f).map_err(|_|Error::Arithmetic)?.1}}
 Ok(out)
}
fn rotate_residue(residue:&[u64],z:u64,r:u64)->Vec<u64>{let phase=pow(z,(r-1)/2,r);let inv=pow(z,r-2,r);let mut scale=phase;residue.iter().map(|&c|{let v=mul(c,scale,r);scale=mul(scale,inv,r);v}).collect()}
fn orbit_history(p:&mut polynomial::Poly,initial:&[u64],history:&[(u64,u64)],m:usize,zeta:u64,r:u64,cap:usize)->Result<(Vec<u64>,usize),Error>{
 if initial.len()<2||initial.len()-1>cap||initial.last()!=Some(&1)||initial.iter().any(|&c|c>=r)||m<2||m as u64>=r||(r-1)%m as u64!=0||zeta==0||zeta>=r||history.is_empty()||history[0].0!=0||(history.len()-1)%m!=0||history.iter().any(|&(a,s)|a>=r||s!=0&&s!=1&&s!=r-1){return Err(Error::Domain)}
 let mut z=1;for j in 1..=m{z=mul(z,zeta,r);if z==1&&j!=m{return Err(Error::Domain)}}if z!=1{return Err(Error::Domain)}
 // Check stability of the ideal before transporting any residue.
 let expected=pow(zeta,(initial.len()-1)as u64,r);let mut scale=1;
 for &c in initial{if mul(c,scale,r)!=mul(c,expected,r){return Err(Error::Domain)}scale=mul(scale,zeta,r)}
 for chunk in history[1..].chunks(m){if chunk[0].0==0{return Err(Error::Domain)}let mut a=chunk[0].0;for &(offset,_)in chunk{if offset!=a{return Err(Error::Domain)}a=mul(a,zeta,r)}}
 let mut state=replay(p,initial,0,history[0].1,r,cap)?;let mut powers=1;
 for chunk in history[1..].chunks(m){let residue=character_residue(p,initial,chunk[0].0,r)?;powers+=1;let mut z=1;
  for &(_,sign)in chunk{let mut c=rotate_residue(&residue,z,r);c[0]=(c[0]+r-sign)%r;state=p.gcd_monic(&state,&c,cap).map_err(|_|Error::Arithmetic)?.polynomial;z=mul(z,zeta,r)}
 }Ok((state,powers))
}
include!("adapter.rs");
fn orbit_exact(){let(mut residues,mut prefixes,mut phase_rejects,mut stable_zero)=(0,0,0,0);
 for r in [7,11,13,17,19,31]{for m in 2..r as usize{if(r-1)%m as u64!=0{continue}let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let mut images=Vec::new();for x in 0..r{let v=pow(x,m as u64,r);if !images.contains(&v){images.push(v)}}if images.len()>8{continue}let mut p=polynomial::Poly::new(r).unwrap();
  for mask in 1..1usize<<images.len(){let selected:Vec<_>=images.iter().enumerate().filter(|&(j,_)|mask&(1<<j)!=0).map(|(_,&v)|v).collect();let f=p.product_tree(&selected).unwrap().polynomial().to_vec();let lifted=pullback(&f,m,r,r as usize).unwrap();if selected.contains(&0){stable_zero+=1}let target=(0..r).find(|&x|eval(&lifted,x,r)==0).unwrap();
   let mut history=vec![(0,pow(target,(r-1)/2,r))];let mut direct=replay(&mut p,&lifted,0,history[0].1,r,r as usize).unwrap();
   for a in [1,2]{let residue=character_residue(&mut p,&lifted,a,r).unwrap();let mut z=1;
    for _ in 0..m{let offset=mul(a,z,r);let rotated=rotate_residue(&residue,z,r);assert_eq!(rotated,character_residue(&mut p,&lifted,offset,r).unwrap());residues+=1;
     let phase=pow(z,(r-1)/2,r);if phase!=1&&rotated.iter().any(|&c|c!=0){phase_rejects+=1}
     let sign=pow((target+offset)%r,(r-1)/2,r);history.push((offset,sign));let mut c=rotated;c[0]=(c[0]+r-sign)%r;let next=p.gcd_monic(&direct,&c,r as usize).unwrap().polynomial;let reference=replay(&mut p,&direct,offset,sign,r,r as usize).unwrap();assert_eq!(next,reference);direct=next;prefixes+=1;z=mul(z,zeta,r);
    }
   }let(combined,powers)=orbit_history(&mut p,&lifted,&history,m,zeta,r,r as usize).unwrap();assert_eq!(combined,direct);assert_eq!(powers,3);
  }
 }}let mut p=polynomial::Poly::new(13).unwrap();assert_eq!(orbit_history(&mut p,&[1,1],&[(0,1),(1,1),(3,1),(9,1)],3,3,13,4).err(),Some(Error::Domain));
 println!("orbit_residue_equalities={residues} replay_prefix_equalities={prefixes} odd_character_phase_controls={phase_rejects} zero_image_subsets={stable_zero}");}
fn main(){pullback_checks();orbit_exact();let(mut cases,mut calls,mut powers,mut retained)=(0,0,0,0);
 for(p,a,b,r,base,m)in[(7,0,3,13,[1,2],3),(7,0,3,13,[1,2],6),(509,2,10,479,[3,68],2),(701,2,8,683,[2,106],11)]{let c=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=Modes::compile(r,m,z,16,256).unwrap();let scalars:Vec<u64>=if r==13{(0..r).collect()}else if r==479{include_str!("../feature-native-monomial-recovery/recovery.tsv").lines().skip(1).map(|l|l.split('\t').next().unwrap().parse().unwrap()).collect()}else{vec![0,1,2,3,7,11,17,31,47,83,127,191,257,341,509,681,682]};
  for k in scalars{let t=multiple(c,bp,k);let extra:Vec<_>=(0..17.min(r)).collect();let(old,os,route)=modes.recover_pulled(&source,c,bp,t,&[1,2,3,4],64,&extra,128).unwrap();ORBIT_POWERS.with(|n|n.set(0));let(new,ns,nroute)=modes.recover_orbit(&source,c,bp,t,&[1,2,3,4],64,&extra,128).unwrap();assert_eq!(new,old);assert_eq!(new,Some(k));assert_eq!(route,nroute);assert_eq!((os.calls,os.replayed,os.extra,os.lifted,os.image_count),(ns.calls,ns.replayed,ns.extra,ns.lifted,ns.image_count));cases+=1;calls+=ns.calls;retained+=ns.replayed;powers+=ORBIT_POWERS.with(|n|n.get());}
 }println!("public_equalities={cases} source_calls_each_adapter={calls} retained_constraints={retained} candidate_retained_residue_powers={powers}");}
