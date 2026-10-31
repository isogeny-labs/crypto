use rand::SeedableRng;
use crypto_lib::cryptanalysis::gaudry_quartic_bin::*;
fn main(){
    for m in [15u32,17] {
        let c = find_bin_curve(m,1,2).unwrap();
        let cv=&c.curve; let f=&cv.f;
        let b=cv.b;
        { let x=f.random(&mut rand::rngs::StdRng::seed_from_u64(5)); let y=f.random(&mut rand::rngs::StdRng::seed_from_u64(6));
 let a0=f.f.muls(); let _=f.mul(&x,&y); let a1=f.f.muls(); let _=f.sq(&x); let a2=f.f.muls(); let _=f.inv(&x); let a3=f.f.muls();
 println!("Fq4 mul={} sq={} inv={} muls per add={}",a1-a0,a2-a1,a3-a2,fq_muls_per_add(cv)); }
        println!("m={} order={} r={} h={}",m,c.group_order,c.r,c.cofactor);
        // j=1/b in F_{2^d}? check b^(2^d)==b for d dividing 4m
        for d in 1..=4*m { if (4*m)%d==0 { println!(" b in F_2^{} : {}", d, f.pow2k_counted(&b,d)==b);} }
        for d in 1..=4*m { if (4*m)%d==0 { println!(" a in F_2^{} : {}", d, f.pow2k_counted(&cv.a,d)==cv.a);} }
    }
}
#[allow(dead_code)]
fn extra(){}
