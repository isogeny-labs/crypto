fn main(){
 let (a0,a1,b0,b1)=(2i128,3i128,5i128,7i128);
 let r0=b0*b0-a0*b1*b1;let r1=2*b0*b1-a1*b1*b1;let s=b1*b1;
 assert_eq!((r0,r1),(-73,-77));
 assert_eq!((r1+a1*s)/(2*b1),b0);
 assert_eq!((r1+a1*s)/(2*(-b1)),-b0);
 assert_eq!((-b0)*(-b0)-a0*(-b1)*(-b1),r0);
 assert_ne!(2*(-b0)*b1-a1*b1*b1,r1);
 assert_eq!(5i128.pow(2),(-5i128).pow(2));
 // Quotient by negation in Z/8: nonzero two-torsion anchor does not orient.
 let pi=|n:i32|{let n=n.rem_euclid(8);n.min((-n).rem_euclid(8))};
 assert_eq!(pi(1),pi(-1));assert_eq!(pi(4+1),pi(4-1));assert_ne!(1,7);
 // In Z/7, wrong recovered sign with unchanged label fails the edge equation.
 assert_eq!((2+2)%7,(1+3)%7);assert_ne!((-2i32-2).rem_euclid(7),(1+3)%7);
 println!("{{\"schema\":\"fixed-algebra-controls/v1\",\"assertions\":11,\"accepted\":true,\"curve_computation\":false}}");
}
