fn checks(){let(mut cases,mut degrees,mut reverse_failures,mut nonunit,mut no_transition)=(0,0,0,0,0);
 for(r,m)in[(7u64,3usize),(13,3),(17,4),(41,5),(97,3),(281,5),(281,35)]{
  let h=(r-1)/2;let l=(h/m as u64)as usize;
  let mut dense=vec![1];for _ in 0..h{let mut next=vec![0;dense.len()+1];for(j,&c)in dense.iter().enumerate(){next[j]=(next[j]+r-c)%r;next[j+1]=(next[j+1]+c)%r;}dense=next;}
  let coeff:Q=dense.iter().skip(1).step_by(m).copied().collect();assert_eq!(coeff.len(),l);
  for g in [vec![0,1],vec![1,0,1],vec![4,r-4,1],vec![0,0,0,1]]{for base in [vec![0],vec![0,1],vec![2,1],vec![1]]{for block in [1usize,2,3,7]{
   let mut ring=Ring{r,g:g.clone(),p:polynomial::Poly::new(r).unwrap()};let v=ring.rem(&base);
   let mut expected=vec![0];for &c in coeff.iter().rev(){expected=ring.qm(&expected,&v);expected=ring.qa(&expected,&vec![c]);}
   let(actual,degree)=ring.blocked(m,&v,block,128).unwrap();assert_eq!(actual,expected,"r={r} m={m} block={block}");cases+=1;
   if l==1{assert_eq!(degree,0);no_transition+=1}else{assert_eq!(degree,m*block.min(l-1));degrees+=1;}
   let gcd=ring.p.gcd_monic(&g,&v,g.len()+v.len()+2).unwrap().polynomial;if gcd.len()>1{nonunit+=1;}
   if l>=3{let x=ring.leaf(m,0,&v);let y=ring.leaf(m,1,&v);let x=ring.ve(&x,0);let y=ring.ve(&y,0);let forward=ring.vc(&y,&x);let reverse=ring.vc(&x,&y);if forward.b!=reverse.b{reverse_failures+=1;}}
   assert!(ring.blocked(m,&v,0,128).is_err());if l>1{assert!(ring.blocked(m,&v,block,0).is_err());}
  }}}
 }
 assert!(nonunit>0&&reverse_failures>0&&no_transition>0);
 println!("quotient_cases={cases} exact_block_degrees={degrees} wrong_order_controls={reverse_failures} nonunit_quotient_cases={nonunit} no_transition_cases={no_transition} cap_rejections=pass");
}
