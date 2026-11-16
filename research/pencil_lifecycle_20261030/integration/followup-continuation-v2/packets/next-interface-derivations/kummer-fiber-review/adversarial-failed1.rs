fn main() {
    // F17, u=X^2, S=0: candidate +/-1 cannot satisfy residual coefficient R1=1.
    let p=17i64;
    let rem1=1;
    let generated=[(1i64,0i64),(16,0)];
    assert_eq!((2*generated[0].0*generated[0].1).rem_euclid(p),0);
    assert_ne!((2*generated[0].0*generated[0].1).rem_euclid(p),rem1);
    assert_ne!((2*generated[1].0*generated[1].1).rem_euclid(p),rem1);
    // Repeated support u=X^2; v=3+2X, S=4: the two reconstructed pairs are opposites.
    let (r0,r1,s)=(9i64,12i64,4i64);
    assert_eq!((3*3).rem_euclid(p),r0);
    assert_eq!((2*3*2).rem_euclid(p),r1);
    assert_eq!((15*15).rem_euclid(p),s);
    // Characteristic2: nonzero b1 does not make 2*b1 invertible.
    assert_eq!((2*1)%2,0);
    println!("{{\"schema\":\"fixed-adversarial-controls/v1\",\"assertions\":7,\"accepted\":true,\"scope\":\"S-zero-invalid-residual,repeated-support,char2-denominator\"}}");
}
