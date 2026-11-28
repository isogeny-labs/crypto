//! Equivalent canonical-monic row test on the bounded relation domain.
//! Counted calls and unsupported shapes retain the original normalization path.
use super::{cmp_monomial, normalise, Ordering, Poly};

pub(super) fn is_canonical_monic(g: &Poly, p: u64, order: Ordering, accounted: bool) -> bool {
    if g.is_empty() || g[0].1 != 1 {
        return false;
    }
    let n = g[0].0.len();
    if accounted
        || !matches!(p, 2147483647 | 9007199254740881)
        || !(3..=16).contains(&n)
        || g.iter()
            .any(|(m, _)| m.len() != n || m.iter().any(|&e| e > 64))
    {
        return *g == normalise(g, p, order);
    }
    // Uniform bounded vectors give the existing comparator a strict total
    // order; fixed points have canonical coefficients and unique sorted terms.
    g.iter().all(|(_, c)| *c != 0 && *c < p)
        && g.windows(2)
            .all(|w| cmp_monomial(&w[0].0, &w[1].0, order) == std::cmp::Ordering::Greater)
}
