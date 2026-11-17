fn signed_image_checks(){
 let(mut products,mut negative,mut states,mut public,mut recovered,mut unresolved,mut updates)=(0,0,0,0,0,0,0);
 for(r,m)in[(17u64,4usize),(41,5),(97,3),(193,3),(281,5),(281,35)]{
  let zeta=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let i=(2..r).find(|&x|mul(x,x,r)==r-1).unwrap();let h=(r-1)/2;
  let stream=StreamModes::new(r,m,zeta,64,2048,7).unwrap();let modes=RecoveryModes::from_modes(&stream).unwrap();
  let mut dense=vec![1];for _ in 0..h{let mut next=vec![0;dense.len()+1];for(j,&c)in dense.iter().enumerate(){next[j]=(next[j]+r-c)%r;next[j+1]=(next[j+1]+c)%r;}dense=next;}
  let coeff:Vec<Vec<u64>>=[0,1,m-1].iter().map(|&e|dense.iter().skip(e).step_by(m).copied().collect()).collect();
  for scale in [1,2,3,r-1]{for g in [vec![0,1],vec![1,0,1],vec![4,r-4,1],vec![0,0,0,1]]{
   let mut ring=QuadRing::new(r,&g).unwrap();let actual=ring.signed_product_scaled(&modes,scale).unwrap();let inv=pow(scale,r-2,r);
   let bases=[Quad([vec![0],vec![scale]]),Quad([vec![0,inv],vec![r-inv]])];let mut values=[Quad([vec![0],vec![0]]),Quad([vec![0],vec![0]])];
   for j in 0..2{for &c in coeff[1].iter().rev(){values[j]=ring.multiply(&values[j],&bases[j]).unwrap();values[j].0[0][0]=(values[j].0[0][0]+c)%r;}}
   let expected=ring.multiply(&values[0],&values[1]).unwrap();for j in 0..2{assert_eq!(ring.rem(&actual.0[j]).unwrap(),ring.rem(&expected.0[j]).unwrap());}products+=1;
  }
   if scale!=1{for v in 1..r{let t=mul(scale,v,r);let true_product=mul(eval(&coeff[1],t,r),eval(&coeff[1],pow(t,r-2,r),r),r);let wrong_product=mul(eval(&coeff[1],t,r),eval(&coeff[1],mul(scale,pow(v,r-2,r),r),r),r);if true_product!=wrong_product{negative+=1;}}}
  }
  for k in 0..r{let mut image:Option<SignedImage>=None;let mut expected:Vec<_>=(0..r).map(|z|z==0||pow(z,(r-1)/m as u64,r)==1).collect();
   for b in [2,1,3,2]{let y=mul(k,pow(b,r-2,r),r);let mut z=1;let signs:Vec<_>=(0..m).map(|_|{let s=pow(mul(b,(y+r-z)%r,r),h,r);z=mul(z,zeta,r);s}).collect();let pair=modes.signed_observed(&signs).unwrap();let f=[signs.iter().fold(0,|a,&s|(a+s)%r),pair[0],pair[1]];
    if let Some(state)=&mut image{state.update(&modes,b,&f).unwrap();}else{image=Some(SignedImage::new(&modes,b,&f,i).unwrap());}
    let a=mul(m as u64,pow(b,h,r),r);let t=pow(pow(b,m as u64,r),r-2,r);
    for z in 0..r{let v=mul(t,z,r);let predicted0=mul(a,eval(&coeff[0],v,r),r);let predicted_product=mul(mul(mul(a,a,r),v,r),mul(eval(&coeff[1],v,r),eval(&coeff[2],v,r),r),r);
     expected[z as usize]&=predicted0==f[0]&&predicted_product==mul(f[1],f[2],r);
     if z==0{expected[0]&=f[1]==0&&f[2]==0;}
    }
    let roots:Vec<_>=(0..r).filter(|&z|expected[z as usize]).collect();let mut p=polynomial::Poly::new(r).unwrap();let oracle=if roots.is_empty(){vec![1]}else{p.product_tree(&roots).unwrap().polynomial().to_vec()};
    let actual=image.as_mut().unwrap().materialize(r as usize,r as usize).unwrap();assert_eq!(actual,oracle,"r={r} m={m} k={k} b={b}");assert!(expected[pow(k,m as u64,r)as usize]);states+=1;
   }
  }
 }
 assert!(negative>0);
 let(p,a,b,r,base,m)=(257,1,7,281,[1,3],5usize);let curve=group::Curve::new(p,a,b);let bp=group::Point{x:base[0],y:base[1]};let source=source::Source::new(p,a,b,r,base,64).unwrap();let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let modes=StreamModes::new(r,m,z,16,256,7).unwrap();let extra:Vec<_>=(0..17).collect();
 for cap in [8usize,16,64]{SIGNED_IMAGE_COUNTS.with(|c|c.set((0,0,0,0)));for k in 0..r{let target=multiple(curve,bp,k);let out=modes.recover_signed_image(&source,curve,bp,target,&[1,2,3,4],cap,&extra,128).unwrap();assert!(out.1.calls<=128);if let Some(v)=out.0{assert_eq!(v,k);recovered+=1}else{unresolved+=1;}public+=1;}updates+=SIGNED_IMAGE_COUNTS.with(|c|c.get().1);}
 assert!(updates>0);let(norm_calls,paired_calls,reads)=SIGNED_TRANSPORT.with(|c|c.get());assert!(norm_calls>0&&paired_calls>0&&reads>0);
 println!("transported_products={products} wrong_chart_counterexamples={negative} exact_image_states={states} public_cases={public} recovered={recovered} unresolved={unresolved} subsequent_updates={updates} norm_calls={norm_calls} paired_calls={paired_calls} coefficient_reads={reads}");
}
