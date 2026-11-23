#[cfg(test)]
mod closed_root_tests {
    use super::*;
    #[derive(Clone, Debug)]
    struct Fast(WordField);
    impl PrimeField for Fast {
        type E=u64;
        fn modulus(&self)->BigUint {self.0.modulus()}
        fn element(&self,n:u64)->u64 {self.0.element(n)}
        fn add(&self,a:&u64,b:&u64)->u64 {self.0.add(a,b)}
        fn sub(&self,a:&u64,b:&u64)->u64 {self.0.sub(a,b)}
        fn mul(&self,a:&u64,b:&u64)->u64 {self.0.mul(a,b)}
        fn hex(&self,a:&u64)->String {self.0.hex(a)}
        fn canonical(&self,a:&u64)->bool {self.0.canonical(a)}
        fn root_shortcuts(&self)->bool {true}
    }
    #[test]
    fn exhaustive_quadratics_include_repeated_and_nonsplit_cases() {
        for p in [5,7,17,19] {
            let a=Arithmetic::new(Fast(WordField(p)),Limits{max_muls:100_000_000,max_split_attempts:10000,timeout:Duration::from_secs(20)}).unwrap();
            for x in 0..p {for y in 0..p {for z in 0..p {
                let poly=a.trim(vec![x,y,z]);
                let result=a.roots(&poly,&mut 17);
                if x==0 && y==0 && z==0 {assert!(result.is_err());continue;}
                let expected:Vec<_>=(0..p).filter(|t|a.eval(&poly,t)==0).collect();
                assert_eq!(result.unwrap(),expected,"p{p}, polynomial{poly:?}");
            }}}
        }
    }
    #[test]
    fn closed_fibers_still_reject_inseparable_and_nonsplit() {
        let a=Arithmetic::new(Fast(WordField(17)),Limits::default()).unwrap();
        let map=RationalMap{numerator:vec![0,0,1],denominator:vec![1]};
        assert!(a.certify_tower(&[map.clone()],0,42).is_err());
        assert!(a.certify_tower(&[map.clone()],3,42).is_err());
        assert_eq!(a.certify_tower(&[map],4,42).unwrap().abscissae().len(),2);
    }
}
