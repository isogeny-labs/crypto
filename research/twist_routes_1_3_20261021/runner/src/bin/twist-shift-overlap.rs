//! Exact all-shift character overlap; independent square-table point census.
//! No DLP solver, timing claim, target selection or solver-cost calculation.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    Ok(fs::write(p, serde_json::to_string_pretty(v)? + "\n")?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn fresh(p: &Path) -> Result<()> {
    if p.exists() {
        return Err("fresh output directory required".into());
    }
    fs::create_dir_all(p)?;
    Ok(())
}
fn fixtures(input: &Path) -> Result<Vec<Value>> {
    let plan = read(input)?;
    let mut f = BTreeMap::new();
    for row in plan["cells"].as_array().ok_or("input cells missing")? {
        let x = &row["task"]["fixture"];
        let id = x["id"].as_str().ok_or("fixture identity missing")?;
        if let Some(old) = f.insert(id.to_owned(), x.clone()) {
            if old != *x {
                return Err("fixture identity collision".into());
            }
        }
    }
    if f.len() != 10 {
        return Err("exactly ten original fixtures required".into());
    }
    Ok(f.into_values().collect())
}
fn number(f: &Value, k: &str) -> u64 {
    f[k].as_u64().unwrap()
}
fn pow(mut x: u64, mut e: u64, p: u64) -> u64 {
    let mut y = 1;
    while e > 0 {
        if e & 1 != 0 {
            y = y * x % p;
        }
        x = x * x % p;
        e >>= 1;
    }
    y
}
fn cubic(x: u64, a: u64, b: u64, p: u64) -> u64 {
    (x * x % p * x % p + a * x + b) % p
}
fn produce(input: &Path, out: &Path) -> Result<()> {
    fresh(out)?;
    let mut records = vec![];
    let mut shifts = 0usize;
    for f in fixtures(input)? {
        let (p, a, b) = (number(&f, "p"), number(&f, "a"), number(&f, "b"));
        let mut signs = vec![];
        for u in 0..p {
            let t = pow(cubic(u, a, b, p), (p - 1) / 2, p);
            if t != 1 && t != p - 1 {
                return Err("zero cubic or nonprime fixture".into());
            }
            signs.push(if t == 1 { 1i64 } else { -1 });
        }
        let mut rows = vec![];
        for c in 0..p {
            let (mut correlation, mut mixed, mut ordinary) = (0i64, 0u64, 0u64);
            for u in 0..p {
                let x = signs[u as usize];
                let y = signs[((u + c) % p) as usize];
                correlation += x * y;
                mixed += u64::from(x == -1 && y == 1);
                ordinary += u64::from(x == 1 && y == 1);
            }
            rows.push(json!({"shift":c,"mixed":mixed,"ordinary":ordinary,"correlation":correlation,"genus_two_points_from_identity":p as i64-correlation,"smooth_genus_two_claim":c!=0}));
        }
        shifts += rows.len();
        let row = json!({"fixture":f,"rows":rows});
        let name = format!("{}.json", row["fixture"]["id"].as_str().unwrap());
        write(out.join(&name), &row)?;
        records.push(json!({"path":name,"sha256":sha(out.join(&name))?,"shifts":p}));
    }
    write(
        out.join("RECORDS.json"),
        &json!({"schema":"twist.shift-overlap/v1","fixtures":records,"all_shifts_including_zero":shifts,"input_sha256":sha(input)?,"performance_claim":false}),
    )
}

