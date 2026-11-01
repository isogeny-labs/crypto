//! Prospective selection from the already admitted parent cold table.
use super::*;

const CSV_SHA: &str = "7d47e6fc77d6dcb631b0bfd0cab0b5e93cd328ad9a9e716e35fdb8d5399f3919";
const HEADER: &str = "p,seed,arm,admitted,successful_solution,censored,median_cold_seconds,median_setup_seconds,median_online_seconds,max_process_rss_kib";

fn nanoseconds(s: &str) -> Result<u64, &'static str> {
    let (seconds, fractional) = s.split_once('.').ok_or("decimal-median")?;
    if seconds.is_empty()
        || fractional.len() != 9
        || !seconds
            .bytes()
            .chain(fractional.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return Err("decimal-median");
    }
    let seconds: u64 = seconds.parse().map_err(|_| "decimal-median")?;
    let fraction: u64 = fractional.parse().map_err(|_| "decimal-median")?;
    seconds
        .checked_mul(1_000_000_000)
        .and_then(|v| v.checked_add(fraction))
        .ok_or("decimal-median")
}

fn select(text: &str) -> Result<Vec<Value>, &'static str> {
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("cold-table-header");
    }
    let mut seen = BTreeSet::new();
    let mut best: BTreeMap<(u64, u64), (u64, usize)> = BTreeMap::new();
    for line in lines {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 10 {
            return Err("cold-table-row");
        }
        let p: u64 = fields[0].parse().map_err(|_| "case")?;
        let seed: u64 = fields[1].parse().map_err(|_| "case")?;
        let arm = ARMS.iter().position(|&v| v == fields[2]).ok_or("arm")?;
        if !PRIMES.contains(&p) || seed > 2 || !seen.insert((p, seed, arm)) {
            return Err("case-coverage");
        }
        if !["true", "false"].contains(&fields[3])
            || !["true", "false"].contains(&fields[4])
            || !["true", "false"].contains(&fields[5])
        {
            return Err("cold-table-flags");
        }
        let ns = nanoseconds(fields[6])?;
        if fields[3] == "true" && fields[4] == "true" && fields[5] == "false" {
            if ns == 0 {
                return Err("zero-price");
            }
            let candidate = (ns, arm);
            let selected = best.entry((p, seed)).or_insert(candidate);
            *selected = (*selected).min(candidate);
        }
    }
    if seen.len() != PRIMES.len() * 3 * ARMS.len() || best.len() != PRIMES.len() * 3 {
        return Err("case-coverage");
    }
    Ok(best
        .into_iter()
        .map(|((p, seed), (ns, arm))| {
            json!({"p":p,"seed":seed,"plain_arm":ARMS[arm],"old_median_cold_ns":ns,
            "reuse_registered_plain_execution":([7,9].contains(&arm))})
        })
        .collect())
}

pub(super) fn select_baseline(csv: &Path, expected_sha: &str, out: &Path) {
    let bytes = std::fs::read(csv).unwrap();
    let sha = hex::encode(hash::sha256::sha256(&bytes));
    assert_eq!(expected_sha, CSV_SHA);
    assert_eq!(sha, CSV_SHA);
    assert!(!out.exists());
    let selected = select(std::str::from_utf8(&bytes).unwrap()).unwrap();
    write(
        out,
        &json!({"schema":"twist-zero-pipeline-baseline-selection/v1",
        "source_cold_csv_sha256":sha,"source_publication_commit":"5abfb020dd7e993eb315d34f93beacc2200e1585",
        "rule":"Minimum recorded admitted successful uncensored five-round median cold interval, exact integer nanoseconds; ties use original arm order",
        "cases":selected,"scope":"Prospective best complete-cold control from the parent's verified ten-arm family only. New measured costs and globally strongest IC/rho and thinned-base prices remain separate obligations."}),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> String {
        let mut text = format!("{HEADER}\n");
        for p in PRIMES {
            for seed in 0..3 {
                for (ai, arm) in ARMS.iter().enumerate() {
                    let (success, censored, ns) = if ai == 3 {
                        (false, true, 1)
                    } else {
                        (true, false, 100 + ai)
                    };
                    text.push_str(&format!("{p},{seed},{arm},true,{success},{censored},0.{ns:09},0.000000001,0.000000001,100\n"));
                }
            }
        }
        text
    }

    #[test]
    fn selection_excludes_the_cheaper_censored_arm_and_covers_all_cases() {
        let rows = select(&table()).unwrap();
        assert_eq!(rows.len(), 21);
        assert!(rows
            .iter()
            .all(|v| v["plain_arm"] == "original" && v["old_median_cold_ns"] == 100));
    }

    #[test]
    fn selection_rejects_missing_duplicate_or_inadmissible_rows() {
        let good = table();
        let last = good.lines().last().unwrap();
        assert!(select(&good.replacen(last, "", 1)).is_err());
        assert!(select(&(good.clone() + last + "\n")).is_err());
        assert_eq!(
            select(&good.replace("0.000000100", "NaN")).unwrap_err(),
            "decimal-median"
        );
        assert_eq!(
            select(&good.replace(",true,true,false,", ",false,true,false,")).unwrap_err(),
            "case-coverage"
        );
    }

    #[test]
    fn cold_medians_are_compared_without_floating_point_roundoff() {
        assert_eq!(nanoseconds("25.907802999"), Ok(25_907_802_999));
        assert_eq!(nanoseconds("25.907803000"), Ok(25_907_803_000));
        assert!(nanoseconds("0.1").is_err());
        assert!(nanoseconds("1e-9").is_err());
    }
}
