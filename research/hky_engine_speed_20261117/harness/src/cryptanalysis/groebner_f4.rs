//! Stub: `Ordering` and `cmp_monomial`, verbatim from
//! `src/cryptanalysis/groebner_f4.rs` lines 65-104.
// ── Monomial ordering ───────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ordering {
    /// Lexicographic: compare leftmost exponent first.
    Lex,
    /// Graded-lex: compare total degree first, break ties by lex.
    Grlex,
    /// Graded-reverse-lex: compare total degree, then reverse-lex.
    Grevlex,
}

/// Compare two monomials under the given ordering.
/// Returns `Ord::Greater` if `a` > `b`.
pub fn cmp_monomial(a: &[u32], b: &[u32], ord: Ordering) -> std::cmp::Ordering {
    let deg_a: u32 = a.iter().sum();
    let deg_b: u32 = b.iter().sum();
    match ord {
        Ordering::Lex => a.cmp(b),
        Ordering::Grlex => match deg_a.cmp(&deg_b) {
            std::cmp::Ordering::Equal => a.cmp(b),
            o => o,
        },
        Ordering::Grevlex => match deg_a.cmp(&deg_b) {
            std::cmp::Ordering::Equal => {
                // Reverse-lex: compare from the right, but the *smaller*
                // rightmost-nonzero exponent wins.
                for i in (0..a.len()).rev() {
                    match a[i].cmp(&b[i]) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            o => o,
        },
    }
}