// Verifier does not call producer cubic, Euler/pow, or sign-array code.
fn prime(p: u64) -> bool {
    p > 3 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn value(x: u64, f: &Value) -> u64 {
    let p = number(f, "p");
    let square = x * x % p;
    ((square + number(f, "a")) % p * x % p + number(f, "b")) % p
}
fn square_counts(p: u64) -> Vec<u64> {
    let mut n = vec![0; p as usize];
    for y in 0..p {
        n[(y * y % p) as usize] += 1;
    }
    n
}
fn inverse(x: u64, p: u64) -> Result<u64> {
    (1..p)
        .find(|y| x * y % p == 1)
        .ok_or_else(|| "inverse absent".into())
}
fn trim(v: &mut Vec<u64>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}
fn remainder(mut a: Vec<u64>, b: &[u64], p: u64) -> Result<Vec<u64>> {
    if b.is_empty() {
        return Err("zero polynomial denominator".into());
    }
    let inv = inverse(*b.last().unwrap(), p)?;
    trim(&mut a);
    while a.len() >= b.len() {
        let k = a.len() - b.len();
        let factor = a.last().unwrap() * inv % p;
        for (i, y) in b.iter().enumerate() {
            a[k + i] = (a[k + i] + p - factor * y % p) % p;
        }
        trim(&mut a);
    }
    Ok(a)
}
fn gcd(mut a: Vec<u64>, mut b: Vec<u64>, p: u64) -> Result<Vec<u64>> {
    while !b.is_empty() {
        let r = remainder(a, &b, p)?;
        a = b;
        b = r;
    }
    Ok(a)
}
fn shifted_cubic(f: &Value, c: u64) -> Vec<u64> {
    let p = number(f, "p");
    vec![value(c, f), (3 * c * c + number(f, "a")) % p, 3 * c % p, 1]
}
fn independently_count(f: &Value, c: u64, n: &[u64], di: u64) -> (u64, u64, u64) {
    let p = number(f, "p");
    let (mut mixed, mut ordinary, mut points) = (0, 0, 0);
    for u in 0..p {
        let l = value(u, f);
        let r = value((u + c) % p, f);
        mixed += u64::from(n[l as usize] == 0 && n[r as usize] == 2);
        ordinary += u64::from(n[l as usize] == 2 && n[r as usize] == 2);
        points += n[(l * r % p * di % p) as usize];
    }
    // Degree six, nonsquare leading coefficient: no rational points at infinity.
    (mixed, ordinary, points)
}
fn verify(input: &Path, raw: &Path, out: &Path) -> Result<()> {
    fresh(out)?;
    let manifest = read(raw.join("RECORDS.json"))?;
    if manifest["input_sha256"] != sha(input)? {
        return Err("input hash mismatch".into());
    }
    let mut summary = vec![];
    let (mut all, mut smooth, mut false_plus_one) = (0usize, 0usize, 0usize);
    let mut gp = String::from("default(parisizemax,4000000000);\n");
    for (f, record) in fixtures(input)?
        .iter()
        .zip(manifest["fixtures"].as_array().ok_or("records missing")?)
    {
        let path = raw.join(record["path"].as_str().ok_or("raw path missing")?);
        if record["sha256"] != sha(&path)? {
            return Err("raw hash mismatch".into());
        }
        let raw = read(&path)?;
        if raw["fixture"] != *f {
            return Err("fixture mismatch".into());
        }
        let (p, d, order) = (number(f, "p"), number(f, "d"), number(f, "order"));
        if !prime(p) || !prime(order) || order % 2 != 1 || p > 20000 {
            return Err("fixture hypotheses failed".into());
        }
        let n = square_counts(p);
        if n[d as usize] != 0 {
            return Err("d not nonsquare".into());
        }
        let di = inverse(d, p)?;
        if (0..p).any(|x| value(x, f) == 0)
            || 1 + (0..p).map(|x| n[value(x, f) as usize]).sum::<u64>() != order
        {
            return Err("source cardinality/no-two-torsion gate".into());
        }
        let rows = raw["rows"].as_array().ok_or("shift rows missing")?;
        if rows.len() != p as usize {
            return Err("all shifts required".into());
        }
        let (mut lo, mut hi, mut sum, mut bounds, mut zero) = (u64::MAX, 0, 0u64, 0usize, 0usize);
        let base = vec![number(f, "b"), number(f, "a"), 0, 1];
        if gcd(base.clone(), vec![number(f, "a"), 0, 3], p)?.len() != 1 {
            return Err("singular source".into());
        }
        for (c, row) in rows.iter().enumerate() {
            let c = c as u64;
            if row["shift"] != c {
                return Err("shift order mismatch".into());
            }
            let (mixed, ordinary, points) = independently_count(f, c, &n, di);
            let correlation = row["correlation"].as_i64().ok_or("correlation missing")?;
            if row["mixed"] != mixed
                || row["ordinary"] != ordinary
                || row["genus_two_points_from_identity"] != points
                || 4 * mixed != points
                || p as i64 - correlation != points as i64
                || mixed + ordinary != (order - 1) / 2
            {
                return Err(format!("finite overlap mismatch {} c={c}", f["id"]).into());
            }
            all += 1;
            sum += mixed;
            false_plus_one += usize::from(4 * (mixed + 1) != points);
            if c == 0 {
                if mixed != 0 || row["smooth_genus_two_claim"] != false {
                    return Err("zero-shift negative control failed".into());
                }
                zero += 1;
                continue;
            }
            if gcd(base.clone(), shifted_cubic(f, c), p)?.len() != 1
                || row["smooth_genus_two_claim"] != true
            {
                return Err("nonsingular genus-two gate failed".into());
            }
            smooth += 1;
            let delta = 4 * mixed as i64 - (p + 1) as i64;
            if delta * delta > 16 * p as i64 {
                return Err("integer Weil-bound check failed".into());
            }
            bounds += 1;
            lo = lo.min(mixed);
            hi = hi.max(mixed);
        }
        let positive = (order - 1) / 2;
        let negative = p - positive;
        if sum != positive * negative {
            return Err("all-shift conservation check failed".into());
        }
        let id = f["id"].as_str().unwrap();
        let trace = (p + 1) as i64 - order as i64;
        for c in [1u64, 3] {
            let expected = rows[c as usize]["genus_two_points_from_identity"]
                .as_u64()
                .unwrap();
            gp+=&format!("f=Mod(1,{p})*(x^3+{}*x+{});P=hyperellcharpoly(f*subst(f,x,x+{c})/Mod({d},{p}));Q=x^2-({trace})*x+{p};Qt=x^2+({trace})*x+{p};if({p}+1+polcoef(P,3)!={expected},error(\"point mismatch\"));print(\"OVERLAP\\t{id}\\t{c}\\t\",{p}+1+polcoef(P,3),\"\\t\",polrem(P,Q)==0,\"\\t\",polrem(P,Qt)==0,\"\\t\",vector(5,i,polcoef(P,i-1)));\n",number(f,"a"),number(f,"b"));
        }
        let cover_points = 2 + (0..p).map(|x| n[value(x * x % p, f) as usize]).sum::<u64>();
        gp+=&format!("P=hyperellcharpoly(Mod(1,{p})*(x^6+{}*x^2+{}));Q=x^2-({trace})*x+{p};if(polrem(P,Q)!=0||{p}+1+polcoef(P,3)!={cover_points},error(\"positive cover failed\"));print(\"POSITIVE\\t{id}\\t0\\t\",{p}+1+polcoef(P,3),\"\\t\",polrem(P,Q)==0,\"\\t\",polrem(P+1,Q)!=0,\"\\t\",vector(5,i,polcoef(P,i-1)));\n",number(f,"a"),number(f,"b"));
        summary.push(json!({"fixture":id,"p":p,"source_order":order,"twist_order":2*(p+1)-order,"full_twist_x_support":negative,"nonzero_shifts":p-1,"mixed_min":lo,"mixed_max":hi,"mixed_sum_over_all_shifts":sum,"zero_shift_controls":zero,"integer_Weil_checks":bounds,"positive_cover_points":cover_points}));
    }
    if summary.len() != 10 || manifest["fixtures"].as_array().unwrap().len() != 10 {
        return Err("fixture count mismatch".into());
    }
    gp += "print(\"ALL_SHIFT_OVERLAP_GP_OK 20 10\");\n";
    fs::write(out.join("verify.gp"), gp)?;
    write(
        out.join("AUDIT.json"),
        &json!({"schema":"twist.shift-overlap-audit/v1","status":"all-shift-independent-square-table-census-verified","all_shifts":all,"smooth_nonzero_shifts":smooth,"false_plus_one_rejected":false_plus_one,"by_fixture":summary,"records_sha256":sha(raw.join("RECORDS.json"))?,"input_sha256":sha(input)?,"gp_script_sha256":sha(out.join("verify.gp"))?,"PARI_admitted":false,"complete_Legendre_genus_Weil_Lean_bridge":false,"performance_claim":false,"novelty_claim":false}),
    )
}
fn divides(a: &[i128], q: [i128; 3]) -> bool {
    let mut r = a.to_vec();
    for j in (2..r.len()).rev() {
        let t = r[j];
        for k in 0..=2 {
            r[j - 2 + k] -= t * q[k];
        }
    }
    r[..2].iter().all(|x| *x == 0)
}
fn admit(input: &Path, audit: &Path, receipt_path: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh admission path required".into());
    }
    let receipt = read(receipt_path)?;
    let a = read(audit.join("AUDIT.json"))?;
    if receipt["exit_code"] != 0
        || !receipt["signal"].is_null()
        || receipt["script_sha256"] != sha(audit.join("verify.gp"))?
    {
        return Err("PARI transport rejected".into());
    }
    let stdout_path = receipt["stdout"].as_str().ok_or("stdout path missing")?;
    let stderr_path = receipt["stderr"].as_str().ok_or("stderr path missing")?;
    if receipt["stdout_sha256"] != sha(stdout_path)?
        || receipt["stderr_sha256"] != sha(stderr_path)?
    {
        return Err("PARI output hash mismatch".into());
    }
    let err = fs::read_to_string(stderr_path)?;
    if !err.lines().all(|l| {
        l.trim().is_empty()
            || l.trim() == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes)."
    }) {
        return Err(format!("PARI stderr rejected: {err}").into());
    }
    let fs = fixtures(input)?
        .into_iter()
        .map(|f| (f["id"].as_str().unwrap().to_owned(), f))
        .collect::<BTreeMap<_, _>>();
    let mut seen = std::collections::BTreeSet::new();
    let (mut covers, mut tested, mut source_factors, mut twist_factors) = (0, 0, 0, 0);
    let mut rows = vec![];
    let stdout = std::fs::read_to_string(stdout_path)?;
    let mut marker = 0;
    for line in stdout.lines() {
        if line == "ALL_SHIFT_OVERLAP_GP_OK 20 10" {
            marker += 1;
            continue;
        }
        let parts = line.split('\t').collect::<Vec<_>>();
        if parts.len() != 7 {
            return Err("malformed PARI row".into());
        }
        let kind = parts[0];
        let f = fs.get(parts[1]).ok_or("unrecognized fixture")?;
        let c: u64 = parts[2].parse()?;
        let points: u64 = parts[3].parse()?;
        let polynomial: Vec<i128> = serde_json::from_str(parts[6])?;
        if polynomial.len() != 5
            || polynomial[4] != 1
            || !seen.insert((kind.to_owned(), parts[1].to_owned(), c))
        {
            return Err("bad polynomial/duplicate GP row".into());
        }
        let p = number(f, "p") as i128;
        let t = p + 1 - number(f, "order") as i128;
        let sf = divides(&polynomial, [p, -t, 1]);
        let tf = divides(&polynomial, [p, t, 1]);
        if points as i128 != p + 1 + polynomial[3] || parts[4] != if sf { "1" } else { "0" } {
            return Err("PARI coefficient/factor mismatch".into());
        }
        if kind == "OVERLAP" {
            if ![1, 3].contains(&c) || parts[5] != if tf { "1" } else { "0" } {
                return Err("wrong overlap factor row".into());
            }
            let n = square_counts(p as u64);
            let (_, _, expected) =
                independently_count(f, c, &n, inverse(number(f, "d"), p as u64)?);
            if points != expected {
                return Err("PARI point count mismatch".into());
            }
            tested += 1;
            source_factors += usize::from(sf);
            twist_factors += usize::from(tf);
        } else if kind == "POSITIVE" {
            let mut wrong = polynomial.clone();
            wrong[0] += 1;
            if c != 0 || !sf || parts[5] != "1" || divides(&wrong, [p, -t, 1]) {
                return Err("positive/negative factor detector failed".into());
            }
            covers += 1;
        } else {
            return Err("unrecognized PARI row kind".into());
        }
        rows.push(json!({"kind":kind,"fixture":parts[1],"shift":c,"points":points,"polynomial_low_to_high":polynomial,"source_factor":sf,"twist_factor":tf}));
    }
    if marker != 1
        || tested != 20
        || covers != 10
        || a["status"] != "all-shift-independent-square-table-census-verified"
        || a["input_sha256"] != sha(input)?
    {
        return Err("incomplete PARI/audit gates".into());
    }
    write(
        out,
        &json!({"schema":"twist.shift-overlap-admission/v1","status":"independent-all-shift-and-PARI-polynomials-verified","all_shifts":a["all_shifts"],"smooth_nonzero_shifts":a["smooth_nonzero_shifts"],"genus_two_curves_tested":tested,"positive_cover_controls":covers,"false_polynomial_controls":covers,"overlap_source_factor_count":source_factors,"overlap_twist_factor_count":twist_factors,"rows":rows,"audit_sha256":sha(audit.join("AUDIT.json"))?,"receipt_sha256":sha(receipt_path)?,"no_DLP_transfer_or_cost_claim":true}),
    )
}
fn export(v: &Path, output: &Path) -> Result<()> {
    if output.exists() {
        return Err("fresh export path required".into());
    }
    let root = fs::canonicalize(output.parent().ok_or("export parent absent")?)?;
    let file = |p: &Path, kind: &str| -> Result<Value> {
        let absolute = fs::canonicalize(p)?;
        let a = absolute.components().collect::<Vec<_>>();
        let b = root.components().collect::<Vec<_>>();
        let common = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
        let mut rel = std::path::PathBuf::new();
        for _ in common..b.len() {
            rel.push("..");
        }
        for x in &a[common..] {
            rel.push(x.as_os_str());
        }
        Ok(json!({"path":rel,"sha256":sha(p)?,"bytes":fs::metadata(p)?.len(),"kind":kind}))
    };
    let pp = v.join("protocol.json");
    let protocol = read(&pp)?;
    let frozen = read(v.join("FROZEN.json"))?;
    let admission_path = v.join("ADMISSION.json");
    let admission = read(&admission_path)?;
    if admission["status"] != "independent-all-shift-and-PARI-polynomials-verified" {
        return Err("admission missing".into());
    }
    let hp = v.join("HOST.json");
    let host = read(&hp)?;
    let source =
        Path::new("research/twist_routes_1_3_20261021/runner/src/bin/twist-shift-overlap.rs");
    let files = vec![
        file(&v.join("runs.tar.xz"), "raw")?,
        file(&v.join("RAW_MEMBERS.json"), "receipt")?,
        file(&admission_path, "receipt")?,
    ];
    let producer = read(v.join("runs/producer-transport.json"))?;
    let mut data = json!({"schema":"experiment.export/v1","export_id":"twist-shift-overlap-20261027-v1","created":std::process::Command::new("date").args(["-u","+%Y-%m-%dT%H:%M:%SZ"]).output().map(|x|String::from_utf8_lossy(&x.stdout).trim().to_owned())?,
        "protocol":{"path":"protocol.json","sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},
        "producer":{"runtime":"codex","model":"GPT-6","session":"twist-shift-overlap-20261027","role":"research-and-implementation"},
        "source":{"repo":"crypto","commit":frozen["source_revision"],"dirty":false},"backend":"local",
        "host":{"path":"HOST.json","sha256":sha(&hp)?,"class_id":null,"summary":host["summary"]},
        "arms":protocol["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect::<Vec<_>>(),
        "runs":[{"run_id":"all-shifts-000","arm":"all-shifts","instance_id":"original-ten-fixtures-all-shifts","seed":0,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,
            "checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},
            "measurements":{"counted":{"checked_shifts":admission["all_shifts"]},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,
                "counters":{"smooth_nonzero_shifts":admission["smooth_nonzero_shifts"],"GP_overlap_polynomials":admission["genus_two_curves_tested"],"positive_covers":admission["positive_cover_controls"],"false_polynomials_rejected":admission["false_polynomial_controls"]}},
            "isolation":{"tier":"L0","contended":null,"blockers":["Exact finite census only; no timing or performance claim."]},
            "verification":{"status":"verified","verifier":"independent square-table point census and strict PARI Frobenius-polynomial replay"},"exclusion":{"excluded":false,"rule":null},
            "artifacts":files,"started_at":producer["started"],"finished_at":producer["finished"],"worker":frozen["binary"]["sha256"],"backend_ref":"runs/overlap-v1"}],
        "flow":{"planned":2,"attempted":2,"completed":2,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},
        "files":[file(&pp,"protocol")?,file(&hp,"host")?,file(&v.join("FROZEN.json"),"receipt")?],
        "analysis":{"script":file(source,"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&v.join("overlap-audit-v1/AUDIT.json"),"analysis")?,file(&admission_path,"receipt")?],"unmasked_at":null},
        "notes":[{"kind":"observation","text":"One producer and one separate independent-verifier execution process the same complete fixed panel; do not pool their shift counts as independent targets. Positive/negative checks are embedded, not fabricated separate executions. Shared author/host; Legendre/genus/Weil Lean bridge and DLP transfer/cost remain unestablished.","by":"native exporter"}],"reviews_dir":"reviews"});
    let mut verifier = data["runs"][0].clone();
    let transport = read(v.join("runs/verifier-transport.json"))?;
    verifier["run_id"] = json!("all-shifts-verifier-000");
    verifier["arm"] = json!("independent");
    verifier["started_at"] = transport["started"].clone();
    verifier["finished_at"] = transport["finished"].clone();
    verifier["backend_ref"] = json!("overlap-audit-v1");
    data["runs"].as_array_mut().unwrap().push(verifier);
    write(output, &data)
}
fn transport(receipt: &Path, args: &[String]) -> Result<()> {
    use std::os::unix::process::ExitStatusExt;
    use std::process::{Command, Stdio};
    if receipt.exists() || !matches!(args.first().map(String::as_str), Some("produce" | "verify")) {
        return Err("fresh receipt and produce/verify command required".into());
    }
    fs::create_dir_all(receipt.parent().ok_or("receipt parent absent")?)?;
    let stdout = receipt.with_extension("stdout");
    let stderr = receipt.with_extension("stderr");
    if stdout.exists() || stderr.exists() {
        return Err("fresh transport output paths required".into());
    }
    let program = std::env::current_exe()?;
    let stamp = || -> Result<String> {
        Ok(String::from_utf8(
            Command::new("date")
                .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
                .output()?
                .stdout,
        )?
        .trim()
        .to_owned())
    };
    let mut argv = vec![
        "-c".to_owned(),
        "ulimit -c 0; ulimit -v 4194304; ulimit -f 131072; exec \"$@\"".into(),
        "shift-overlap".into(),
        "/usr/bin/timeout".into(),
        "--signal=TERM".into(),
        "--kill-after=1s".into(),
        "600s".into(),
        program.to_string_lossy().into_owned(),
    ];
    argv.extend_from_slice(args);
    let started = stamp()?;
    let status = Command::new("/bin/sh")
        .args(&argv)
        .env("RAYON_NUM_THREADS", "1")
        .env("OMP_NUM_THREADS", "1")
        .stdout(Stdio::from(fs::File::create(&stdout)?))
        .stderr(Stdio::from(fs::File::create(&stderr)?))
        .status()?;
    write(
        receipt,
        &json!({"schema":"twist.exact-stage-transport/v1","argv":([vec!["/bin/sh".to_owned()],argv].concat()),"started":started,"finished":stamp()?,"exit_code":status.code(),"signal":status.signal(),"binary_sha256":sha(&program)?,"stdout":stdout,"stderr":stderr,"stdout_sha256":sha(&stdout)?,"stderr_sha256":sha(&stderr)?,"wall_time_admitted":false,"address_space_bytes":4294967296u64,"file_limit_bytes":67108864u64,"timeout_seconds":600,"environment_overrides":{"RAYON_NUM_THREADS":"1","OMP_NUM_THREADS":"1"}}),
    )?;
    if !status.success() {
        return Err("bounded stage failed; receipt retained".into());
    }
    Ok(())
}
fn main() -> Result<()> {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    match a.first().map(String::as_str) {
        Some("produce") if a.len() == 3 => produce(Path::new(&a[1]), Path::new(&a[2])),
        Some("verify") if a.len() == 4 => {
            verify(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]))
        }
        Some("admit") if a.len() == 5 => admit(
            Path::new(&a[1]),
            Path::new(&a[2]),
            Path::new(&a[3]),
            Path::new(&a[4]),
        ),
        Some("export") if a.len() == 3 => export(Path::new(&a[1]), Path::new(&a[2])),
        Some("transport") if a.len() >= 4 => transport(Path::new(&a[1]), &a[2..]),
        _ => Err(
            "usage: produce INPUT OUT | verify INPUT RAW OUT | admit INPUT AUDIT RECEIPT OUT.json"
                .into(),
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_small_curve_and_zero_shift() {
        let f = json!({"p":7,"a":1,"b":1,"order":5,"d":3});
        let n = square_counts(7);
        for c in 0..7 {
            let (m, o, pts) = independently_count(&f, c, &n, inverse(3, 7).unwrap());
            let correlation = (0..7)
                .map(|x| {
                    let l = pow(cubic(x, 1, 1, 7), 3, 7);
                    let r = pow(cubic((x + c) % 7, 1, 1, 7), 3, 7);
                    if l == r {
                        1i64
                    } else {
                        -1
                    }
                })
                .sum::<i64>();
            assert_eq!(4 * m, pts);
            assert_eq!(pts as i64, 7 - correlation);
            assert_eq!(m + o, 2);
            if c == 0 {
                assert_eq!(m, 0);
            } else {
                assert_eq!(
                    gcd(vec![1, 1, 0, 1], shifted_cubic(&f, c), 7)
                        .unwrap()
                        .len(),
                    1
                );
            }
        }
    }
    #[test]
    fn factor_detector_rejects_changed_constant() {
        let p = vec![6i128, 7, 7, 3, 1];
        assert!(divides(&p, [2, 1, 1]));
        let mut bad = p;
        bad[0] += 1;
        assert!(!divides(&bad, [2, 1, 1]));
    }
}
