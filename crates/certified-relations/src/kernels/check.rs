pub fn check_reference(basis: &[Row], inputs: &[Poly], n: usize, p: u64) -> bool {
    check_reference_impl(basis, inputs, n, p, false)
}
/// Opt-in Buchberger product criterion. Membership and all input containments
/// remain exact; only coprime leading-monomial pairs avoid explicit division.
pub fn check_reference_product(basis: &[Row], inputs: &[Poly], n: usize, p: u64) -> bool {
    check_reference_impl(basis, inputs, n, p, true)
}
fn check_reference_impl(basis: &[Row], inputs: &[Poly], n: usize, p: u64, product: bool) -> bool {
    if basis.is_empty()
        || basis
            .iter()
            .any(|g| g.poly.is_empty() || !verify(g, inputs, p))
    {
        return false;
    }
    let bare: Vec<Row> = basis
        .iter()
        .map(|r| Row {
            poly: r.poly.clone(),
            proof: Proof::new(),
        })
        .collect();
    let basis = bare.as_slice();
    if input_rows(inputs, n)
        .into_iter()
        .any(|r| !normal_form(r, basis, p).poly.is_empty())
    {
        return false;
    }
    for j in 0..basis.len() {
        for i in 0..j {
            if product
                && lead(&basis[i].poly)
                    .iter()
                    .zip(lead(&basis[j].poly))
                    .all(|(a, b)| *a == 0 || *b == 0)
            {
                continue;
            }
            if !normal_form(s_polynomial(&basis[i], &basis[j], p), basis, p)
                .poly
                .is_empty()
            {
                return false;
            }
        }
    }
    true
}
