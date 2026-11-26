pub fn fp_p31_subexp_mersenne_instance() -> PrimeInstance {
    let curve = PrimeCurve {
        p: 2_147_483_647,
        a: 2_147_483_644,
        b: 21,
    };
    let r = 2_147_487_637;
    PrimeInstance {
        name: prime_curve_id(&curve, r).slug,
        curve,
        group_order: r,
        r,
        cofactor: 1,
        generator: (1_391_443_431, 177_979_044),
        representation: None,
    }
}
