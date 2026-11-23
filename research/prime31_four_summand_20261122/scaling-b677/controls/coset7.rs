//! Independent scalar diagnostic for two contradictory historical spellings.
//! This is neither a relation search nor a factor-base replacement.
fn main() {
    let p = 2_147_483_647_u128;
    let y = 136_769_428_u128;
    let mut repeated = 1_u128;
    for _ in 0..7 {
        repeated = repeated * y % p;
    }
    let y2 = y * y % p;
    let y4 = y2 * y2 % p;
    let chain = ((y4 * y2 % p) * y) % p;
    assert_eq!(repeated, chain);
    println!(
        "{{\"p\":{p},\"y0\":{y},\"exponent\":7,\"repeated_product\":{repeated},\"addition_chain\":{chain},\"later_spelling\":1852955275,\"earlier_spelling\":1852957275,\"matches_later\":{},\"matches_earlier\":{}}}",
        repeated == 1_852_955_275, repeated == 1_852_957_275
    );
}
