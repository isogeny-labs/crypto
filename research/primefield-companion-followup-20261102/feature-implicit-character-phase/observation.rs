fn gcd(mut a:usize,mut b:usize)->usize{while b!=0{let t=a%b;a=b;b=t;}a}
fn bracelets(m:usize)->u128{assert!((3..=60).contains(&m));let rotations:u128=(0..m).map(|j|1u128<<gcd(m,j)).sum();let reflections=if m%2==1{m as u128*(1u128<<((m+1)/2))}else{(m/2)as u128*((1u128<<(m/2+1))+(1u128<<(m/2)))};(rotations+reflections)/(2*m)as u128}
fn transform_word(word:u64,m:usize,a:usize,reverse:bool)->u64{let mut out=0;for j in 0..m{let at=if reverse{(a+m-j)%m}else{(a+j)%m};out|=((word>>at)&1)<<j;}out}
fn summary(signs:&[u64],zeta:u64,r:u64)->(u64,u64){let(mut f0,mut fp,mut fm,mut z)=(0,0,0,1);for &s in signs{f0=(f0+s)%r;fp=(fp+mul(z,s,r))%r;fm=(fm+mul(pow(z,r-2,r),s,r))%r;z=mul(z,zeta,r);}(f0,mul(fp,fm,r))}
fn orbit(k:u64,b:u64,m:usize,zeta:u64,r:u64)->Vec<u64>{let mut z=1;(0..m).map(|_|{let v=pow((k+r-mul(b,z,r))%r,(r-1)/2,r);z=mul(z,zeta,r);v}).collect()}
fn primitive(r:u64,m:usize)->u64{(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap()}
fn observation_checks(){let(mut words,mut invariant_checks,mut formulas)=(0,0,0);
 for m in 3..=12{let r=[7u64,13,17,19,23,29,31,37,41,61,73,97].into_iter().find(|&r|((r-1)/2)%m as u64==0).unwrap();let zeta=primitive(r,m);let mut representatives=BTreeSet::new();
  for word in 0..(1u64<<m){let signs:Vec<_>=(0..m).map(|j|if word>>j&1==0{1}else{r-1}).collect();let s=summary(&signs,zeta,r);let mut canonical=word;
   for reverse in [false,true]{for a in 0..m{let v=transform_word(word,m,a,reverse);canonical=canonical.min(v);let other:Vec<_>=(0..m).map(|j|if v>>j&1==0{1}else{r-1}).collect();assert_eq!(s,summary(&other,zeta,r));invariant_checks+=1;}}
   representatives.insert(canonical);words+=1;
  }assert_eq!(representatives.len()as u128,bracelets(m));formulas+=1;
 }
 let mut fibers=0;
 for(r,m)in[(19u64,3usize),(31,5),(37,6),(97,3),(97,8),(281,5)]{let zeta=primitive(r,m);let l=(r-1)/2/m as u64;
  for b in [1,2,3]{for sign in [1,r-1]{let epsilon=mul(sign,pow(b,(r-1)/2,r),r);let mut groups:BTreeMap<(u64,u64),(BTreeSet<u64>,BTreeSet<u64>)>=BTreeMap::new();let mut all_v=BTreeSet::new();let mut all_w=BTreeSet::new();
   for k in 1..r{if pow(k,(r-1)/2,r)!=sign{continue}let signs=orbit(k,b,m,zeta,r);if signs.contains(&0){continue}let s=summary(&signs,zeta,r);let v=pow(mul(k,pow(b,r-2,r),r),m as u64,r);let w=(v+pow(v,r-2,r))%r;let g=groups.entry(s).or_default();g.0.insert(v);g.1.insert(w);all_v.insert(v);all_w.insert(w);}
   let delta=u64::from(epsilon==1);let endpoints=u64::from(epsilon==1)+u64::from((if l%2==0{1}else{r-1})==epsilon);let nw=(l+endpoints)/2-delta;
   assert_eq!(all_v.len()as u64,l-delta);assert_eq!(all_w.len()as u64,nw);let count=bracelets(m)as u64;assert!(groups.len()as u64<=count);
   let maxv=groups.values().map(|g|g.0.len()).max().unwrap_or(0)as u64;let maxw=groups.values().map(|g|g.1.len()).max().unwrap_or(0)as u64;
   assert!(maxv>=(l-delta+count-1)/count);assert!(maxw>=(nw+count-1)/count);fibers+=1;
   println!("fiber r={r} m={m} b={b} sign={sign} epsilon={epsilon} images={} W_values={nw} summaries={} bracelet_bound={count} max_V_fiber={maxv} max_W_fiber={maxw}",l-delta,groups.len());
  }}
 }
 println!("binary_words={words} dihedral_invariance_equalities={invariant_checks} bracelet_formula_equalities={formulas} observation_fiber_controls={fibers} B3={} B5={} B35={}",bracelets(3),bracelets(5),bracelets(35));
}
