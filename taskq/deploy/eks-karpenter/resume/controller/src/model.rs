//! Durable transitions. Every side effect is reconciled against this state and
//! immutable attempt/generation identities. Missing evidence is never a fence.
use crate::contract::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Provisioning,
    Running,
    Checkpointing,
    Fencing,
    Restoring,
    VerifyingComputation,
    Benchmark,
    Succeeded,
    Failed,
    /// Command mode terminal: outputs uploaded, exit code recorded (any value),
    /// no native replay and no fresh benchmark epoch.
    CommandCompleted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Generation {
    pub number: u64,
    pub request_sha256: String,
    pub snapshot_name: String,
    pub snapshot_id: String,
    pub archive_key: String,
    pub archive_version: String,
    pub archive_sha256: String,
    pub manifest_sha256: String,
    pub archive_bytes: u64,
    pub source_instance: String,
    pub source_pod_uid: String,
    pub committed_unix_seconds: u64,
    pub snapshot_ready: bool,
    pub encrypted: bool,
    pub readback_verified: bool,
}

impl Generation {
    pub fn eligible(&self, request_hash: &str) -> bool {
        self.request_sha256 == request_hash
            && self.snapshot_ready
            && self.encrypted
            && self.readback_verified
            && self.archive_bytes > 0
            && self.snapshot_id.starts_with("snap-")
            && !self.snapshot_name.is_empty()
            && !self.archive_key.is_empty()
            && !self.archive_version.is_empty()
            && crate::contract::hex(&self.archive_sha256, 64)
            && crate::contract::hex(&self.manifest_sha256, 64)
            && self.source_instance.starts_with("i-")
            && !self.source_pod_uid.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub schema: String,
    pub request: Request,
    pub request_sha256: String,
    pub owner_build: String,
    pub phase: Phase,
    pub attempt: u32,
    pub restores: u32,
    pub next_generation: u64,
    #[serde(default)]
    pub checkpoint_generation: Option<u64>,
    pub current_pod_uid: Option<String>,
    pub current_instance: Option<String>,
    #[serde(default)]
    pub original_node_runtime: Option<serde_json::Value>,
    pub terminated_instances: Vec<String>,
    pub generations: Vec<Generation>,
    pub last_checkpoint_at: u64,
    pub selected_generation: Option<u64>,
    pub failure: Option<String>,
    pub benchmark_job: Option<String>,
    pub benchmark_uid: Option<String>,
    pub benchmark_manifest: Option<serde_json::Value>,
    pub benchmark_create_attempted: bool,
    /// Command mode only; omitted from every ecbench state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_exit_code: Option<i32>,
}

impl State {
    pub fn new(request: Request, owner: String, now: u64) -> Result<Self, String> {
        request.validate()?;
        request.validate_new_deadline(now)?;
        Ok(Self {
            schema: "isolab.eks.recovery-state/v1".into(),
            request_sha256: request.hash(),
            request,
            owner_build: owner,
            phase: Phase::Provisioning,
            attempt: 0,
            restores: 0,
            next_generation: 1,
            checkpoint_generation: None,
            current_pod_uid: None,
            current_instance: None,
            original_node_runtime: None,
            terminated_instances: vec![],
            generations: vec![],
            last_checkpoint_at: now,
            selected_generation: None,
            failure: None,
            benchmark_job: None,
            benchmark_uid: None,
            benchmark_manifest: None,
            benchmark_create_attempted: false,
            command_exit_code: None,
        })
    }
    pub fn terminal(&self) -> bool {
        matches!(
            self.phase,
            Phase::Succeeded | Phase::Failed | Phase::CommandCompleted
        )
    }
    pub fn bind_request(&self, req: &Request) -> Result<(), String> {
        if self.schema != "isolab.eks.recovery-state/v1"
            || self.request_sha256 != self.request.hash()
            || self.attempt != self.restores
            || self.restores > self.request.max_restores
        {
            return Err("invalid durable recovery state".into());
        }
        if self.request_sha256 != req.hash() || self.request != *req {
            return Err("run name already belongs to a different immutable request".into());
        }
        Ok(())
    }
    pub fn claim(&mut self, owner: &str, old_build_status: Option<&str>) -> Result<(), String> {
        if self.owner_build == owner {
            return Ok(());
        }
        if !matches!(
            old_build_status,
            Some("SUCCEEDED" | "FAILED" | "STOPPED" | "TIMED_OUT" | "FAULT")
        ) {
            return Err("prior controller is not confirmed terminal; takeover refused".into());
        }
        self.owner_build = owner.to_owned();
        Ok(())
    }
    pub fn enforce_deadline(&mut self, now: u64) -> Result<(), String> {
        if !self.terminal() && now >= self.request.deadline_unix_seconds {
            self.fail("original UTC deadline expired");
            return Err("original UTC deadline expired".into());
        }
        Ok(())
    }
    pub fn fail(&mut self, reason: &str) {
        self.phase = Phase::Failed;
        self.failure = Some(reason.to_owned());
    }
    pub fn commit(&mut self, g: Generation) -> Result<(), String> {
        if self.phase != Phase::Checkpointing
            || self.checkpoint_generation != Some(g.number)
            || g.number != self.next_generation
            || !g.eligible(&self.request_sha256)
            || self.current_instance.as_deref() != Some(g.source_instance.as_str())
            || self.current_pod_uid.as_deref() != Some(g.source_pod_uid.as_str())
        {
            return Err("generation is incomplete or not bound to the current attempt".into());
        }
        self.next_generation += 1;
        self.last_checkpoint_at = g.committed_unix_seconds;
        self.generations.push(g);
        if self.generations.len() > 2 {
            self.generations.remove(0);
        }
        // Running is not persisted until the backend has reconciled SIGCONT.
        Ok(())
    }
    pub fn begin_checkpoint(&mut self) -> Result<(), String> {
        if self.phase != Phase::Running {
            return Err("checkpoint requires a running computation".into());
        }
        self.checkpoint_generation = Some(self.next_generation);
        self.phase = Phase::Checkpointing;
        Ok(())
    }
    pub fn intended_checkpoint(&self) -> Result<u64, String> {
        if self.phase != Phase::Checkpointing {
            return Err("checkpoint reconciliation requires checkpointing phase".into());
        }
        self.checkpoint_generation
            .ok_or_else(|| "checkpoint intent is missing; refusing ambiguous reconciliation".into())
    }
    /// A running supervisor retains its last generation after SIGCONT. Only
    /// the generation requested by this transition can complete this intent.
    pub fn checkpoint_resumed(&self, observed: Option<u64>) -> Result<bool, String> {
        let intended = self.intended_checkpoint()?;
        Ok(observed == Some(intended) && self.generations.iter().any(|g| g.number == intended))
    }
    pub fn finish_checkpoint(&mut self) {
        self.phase = Phase::Running;
        self.checkpoint_generation = None;
    }
    pub fn confirm_terminated(
        &mut self,
        instance: &str,
        aws_state: Option<&str>,
    ) -> Result<(), String> {
        if self.current_instance.as_deref() != Some(instance) || aws_state != Some("terminated") {
            return Err("EC2 has not confirmed termination of the exact previous writer".into());
        }
        if !self.terminated_instances.iter().any(|x| x == instance) {
            self.terminated_instances.push(instance.to_owned());
        }
        Ok(())
    }
    pub fn begin_restore(&mut self, generation: u64, now: u64) -> Result<(), String> {
        self.enforce_deadline(now)?;
        if self.phase != Phase::Fencing || self.restores >= self.request.max_restores {
            return Err("restore is not admitted in this phase/budget".into());
        }
        let instance = self
            .current_instance
            .as_ref()
            .ok_or("missing previous writer")?;
        if !self.terminated_instances.contains(instance) {
            return Err("previous writer is not fenced".into());
        }
        if !self
            .generations
            .iter()
            .any(|g| g.number == generation && g.eligible(&self.request_sha256))
        {
            return Err("no verified committed generation selected".into());
        }
        self.attempt += 1;
        self.restores += 1;
        self.current_pod_uid = None;
        self.current_instance = None;
        self.selected_generation = Some(generation);
        self.checkpoint_generation = None;
        self.phase = Phase::Restoring;
        Ok(())
    }
    pub fn begin_benchmark(&mut self, native_replay_ok: bool) -> Result<(), String> {
        if self.phase != Phase::VerifyingComputation || !native_replay_ok {
            return Err("native computation verification is required".into());
        }
        if self.request.is_command() {
            return Err("command mode never starts a fresh benchmark epoch".into());
        }
        // One deterministic epoch, never another name following an A/A failure.
        self.benchmark_job = Some(format!("{}-epoch-1", self.request.run_name));
        self.phase = Phase::Benchmark;
        Ok(())
    }
    /// Command mode replaces native replay and the fresh epoch with one verified
    /// output upload. Any exit code is terminal and recorded; nothing is retried.
    pub fn complete_command(&mut self, exit_code: i32, now: u64) -> Result<(), String> {
        if !self.request.is_command() || self.phase != Phase::VerifyingComputation {
            return Err("command completion requires a command run verifying its outputs".into());
        }
        self.enforce_deadline(now)?;
        self.command_exit_code = Some(exit_code);
        self.phase = Phase::CommandCompleted;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn state() -> State {
        State::new(crate::contract::fixture(), "project:old".into(), 1000).unwrap()
    }
    fn generation(s: &State) -> Generation {
        Generation {
            number: s.next_generation,
            request_sha256: s.request_sha256.clone(),
            snapshot_name: "snap-test".into(),
            snapshot_id: "snap-123".into(),
            archive_key: "recovery/run/g1".into(),
            archive_version: "v1".into(),
            archive_sha256: "e".repeat(64),
            manifest_sha256: "d".repeat(64),
            archive_bytes: 1024,
            source_instance: "i-old".into(),
            source_pod_uid: "pod-old".into(),
            committed_unix_seconds: 1100,
            snapshot_ready: true,
            encrypted: true,
            readback_verified: true,
        }
    }
    fn committing() -> State {
        let mut s = state();
        s.phase = Phase::Running;
        s.begin_checkpoint().unwrap();
        s.current_instance = Some("i-old".into());
        s.current_pod_uid = Some("pod-old".into());
        s
    }
    #[test]
    fn periodic_checkpoints_survive_restart_on_both_sides_of_continue() {
        fn restart(s: &State) -> State {
            serde_json::from_slice(&serde_json::to_vec(s).unwrap()).unwrap()
        }
        let mut s = committing();
        assert_eq!(s.intended_checkpoint().unwrap(), 1);
        assert!(!s.checkpoint_resumed(None).unwrap());
        s.commit(generation(&s)).unwrap();
        s = restart(&s); // committed, before the continue RPC
        assert_eq!(s.intended_checkpoint().unwrap(), 1);
        assert_eq!(s.next_generation, 2);
        assert!(s.checkpoint_resumed(Some(1)).unwrap());
        s = restart(&s); // continue succeeded, Running save was lost
        assert!(s.checkpoint_resumed(Some(1)).unwrap());
        s.finish_checkpoint();
        s = restart(&s);
        s.begin_checkpoint().unwrap(); // the next interval expired
        s = restart(&s); // persisted intent, before checkpoint RPC
        assert_eq!(s.intended_checkpoint().unwrap(), 2);
        assert!(!s.checkpoint_resumed(Some(1)).unwrap());
        s.commit(generation(&s)).unwrap();
        assert_eq!(s.generations.len(), 2);
        assert_eq!(s.next_generation, 3);
        s = restart(&s);
        assert!(!s.checkpoint_resumed(Some(1)).unwrap());
        assert!(s.checkpoint_resumed(Some(2)).unwrap());
        s.finish_checkpoint();
        assert_eq!(s.phase, Phase::Running);
        assert_eq!(s.checkpoint_generation, None);
    }
    #[test]
    fn legacy_checkpoint_state_without_intent_is_not_guessed() {
        let s = committing();
        let mut value = serde_json::to_value(&s).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .remove("checkpoint_generation");
        let legacy: State = serde_json::from_value(value).unwrap();
        assert!(legacy.intended_checkpoint().is_err());
        assert!(legacy.checkpoint_resumed(Some(1)).is_err());
    }
    #[test]
    fn ambiguous_controller_ownership_is_not_a_lease_expiry() {
        for status in [None, Some("IN_PROGRESS"), Some("UNKNOWN")] {
            let mut s = state();
            assert!(s.claim("project:new", status).is_err());
            assert_eq!(s.owner_build, "project:old");
        }
        let mut s = state();
        s.claim("project:new", Some("FAILED")).unwrap();
        assert_eq!(s.owner_build, "project:new");
    }
    #[test]
    fn incomplete_generation_keeps_prior_committed_progress() {
        let mut s = committing();
        s.commit(generation(&s)).unwrap();
        let mut bad = generation(&s);
        bad.readback_verified = false;
        assert!(s.commit(bad).is_err());
        assert_eq!(s.generations.len(), 1);
        assert_eq!(s.generations[0].number, 1);
        assert_eq!(s.next_generation, 2);
    }
    #[test]
    fn generation_cannot_move_between_runs_or_attempts() {
        let mut s = committing();
        let mut g = generation(&s);
        g.source_pod_uid = "other-pod".into();
        assert!(s.commit(g).is_err());
        let mut g = generation(&s);
        g.request_sha256 = "f".repeat(64);
        assert!(s.commit(g).is_err());
    }
    #[test]
    fn missing_or_stopped_worker_is_not_a_fence() {
        let mut s = committing();
        s.commit(generation(&s)).unwrap();
        s.phase = Phase::Fencing;
        for status in [
            None,
            Some("stopped"),
            Some("stopping"),
            Some("running"),
            Some("shutting-down"),
        ] {
            assert!(s.confirm_terminated("i-old", status).is_err());
            assert!(s.begin_restore(1, 1200).is_err());
        }
        assert!(s.confirm_terminated("i-other", Some("terminated")).is_err());
        s.confirm_terminated("i-old", Some("terminated")).unwrap();
        s.begin_restore(1, 1200).unwrap();
        assert_eq!(s.phase, Phase::Restoring);
        assert_eq!(s.request.deadline_unix_seconds, 5000);
        assert_eq!(s.restores, 1);
    }
    #[test]
    fn expired_original_deadline_prevents_restore() {
        let mut s = committing();
        s.commit(generation(&s)).unwrap();
        s.phase = Phase::Fencing;
        s.confirm_terminated("i-old", Some("terminated")).unwrap();
        assert!(s.begin_restore(1, 5000).is_err());
        assert_eq!(s.phase, Phase::Failed);
    }
    #[test]
    fn same_run_name_never_accepts_a_new_command_or_deadline() {
        let s = state();
        let mut changed = s.request.clone();
        changed.deadline_unix_seconds += 600;
        assert!(s.bind_request(&changed).is_err());
        assert!(s.bind_request(&s.request).is_ok());
    }
    #[test]
    fn failed_measurement_cannot_spawn_a_second_epoch() {
        let mut s = state();
        s.phase = Phase::VerifyingComputation;
        assert!(s.begin_benchmark(false).is_err());
        s.begin_benchmark(true).unwrap();
        let name = s.benchmark_job.clone();
        s.fail("A/A not admitted");
        assert!(s.begin_benchmark(true).is_err());
        assert_eq!(s.benchmark_job, name);
    }
    #[test]
    fn recorded_ecbench_state_round_trips_without_command_fields() {
        let raw = include_bytes!("../../validation/20261022-periodic/attempt-b/final-state.json");
        let state: State = serde_json::from_slice(raw).unwrap();
        assert_eq!(state.request_sha256, state.request.hash());
        assert!(state.command_exit_code.is_none());
        assert_eq!(
            serde_json::to_value(&state).unwrap(),
            serde_json::from_slice::<serde_json::Value>(raw).unwrap()
        );
        assert!(state.bind_request(&state.request).is_ok());
    }
    #[test]
    fn command_completion_is_terminal_records_any_exit_and_skips_benchmark() {
        let mut s =
            State::new(crate::contract::command_fixture(), "project:a".into(), 1000).unwrap();
        assert!(s.complete_command(0, 1100).is_err(), "not verifying yet");
        s.phase = Phase::VerifyingComputation;
        assert!(s.begin_benchmark(true).is_err());
        assert!(s.benchmark_job.is_none());
        s.complete_command(3, 1100).unwrap();
        assert_eq!(s.phase, Phase::CommandCompleted);
        assert_eq!(s.command_exit_code, Some(3));
        assert!(s.terminal());
        let v = serde_json::to_value(&s).unwrap();
        assert_eq!(v["phase"], "command_completed");
        assert_eq!(v["command_exit_code"], 3);
        let back: State = serde_json::from_value(v).unwrap();
        assert!(back.bind_request(&s.request).is_ok());
        // Deadline still applies to the completion transition.
        let mut late =
            State::new(crate::contract::command_fixture(), "project:a".into(), 1000).unwrap();
        late.phase = Phase::VerifyingComputation;
        assert!(late.complete_command(0, 5000).is_err());
        assert_eq!(late.phase, Phase::Failed);
        // An ecbench run can never take the command terminal.
        let mut e = state();
        e.phase = Phase::VerifyingComputation;
        assert!(e.complete_command(0, 1100).is_err());
        assert!(serde_json::to_value(&e)
            .unwrap()
            .get("command_exit_code")
            .is_none());
    }
    #[test]
    fn command_mode_admits_many_restores_within_its_budget() {
        let mut s =
            State::new(crate::contract::command_fixture(), "project:a".into(), 1000).unwrap();
        assert_eq!(s.request.max_restores, 40);
        s.restores = 39;
        s.attempt = 39;
        s.current_instance = Some("i-old".into());
        s.current_pod_uid = Some("pod-old".into());
        s.phase = Phase::Running;
        s.begin_checkpoint().unwrap();
        s.commit(generation(&s)).unwrap();
        s.phase = Phase::Fencing;
        s.confirm_terminated("i-old", Some("terminated")).unwrap();
        s.begin_restore(1, 1200).unwrap();
        assert_eq!(s.restores, 40);
        assert!(s.bind_request(&s.request).is_ok());
        s.phase = Phase::Fencing;
        s.current_instance = Some("i-next".into());
        s.confirm_terminated("i-next", Some("terminated")).unwrap();
        assert!(s.begin_restore(1, 1300).is_err(), "budget exhausted");
    }
}
