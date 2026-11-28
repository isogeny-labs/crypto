fn power(mut a:u64,mut n:u64,p:u64)->u64{let mut v=1;while n!=0{if n&1!=0{v=v*a%p;}a=a*a%p;n>>=1;}v}
#[test]fn quotient_composition_and_context(){let mut pairs=0;
 for p in [5u64,7,13,29,509]{let e=Poly::new(p).unwrap();let q=1u64<<(p-1).trailing_zeros();let m=(p-1)/q;
  for x in 0..p{for y in 0..p{
   let got=e.image_class(x).unwrap().combine(e.image_class(y).unwrap()).unwrap().project();assert_eq!(got,power(x*y%p,m,p));
   if y!=0{let alternate=x*power(y,q,p)%p;assert_eq!(e.image_class(alternate).unwrap().project(),power(x,m,p));}
   pairs+=1;
  }
  assert_eq!(e.image_class(x).unwrap().square().project(),power(x*x%p,m,p));assert_eq!(e.image_class(x).unwrap().negate().project(),power((p-x)%p,m,p));assert_eq!(e.image_class(x+p).unwrap().project(),power(x,m,p));
  }
  assert!(e.image_class(0).unwrap().is_zero());assert!(!e.image_class(1).unwrap().is_zero());
 }
 let a=Poly::new(5).unwrap().image_class(1).unwrap();let b=Poly::new(13).unwrap().image_class(1).unwrap();assert!(matches!(a.combine(b),Err(Error::Domain)));
 assert!(Poly::new(17).unwrap().image_class(1).is_err());println!("quotient_product_pairs={pairs}; mixed_context_rejected=true");
}
