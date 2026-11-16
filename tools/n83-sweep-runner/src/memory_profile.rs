//! Native validation of container memory samples; never worker heap attribution.
use super::{hash, memory_limit, number, read, require, write, R};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

fn counter(v: &Value, key: &str) -> R<u64> {
    let raw = v[key]["raw"].as_str().ok_or("missing memory counter")?;
    require(
        !raw.is_empty() && raw.bytes().all(|b| b.is_ascii_digit()),
        "invalid memory counter",
    )?;
    Ok(raw.parse()?)
}
fn pairs(raw: &str) -> R<BTreeMap<String, u64>> {
    let mut values = BTreeMap::new();
    for line in raw.lines() {
        let mut fields = line.split_whitespace();
        let key = fields.next().ok_or("empty counter line")?;
        let value = fields.next().ok_or("missing counter value")?.parse()?;
        require(
            fields.next().is_none() && values.insert(key.to_owned(), value).is_none(),
            "duplicate or malformed counter",
        )?;
    }
    require(!values.is_empty(), "empty counters")?;
    Ok(values)
}
fn validate(raw: &str, pid: u64, limit: u64, outer_ms: Option<f64>) -> R<Value> {
    let mut rows = Vec::new();
    let mut previous_ms = -1.;
    let mut previous_peak = 0;
    let mut previous_events = BTreeMap::new();
    let mut cgroup = None;
    let mut max_current = 0;
    let mut max_gap: f64 = 0.;
    for line in raw.lines() {
        let v: Value = serde_json::from_str(line)?;
        require(
            v["schema"] == "n83.memory-sample/v1" && v["pid"] == pid && pid > 0,
            "memory sample schema/PID mismatch",
        )?;
        require(
            v.get("cgroup_error").is_none() && v.get("process_error").is_none(),
            "unavailable memory sample",
        )?;
        let ms = v["elapsed_ms"].as_f64().ok_or("missing sample time")?;
        require(
            ms.is_finite() && ms >= 0. && ms > previous_ms,
            "invalid sample order",
        )?;
        require(
            outer_ms.is_none_or(|end| end.is_finite() && ms <= end),
            "sample past terminal",
        )?;
        if previous_ms >= 0. {
            let gap = ms - previous_ms;
            require(gap >= 60_000., "sample interval below frozen 60 seconds")?;
            max_gap = max_gap.max(gap);
        }
        let path = v["cgroup_path"].as_str().ok_or("missing cgroup path")?;
        let relative = path
            .strip_prefix("/sys/fs/cgroup/")
            .ok_or("non-container cgroup")?;
        require(
            !relative.is_empty()
                && relative
                    .split('/')
                    .all(|c| !c.is_empty() && c != "." && c != ".."),
            "unsafe cgroup path",
        )?;
        if let Some(previous) = &cgroup {
            require(previous == path, "cgroup changed")?;
        } else {
            cgroup = Some(path.to_owned());
        }
        require(
            counter(&v, "memory.max")? == limit && counter(&v, "memory.swap.max")? == 0,
            "sampled cgroup limits differ",
        )?;
        let current = counter(&v, "memory.current")?;
        let peak = counter(&v, "memory.peak")?;
        require(
            current <= peak && peak <= limit && peak >= previous_peak,
            "invalid kernel memory peak",
        )?;
        let stats = pairs(
            v["memory.stat"]["raw"]
                .as_str()
                .ok_or("missing memory.stat")?,
        )?;
        let events = pairs(
            v["memory.events"]["raw"]
                .as_str()
                .ok_or("missing memory.events")?,
        )?;
        for key in ["oom", "oom_kill", "max", "high"] {
            require(events.contains_key(key), "missing event counter")?;
        }
        for (key, before) in &previous_events {
            require(
                events.get(key).is_some_and(|after| after >= before),
                "memory event counter regressed",
            )?;
        }
        require(
            stats.contains_key("anon") && stats.contains_key("file"),
            "missing memory breakdown",
        )?;
        let mut process = BTreeMap::new();
        for field in v["process_status"]
            .as_array()
            .ok_or("missing init process status")?
        {
            let (key, value) = field
                .as_str()
                .ok_or("invalid process status")?
                .split_once(':')
                .ok_or("invalid process field")?;
            let mut parts = value.split_whitespace();
            let kib: u64 = parts.next().ok_or("missing process counter")?.parse()?;
            require(
                parts.next() == Some("kB")
                    && parts.next().is_none()
                    && process.insert(key.to_owned(), kib).is_none(),
                "invalid process units/duplicate",
            )?;
        }
        require(
            process
                .get("VmHWM")
                .zip(process.get("VmRSS"))
                .is_some_and(|(peak, rss)| peak >= rss),
            "invalid init RSS/HWM",
        )?;
        rows.push(
            json!({"elapsed_ms":ms,"current_bytes":current,"kernel_peak_bytes":peak,
            "anon_bytes":stats["anon"],"file_bytes":stats["file"],"events":events,
            "init_timeout_rss_kib":process["VmRSS"],"init_timeout_hwm_kib":process["VmHWM"]}),
        );
        max_current = max_current.max(current);
        previous_ms = ms;
        previous_peak = peak;
        previous_events = events;
    }
    require(!rows.is_empty(), "empty memory profile")?;
    Ok(
        json!({"schema":"n83.memory-profile-validation/v1","scope":"container cgroup charges; init PID is startup/timeout supervisor, not worker RSS; no internal allocation attribution",
        "n_samples":rows.len(),"memory_limit_bytes":limit,"pid":pid,"cgroup_path":cgroup,
        "max_sampled_current_bytes":max_current,"max_sampled_kernel_peak_bytes":previous_peak,
        "max_observed_interval_ms":max_gap,"last_sample_elapsed_ms":previous_ms,
        "unsampled_tail_ms":outer_ms.map(|end| end-previous_ms),
        "peak_scope":"largest sampled kernel peak; full-run peak can exceed it after last sample",
        "samples":rows}),
    )
}
pub fn verify_complete(raw: &str, pid: u64, limit: u64, outer_ms: f64) -> R<()> {
    validate(raw, pid, limit, Some(outer_ms)).map(|_| ())
}

