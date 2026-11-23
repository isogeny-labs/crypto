fn profile(family: &str, n: u32, m: usize) -> bool {
    matches!(family, "binary" | "koblitz")
        && ((23..=31).contains(&n) || (47..=61).contains(&n))
        && (2..=15).contains(&m)
}
