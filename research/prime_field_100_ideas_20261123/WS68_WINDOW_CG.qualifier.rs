// Exact native EC controls: #80 de Bruijn windows, #88 field-CG breakdown.
fn main(){
    // Binary order-2 de Bruijn cycle 0011, read cyclically.
    let bits=[0i64,0,1,1];
    let values:[i64;4]=std::array::from_fn(|i|2*bits[i]+bits[(i+1)%4]);
    assert_eq!(values,[0,1,3,2]);
    let residues=values.map(|v|v.rem_euclid(3));
    assert_eq!(residues,[0,1,0,2]);
    // E/F_3: y^2=x^3+2x+1 is nonsingular since derivative 2 never vanishes.
    // Every mapped x has the canonical point (x,1), including the duplicate.
    for x in residues {assert_eq!((x*x*x+2*x+1).rem_euclid(3),1);}
    assert_eq!((residues[0],1),(residues[2],1));

    // On the previously checked prime-order E(F_1009) group of order 1013,
    // the equations I_2*x=b with b=(1,45) are valid point-log equations.
    // Standard finite-field CG starts at x=0, r=b, p=r. Its first denominator
    // p^T I p is zero although b is nonzero and solution x=b exists.
    let n=1013i64;let b=[1i64,45];
    let dot=(b[0]*b[0]+b[1]*b[1]).rem_euclid(n);
    assert_eq!(dot,0);assert_ne!(b,[0,0]);
    assert_eq!((b[0],b[1]),(1,45));
    println!("debruijn_cycle=0011 windows={values:?} x_mod_3={residues:?} all_lift_to_E_F3=true duplicate_point=(0,1)");
    println!("ec_group_order={n} matrix=I2 rhs={b:?} nonzero_rhs=true initial_CG_denominator={dot} exact_solution={b:?}");
}