pub fn summarize(dir: &Path, output: &Path) -> R<()> {
    let cfg = read(dir.join("config.json"))?;
    let live = read(dir.join("inspect-live.json"))?;
    let profile = dir.join("memory-profile.jsonl");
    let digest = hash(&profile)?;
    let completed = dir.join("outer.json").exists();
    let end = if completed {
        let outer = read(dir.join("outer.json"))?;
        require(
            outer["memory_profile_blake3"] == digest,
            "memory profile custody mismatch",
        )?;
        Some(
            outer["outer_wall_ms"]
                .as_f64()
                .ok_or("missing outer time")?,
        )
    } else {
        require(
            dir.join("launcher-error.json").exists() && !dir.join("audit.json").exists(),
            "missing terminal without retained launcher failure",
        )?;
        None
    };
    let raw = fs::read_to_string(&profile)?;
    let checked = validate(
        &raw,
        number(&live["state"], "Pid")?,
        memory_limit(&cfg)?,
        end,
    );
    let mut result = match checked {
        Ok(value) => value,
        Err(error) => {
            let observations: Vec<Value> = raw.lines().map(|line| match serde_json::from_str::<Value>(line) {
                Ok(v) => json!({"elapsed_ms":v["elapsed_ms"],"current_bytes":counter(&v,"memory.current").ok(),
                    "kernel_peak_bytes":counter(&v,"memory.peak").ok(),"memory_max_raw":v["memory.max"]["raw"],
                    "swap_max_raw":v["memory.swap.max"]["raw"]}),
                Err(e) => json!({"parse_error":e.to_string()}),
            }).collect();
            write(
                output,
                &json!({"schema":"n83.memory-profile-validation/v1","status":"REJECTED_MEMORY_PROFILE",
                "error":error.to_string(),"profile_blake3":digest,"dir":dir,"admitted":false,
                "observations":observations,"scope":"raw observations retained despite failed resource-profile validation"}),
            )?;
            return Err(error);
        }
    };
    result["status"] = json!(if completed {
        "PASS_CONTAINER_MEMORY_PROFILE_CUSTODY_AND_COUNTERS"
    } else {
        "PREFIX_ONLY_MISSING_TERMINAL_CUSTODY_NOT_ADMITTED"
    });
    result["profile_blake3"] = json!(digest);
    result["dir"] = json!(dir);
    result["search_admission"] =
        json!("separate native audit required; memory validation grants no search admission");
    write(output, &result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn row(ms: f64, current: u64, peak: u64) -> Value {
        json!({"schema":"n83.memory-sample/v1","elapsed_ms":ms,"pid":10,"cgroup_path":"/sys/fs/cgroup/system.slice/docker-test.scope",
            "memory.current":{"raw":current.to_string()},"memory.peak":{"raw":peak.to_string()},
            "memory.max":{"raw":"10000"},"memory.swap.max":{"raw":"0"},
            "memory.stat":{"raw":"anon 100\nfile 200"},"memory.events":{"raw":"oom 0\noom_kill 0\nmax 0\nhigh 0"},
            "process_status":["VmRSS: 2 kB","VmHWM: 3 kB"]})
    }
    fn run(a: &Value, b: &Value) -> R<Value> {
        validate(&format!("{a}\n{b}\n"), 10, 10000, Some(100000.))
    }
    #[test]
    fn accepts_reclaim_and_reports_sampled_peak_scope() {
        let v = run(&row(1., 500, 600), &row(60002., 400, 700)).unwrap();
        assert_eq!(v["max_sampled_current_bytes"], 500);
        assert_eq!(v["max_sampled_kernel_peak_bytes"], 700);
        assert_eq!(v["unsampled_tail_ms"], 39998.);
    }
    #[test]
    fn rejects_order_pid_path_limit_and_counter_corruption() {
        let a = row(1., 500, 600);
        for (key, value) in [
            ("pid", json!(11)),
            ("elapsed_ms", json!(0)),
            ("elapsed_ms", json!(60000)),
            ("cgroup_path", json!("/sys/fs/cgroup/../other")),
            ("cgroup_path", json!("/sys/fs/cgroup/other")),
            ("memory.max", json!({"raw":"20000"})),
            ("memory.swap.max", json!({"raw":"1"})),
            ("memory.current", json!({"raw":"800"})),
            ("memory.current", json!({"raw":"-1"})),
            ("memory.peak", json!({"raw":"599"})),
            ("memory.stat", json!({"raw":"anon 1\nanon 2\nfile 3"})),
            ("process_error", json!("gone")),
            ("cgroup_error", json!("gone")),
        ] {
            let mut b = row(60002., 400, 700);
            b[key] = value;
            assert!(run(&a, &b).is_err(), "accepted {key}: {b}");
        }
        assert!(validate("", 10, 10000, None).is_err());
    }
}
