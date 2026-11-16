fn class_checks(){let(mut pairs,mut resultants)=(0,0);
 for p in [5,7,17,41,97,193,257]{let poly=polynomial::Poly::new(p).unwrap();let q=1u64<<(p-1).trailing_zeros();let odd=(p-1)/q;
  for x in 0..p{for y in 0..p{assert_eq!(poly.image_class(x).unwrap().combine(poly.image_class(y).unwrap()).unwrap().project(),pow(mul(x,y,p),odd,p));pairs+=1;}
   for e in [0,1,2,q-1,q,q+1,u64::MAX]{assert_eq!(poly.image_class(x).unwrap().power(e).project(),pow(pow(x,e,p),odd,p));}
   if x!=0{assert_eq!(poly.image_class(x).unwrap().inverse().unwrap().project(),pow(pow(x,p-2,p),odd,p));}else{assert!(poly.image_class(x).unwrap().inverse().is_err());}
  }
  for n in 1..=16{for m in 1..=16{let roots:Vec<_>=(0..n).map(|i|(3*i+1)%p).collect();let points:Vec<_>=(0..m).map(|i|(5*i+2)%p).collect();let mut poly=polynomial::Poly::new(p).unwrap();let want=points.iter().fold(1,|v,&x|roots.iter().fold(v,|v,&y|mul(v,(x+p-y)%p,p)));assert_eq!(poly.root_resultant_class(&roots,&points).unwrap().project(),pow(want,odd,p));resultants+=1;}}
 }
 assert!(polynomial::Poly::new(17).unwrap().image_class(1).unwrap().combine(polynomial::Poly::new(41).unwrap().image_class(1).unwrap()).is_err());
 println!("general_class_pairs={pairs} projected_root_resultants={resultants} exponent_wrap_and_zero_controls=true");
}
