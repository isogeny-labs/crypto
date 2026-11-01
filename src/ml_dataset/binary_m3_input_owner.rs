//! Immutable encoding/identity owner. Only this module can mutate these bytes.
use super::*;
use sha2::Digest;

pub(super) struct InputBytes {
    bytes: Vec<u8>,
    identity: Option<Identity>,
}
struct Identity {
    ledger_id: u64,
    binding: capture::AttemptBinding,
    public: [u8; 32],
    factor_base: [u8; 32],
    digest: [u8; 32],
}
impl InputBytes {
    pub(super) fn empty() -> Self {
        Self {
            bytes: Vec::new(),
            identity: None,
        }
    }
    pub(super) fn encode(
        &mut self,
        input: &InputArtifact,
        ledger: &mut Budget,
    ) -> Result<(), String> {
        if !self.bytes.is_empty() || self.identity.is_some() {
            return Err("input owner already used".into());
        }
        ledger
            .reserve_checking(8 * INPUT_CAP as u64, 0, Instant::now())
            .map_err(|s| format!("ordinary input counting admission: {s:?}"))?;
        let mut count = InputCount(0);
        serde_json::to_writer(&mut count, input).map_err(|e| {
            let _ = ledger.fail::<()>(Stop::Serialization);
            format!("ordinary input counting: {e}")
        })?;
        ledger
            .reserve_checking(32 * count.0 as u64, 0, Instant::now())
            .map_err(|s| format!("ordinary exact input encoding admission: {s:?}"))?;
        self.bytes.try_reserve_exact(INPUT_CAP).map_err(|_| {
            let _ = ledger.fail::<()>(Stop::Allocation);
            "ordinary input allocation".to_owned()
        })?;
        serde_json::to_writer(InputWriter(&mut self.bytes), input).map_err(|e| {
            let _ = ledger.fail::<()>(Stop::Serialization);
            format!("ordinary input serialization: {e}")
        })?;
        // Additional exact-byte hash tariff. The subsequent capture/root writer
        // retains its own original charges; this immutable digest replaces two
        // future whole-input parse/hash operations, never external disk replay.
        ledger
            .reserve_checking(8 * self.bytes.len() as u64, 1024, Instant::now())
            .map_err(|s| format!("ordinary sealed identity admission: {s:?}"))?;
        self.identity = Some(Identity {
            ledger_id: ledger.ownership_id,
            binding: input.binding.clone(),
            public: input.original_public_sha256,
            factor_base: input.original_factor_base_sha256,
            digest: sha2::Sha256::digest(&self.bytes).into(),
        });
        Ok(())
    }
    pub(super) fn origin_matches(&self, ledger: &Budget) -> bool {
        self.identity
            .as_ref()
            .is_some_and(|id| id.ledger_id == ledger.ownership_id)
    }
    pub(super) fn matches(
        &self,
        original: &VerifiedWorkerOriginal<'_>,
        ledger: &Budget,
    ) -> Result<[u8; 32], String> {
        let identity = self
            .identity
            .as_ref()
            .ok_or("ordinary input is not sealed")?;
        if identity.ledger_id != ledger.ownership_id
            || Some(&identity.binding) != original.binding
            || identity.public != original.public_sha256
            || identity.factor_base != original.factor_base_sha256
        {
            return Err("ordinary sealed input differs from original".into());
        }
        Ok(identity.digest)
    }
    pub(super) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
impl std::ops::Deref for InputBytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
impl AsRef<[u8]> for InputBytes {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}
impl std::fmt::Debug for InputBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bytes.fmt(f)
    }
}
impl PartialEq<Vec<u8>> for InputBytes {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.bytes == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn immutable_input_owner_binds_actual_serialization_and_keeps_refused_bytes() {
        let input: InputArtifact =
            serde_json::from_str(include_str!("binary_m3_retained_input.json")).unwrap();
        for refused_hash in [false, true] {
            let now = Instant::now();
            let mut ledger = Budget::new(
                16384,
                256 << 20,
                128 << 20,
                now + Duration::from_secs(30),
                now,
            )
            .unwrap();
            let expected = serde_json::to_vec(&input).unwrap();
            if refused_hash {
                ledger.native_work_limit = 8 * INPUT_CAP as u64 + 32 * expected.len() as u64;
            }
            let mut owner = InputBytes::empty();
            let result = owner.encode(&input, &mut ledger);
            assert_eq!(result.is_err(), refused_hash);
            assert_eq!(&owner.bytes, &expected);
            if refused_hash {
                assert!(owner.identity.is_none());
                assert_eq!(ledger.stopped, Some(Stop::Work));
            } else {
                let identity = owner.identity.as_ref().unwrap();
                assert_eq!(identity.binding, input.binding);
                assert_eq!(identity.public, input.original_public_sha256);
                assert_eq!(identity.factor_base, input.original_factor_base_sha256);
                assert_eq!(
                    identity.digest,
                    <[u8; 32]>::from(sha2::Sha256::digest(&expected))
                );
                let before = ledger.checking_work;
                assert!(owner.encode(&input, &mut ledger).is_err());
                assert_eq!(ledger.checking_work, before);
            }
            let moved_ledger = ledger;
            assert_eq!(owner.origin_matches(&moved_ledger), !refused_hash);
            let ptr = owner.as_ptr();
            let bytes = owner.into_bytes();
            assert_eq!(ptr, bytes.as_ptr());
            assert_eq!(bytes, expected);
        }
    }
}
