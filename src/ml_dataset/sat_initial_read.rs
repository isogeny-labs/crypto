//! Open-descriptor actual-length admission. Failure leaves all read owners in Frame.
use super::{DecodeBudget, ReplayBudget};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    time::Instant,
};
pub(super) struct Pending {
    path: PathBuf,
    file: Option<File>,
    bytes: Vec<u8>,
    observed: usize,
    expected: usize,
}
fn fill<R: Read>(
    reader: &mut R,
    bytes: &mut [u8],
    observed: &mut usize,
    expected: usize,
    budget: &mut ReplayBudget,
) -> Result<(), String> {
    loop {
        DecodeBudget::check(budget, Instant::now()).map_err(|_| "read deadline refused")?;
        if *observed == bytes.len() {
            return Err("read grew past admitted length".into());
        }
        match reader.read(&mut bytes[*observed..]) {
            Ok(0) => break,
            Ok(n) => *observed += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.to_string()),
        }
    }
    DecodeBudget::check(budget, Instant::now()).map_err(|_| "read deadline refused")?;
    if *observed != expected {
        return Err("read length changed".into());
    }
    Ok(())
}
pub(super) fn read(
    dir: &Path,
    path: &str,
    cap: u64,
    budget: &mut ReplayBudget,
    pending: &mut Option<Pending>,
) -> Result<Vec<u8>, String> {
    if pending.is_some() {
        return Err("retained read cannot be replaced".into());
    }
    // contained_path constructs a parts vector plus a growing path. Its complete
    // path-byte/segment bound is reserved before calling it; fixed error storage
    // comes from the central reporting reservation.
    let path_bytes = dir
        .as_os_str()
        .len()
        .checked_add(path.len())
        .and_then(|n| n.checked_add(1))
        .ok_or("read path overflow")? as u64;
    let scratch = path_bytes
        .checked_mul(64)
        .and_then(|n| n.checked_add(4096))
        .ok_or("read path quote overflow")?;
    budget
        .reserve_checking(scratch, scratch, Instant::now())
        .map_err(|_| "read path admission refused")?;
    let owned_path = crate::ml_dataset::storage::contained_path(dir, path, false)?;
    *pending = Some(Pending {
        path: owned_path,
        file: None,
        bytes: Vec::new(),
        observed: 0,
        expected: 0,
    });
    let p = pending.as_mut().unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        p.file = Some(
            std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
                .open(&p.path)
                .map_err(|e| e.to_string())?,
        );
    }
    #[cfg(not(unix))]
    return Err("descriptor-bound replay reads require Unix".into());
    let metadata = p
        .file
        .as_ref()
        .unwrap()
        .metadata()
        .map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.len() > cap {
        return Err("read is not regular/admitted".into());
    }
    let admitted = metadata
        .len()
        .checked_add(1)
        .ok_or("read sentinel overflow")?;
    let allocation = usize::try_from(admitted).map_err(|_| "read allocation overflow")?;
    budget
        .reserve_checking(
            admitted.checked_mul(32).ok_or("read work overflow")?,
            admitted,
            Instant::now(),
        )
        .map_err(|_| "read admission refused")?;
    // A fixed buffer prevents geometric read_to_end growth and admits the sentinel.
    p.expected = allocation - 1;
    p.bytes = vec![0; allocation];
    fill(
        p.file.as_mut().unwrap(),
        &mut p.bytes,
        &mut p.observed,
        p.expected,
        budget,
    )?;
    p.bytes.truncate(p.observed);
    let mut finished = pending.take().unwrap();
    Ok(std::mem::take(&mut finished.bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn budget() -> ReplayBudget {
        ReplayBudget {
            policy: super::super::Policy {
                work: 1_000_000,
                bytes: 1_000_000,
                deadline: Instant::now() + std::time::Duration::from_secs(30),
                stream_bytes: 1_000_000,
                payload_bytes: 1_000_000,
            },
            work: 0,
            bytes: 0,
            first_stop: None,
        }
    }
    #[test]
    fn exact_length_sentinel_and_partial_error_preserve_buffer() {
        let mut b = budget();
        let mut bytes = vec![0; 4];
        let mut count = 0;
        fill(&mut &b"abc"[..], &mut bytes, &mut count, 3, &mut b).unwrap();
        assert_eq!(count, 3);
        assert_eq!(&bytes[..count], b"abc");
        count = 0;
        assert!(fill(&mut &b"abcd"[..], &mut bytes, &mut count, 3, &mut b).is_err());
        assert_eq!(count, 4);
        assert_eq!(&bytes, b"abcd");
        count = 0;
        assert!(fill(&mut &b"ab"[..], &mut bytes, &mut count, 3, &mut b).is_err());
        assert_eq!(count, 2);
        assert_eq!(&bytes[..count], b"ab");
        struct PartialIo {
            first: bool,
        }
        impl Read for PartialIo {
            fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
                if self.first {
                    self.first = false;
                    out[0] = 42;
                    Ok(1)
                } else {
                    Err(std::io::Error::other("fixture IO refusal"))
                }
            }
        }
        count = 0;
        assert!(fill(
            &mut PartialIo { first: true },
            &mut bytes,
            &mut count,
            3,
            &mut b
        )
        .is_err());
        assert_eq!(count, 1);
        assert_eq!(bytes[0], 42);
    }
    #[test]
    fn actual_small_file_charges_actual_length_not_cap_and_refusal_retains_owner() {
        let root = std::env::temp_dir().join(format!(
            "sat-read-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join("small"), b"abc").unwrap();
        let mut b = budget();
        let mut pending = None;
        let got = read(&root, "small", 1_000_000, &mut b, &mut pending).unwrap();
        assert_eq!(got, b"abc");
        assert!(pending.is_none());
        let scratch = ((root.as_os_str().len() + 5 + 1) as u64) * 64 + 4096;
        assert_eq!(b.work, scratch + 4 * 32);
        assert_eq!(b.bytes, scratch + 4);
        let mut b = budget();
        b.policy.work = scratch + 127;
        assert!(read(&root, "small", 1_000_000, &mut b, &mut pending).is_err());
        let p = pending.as_ref().unwrap();
        assert!(p.file.is_some());
        assert!(p.bytes.is_empty());
        assert_eq!(b.work, scratch);
        assert!(b.first_stop.is_some());
        assert!(read(&root, "small", 1_000_000, &mut b, &mut pending).is_err());
        assert!(pending.as_ref().unwrap().bytes.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
}
