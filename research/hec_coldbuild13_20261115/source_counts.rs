pub fn source_counts(coeff: &[u64]) -> (u64, u64) {
    let mut answer = [1u64; 2];
    for k in 1..=2 {
        let e = Ext::new(127, k);
        let fp = hec_native_ic::fp::Fp::new(127);
        for i in 0..e.q() {
            let x = e.element(i);
            let v = coeff
                .iter()
                .rev()
                .fold([0; 3], |a, c| e.add(e.mul(a, x), e.scalar(*c)));
            let norm = if k == 1 {
                v[0]
            } else {
                (v[0] * v[0] + v[0] * v[1] * e.red[1] + 127 * 127 - v[1] * v[1] % 127 * e.red[0])
                    % 127
            };
            let chi = fp.legendre(norm);
            answer[k - 1] += if chi == 0 {
                1
            } else if chi == 1 {
                2
            } else {
                0
            };
        }
    }
    (answer[0], answer[1])
}
