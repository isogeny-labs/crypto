//! Explicit finite replay invocation. Owns every admitted central frame until reporting ends.
use super::*;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
const POLICY_CAP: u64 = 4096;
const RUN_CAP: u64 = 4096;
const CALLER_SETUP_WORK: u64 = 65536;
const RUN_READ_WORK: u64 = 2 * (RUN_CAP + 1) + 32;
const RUN_READ_BYTES: u64 = 8192;
struct CallerAdmission {
    remaining: u64,
    work: u64,
    bytes: u64,
    deadline: Instant,
}
impl CallerAdmission {
    fn reserve_read(&mut self) -> Result<(), String> {
        if self.remaining == 0 {
            return Err("affine-row caller read allowance exhausted".into());
        }
        self.remaining -= 1;
        self.work += RUN_READ_WORK;
        self.bytes += RUN_READ_BYTES;
        if Instant::now() >= self.deadline {
            return Err("affine-row caller original deadline exhausted".into());
        }
        Ok(())
    }
    fn hash_run(&mut self, path: &Path) -> Result<String, String> {
        use sha2::{Digest, Sha256};
        self.reserve_read()?;
        // Admission precedes open/read; no metadata-driven allocation or EOF hash helper.
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let mut file = options.open(path).map_err(|e| e.to_string())?;
        let meta = file.metadata().map_err(|e| e.to_string())?;
        if !meta.is_file() || meta.len() > RUN_CAP {
            return Err("affine-row run binding exceeds bounded regular-file cap".into());
        }
        let mut hash = Sha256::new();
        let mut buffer = [0u8; 512];
        let mut total = 0u64;
        loop {
            if Instant::now() >= self.deadline {
                return Err("affine-row caller original deadline exhausted".into());
            }
            let remaining = (RUN_CAP + 1 - total).min(buffer.len() as u64) as usize;
            let n = file
                .read(&mut buffer[..remaining])
                .map_err(|e| e.to_string())?;
            if Instant::now() >= self.deadline {
                return Err("affine-row caller original deadline exhausted".into());
            }
            if n == 0 {
                break;
            }
            total += n as u64;
            if total > RUN_CAP {
                return Err("affine-row run binding grew past admitted cap".into());
            }
            hash.update(&buffer[..n]);
        }
        if total != meta.len() {
            return Err("affine-row run binding changed length during read".into());
        }
        Ok(format!("{:x}", hash.finalize()))
    }
}
const MAX_CELLS: usize = 64;
const FIXED_OWNER: u64 = 65536;
const RECEIPT_PER_SLOT: u64 = 4096;
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PolicyFile {
    schema: String,
    mode: String,
    run_sha256: String,
    receipt_path: PathBuf,
    cell_indices: Vec<u64>,
    seconds: u64,
    work_per_cell: u64,
    bytes_per_cell: u64,
    stream_bytes_per_cell: u64,
    payload_bytes_per_cell: u64,
    aggregate_work: u64,
    aggregate_bytes: u64,
}
struct Slot {
    index: u64,
    consumed: bool,
    visits: u8,
    projections:u8,
    failed: bool,
    custody: Option<solvers::AffineReplayCustody>,
}
pub(super) struct Session {
    policy: PolicyFile,
    policy_sha256: String,
    deadline: Instant,
    slots: Vec<Slot>,
    reserved_work: u64,
    reserved_bytes: u64,
    report_taken: bool,
    caller: CallerAdmission,
}
impl Session {
    pub(super) fn from_policy(root: &Path, path: &Path, receipt: &Path) -> Result<Self, String> {
        let started = Instant::now();
        // Fixed <=4097-byte bootstrap, charged in CALLER_SETUP_WORK/FIXED_OWNER. Never producer allowance.
        let mut raw = Vec::new();
        raw.try_reserve_exact(POLICY_CAP as usize + 1)
            .map_err(|_| "policy allocation")?;
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let mut file = options.open(path).map_err(|e| e.to_string())?;
        let meta = file.metadata().map_err(|e| e.to_string())?;
        if !meta.is_file() || meta.len() > POLICY_CAP {
            return Err("bounded regular policy file required".into());
        }
        let setup_deadline = started + Duration::from_secs(5);
        let mut buffer = [0u8; 512];
        loop {
            if Instant::now() >= setup_deadline {
                return Err("policy bootstrap deadline exhausted".into());
            }
            let take = (POLICY_CAP as usize + 1 - raw.len()).min(buffer.len());
            let count = file.read(&mut buffer[..take]).map_err(|e| e.to_string())?;
            if Instant::now() >= setup_deadline {
                return Err("policy bootstrap deadline exhausted".into());
            }
            if count == 0 {
                break;
            }
            raw.extend_from_slice(&buffer[..count]);
            if raw.len() as u64 > POLICY_CAP {
                break;
            }
        }
        if raw.len() as u64 > POLICY_CAP {
            return Err("affine-row replay policy exceeds4096 bytes".into());
        }
        let p: PolicyFile = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
        if !p.receipt_path.is_absolute()
            || p.receipt_path != receipt
            || p.schema != "crypto.ml-dataset.affine-replay-policy/v1"
            || p.mode != "checked_affine_unit_identity_v1"
            || p.cell_indices.is_empty()
            || p.cell_indices.len() > MAX_CELLS
            || p.cell_indices.windows(2).any(|v| v[0] >= v[1])
            || p.seconds == 0
            || p.seconds > 3600
            || p.work_per_cell == 0
            || p.bytes_per_cell == 0
            || p.stream_bytes_per_cell == 0
            || p.payload_bytes_per_cell == 0
        {
            return Err("invalid finite affine-row replay policy".into());
        }
        if p.run_sha256.len() != 64
            || !p
                .run_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err("invalid authorized run digest".into());
        }
        let count = p.cell_indices.len() as u64;
        let reads = count
            .checked_mul(8)
            .and_then(|n| n.checked_add(2))
            .ok_or("caller read count overflow")?;
        let caller_work = reads
            .checked_mul(RUN_READ_WORK)
            .and_then(|n| n.checked_add(CALLER_SETUP_WORK))
            .ok_or("caller work overflow")?;
        let caller_bytes = reads
            .checked_mul(RUN_READ_BYTES)
            .ok_or("caller bytes overflow")?;
        let reserved_work = p
            .work_per_cell
            .checked_mul(count)
            .and_then(|n| n.checked_add(caller_work))
            .ok_or("affine-row aggregate work overflow")?;
        let slot_bytes = (std::mem::size_of::<Slot>() as u64)
            .checked_add(solvers::AffineReplayCustody::retained_slot_bytes() as u64)
            .and_then(|n| n.checked_add(RECEIPT_PER_SLOT))
            .ok_or("affine-row slot overflow")?;
        let reserved_bytes = p
            .bytes_per_cell
            .checked_add(slot_bytes)
            .and_then(|n| n.checked_mul(count))
            .and_then(|n| n.checked_add(FIXED_OWNER))
            .and_then(|n| n.checked_add(caller_bytes))
            .ok_or("affine-row aggregate bytes overflow")?;
        if reserved_work > p.aggregate_work || reserved_bytes > p.aggregate_bytes {
            return Err("affine-row aggregate admission refused".into());
        }
        // Full cell envelopes and terminal receipt owners reserved once, no refunds.
        let deadline = started
            .checked_add(Duration::from_secs(p.seconds))
            .ok_or("affine-row deadline overflow")?;
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(p.cell_indices.len())
            .map_err(|_| "affine-row slot allocation")?;
        for &index in &p.cell_indices {
            slots.push(Slot {
                index,
                consumed: false,
                visits: 0,
                projections:0,
                failed: false,
                custody: None,
            });
        }
        let mut session = Self {
            policy: p,
            policy_sha256: digest(&raw),
            deadline,
            slots,
            reserved_work,
            reserved_bytes,
            report_taken: false,
            caller: CallerAdmission {
                remaining: reads,
                work: CALLER_SETUP_WORK,
                bytes: 0,
                deadline,
            },
        };
        if root.join("run.json").exists()
            && session.caller.hash_run(&root.join("run.json"))? != session.policy.run_sha256
        {
            return Err("affine-row replay policy run mismatch".into());
        }
        Ok(session)
    }
    pub(super) fn verify(&mut self, dir: &Path, manifest: &CellManifest) -> Result<(), String> {
        let slot = self
            .slots
            .iter_mut()
            .find(|s| s.index == manifest.cell_index)
            .ok_or("cell absent from explicit affine-row replay policy")?;
        if slot.failed {
            return Err("affine-row replay slot previously failed; custody retained".into());
        }
        slot.failed = true; // Any local/central admission failure is sticky, including I/O.
        if slot.visits >= 8 {
            return Err("affine-row replay cell visit limit exhausted; custody retained".into());
        }
        slot.visits += 1;
        if slot.consumed && slot.custody.is_none() {
            return Err("affine-row replay admission already failed; no retry allowance".into());
        }
        slot.consumed = true; // Sticky even on expired deadline or subsequent failure.
        if Instant::now() >= self.deadline {
            return Err("affine-row replay original deadline exhausted".into());
        }
        if self.caller.hash_run(&dir.join("run.json"))? != self.policy.run_sha256 {
            return Err("affine-row cell differs from authorized run".into());
        }
        if let Some(custody) = slot.custody.as_mut() {
            // REQUIRED dependency: retained-owner revalidation. No cached scalar verdict,
            // no new Policy, no transcript/payload work outside the retained central ledger.
            if matches!(
                custody.verdict(),
                solvers::AffineReplayVerdict::CheckedRow
                   
            ) {
                custody.revalidate(
                    dir,
                    &manifest.config,
                    &manifest.outcome,
                    &manifest.artifacts,
                );
            } // Failed/Unknown/Incomplete owners never acquire a second attempt.
        } else {
            let policy = solvers::SatReplayPolicy {
                work: self.policy.work_per_cell,
                bytes: self.policy.bytes_per_cell,
                deadline: self.deadline,
                stream_bytes: self.policy.stream_bytes_per_cell,
                payload_bytes: self.policy.payload_bytes_per_cell,
            };
            slot.custody = Some(solvers::verify_affine_row_cell(
                dir,
                &manifest.config,
                &manifest.outcome,
                &manifest.artifacts,
                policy,
            ));
        }
        // Never map/drop a Failed/Unknown central frame into an unowned String.
        match slot.custody.as_ref().unwrap().verdict() {
            solvers::AffineReplayVerdict::CheckedRow
            => {
                slot.failed = false;
                Ok(())
            }
            solvers::AffineReplayVerdict::Incomplete => {
                Err("affine-row replay incomplete; custody retained".into())
            }
            solvers::AffineReplayVerdict::Unknown => {
                Err("affine-row replay resource/semantic unknown; custody retained".into())
            }
            solvers::AffineReplayVerdict::Rejected => {
                Err("affine-row replay rejected; custody retained".into())
            }
        }
    }
    pub(super) fn project(&mut self,dir:&Path,manifest:&CellManifest)->Result<Value,String>{
        self.verify(dir,manifest)?;
        let slot=self.slots.iter_mut().find(|s|s.index==manifest.cell_index).ok_or("affine projection slot missing")?;
        slot.failed=true;
        let value=slot.custody.as_mut().ok_or("affine projection owner missing")?.project_identity()?;
        slot.projections+=1;slot.failed=false;Ok(value)
    }
    pub(super) fn write_receipt(&mut self, path: &Path, success: bool) -> Result<(), String> {
        if self.report_taken {
            return Err("affine-row replay receipt already attempted".into());
        }
        self.report_taken = true;
        let rows:Vec<Value>=self.slots.iter().map(|s|{
            let verdict=match s.custody.as_ref().map(|c|c.verdict()){
                Some(solvers::AffineReplayVerdict::CheckedRow)=>"checked_finite_identity",
                Some(solvers::AffineReplayVerdict::Incomplete)=>"incomplete",
                Some(solvers::AffineReplayVerdict::Unknown)=>"unknown",
                Some(solvers::AffineReplayVerdict::Rejected)=>"rejected",
                None if s.consumed=>"admission_refused",
                None=>"unattempted",
            };
            json!({"cell_index":s.index,"mode":"checked_affine_unit_identity_v1","consumed":s.consumed,"visits":s.visits,"failed":s.failed,"verdict":verdict,"finite_identity_projections":s.projections,"charged":s.custody.as_ref().map(|c|c.charged())})
        }).collect();
        let value = json!({"schema":"crypto.ml-dataset.affine-replay-invocation/v1","run_sha256":self.policy.run_sha256,"verification_returned_ok":success,"reserved_work":self.reserved_work,"reserved_bytes":self.reserved_bytes,"caller_charged_work":self.caller.work,"caller_charged_bytes":self.caller.bytes,"slots":rows,"solver_completed":false,"training_export_admitted":false});
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        serde_json::to_writer(&mut file, &value).map_err(|e| e.to_string())?;
        file.write_all(b"\n")
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())
    }
}

