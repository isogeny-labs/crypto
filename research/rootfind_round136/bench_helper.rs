
pub fn round136_cases(p:u64,kind:&str)->Vec<Vec<u64>> {
    let mut tally=FieldTally::default();
    let mut f=Fp{p,tally:&mut tally};
    let mut state=p^0x136_2026_11_23;
    let mut next=||{state^=state<<13;state^=state>>7;state^=state<<17;state%p};
    let mut nonsquare=2;
    if kind=="quad_nonsquare" {
        while Fp::jacobi_symbol(nonsquare,p)!=-1{nonsquare+=1;}
    }
    let mut cases=Vec::new();
    for i in 0..32u64 {
        let a=next();let b=next();
        cases.push(match kind {
            "quadratic"=>round129_linear_factors(&mut f,&[(3+131*i)%p,(11+197*i)%p]),
            "quad_random"=>vec![a,b,1],
            "quad_nonsquare"=>{
                let square=f.mul(if a==0{1}else{a},if a==0{1}else{a});
                let disc=f.mul(nonsquare,square);
                vec![f.sub(0,disc),0,1]
            },
            "cubic"=>round129_linear_factors(&mut f,&[a,b,(7+197*i)%p]),
            _=>panic!("unsupported {kind}")
        });
    }
    cases
}
pub fn round136_oracle(){
    for p in [3u64,7,11,17,19,31,97,101,103,257]{
        let mut tally=FieldTally::default();
        let mut f=Fp{p,tally:&mut tally};
        let bound=if p<=103{p}else{40};
        for b in 0..bound{for c in 0..bound{
            let expected:Vec<u64>=(0..p).filter(|&x|{
                let xx=f.mul(x,x);let bx=f.mul(b,x);
                f.add(f.add(xx,bx),c)==0
            }).collect();
            assert_eq!(f.roots(&[c,b,1]),expected,"p={p} b={b} c={c}");
        }}
    }
    for p in [65_537u64,167_772_161,998_244_353,2_013_265_921]{
        let mut tally=FieldTally::default();
        let mut f=Fp{p,tally:&mut tally};
        for i in 0..256u64{
            let a=(i*786_433+19)%p;
            let expected=a==0||f.pow(a,(p-1)/2)==1;
            let root=f.sqrt_tonelli_shanks(a);
            assert_eq!(root.is_some(),expected,"Euler p={p} a={a}");
            if let Some(x)=root{assert_eq!(f.mul(x,x),a);}
        }
        for poly in round136_cases(p,"quadratic"){assert_eq!(f.roots(&poly).len(),2);}
    }
    for p in [65_519u64,1_000_000_007,4_294_967_291,(1u64<<61)-1]{
        let mut tally=FieldTally::default();
        let mut f=Fp{p,tally:&mut tally};
        for poly in round136_cases(p,"quad_nonsquare"){
            assert!(f.roots(&poly).is_empty(),"nonsquare p={p}");
        }
    }
    println!("round136 independent Euler/roots oracle: PASS");
}
