fn stream_checks(){
 let(mut sequences,mut coefficients,mut residues,mut evaluations,mut states)=(0,0,0,0,0);
 for r in [7u64,11,13,17,31,97,257]{for m in 2..r as usize{if(r-1)%m as u64!=0{continue}
  let z=(2..r).find(|&z|pow(z,m as u64,r)==1&&(1..m).all(|j|pow(z,j as u64,r)!=1)).unwrap();let dense=Modes::compile(r,m,z,m,256).unwrap();
  for block in [1usize,2,7,64]{let stream=StreamModes::new(r,m,z,m,256,block).unwrap();
   for e in 0..m.min(3){let mut s=stream.stream(e).unwrap();let mut got=Vec::new();while s.left>0{let count=s.left.min(block);let part=s.take_desc(count).unwrap();assert!(part.len()<=block);got.extend(part);}assert!(s.take_desc(1).is_err());got.reverse();assert_eq!(got,dense.coeff[e]);sequences+=1;coefficients+=got.len();
    for x in [0,1,r-1]{assert_eq!(stream.evaluate(e,x).unwrap(),eval(&dense.coeff[e],x,r));evaluations+=1;}
    for n in [1usize,2,7]{for t in [1,2,r-1]{let mut g:Vec<_>=(0..n).map(|i|((i+1)as u64*3)%r).collect();g.push(1);let mut p=polynomial::Poly::new(r).unwrap();let expected=ScaledResidue::new(r,&g,t).unwrap().reduce(&mut p,&dense.coeff[e]).unwrap();assert_eq!(stream.residue(&mut p,e,t,&g).unwrap(),expected);residues+=1;}}
   }
   for k in 0..r{let mut old=ImageDomain::new(r,m).unwrap();let mut new=ImageDomain::new(r,m).unwrap();let mut p=polynomial::Poly::new(r).unwrap();
    for b in [1,2,r-1]{let mut root=1;let signs:Vec<_>=(0..m).map(|_|{let v=pow((k+r-mul(b,root,r))%r,(r-1)/2,r);root=mul(root,z,r);v}).collect();let f=dense.observed(&signs).unwrap();assert_eq!(stream.observed(&signs).unwrap(),f);dense.intersect_modes(&mut p,&mut old,b,&f).unwrap();stream.intersect_modes(&mut p,&mut new,b,&f).unwrap();assert_eq!(old.dense,new.dense);assert_eq!(old.degree(),new.degree());states+=1;
    }
   }
  }
 }}
 for m in [64usize,128,256,512]{let r=65537;let z=pow(3,(r-1)/m as u64,r);let dense=Modes::compile(r,m,z,512,1024).unwrap();let stream=StreamModes::new(r,m,z,512,1024,17).unwrap();
  for e in 0..3{let mut s=stream.stream(e).unwrap();let mut all=Vec::new();while s.left>0{all.extend(s.take_desc(s.left.min(17)).unwrap());}all.reverse();assert_eq!(all,dense.coeff[e]);sequences+=1;coefficients+=all.len();}
 }
 let r=(1u64<<61)-1;let large=StreamModes::new(r,2,r-1,2,usize::MAX,8).unwrap();assert_eq!(large.m,2);assert_eq!(std::mem::size_of::<StreamModes>(),32);
 assert!(StreamModes::new(13,3,3,3,1,4).is_err());assert!(StreamModes::new(13,3,1,3,64,4).is_err());assert!(StreamModes::new(13,3,3,3,64,0).is_err());let mut limited=CoeffStream::new(97,2,0,2).unwrap();assert!(limited.take_desc(3).is_err());assert!(limited.take_desc(0).is_err());
 println!("stream_coefficient_sequences={sequences} exact_coefficients={coefficients} scaled_residues={residues} scalar_evaluations={evaluations} sequential_states={states} large_parameter_only_r=2^61-1 mode_object_bytes=32");
}