// Scoped invocation context; owns frames, never pointers into another stack. The CLI
// executes verification on this thread. Other threads have no authority and fail closed.
thread_local! {
    static CURRENT:std::cell::RefCell<Option<Session>>=const {std::cell::RefCell::new(None)};
    static ENTERED:std::cell::Cell<bool>=const {std::cell::Cell::new(false)};
}
struct Scope;
impl Drop for Scope {
    fn drop(&mut self) {
        CURRENT.with(|c| {
            c.borrow_mut().take();
        });
        ENTERED.with(|c| c.set(false));
    }
}
pub(super) fn with_current<T>(
    f: impl FnOnce(Option<&mut Session>) -> Result<T, String>,
) -> Result<T, String> {
    CURRENT.with(|c| {
        let mut current = c
            .try_borrow_mut()
            .map_err(|_| "nested affine-row replay session borrow refused")?;
        f(current.as_mut())
    })
}
fn command_root(command: &DatasetCommand) -> Option<PathBuf> {
    match command {
        DatasetCommand::Generate(a) => Some(a.out.clone()),
        DatasetCommand::Verify { out }
        | DatasetCommand::Status { out }
        | DatasetCommand::Report { out, .. }
        | DatasetCommand::Export { out, .. }
        | DatasetCommand::Publish { out, .. }
        | DatasetCommand::Restore { out, .. } => Some(out.clone()),
        DatasetCommand::Custody(a) => match &a.command {
            custody::CustodyCommand::Create { out, .. }
            | custody::CustodyCommand::Publish { out, .. } => Some(out.clone()),
            custody::CustodyCommand::Restore { destination, .. }
            | custody::CustodyCommand::RestoreRemote { destination, .. } => {
                Some(destination.join("run"))
            }
            custody::CustodyCommand::Verify { .. } => None,
        },
        _ => None,
    }
}
pub(super) fn invoke(
    args: &DatasetArgs,
    body: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    if args.affine_replay_policy.is_some() && args.sat_replay_policy.is_some() {
        return Err("simultaneous affine and SAT replay policies are ambiguous".into());
    }
    if ENTERED.with(|c| c.replace(true)) {
        return Err("nested dataset invocation refused; existing owner retained".into());
    }
    let _scope = Scope;
    match (&args.affine_replay_policy, &args.affine_replay_receipt) {
        (None, None) => body(),
        (Some(policy), Some(receipt)) => {
            let root = command_root(&args.command)
                .ok_or("affine-row replay policy unsupported for this command")?;
            let mut start_name = receipt.as_os_str().to_os_string();
            start_name.push(".started.json");
            let start_path = PathBuf::from(start_name);
            if receipt.exists() || start_path.exists() {
                return Err("existing affine-row replay invocation; reconcile instead of retry".into());
            }
            let session = Session::from_policy(&root, policy, receipt)?;
            let start = json!({"schema":"crypto.ml-dataset.affine-replay-start/v1","policy_sha256":session.policy_sha256,"run_sha256":session.policy.run_sha256});
            let mut file = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(start_path)
                .map_err(|e| e.to_string())?;
            serde_json::to_writer(&mut file, &start).map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            CURRENT.with(|c| *c.borrow_mut() = Some(session));
            let result = body();
            // All owners survive BOTH the command and the receipt write attempt.
            let terminal = with_current(|s| {
                s.ok_or("missing invocation owner")?
                    .write_receipt(receipt, result.is_ok())
            });
            match (result, terminal) {
                (Ok(()), Ok(())) => Ok(()),
                (Err(e), Ok(())) => Err(e),
                (Ok(()), Err(e)) => Err(e),
                (Err(e), Err(t)) => Err(format!("{e}; affine-row terminal receipt failed: {t}")),
            }
        }
        _ => Err("affine-row replay policy and receipt must be supplied together".into()),
    }
}
fn active_request(channels: Option<&types::ChannelRequests>) -> bool {
    channels.is_some_and(|c| {
        c.schema == "crypto.ml-dataset.channel-requests/v3" && c.affine_boolean_row.is_some()
    })
}
pub(super) fn authorize_generation(plan: &DatasetPlan, binding: &Value) -> Result<(), String> {
    if !active_request(plan.channels.as_ref()) {
        return Ok(());
    }
    // Match storage::write_json through an admitted bounded writer.
    with_current(|s| {
        let s = s.ok_or("explicit affine-row replay policy required before generation")?;
        s.caller.reserve_read()?;
        struct Bounded(Vec<u8>);
        impl std::io::Write for Bounded {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                if b.len() > RUN_CAP as usize - self.0.len() {
                    return Err(std::io::Error::other("run encoding exceeds cap"));
                }
                self.0.extend_from_slice(b);
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut output = Bounded(Vec::new());
        output
            .0
            .try_reserve_exact(RUN_CAP as usize)
            .map_err(|_| "run encoding allocation")?;
        serde_json::to_writer_pretty(&mut output, binding).map_err(|e| e.to_string())?;
        output.write_all(b"\n").map_err(|e| e.to_string())?;
        let encoded = output.0;
        if digest(&encoded) != s.policy.run_sha256 {
            return Err("generation run differs from authorized affine-row replay run".into());
        }
        if Instant::now() >= s.deadline {
            return Err("affine-row replay deadline expired before generation".into());
        }
        Ok(())
    })
}
pub(super) fn authorize_cell_dispatch(index: u64, config: &CaseConfig) -> Result<(), String> {
    if !active_request(config.channels.as_ref()) {
        return Ok(());
    }
    with_current(|s| {
        let s = s.ok_or("explicit affine-row replay policy required before cell dispatch")?;
        if Instant::now() >= s.deadline
            || !s
                .slots
                .iter()
                .any(|slot| slot.index == index && !slot.consumed)
        {
            return Err("affine-row dispatch outside remaining authorized cells/deadline".into());
        }
        Ok(())
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture {
        root: PathBuf,
        policy: PathBuf,
        receipt: PathBuf,
    }
    impl Fixture {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let root = std::env::temp_dir().join(format!(
                "affine-caller-v1-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            fs::write(root.join("run.json"), b"{}\n").unwrap();
            let policy = root.join("policy.json");
            let receipt = root.join("receipt.json");
            fs::write(
                &policy,
                serde_json::to_vec(&json!({
                    "schema":"crypto.ml-dataset.affine-replay-policy/v1","mode":"checked_affine_unit_identity_v1","run_sha256":digest(b"{}\n"),
                    "receipt_path":receipt,"cell_indices":[0],"seconds":60,
                    "work_per_cell":1024,"bytes_per_cell":1024,"stream_bytes_per_cell":1024,
                    "payload_bytes_per_cell":1024,"aggregate_work":1000000,"aggregate_bytes":1048576
                }))
                .unwrap(),
            )
            .unwrap();
            Self {
                root,
                policy,
                receipt,
            }
        }
        fn args(&self) -> DatasetArgs {
            DatasetArgs {
                owned_replay_policy: None,
                owned_replay_receipt: None,
                search_replay_policy:None,search_replay_receipt:None,
                sat_replay_policy:None,sat_replay_receipt:None,
                affine_replay_policy: Some(self.policy.clone()),
                affine_replay_receipt: Some(self.receipt.clone()),
                command: DatasetCommand::Verify {
                    out: self.root.clone(),
                },
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    #[test]
    fn actual_command_failure_writes_terminal_and_exclusive_marker_prevents_retry() {
        let f = Fixture::new();
        let calls = std::cell::Cell::new(0);
        let e = invoke(&f.args(), || {
            calls.set(calls.get() + 1);
            Err("actual caller failure".into())
        })
        .unwrap_err();
        assert_eq!(e, "actual caller failure");
        let receipt: Value = read_json(&f.receipt).unwrap();
        assert_eq!(receipt["verification_returned_ok"], false);
        assert_eq!(receipt["slots"][0]["verdict"], "unattempted");
        assert!(invoke(&f.args(), || {
            calls.set(calls.get() + 1);
            Ok(())
        })
        .unwrap_err()
        .contains("existing affine-row replay invocation"));
        assert_eq!(calls.get(), 1);
    }
    #[test]
    fn nested_invocation_cannot_replace_outer_owner_and_receipt_failure_keeps_command_error() {
        let f = Fixture::new();
        let e = invoke(&f.args(), || {
            assert!(invoke(&f.args(), || panic!("nested body ran"))
                .unwrap_err()
                .contains("nested"));
            with_current(|s| {
                assert!(s.is_some());
                Ok(())
            })?;
            // Real create_new conflict after command entry, not an injected serializer.
            fs::write(&f.receipt, b"reserved by external writer").unwrap();
            Err("command failed after entry".into())
        })
        .unwrap_err();
        assert!(e.contains("command failed after entry"));
        assert!(e.contains("terminal receipt failed"));
        assert_eq!(
            fs::read(&f.receipt).unwrap(),
            b"reserved by external writer"
        );
        with_current(|s| {
            assert!(s.is_none());
            Ok(())
        })
        .unwrap();
    }
    #[test]
    fn mutation_of_bound_run_refuses_before_command_and_missing_pair_never_runs() {
        let f = Fixture::new();
        fs::write(f.root.join("run.json"), b"changed\n").unwrap();
        assert!(invoke(&f.args(), || panic!("unauthorized command ran"))
            .unwrap_err()
            .contains("run mismatch"));
        let mut a = f.args();
        a.affine_replay_receipt = None;
        assert!(invoke(&a, || panic!("unpaired command ran"))
            .unwrap_err()
            .contains("supplied together"));
        assert!(!f.receipt.exists());
    }
    #[test]
    fn oversized_regular_run_is_refused_after_charge_and_allowance_never_resets() {
        let f = Fixture::new();
        let path = f.root.join("oversized-run.json");
        std::fs::File::create(&path)
            .unwrap()
            .set_len(1 << 30)
            .unwrap();
        let mut owner = CallerAdmission {
            remaining: 1,
            work: 0,
            bytes: 0,
            deadline: Instant::now() + Duration::from_secs(60),
        };
        assert!(owner
            .hash_run(&path)
            .unwrap_err()
            .contains("bounded regular-file cap"));
        assert_eq!(
            (owner.remaining, owner.work, owner.bytes),
            (0, RUN_READ_WORK, RUN_READ_BYTES)
        );
        fs::write(&path, b"{}\n").unwrap();
        assert!(owner
            .hash_run(&path)
            .unwrap_err()
            .contains("allowance exhausted"));
        assert_eq!((owner.work, owner.bytes), (RUN_READ_WORK, RUN_READ_BYTES));
    }
    #[test]
    fn actual_bounded_digest_matches_bytes_and_deadline_refusal_keeps_charge() {
        let f = Fixture::new();
        let mut owner = CallerAdmission {
            remaining: 2,
            work: 0,
            bytes: 0,
            deadline: Instant::now() + Duration::from_secs(60),
        };
        assert_eq!(
            owner.hash_run(&f.root.join("run.json")).unwrap(),
            digest(b"{}\n")
        );
        owner.deadline = Instant::now();
        assert!(owner
            .hash_run(&f.root.join("run.json"))
            .unwrap_err()
            .contains("deadline exhausted"));
        assert_eq!(
            (owner.remaining, owner.work, owner.bytes),
            (0, 2 * RUN_READ_WORK, 2 * RUN_READ_BYTES)
        );
    }
    #[test]
    fn custody_cli_restore_roots_join_native_run_and_hash_only_verify_has_no_policy() {
        use clap::Parser;
        #[derive(clap::Parser)]
        struct Cli {
            #[command(flatten)]
            dataset: DatasetArgs,
        }
        for verb in ["restore", "restore-remote"] {
            let mut argv = vec![
                "test",
                "--affine-replay-policy",
                "policy.json",
                "--affine-replay-receipt",
                "receipt.json",
                "custody",
                verb,
                "--bundle",
                "bundle",
                "--destination",
                "destination",
            ];
            if verb == "restore-remote" {
                argv.extend(["--receipt", "publication.json"]);
            }
            let cli = Cli::try_parse_from(argv).unwrap();
            assert_eq!(
                command_root(&cli.dataset.command),
                Some(PathBuf::from("destination/run"))
            );
            assert_eq!(
                cli.dataset.affine_replay_policy,
                Some(PathBuf::from("policy.json"))
            );
        }
        let f = Fixture::new();
        let mut a = f.args();
        a.command = DatasetCommand::Custody(custody::CustodyArgs {
            command: custody::CustodyCommand::Verify {
                bundle: f.root.clone(),
            },
        });
        assert!(invoke(&a, || panic!(
            "hash-only custody command received native replay authorization"
        ))
        .unwrap_err()
        .contains("unsupported for this command"));
        assert!(!f.receipt.exists());
    }
}
