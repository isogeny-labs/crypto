//! Public-base membership polynomials, with no relation or sign enumeration.
use super::*;

pub(super) fn fibers(base: &Base) -> Result<BTreeMap<i64, Vec<i64>>, String> {
    if !(4..=5).contains(&base.depth) {
        return Err("liftable fibers require depth four or five".into());
    }
    let p = base.curve.p;
    let mut seen = BTreeSet::new();
    let mut pools = BTreeMap::<i64, Vec<i64>>::new();
    for point in &base.unsigned {
        if !(0..p).contains(&point.x)
            || !(1..p).contains(&point.y)
            || point.y * point.y % p != base.curve.rhs(point.x)
            || !seen.insert(point.x)
        {
            return Err("invalid or duplicate liftable-base abscissa".into());
        }
        pools
            .entry(dickson(point.x, base.depth - 3, p))
            .or_default()
            .push(point.x);
    }
    if pools.values().any(|xs| xs.len() > (1 << (base.depth - 3))) {
        return Err("liftable fiber exceeds the map degree".into());
    }
    Ok(pools
        .into_iter()
        .map(|(b, xs)| (b, cutoff::root_polynomial(&xs, p)))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fiber_roots_are_exactly_the_public_liftable_base() {
        for (curve, depth) in [(FIXTURES[0], 4), (FIXTURES[1], 4), (FIXTURES[1], 5)] {
            let base = Base::new(curve, depth);
            let polys = fibers(&base).unwrap();
            assert_eq!(
                polys.values().map(|q| q.len() - 1).sum::<usize>(),
                base.unsigned.len()
            );
            for (b, q) in polys {
                assert_eq!(q.last(), Some(&1));
                for x in 0..curve.p {
                    let member = base.unsigned.iter().any(|point| point.x == x)
                        && dickson(x, depth - 3, curve.p) == b;
                    assert_eq!(evaluate(&q, x, curve.p) == 0, member);
                }
            }
        }
    }

    #[test]
    fn malformed_base_and_fiber_shapes_are_rejected() {
        let mut base = Base::new(FIXTURES[0], 4);
        let polys = fibers(&base).unwrap();
        let chart = cutoff::qualified_catalog(&base).unwrap().remove(0);
        let mut missing = polys.clone();
        missing.remove(&chart.images[0]);
        assert!(System::fiber_factors(base.curve, base.curve.q, chart.images, &missing).is_err());
        let mut nonmonic = polys.clone();
        *nonmonic
            .get_mut(&chart.images[0])
            .unwrap()
            .last_mut()
            .unwrap() = 2;
        assert!(System::fiber_factors(base.curve, base.curve.q, chart.images, &nonmonic).is_err());
        let mut too_short = polys;
        too_short.insert(chart.images[0], vec![1]);
        assert!(System::fiber_factors(base.curve, base.curve.q, chart.images, &too_short).is_err());
        base.unsigned.push(base.unsigned[0]);
        assert!(fibers(&base).is_err());
        base.unsigned.pop();
        base.unsigned[0].y = 0;
        assert!(fibers(&base).is_err());
    }
}
