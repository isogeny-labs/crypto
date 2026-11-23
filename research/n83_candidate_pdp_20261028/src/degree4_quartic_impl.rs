use std::collections::HashMap;

#[derive(Clone, Copy)]
struct QuarticLimits {
    rows: usize,
    bytes: usize,
    attempts: usize,
    weight: usize,
}
const QUARTIC_LIMITS: QuarticLimits = QuarticLimits {
    rows: 16384,
    bytes: 2usize << 30,
    attempts: 131072,
    weight: 1048576,
};
fn choose4(n: usize) -> usize {
    n * n.saturating_sub(1) * n.saturating_sub(2) * n.saturating_sub(3) / 24
}
struct QuarticSpace {
    low: Space,
    cols: usize,
    low_monomials: Vec<[u8; 3]>,
}
impl QuarticSpace {
    fn new(n: usize) -> Self {
        let low = Space::new(n, 3);
        let mut codes = vec![[0; 3]];
        for i in 0..n {
            codes.push([(i + 1) as u8, 0, 0]);
        }
        for j in 1..n {
            for i in 0..j {
                codes.push([(i + 1) as u8, (j + 1) as u8, 0]);
            }
        }
        for k in 2..n {
            for j in 1..k {
                for i in 0..j {
                    codes.push([(i + 1) as u8, (j + 1) as u8, (k + 1) as u8]);
                }
            }
        }
        assert_eq!(codes.len(), low.cols);
        let cols = low.cols + choose4(n);
        assert!(cols <= u32::MAX as usize);
        Self {
            low,
            cols,
            low_monomials: codes,
        }
    }
    fn col(&self, m: &[usize]) -> usize {
        if m.len() <= 3 {
            return self.low.col(m);
        }
        assert_eq!(m.len(), 4);
        assert!(m.windows(2).all(|w| w[0] < w[1]) && m[3] < self.low.n);
        self.low.cols + choose4(m[3]) + choose3(m[2]) + choose2(m[1]) + m[0]
    }
    fn multiply(&self, source: &QuarticRow, v: usize) -> QuarticRow {
        assert!(
            v < self.low.n && source.high.is_empty(),
            "ordinary multiplication exceeds degree4"
        );
        let mut low = vec![0; self.low.words];
        let mut high = Vec::new();
        for (word, &value) in source.low.iter().enumerate() {
            let mut bits = value;
            while bits != 0 {
                let c = word * 64 + bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let code = self.low_monomials[c];
                let mut m = [0usize; 4];
                let mut len = 0;
                for x in code.into_iter().filter(|x| *x != 0) {
                    m[len] = (x - 1) as usize;
                    len += 1;
                }
                if let Err(at) = m[..len].binary_search(&v) {
                    m.copy_within(at..len, at + 1);
                    m[at] = v;
                    len += 1;
                }
                let at = self.col(&m[..len]);
                if at < self.low.cols {
                    low[at / 64] ^= 1u64 << (at % 64);
                } else {
                    high.push(at as u32);
                }
            }
        }
        high.sort_unstable();
        let mut unique = Vec::with_capacity(high.len());
        let mut i = 0;
        while i < high.len() {
            let mut j = i + 1;
            while j < high.len() && high[j] == high[i] {
                j += 1;
            }
            if (j - i) % 2 == 1 {
                unique.push(high[i]);
            }
            i = j;
        }
        QuarticRow { low, high: unique }
    }
    fn row(&self, p: &Poly) -> QuarticRow {
        let mut row = QuarticRow {
            low: vec![0; self.low.words],
            high: vec![],
        };
        for m in p {
            let c = self.col(m);
            if c < self.low.cols {
                row.low[c / 64] ^= 1u64 << (c % 64);
            } else {
                row.high.push(c as u32);
            }
        }
        row.high.sort_unstable();
        row
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct QuarticRow {
    low: Vec<u64>,
    high: Vec<u32>,
}
impl QuarticRow {
    fn pivot(&self) -> Option<usize> {
        self.high
            .last()
            .map(|x| *x as usize)
            .or_else(|| pivot(&self.low))
    }
    fn degree(&self, sp: &QuarticSpace) -> usize {
        if self.high.is_empty() {
            sp.low.deg(&self.low)
        } else {
            4
        }
    }
    fn value_bytes(&self) -> Vec<u8> {
        let mut b = (self.high.len() as u64).to_le_bytes().to_vec();
        b.extend(row_bytes(&self.low));
        for x in &self.high {
            b.extend(x.to_le_bytes());
        }
        b
    }
    fn record(&self, id: u64) -> Vec<u8> {
        let mut b = id.to_le_bytes().to_vec();
        b.extend(self.value_bytes());
        b
    }
    fn record_len(&self) -> usize {
        16 + self.low.len() * 8 + self.high.len() * 4
    }
    fn xor_assign(&mut self, other: &Self) {
        self.high = quartic_xor_sorted(&self.high, &other.high);
        xor(&mut self.low, &other.low);
    }
}
// Sorted symmetric difference adapted from src/cryptanalysis/sparse_macaulay.rs,
// SHA256 9f794a1e73774dcce0fa24d4468d73a2500d939858857700cfc9fcfe30ab4efa.
fn quartic_xor_sorted(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                i += 1;
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct QuarticQueue {
    groups: Vec<Vec<usize>>,
    group: usize,
    variable: usize,
    index: usize,
    urgent: VecDeque<(usize, usize)>,
    n: usize,
}
impl QuarticQueue {
    fn new(rows: &[QuarticRow], old_v3_rows: usize, sp: &QuarticSpace) -> Self {
        let added = (old_v3_rows..rows.len())
            .filter(|i| rows[*i].degree(sp) <= 3)
            .collect();
        let old = (0..old_v3_rows)
            .filter(|i| rows[*i].degree(sp) == 3)
            .collect();
        let mut q = Self {
            groups: vec![added, old],
            group: 0,
            variable: 0,
            index: 0,
            urgent: VecDeque::new(),
            n: sp.low.n,
        };
        q.normalize();
        q
    }
    fn normalize(&mut self) {
        while self.group < self.groups.len() {
            if self.groups[self.group].is_empty() || self.variable == self.n {
                self.group += 1;
                self.variable = 0;
                self.index = 0;
            } else if self.index == self.groups[self.group].len() {
                self.index = 0;
                self.variable += 1;
            } else {
                break;
            }
        }
    }
    fn peek(&self) -> Option<(usize, usize, &'static str)> {
        if let Some(&(row, v)) = self.urgent.front() {
            return Some((row, v, "new_consequence"));
        }
        if self.group == self.groups.len() {
            return None;
        }
        Some((
            self.groups[self.group][self.index],
            self.variable,
            if self.group == 0 {
                "initial_added"
            } else {
                "initial_v3"
            },
        ))
    }
    fn advance(&mut self) {
        if let Some(front) = self.urgent.front_mut() {
            front.1 += 1;
            if front.1 == self.n {
                self.urgent.pop_front();
            }
        } else {
            assert!(self.peek().is_some());
            self.index += 1;
            self.normalize();
        }
    }
    fn pending(&self) -> usize {
        let urgent: usize = self.urgent.iter().map(|(_, v)| self.n - v).sum();
        if self.group == self.groups.len() {
            return urgent;
        }
        urgent + (self.n - self.variable) * self.groups[self.group].len() - self.index
            + self.groups[self.group + 1..]
                .iter()
                .map(|g| self.n * g.len())
                .sum::<usize>()
    }
    fn json(&self) -> Value {
        json!({"schema":"n83.quartic-product-queue/v1","initial_groups":self.groups,"group":self.group,"variable":self.variable,"index":self.index,"urgent_rows_next_variables":self.urgent,"variables":self.n,"pending_products":self.pending()})
    }
}

fn quartic_header(sp: &QuarticSpace, initial: usize) -> Vec<u8> {
    let mut b = b"N83Q4H1\n".to_vec();
    b.extend((sp.low.n as u32).to_le_bytes());
    for x in [sp.low.cols, sp.cols, sp.low.words, initial] {
        b.extend((x as u64).to_le_bytes());
    }
    assert_eq!(b.len(), 44);
    b
}

fn quartic_affine(rows: &[QuarticRow], sp: &QuarticSpace, polys: &[(usize, Poly)]) -> Value {
    let ids: Vec<_> = (0..rows.len())
        .filter(|i| rows[*i].degree(sp) <= 1)
        .collect();
    let low: Vec<_> = ids.iter().map(|i| rows[*i].low.clone()).collect();
    let mut a = propagation_affine(&low, &sp.low, polys);
    if let Some(i) = a["unit_row_id"].as_u64() {
        a["unit_row_id"] = json!(ids[i as usize]);
    }
    a["linear_basis_row_ids"] = json!(a["linear_basis_row_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| ids[v.as_u64().unwrap() as usize])
        .collect::<Vec<_>>());
    a
}

// Preconditions are established by replaying the immutable complete-V3 and
// V4-membership parents. The first old_v3_rows rows form complete V3.
fn quartic_closure(
    input: &Value,
    text: &str,
    parent: Vec<Vec<u64>>,
    old_v3_rows: usize,
    out: &Path,
    limits: QuarticLimits,
    inherited_lower: Option<usize>,
    replay: bool,
) -> (Value, Vec<QuarticRow>) {
    let n = input["n_vars"].as_u64().unwrap() as usize;
    let sp = QuarticSpace::new(n);
    let polys = equations(text, n);
    let mut rows: Vec<_> = parent
        .into_iter()
        .map(|low| QuarticRow { low, high: vec![] })
        .collect();
    let initial = rows.len();
    assert!(old_v3_rows <= initial);
    assert!(inherited_lower.is_none() || inherited_lower == Some(4));
    let mut low_map = vec![None; sp.low.cols];
    let mut high_map: HashMap<usize, usize> = HashMap::new();
    for (i, r) in rows.iter().enumerate() {
        assert_eq!(r.low.len(), sp.low.words);
        if sp.low.cols % 64 != 0 {
            assert_eq!(r.low[sp.low.words - 1] >> (sp.low.cols % 64), 0);
        }
        let p = r.pivot().expect("parent cannot contain zero rows");
        assert!(low_map[p].replace(i).is_none(), "duplicate parent pivot");
    }
    for (_, p) in &polys {
        let mut r = sp.row(p);
        assert!(
            r.high.is_empty(),
            "this continuation requires input degree <=3"
        );
        while let Some(p) = r.pivot() {
            let i = low_map[p].expect("input missing from verified parent span");
            xor(&mut r.low[..=p / 64], &rows[i].low[..=p / 64]);
        }
    }
    let mut queue = QuarticQueue::new(&rows, old_v3_rows, &sp);
    if !replay {
        fs::create_dir(out).unwrap();
    }
    let mut data = WitnessSink::new(&out.join("added.rows.hybrid"), replay);
    let mut blocked_data = WitnessSink::new(&out.join("blocked.row.hybrid"), replay);
    let mut ops = WitnessSink::new(&out.join("operations.jsonl"), replay);
    let mut progress = WitnessSink::new(&out.join("progress.jsonl"), replay);
    data.emit(&quartic_header(&sp, initial));
    blocked_data.emit(&quartic_header(&sp, initial));
    let (mut attempts, mut completed, mut vanished, mut payload) = (0usize, 0usize, 0usize, 0usize);
    let mut by_degree = [0usize; 5];
    let mut peak_weight = 0usize;
    let mut blocked = None;
    let reason = loop {
        if low_map[0].is_some() {
            break "unit_certificate";
        }
        if queue.peek().is_none() {
            break "complete_v4_normal_forms";
        }
        if rows.len() - initial >= limits.rows {
            break "row_cap";
        }
        if attempts >= limits.attempts {
            break "operation_cap";
        }
        let (source, variable, kind) = queue.peek().unwrap();
        assert!(rows[source].high.is_empty());
        let mut r = sp.multiply(&rows[source], variable);
        let product_hash = sha(&r.value_bytes());
        let mut reducers = vec![];
        let reduction_complete = loop {
            peak_weight = peak_weight.max(r.high.len());
            if r.high.len() > limits.weight {
                break false;
            }
            match r.pivot() {
                None => break true,
                Some(p) => {
                    let reducer = if p < sp.low.cols {
                        low_map[p]
                    } else {
                        high_map.get(&p).copied()
                    };
                    if let Some(i) = reducer {
                        if p < sp.low.cols {
                            xor(&mut r.low[..=p / 64], &rows[i].low[..=p / 64]);
                        } else {
                            r.xor_assign(&rows[i]);
                        }
                        reducers.push(i);
                    } else {
                        break true;
                    }
                }
            }
        };
        let degree = r.degree(&sp);
        let remainder_hash = sha(&r.value_bytes());
        let storage_blocked =
            reduction_complete && r.pivot().is_some() && payload + r.record_len() > limits.bytes;
        let stop = if !reduction_complete {
            Some("quartic_weight_cap")
        } else if storage_blocked {
            Some("basis_payload_cap")
        } else {
            None
        };
        let mut inserted = None;
        if let Some(stop) = stop {
            blocked_data.emit(&r.record(u64::MAX));
            blocked = Some(
                json!({"source_row":source,"variable":variable,"source_kind":kind,
                "reduction_complete":reduction_complete,"stop_reason":stop,"remainder_sha256":remainder_hash,
                "quartic_nonzeros":r.high.len(),"record_bytes":r.record_len(),"xor_basis_rows":reducers}),
            );
        } else {
            // Consume the current task before appending an urgent consequence.
            queue.advance();
            completed += 1;
            if let Some(p) = r.pivot() {
                let id = rows.len();
                if p < sp.low.cols {
                    assert!(low_map[p].replace(id).is_none());
                } else {
                    assert!(high_map.insert(p, id).is_none());
                }
                let record = r.record(id as u64);
                payload += record.len();
                data.emit(&record);
                by_degree[degree] += 1;
                if degree <= 3 {
                    queue.urgent.push_back((id, 0));
                }
                rows.push(r);
                inserted = Some(id);
            } else {
                vanished += 1;
            }
        }
        let op = json!({"attempt":attempts,"source_row":source,"variable":variable,"source_kind":kind,
            "product_sha256":product_hash,"xor_basis_rows":reducers,"remainder_sha256":remainder_hash,
            "remainder_degree":degree,"reduction_complete":reduction_complete,"inserted_row":inserted,
            "blocked":stop.is_some(),"membership_degree_bound":4});
        ops.emit((serde_json::to_string(&op).unwrap() + "\n").as_bytes());
        attempts += 1;
        if attempts % 32 == 0 {
            data.flush();
            blocked_data.flush();
            ops.flush();
            let p = json!({"attempts":attempts,"completed_products":completed,"rank":rows.len(),
                "added_rows":rows.len()-initial,"pending_products":queue.pending(),"new_basis_payload_bytes":payload,
                "added_rows_by_degree":by_degree,"peak_quartic_nonzeros":peak_weight});
            progress.emit((serde_json::to_string(&p).unwrap() + "\n").as_bytes());
            progress.flush();
        }
        if let Some(stop) = stop {
            break stop;
        }
    };
    let complete = queue.peek().is_none() && blocked.is_none();
    let state = json!({"complete_v4_normal_forms":complete,"initial_rows":initial,"complete_v3_rows":old_v3_rows,
        "rank":rows.len(),"added_rows":rows.len()-initial,"added_rows_by_degree":by_degree,
        "attempted_products":attempts,"completed_products":completed,"zero_remainders":vanished,
        "new_basis_payload_bytes":payload,"peak_quartic_nonzeros":peak_weight,
        "pending_products":queue.pending(),"next_source":queue.peek(),"blocked_candidate":blocked,
        "max_new_rows":limits.rows,"max_new_basis_payload_bytes":limits.bytes,
        "max_product_attempts":limits.attempts,"max_quartic_nonzeros":limits.weight,"stop_reason":reason});
    progress.emit((serde_json::to_string(&state).unwrap() + "\n").as_bytes());
    data.finish();
    blocked_data.finish();
    ops.finish();
    progress.finish();
    witness_json(&out.join("state.json"), &state, replay);
    witness_json(&out.join("pending.json"), &queue.json(), replay);
    let affine = quartic_affine(&rows, &sp, &polys);
    witness_json(&out.join("affine.json"), &affine, replay);
    let upper = affine["contains_one"] == true || affine["exhaustive_affine_completion"] == true;
    let result = json!({"schema":"n83.degree-four-quartic/v1","working_degree":4,"membership_degree_bound":4,
        "state":state,"complete_v4_normal_forms":complete,"linear_rank":affine["linear_rank"],
        "contains_one":affine["contains_one"],"exhaustive_affine_completion":affine["exhaustive_affine_completion"],
        "assignments_checked":affine["assignments_checked"],"inherited_last_fall_lower_bound":inherited_lower,
        "last_fall_lower_bound":inherited_lower,"last_fall_upper_bound":if upper {Some(4)}else{None},
        "last_fall_degree":if upper && inherited_lower == Some(4) {Some(4)}else{None},
        "upper_bound_argument":if affine["contains_one"]==true {"Verified unit in V4 gives global upper4."}
            else if upper {"Exhaustive affine ideal equality and degree-compatible substitution give global upper4."}
            else {"No global upper certificate. Neither complete V4 nor a cap implies a global upper or lower5."},
        "independent_review":false});
    witness_json(&out.join("closure.json"), &result, replay);
    (result, rows)
}

fn quartic_parent_refs(dir: &Path) -> Value {
    let mut refs = serde_json::Map::new();
    for name in [
        "added.rows.u64le",
        "operations.jsonl",
        "progress.jsonl",
        "state.json",
        "pending.json",
        "affine.json",
        "propagation.json",
        "result.json",
    ] {
        let p = dir.join(name);
        refs.insert(
            name.into(),
            json!({"sha256":file_hash(&p),"bytes":p.metadata().unwrap().len()}),
        );
    }
    Value::Object(refs)
}
fn quartic_case(
    case: &Value,
    input_root: &Path,
    v2: &Path,
    prefix: &Path,
    suffix: &Path,
    witness: &Path,
    propagation: &Path,
    out: &Path,
    replay: bool,
) -> Value {
    assert_eq!(quartic_parent_refs(propagation), case["propagation_files"]);
    let (parent, rows) = propagation_case_state(
        &case["propagation_case"],
        input_root,
        v2,
        prefix,
        suffix,
        witness,
        propagation,
        true,
    );
    assert_eq!(parent["last_fall_lower_bound"], 4);
    assert_eq!(
        parent["state"]["rank"].as_u64().unwrap() as usize,
        rows.len()
    );
    assert_eq!(parent["state"]["restricted_queue_exhausted"], true);
    let old_v3 = parent["state"]["parent_rows"].as_u64().unwrap() as usize;
    let (input, text) = load_case(input_root, case["id"].as_str().unwrap());
    let (mut result, _) = quartic_closure(
        &input,
        &text,
        rows,
        old_v3,
        out,
        QUARTIC_LIMITS,
        Some(4),
        replay,
    );
    result["key"] = case["key"].clone();
    result["input_binding"] = case.clone();
    result["propagation_parent_result_sha256"] = json!(file_hash(&propagation.join("result.json")));
    witness_json(&out.join("result.json"), &result, replay);
    result
}

fn quartic_summary(root: &Path) {
    let protocol = read(&root.join("protocol-degree4-quartic.json"));
    let mut rows = vec![];
    for case in protocol["extensions"]["cases"].as_array().unwrap() {
        let key = case["key"].as_str().unwrap();
        let path = root.join("status").join(format!("{key}.json"));
        let status = if path.exists() {
            read(&path)
        } else {
            json!({"run_exit_code":null,"replay_exit_code":null})
        };
        let ok = status["run_exit_code"] == 0 && status["replay_exit_code"] == 0;
        let mut row = json!({"key":key,"arm":case["arm"],"dataset":case["dataset"],"family":case["family"],
            "target_kind":case["target_kind"],"verified":ok,"run_exit_code":status["run_exit_code"],"replay_exit_code":status["replay_exit_code"]});
        if ok {
            let r = read(&root.join("cases").join(key).join("result.json"));
            for name in [
                "linear_rank",
                "contains_one",
                "exhaustive_affine_completion",
                "assignments_checked",
                "inherited_last_fall_lower_bound",
                "last_fall_lower_bound",
                "last_fall_upper_bound",
                "last_fall_degree",
                "complete_v4_normal_forms",
                "state",
            ] {
                row[name] = r[name].clone();
            }
        }
        rows.push(row);
    }
    save(
        &root.join("summary.json"),
        &json!({"schema":"n83.degree-four-quartic-summary/v1",
        "verified_cases":rows.iter().filter(|r|r["verified"]==true).count(),"rows":rows,
        "scope":"Complete-or-capped Boolean V4 normal forms; global upper only from unit or exhaustive affine completion; no elite-base rank or higher-fall inference from caps"}),
    );
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("run") | Some("replay") => {
            assert_eq!(a.len(), 10);
            let r = quartic_case(&read(Path::new(&a[2])), Path::new(&a[3]), Path::new(&a[4]), Path::new(&a[5]),
                Path::new(&a[6]), Path::new(&a[7]), Path::new(&a[8]), Path::new(&a[9]), a[1]=="replay");
            println!("{}", json!({"key":r["key"],"added_rows":r["state"]["added_rows"],"stop_reason":r["state"]["stop_reason"],
                "complete_v4_normal_forms":r["complete_v4_normal_forms"],"last_fall_lower_bound":r["last_fall_lower_bound"],
                "last_fall_upper_bound":r["last_fall_upper_bound"],"last_fall_degree":r["last_fall_degree"]}));
        }
        Some("selfcheck") => { assert_eq!(a.len(),3); println!("{}",quartic_controls(Path::new(&a[2]))); }
        Some("summary") => { assert_eq!(a.len(),3); quartic_summary(Path::new(&a[2])); }
        _ => panic!("run|replay CASE INPUT_ROOT V2 PREFIX SUFFIX WITNESS PROPAGATION OUT; selfcheck OUT; summary ROOT"),
    }
}

// Independent tiny reference: explicitly enumerate variable subsets, then use
// a single dense word with columns sorted by (degree, subset integer). It uses
// neither combinatorial ranks nor the sparse product/elimination routines.
fn quartic_tiny_monomials(n: usize, cap: u32) -> Vec<u64> {
    assert!((1..=6).contains(&n) && cap <= 4);
    let mut m: Vec<_> = (0..1u64 << n).filter(|x| x.count_ones() <= cap).collect();
    m.sort_by_key(|x| (x.count_ones(), *x));
    assert!(m.len() <= 64);
    m
}
fn quartic_tiny_reduce(mut r: u64, basis: &[u64]) -> u64 {
    let mut b = basis.to_vec();
    b.sort_by_key(|x| std::cmp::Reverse(63 - x.leading_zeros()));
    for x in b {
        let p = 63 - x.leading_zeros();
        if r >> p & 1 != 0 {
            r ^= x;
        }
    }
    r
}
fn quartic_tiny_closure(n: usize, cap: u32, polys: &[(usize, Poly)]) -> Vec<u64> {
    let mons = quartic_tiny_monomials(n, cap);
    let mut work = VecDeque::new();
    for (_, p) in polys {
        if p.iter().any(|m| m.len() > cap as usize) {
            continue;
        }
        let r = p.iter().fold(0u64, |r, m| {
            let mask = m.iter().fold(0u64, |a, v| a | 1u64 << v);
            r ^ 1u64 << mons.iter().position(|x| *x == mask).unwrap()
        });
        work.push_back(r);
    }
    let mut basis = vec![];
    while let Some(r) = work.pop_front() {
        let r = quartic_tiny_reduce(r, &basis);
        if r == 0 {
            continue;
        }
        let degree = mons[(63 - r.leading_zeros()) as usize].count_ones();
        basis.push(r);
        if degree < cap {
            for v in 0..n {
                let mut product = 0;
                for (i, m) in mons.iter().enumerate() {
                    if r >> i & 1 == 0 {
                        continue;
                    }
                    let new_mask = m | (1u64 << v);
                    product ^= 1u64 << mons.iter().position(|x| *x == new_mask).unwrap();
                }
                work.push_back(product);
            }
        }
    }
    basis
}
fn quartic_tiny_parent(n: usize, text: &str) -> Vec<Vec<u64>> {
    quartic_tiny_closure(n, 3, &equations(text, n))
        .into_iter()
        .map(|r| vec![r])
        .collect()
}
fn quartic_tiny_compare(n: usize, polys: &[(usize, Poly)], rows: &[QuarticRow]) -> Vec<u64> {
    let expected = quartic_tiny_closure(n, 4, polys);
    let actual: Vec<_> = rows
        .iter()
        .map(|r| {
            assert_eq!(r.low.len(), 1);
            r.high.iter().fold(r.low[0], |a, c| a | 1u64 << c)
        })
        .collect();
    assert_eq!(
        actual.len(),
        expected.len(),
        "hybrid and independent dense ranks differ"
    );
    for r in &actual {
        assert_eq!(quartic_tiny_reduce(*r, &expected), 0);
    }
    for r in &expected {
        assert_eq!(quartic_tiny_reduce(*r, &actual), 0);
    }
    expected
}
fn quartic_controls(out: &Path) -> Value {
    fs::create_dir(out).unwrap();
    let mut cases = vec![];
    for (name, n, eqs, lower, upper) in [
        (
            "quartic_cancellation",
            4,
            vec!["v(1)*v(2)*v(3)+v(4)", "v(1)*v(2)*v(4)+v(3)"],
            None,
            false,
        ),
        ("xyz_plus_one", 3, vec!["v(1)*v(2)*v(3)+1"], Some(4), true),
        ("xyz", 4, vec!["v(1)*v(2)*v(3)"], None, false),
    ] {
        let root = out.join(name);
        fs::create_dir(&root).unwrap();
        let text = control_text(&eqs, n);
        let input = json!({"n_vars":n,"input_equations":n+eqs.len()});
        fs::write(root.join("system.sing"), &text).unwrap();
        save(&root.join("input.json"), &input);
        let parent = quartic_tiny_parent(n, &text);
        let count = parent.len();
        save(&root.join("independent-v3.json"), &json!(parent));
        let (result, rows) = quartic_closure(
            &input,
            &text,
            parent.clone(),
            count,
            &root.join("quartic"),
            QUARTIC_LIMITS,
            lower,
            false,
        );
        let (replayed, _) = quartic_closure(
            &input,
            &text,
            parent,
            count,
            &root.join("quartic"),
            QUARTIC_LIMITS,
            lower,
            true,
        );
        assert_eq!(result, replayed);
        assert_eq!(result["complete_v4_normal_forms"], true);
        assert_eq!(
            result["last_fall_upper_bound"],
            if upper { json!(4) } else { Value::Null }
        );
        assert_eq!(
            result["last_fall_degree"],
            if upper && lower == Some(4) {
                json!(4)
            } else {
                Value::Null
            }
        );
        let reference = quartic_tiny_compare(n, &equations(&text, n), &rows);
        save(&root.join("independent-v4.json"), &json!(reference));
        if name == "quartic_cancellation" {
            let linear = (1u64 << 3) | (1u64 << 4);
            let v3 = quartic_tiny_closure(n, 3, &equations(&text, n));
            assert_ne!(quartic_tiny_reduce(linear, &v3), 0);
            assert_eq!(quartic_tiny_reduce(linear, &reference), 0);
            assert!(
                rows.iter().any(|r| !r.high.is_empty()),
                "control must admit actual quartics"
            );
            assert!(result["linear_rank"].as_u64().unwrap() > 0);
        }
        cases.push(json!({"case":name,"result":result,"independent_dense_v4_passed":true,"fresh_replay_passed":true}));
    }
    let result =
        json!({"schema":"n83.degree-four-quartic-controls/v1","all_passed":true,"cases":cases});
    save(&out.join("controls.json"), &result);
    result
}

#[cfg(test)]
mod quartic_tests {
    use super::*;
    #[test]
    fn fill_in_weight_cap_retains_exact_reducer_prefix() {
        let n = 6;
        let text = control_text(
            &[
                "v(3)*v(4)*v(5)+v(1)*v(2)*v(5)+v(1)*v(4)*v(5)",
                "v(4)*v(5)*v(6)+v(1)*v(2)*v(6)+v(1)*v(4)*v(6)",
            ],
            n,
        );
        let parent = quartic_tiny_parent(n, &text);
        let input = json!({"n_vars":n});
        let out = tmp("fill_in_weight");
        let limits = QuarticLimits {
            weight: 3,
            ..QUARTIC_LIMITS
        };
        let (result, _) = quartic_closure(
            &input,
            &text,
            parent.clone(),
            parent.len(),
            &out,
            limits,
            None,
            false,
        );
        assert_eq!(result["state"]["stop_reason"], "quartic_weight_cap");
        let blocked = &result["state"]["blocked_candidate"];
        assert_eq!(blocked["reduction_complete"], false);
        assert!(!blocked["xor_basis_rows"].as_array().unwrap().is_empty());
        assert_eq!(
            result,
            quartic_closure(
                &input,
                &text,
                parent.clone(),
                parent.len(),
                &out,
                limits,
                None,
                true
            )
            .0
        );
    }
    fn tmp(tag: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("n83-quartic-{}-{tag}", std::process::id()))
    }
    fn fixture() -> (Value, String, Vec<Vec<u64>>) {
        let t = control_text(&["v(1)*v(2)*v(3)+v(4)", "v(1)*v(2)*v(4)+v(3)"], 4);
        (json!({"n_vars":4}), t.clone(), quartic_tiny_parent(4, &t))
    }
    #[test]
    fn independent_controls() {
        quartic_controls(&tmp("controls"));
    }
    #[test]
    fn indices_and_boolean_products_match_explicit_subsets() {
        for n in 1..=6 {
            let sp = QuarticSpace::new(n);
            let mons = quartic_tiny_monomials(n, 4);
            for (c, mask) in mons.iter().enumerate() {
                let m: Vec<_> = (0..n).filter(|v| mask >> v & 1 != 0).collect();
                assert_eq!(sp.col(&m), c);
                if m.len() <= 3 {
                    let row = sp.row(&[m].into());
                    for v in 0..n {
                        let p = sp.multiply(&row, v);
                        let want = mons.iter().position(|x| *x == mask | (1 << v)).unwrap();
                        let actual = p.high.iter().fold(p.low[0], |a, c| a | 1u64 << c);
                        assert_eq!(actual, 1u64 << want);
                    }
                }
            }
            // Products of sums test cancellation when repeated variables collapse.
            for seed in 0..128u64 {
                let value = (seed.wrapping_mul(0x9e3779b97f4a7c15)) & ((1u64 << sp.low.cols) - 1);
                let r = QuarticRow {
                    low: vec![value],
                    high: vec![],
                };
                for v in 0..n {
                    let p = sp.multiply(&r, v);
                    let mut expected = 0;
                    for (c, m) in mons.iter().enumerate().take(sp.low.cols) {
                        if value >> c & 1 != 0 {
                            expected ^=
                                1u64 << mons.iter().position(|x| *x == m | (1 << v)).unwrap();
                        }
                    }
                    assert_eq!(p.high.iter().fold(p.low[0], |a, c| a | 1u64 << c), expected);
                }
            }
        }
        let sp = QuarticSpace::new(164);
        assert_eq!(sp.low.cols, 735295);
        assert_eq!(sp.col(&[160, 161, 162, 163]), sp.cols - 1);
        let r = sp.row(&[vec![0, 63, 127], vec![63, 127, 163]].into());
        assert_eq!(
            sp.multiply(&r, 163),
            sp.row(&[vec![0, 63, 127, 163], vec![63, 127, 163]].into())
        );
    }
    #[test]
    fn sparse_xor_matches_dense() {
        for a in 0..256u32 {
            for b in 0..256u32 {
                let terms = |m| (0..8).filter(|i| m >> i & 1 != 0).collect::<Vec<u32>>();
                assert_eq!(quartic_xor_sorted(&terms(a), &terms(b)), terms(a ^ b));
            }
        }
    }
    #[test]
    fn queue_coverage_and_urgent_priority() {
        let sp = QuarticSpace::new(4);
        let rows = vec![
            sp.row(&parse("v(1)", 4)),
            sp.row(&parse("v(1)*v(2)*v(3)", 4)),
            sp.row(&parse("v(2)*v(3)", 4)),
        ];
        let mut q = QuarticQueue::new(&rows, 2, &sp);
        assert_eq!(q.pending(), 8);
        assert_eq!(q.peek(), Some((2, 0, "initial_added")));
        q.advance();
        q.urgent.push_back((3, 0));
        for v in 0..4 {
            assert_eq!(q.peek(), Some((3, v, "new_consequence")));
            q.advance();
        }
        let mut got = vec![];
        while let Some(x) = q.peek() {
            got.push((x.0, x.1));
            let p = q.pending();
            q.advance();
            assert_eq!(q.pending(), p - 1);
        }
        assert_eq!(
            got,
            vec![(2, 1), (2, 2), (2, 3), (1, 0), (1, 1), (1, 2), (1, 3)]
        );
        assert_eq!(q.pending(), 0);
    }
    #[test]
    fn all_caps_preserve_pending_and_blocked_rows() {
        for (tag, limits, reason, blocked) in [
            (
                "row",
                QuarticLimits {
                    rows: 1,
                    ..QUARTIC_LIMITS
                },
                "row_cap",
                false,
            ),
            (
                "operation",
                QuarticLimits {
                    attempts: 1,
                    ..QUARTIC_LIMITS
                },
                "operation_cap",
                false,
            ),
            (
                "payload",
                QuarticLimits {
                    bytes: 0,
                    ..QUARTIC_LIMITS
                },
                "basis_payload_cap",
                true,
            ),
            (
                "weight",
                QuarticLimits {
                    weight: 0,
                    ..QUARTIC_LIMITS
                },
                "quartic_weight_cap",
                true,
            ),
        ] {
            let (i, t, b) = fixture();
            let out = tmp(tag);
            let (r, _) = quartic_closure(&i, &t, b.clone(), b.len(), &out, limits, None, false);
            assert_eq!(r["state"]["stop_reason"], reason);
            assert_eq!(r["complete_v4_normal_forms"], false);
            assert!(r["state"]["pending_products"].as_u64().unwrap() > 0);
            assert!(r["last_fall_upper_bound"].is_null());
            assert_eq!(!r["state"]["blocked_candidate"].is_null(), blocked);
            assert_eq!(
                out.join("blocked.row.hybrid").metadata().unwrap().len() > 44,
                blocked
            );
            assert_eq!(
                r,
                quartic_closure(&i, &t, b.clone(), b.len(), &out, limits, None, true).0
            );
        }
    }
    #[test]
    fn unit_and_affine_upper_do_not_require_queue_exhaustion() {
        let t = control_text(&["1"], 3);
        let i = json!({"n_vars":3});
        let b = quartic_tiny_parent(3, &t);
        let (r, _) = quartic_closure(
            &i,
            &t,
            b.clone(),
            b.len(),
            &tmp("unit"),
            QUARTIC_LIMITS,
            None,
            false,
        );
        assert_eq!(r["contains_one"], true);
        assert_eq!(r["last_fall_upper_bound"], 4);
        assert!(r["last_fall_degree"].is_null());
        let t = control_text(&["v(1)*v(2)*v(3)+1"], 3);
        let b = quartic_tiny_parent(3, &t);
        let (r, _) = quartic_closure(
            &i,
            &t,
            b.clone(),
            b.len(),
            &tmp("affine_cap"),
            QuarticLimits {
                rows: 6,
                ..QUARTIC_LIMITS
            },
            Some(4),
            false,
        );
        assert_eq!(r["last_fall_degree"], 4);
        assert_eq!(r["complete_v4_normal_forms"], false);
    }
    #[test]
    fn replay_rejects_mutations_truncation_and_tails() {
        for (tag, name, mode, limits) in [
            ("mutated_added", "added.rows.hybrid", 0, QUARTIC_LIMITS),
            ("truncated_added", "added.rows.hybrid", 1, QUARTIC_LIMITS),
            ("extra_added", "added.rows.hybrid", 2, QUARTIC_LIMITS),
            (
                "mutated_blocked",
                "blocked.row.hybrid",
                0,
                QuarticLimits {
                    bytes: 0,
                    ..QUARTIC_LIMITS
                },
            ),
            ("mutated_operation", "operations.jsonl", 0, QUARTIC_LIMITS),
            ("mutated_queue", "pending.json", 0, QUARTIC_LIMITS),
            ("mutated_state", "state.json", 0, QUARTIC_LIMITS),
            ("mutated_affine", "affine.json", 0, QUARTIC_LIMITS),
            ("mutated_upper", "closure.json", 0, QUARTIC_LIMITS),
        ] {
            let (i, t, b) = fixture();
            let out = tmp(tag);
            quartic_closure(&i, &t, b.clone(), b.len(), &out, limits, None, false);
            let path = out.join(name);
            let mut bytes = fs::read(&path).unwrap();
            match mode {
                0 => {
                    let k = if name.ends_with("hybrid") { 44 } else { 0 };
                    bytes[k] ^= 1;
                }
                1 => {
                    bytes.pop();
                }
                _ => bytes.push(0),
            }
            fs::write(path, bytes).unwrap();
            assert!(std::panic::catch_unwind(|| quartic_closure(
                &i,
                &t,
                b.clone(),
                b.len(),
                &out,
                limits,
                None,
                true
            ))
            .is_err());
        }
    }
    #[test]
    fn deterministic_small_systems_match_independent_dense_closure() {
        for sample in 0..32u64 {
            let n = 5;
            let mut polys = vec![];
            let mons = quartic_tiny_monomials(n, 3);
            for j in 0..3u64 {
                let word = (sample + 7 * j + 1).wrapping_mul(0x9e3779b97f4a7c15);
                let p: Poly = mons
                    .iter()
                    .enumerate()
                    .filter(|(c, _)| word >> c & 1 != 0)
                    .map(|(_, m)| (0..n).filter(|v| m >> v & 1 != 0).collect())
                    .collect();
                polys.push(p);
            }
            let eqs: Vec<_> = polys
                .iter()
                .map(|p| {
                    p.iter()
                        .map(|m| {
                            if m.is_empty() {
                                "1".to_string()
                            } else {
                                m.iter()
                                    .map(|v| format!("v({})", v + 1))
                                    .collect::<Vec<_>>()
                                    .join("*")
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("+")
                })
                .collect();
            let refs: Vec<_> = eqs.iter().map(String::as_str).collect();
            let text = control_text(&refs, n);
            let parent = quartic_tiny_parent(n, &text);
            let input = json!({"n_vars":n});
            let (r, rows) = quartic_closure(
                &input,
                &text,
                parent.clone(),
                parent.len(),
                &tmp(&format!("dense{sample}")),
                QUARTIC_LIMITS,
                None,
                false,
            );
            if r["complete_v4_normal_forms"] == true {
                quartic_tiny_compare(n, &equations(&text, n), &rows);
            } else {
                assert_eq!(r["contains_one"], true);
            }
        }
    }
}
