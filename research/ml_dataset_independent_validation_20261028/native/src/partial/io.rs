//! Linux descriptor-rooted exact-byte I/O and pre-materialization JSON admission.
use super::{budget::Budget, schema::validate_path};
use anyhow::{bail, ensure, Context, Result};
use serde::de::{self, DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path};

struct Admission<'a> {
    budget: &'a mut Budget,
    depth: usize,
}
impl<'de> DeserializeSeed<'de> for Admission<'_> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> std::result::Result<(), D::Error> {
        if self.depth >= 64 {
            return Err(de::Error::custom(
                "consumer_resource_limit: partial JSON depth",
            ));
        }
        self.budget
            .charge(1024, "JSON node and retained typed fields")
            .map_err(de::Error::custom)?;
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Admission<'_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded duplicate-free JSON")
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> std::result::Result<(), E> {
        if value.is_finite() {
            Ok(())
        } else {
            Err(E::custom("nonfinite partial JSON number"))
        }
    }
    fn visit_unit<E: de::Error>(self) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, value: &str) -> std::result::Result<(), E> {
        self.budget
            .charge(
                value
                    .len()
                    .checked_mul(8)
                    .ok_or_else(|| E::custom("consumer_resource_limit: string"))?,
                "JSON strings and copies",
            )
            .map_err(E::custom)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> std::result::Result<(), A::Error> {
        while seq
            .next_element_seed(Admission {
                budget: self.budget,
                depth: self.depth + 1,
            })?
            .is_some()
        {}
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> std::result::Result<(), A::Error> {
        let mut keys = BTreeSet::new();
        while let Some(key) = map.next_key::<String>()? {
            self.budget
                .charge(
                    key.len()
                        .checked_mul(8)
                        .and_then(|v| v.checked_add(1024))
                        .ok_or_else(|| de::Error::custom("consumer_resource_limit: key"))?,
                    "JSON key and duplicate set",
                )
                .map_err(de::Error::custom)?;
            if !keys.insert(key) {
                return Err(de::Error::custom("duplicate decoded partial JSON key"));
            }
            map.next_value_seed(Admission {
                budget: self.budget,
                depth: self.depth + 1,
            })?;
        }
        Ok(())
    }
}
pub(crate) fn typed<T: DeserializeOwned>(raw: &[u8], budget: &mut Budget) -> Result<T> {
    let mut parser = serde_json::Deserializer::from_slice(raw);
    Admission { budget, depth: 0 }.deserialize(&mut parser)?;
    parser.end()?;
    serde_json::from_slice(raw).context("partial strict wire")
}

/// A directory FD is retained through traversal, reads, writes and enumeration.
pub(crate) struct Root {
    dir: File,
}
fn opened(fd: libc::c_int) -> Result<File> {
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: successful open/dup returns one new owned FD.
    Ok(unsafe { File::from_raw_fd(fd) })
}
fn name(value: &str) -> Result<CString> {
    Ok(CString::new(value)?)
}
fn dir_at(parent: &File, value: &str) -> Result<File> {
    let value = name(value)?;
    // SAFETY: C string lives through openat; flags do not require a mode argument.
    opened(unsafe {
        libc::openat(
            parent.as_raw_fd(),
            value.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    })
}
impl Root {
    pub(crate) fn try_clone(&self) -> Result<Self> {
        Ok(Self {
            dir: self.dir.try_clone()?,
        })
    }
    pub(crate) fn open(path: &Path) -> Result<Self> {
        ensure!(
            cfg!(target_os = "linux"),
            "unsupported partial filesystem platform"
        );
        ensure!(
            path.as_os_str().as_bytes().len() <= 4096,
            "partial input path length"
        );
        let absolute = if path.is_absolute() {
            path.to_owned()
        } else {
            std::env::current_dir()?.join(path)
        };
        ensure!(
            absolute.as_os_str().as_bytes().len() <= 4096,
            "partial absolute path length"
        );
        let mut dir = opened(unsafe {
            libc::open(
                c"/".as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        })?;
        for part in absolute.components() {
            match part {
                Component::RootDir | Component::CurDir => (),
                Component::Normal(value) => {
                    dir = dir_at(&dir, value.to_str().context("non-UTF8 root component")?)?;
                }
                _ => bail!("partial root parent traversal"),
            }
        }
        Ok(Self { dir })
    }
    pub(crate) fn file_parent(path: &Path) -> Result<(Self, String)> {
        ensure!(
            path.as_os_str().as_bytes().len() <= 4096,
            "partial input path length"
        );
        let leaf = path
            .file_name()
            .and_then(|s| s.to_str())
            .context("partial file basename")?;
        ensure!(
            leaf.len() <= 255 && leaf != "." && leaf != "..",
            "partial file basename bound"
        );
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        Ok((Self::open(parent)?, leaf.to_owned()))
    }
    pub(crate) fn new_destination(path: &Path) -> Result<Self> {
        let (parent, leaf) = Self::file_parent(path)?;
        let leaf = name(&leaf)?;
        ensure!(
            unsafe { libc::mkdirat(parent.dir.as_raw_fd(), leaf.as_ptr(), 0o700) } == 0,
            "partial destination must be new: {}",
            std::io::Error::last_os_error()
        );
        parent.dir.sync_all()?;
        Ok(Self {
            dir: dir_at(&parent.dir, leaf.to_str()?)?,
        })
    }
    fn parent(&self, relative: &str, create: bool) -> Result<(File, CString)> {
        validate_path(relative)?;
        let mut parts = relative.split('/').peekable();
        let mut dir = self.dir.try_clone()?;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() {
                return Ok((dir, name(part)?));
            }
            if create {
                let item = name(part)?;
                let result = unsafe { libc::mkdirat(dir.as_raw_fd(), item.as_ptr(), 0o700) };
                if result != 0
                    && std::io::Error::last_os_error().raw_os_error() != Some(libc::EEXIST)
                {
                    return Err(std::io::Error::last_os_error().into());
                }
                if result == 0 {
                    #[cfg(test)]
                    fault("intermediate-sync")?;
                    dir.sync_all()?;
                    let child = dir_at(&dir, part)?;
                    child.sync_all()?;
                }
            }
            dir = dir_at(&dir, part)?;
        }
        bail!("partial empty relative path")
    }
    pub(crate) fn open_file(&self, relative: &str) -> Result<File> {
        let (dir, leaf) = self.parent(relative, false)?;
        let file = opened(unsafe {
            libc::openat(
                dir.as_raw_fd(),
                leaf.as_ptr(),
                libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
            )
        })?;
        let meta = file.metadata()?;
        ensure!(
            meta.is_file() && meta.nlink() == 1,
            "partial input is not a single-link regular file"
        );
        Ok(file)
    }
    pub(crate) fn child(&self, relative: &str) -> Result<Self> {
        let (dir, leaf) = self.parent(relative, false)?;
        Ok(Self {
            dir: dir_at(&dir, leaf.to_str()?)?,
        })
    }
    pub(crate) fn create_file(&self, relative: &str) -> Result<File> {
        let (dir, leaf) = self.parent(relative, true)?;
        opened(unsafe {
            libc::openat(
                dir.as_raw_fd(),
                leaf.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        })
    }
    pub(crate) fn anchor_bytes(&self, relative: &str, budget: &mut Budget) -> Result<Vec<u8>> {
        budget.charge(
            relative
                .len()
                .checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .context("partial path reservation")?,
            "anchor opened path",
        )?;
        let mut file = self.open_file(relative)?;
        let size = file.metadata()?.len();
        ensure!(
            size <= super::budget::ANCHOR_MAX,
            "consumer_resource_limit: partial anchor"
        );
        budget.raw(size)?;
        exact_bytes(&mut file, size, "", budget)
    }
    pub(crate) fn write_bytes(
        &self,
        relative: &str,
        bytes: &[u8],
        budget: &mut Budget,
    ) -> Result<()> {
        budget.charge(
            relative
                .len()
                .checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .context("partial path reservation")?,
            "written path components",
        )?;
        let mut file = self.create_file(relative)?;
        counted_write(&mut file, bytes, budget)?;
        #[cfg(test)]
        fault("file-sync")?;
        file.sync_all()?;
        self.sync_parent(relative)
    }
    pub(crate) fn sync_parent(&self, relative: &str) -> Result<()> {
        let (parent, _) = self.parent(relative, false)?;
        parent.sync_all()?;
        self.dir.sync_all()?;
        Ok(())
    }
    pub(crate) fn rename_new(&self, from: &str, to: &str) -> Result<()> {
        #[cfg(test)]
        fault("rename-before")?;
        let (old_dir, old) = self.parent(from, false)?;
        let (new_dir, new) = self.parent(to, true)?;
        if let Err(error) = rename_noreplace(&old_dir, &old, &new_dir, &new) {
            ensure!(
                matches!(
                    error.raw_os_error(),
                    Some(libc::ENOSYS | libc::EINVAL | libc::EOPNOTSUPP)
                ),
                "partial exclusive rename: {error}"
            );
            // Some supported Linux mounts (including the migration NFS mount)
            // lack RENAME_NOREPLACE. linkat atomically installs a new name and
            // refuses any existing name. Never fall back to overwriting rename.
            let stage = opened(unsafe {
                libc::openat(
                    old_dir.as_raw_fd(),
                    old.as_ptr(),
                    libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
                )
            })?;
            let metadata = stage.metadata()?;
            ensure!(
                metadata.is_file() && metadata.nlink() == 1,
                "partial link-install source must be one regular staged file"
            );
            #[cfg(test)]
            LINK_HOOK.with(|hook| {
                if let Some(run) = hook.borrow_mut().take() {
                    run();
                }
            });
            ensure!(
                unsafe {
                    libc::linkat(
                        old_dir.as_raw_fd(),
                        old.as_ptr(),
                        new_dir.as_raw_fd(),
                        new.as_ptr(),
                        0,
                    )
                } == 0,
                "partial exclusive link installation: {}",
                std::io::Error::last_os_error()
            );
            // A retained stage FD binds the intended inode. The name used by
            // linkat may have been replaced after open; reject that case before
            // removing any staged name. Final audit separately binds all bytes.
            for (directory, leaf) in [(&old_dir, &old), (&new_dir, &new)] {
                let linked = opened(unsafe {
                    libc::openat(
                        directory.as_raw_fd(),
                        leaf.as_ptr(),
                        libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
                    )
                })?;
                let current = linked.metadata()?;
                ensure!(
                    current.is_file()
                        && current.dev() == metadata.dev()
                        && current.ino() == metadata.ino()
                        && current.nlink() == 2,
                    "partial link-install source identity changed"
                );
            }
            #[cfg(test)]
            fault("link-after")?;
            // Flush the new name before removing the old one. A failure at
            // either boundary retains the incomplete marker and both names;
            // no later command adopts that destination or infers rollback.
            #[cfg(test)]
            fault("link-directory-sync")?;
            new_dir.sync_all()?;
            #[cfg(test)]
            fault("link-unlink-before")?;
            ensure!(
                unsafe { libc::unlinkat(old_dir.as_raw_fd(), old.as_ptr(), 0) } == 0,
                "partial staged link cleanup: {}",
                std::io::Error::last_os_error()
            );
        }
        #[cfg(test)]
        fault("rename-after")?;
        old_dir.sync_all()?;
        new_dir.sync_all()?;
        self.dir.sync_all()?;
        Ok(())
    }
    pub(crate) fn remove_file(&self, relative: &str) -> Result<()> {
        #[cfg(test)]
        fault("marker-remove")?;
        let (dir, leaf) = self.parent(relative, false)?;
        ensure!(
            unsafe { libc::unlinkat(dir.as_raw_fd(), leaf.as_ptr(), 0) } == 0,
            "partial marker removal: {}",
            std::io::Error::last_os_error()
        );
        dir.sync_all()?;
        self.dir.sync_all()?;
        Ok(())
    }
    pub(crate) fn space(&self, required: u64) -> Result<()> {
        let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
        ensure!(
            unsafe { libc::fstatvfs(self.dir.as_raw_fd(), stat.as_mut_ptr()) } == 0,
            "partial destination free-space observation failed"
        );
        let stat = unsafe { stat.assume_init() };
        let available = u128::from(stat.f_bavail) * u128::from(stat.f_frsize);
        ensure!(
            available >= u128::from(required),
            "consumer_resource_limit: insufficient observed destination space"
        );
        Ok(())
    }
    pub(crate) fn inventory(&self, budget: &mut Budget) -> Result<BTreeSet<(String, bool)>> {
        let mut result = BTreeSet::new();
        self.walk("", budget, &mut result)?;
        Ok(result)
    }
    fn walk(
        &self,
        prefix: &str,
        budget: &mut Budget,
        result: &mut BTreeSet<(String, bool)>,
    ) -> Result<()> {
        let duplicate = dir_at(&self.dir, ".")?.into_raw_fd();
        ensure!(duplicate >= 0, "partial directory descriptor duplication");
        let stream = unsafe { libc::fdopendir(duplicate) };
        if stream.is_null() {
            unsafe {
                libc::close(duplicate);
            }
            return Err(std::io::Error::last_os_error().into());
        }
        struct Stream(*mut libc::DIR);
        impl Drop for Stream {
            fn drop(&mut self) {
                unsafe {
                    libc::closedir(self.0);
                }
            }
        }
        let stream = Stream(stream);
        unsafe {
            libc::rewinddir(stream.0);
        }
        loop {
            unsafe {
                *libc::__errno_location() = 0;
            }
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                ensure!(
                    std::io::Error::last_os_error().raw_os_error() == Some(0),
                    "partial directory enumeration failed"
                );
                break;
            }
            let item = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_str()?;
            if item == "." || item == ".." {
                continue;
            }
            ensure!(
                result.len() < 200_000,
                "consumer_resource_limit: directory entries"
            );
            let size = prefix
                .len()
                .checked_add(item.len())
                .and_then(|n| n.checked_add(1))
                .context("partial inventory path overflow")?;
            budget.charge(
                size.checked_mul(8)
                    .and_then(|n| n.checked_add(1024))
                    .context("partial inventory allocation overflow")?,
                "inventory paths/set",
            )?;
            let path = if prefix.is_empty() {
                item.to_owned()
            } else {
                format!("{prefix}/{item}")
            };
            validate_path(&path)?;
            let item_c = name(item)?;
            let file = opened(unsafe {
                libc::openat(
                    self.dir.as_raw_fd(),
                    item_c.as_ptr(),
                    libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
                )
            })?;
            let meta = file.metadata()?;
            ensure!(
                meta.is_file() || meta.is_dir(),
                "partial nonregular namespace member"
            );
            ensure!(
                !meta.is_file() || meta.nlink() == 1,
                "partial hardlink member"
            );
            let is_dir = meta.is_dir();
            ensure!(
                result.insert((path.clone(), is_dir)),
                "partial duplicate inventory path"
            );
            if is_dir {
                Self { dir: file }.walk(&path, budget, result)?;
            }
        }
        Ok(())
    }
}

fn rename_noreplace(old_dir: &File, old: &CStr, new_dir: &File, new: &CStr) -> std::io::Result<()> {
    #[cfg(test)]
    if FORCE_LINK.with(std::cell::Cell::get) {
        return Err(std::io::Error::from_raw_os_error(libc::EOPNOTSUPP));
    }
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            old_dir.as_raw_fd(),
            old.as_ptr(),
            new_dir.as_raw_fd(),
            new.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

pub(crate) fn exact_bytes(
    reader: &mut dyn Read,
    size: u64,
    sha: &str,
    budget: &mut Budget,
) -> Result<Vec<u8>> {
    let size = usize::try_from(size)?;
    let mut raw = Vec::new();
    raw.try_reserve_exact(size)?;
    raw.resize(size, 0);
    let mut offset = 0;
    while offset < raw.len() {
        let count = reader.read(&mut raw[offset..])?;
        budget.read(count as u64)?;
        ensure!(count > 0, "partial object truncated");
        offset += count;
    }
    let mut extra = [0];
    let count = reader.read(&mut extra)?;
    budget.read(count as u64)?;
    ensure!(count == 0, "partial object exceeds declared length");
    ensure!(
        sha.is_empty() || crate::common::digest(&raw) == sha,
        "partial consumed object digest mismatch"
    );
    Ok(raw)
}

pub(crate) fn copy_exact(
    reader: &mut dyn Read,
    output: &mut File,
    size: u64,
    sha: &str,
    budget: &mut Budget,
) -> Result<()> {
    budget.charge(256, "stream digest formatting")?;
    let mut remaining = size;
    let mut buffer = [0; 65536];
    let mut hash = Sha256::new();
    while remaining > 0 {
        let limit = usize::try_from(remaining.min(buffer.len() as u64))?;
        let count = reader.read(&mut buffer[..limit])?;
        budget.read(count as u64)?;
        ensure!(count > 0, "partial artifact truncated");
        counted_write(output, &buffer[..count], budget)?;
        hash.update(&buffer[..count]);
        remaining -= count as u64;
    }
    let mut extra = [0];
    let count = reader.read(&mut extra)?;
    budget.read(count as u64)?;
    ensure!(count == 0, "partial artifact exceeds declared length");
    ensure!(
        format!("{:x}", hash.finalize()) == sha,
        "partial artifact digest mismatch"
    );
    output.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_depth_and_allocation_preflight() {
        let mut b = Budget::new();
        assert!(typed::<serde_json::Value>(br#"{"a":0,"\u0061":1}"#, &mut b).is_err());
        let raw = format!("{}0{}", "[".repeat(65), "]".repeat(65));
        assert!(typed::<serde_json::Value>(raw.as_bytes(), &mut b).is_err());
        let mut b = Budget::with_limit(1023);
        assert!(typed::<serde_json::Value>(b"null", &mut b).is_err());
        assert_eq!(b.used(), 0);
    }
    #[test]
    fn exact_reader_stops_at_first_extra_byte() {
        struct Infinite(usize);
        impl Read for Infinite {
            fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
                self.0 += out.len();
                assert!(self.0 <= 4);
                out.fill(b'x');
                Ok(out.len())
            }
        }
        let mut reader = Infinite(0);
        assert!(exact_bytes(&mut reader, 3, "", &mut Budget::new()).is_err());
        assert_eq!(reader.0, 4);
    }
    #[test]
    fn nofollow_ancestor_fifo_and_same_size_replacement() {
        use std::os::unix::fs::symlink;
        let temp = tempfile::tempdir().unwrap();
        std::fs::create_dir(temp.path().join("d")).unwrap();
        std::fs::write(temp.path().join("d/x"), b"old").unwrap();
        let root = Root::open(temp.path()).unwrap();
        let mut pinned = root.open_file("d/x").unwrap();
        std::fs::rename(temp.path().join("d"), temp.path().join("saved")).unwrap();
        symlink(temp.path().join("saved"), temp.path().join("d")).unwrap();
        assert!(root.open_file("d/x").is_err());
        assert_eq!(
            exact_bytes(
                &mut pinned,
                3,
                &crate::common::digest(b"old"),
                &mut Budget::new()
            )
            .unwrap(),
            b"old"
        );
        std::fs::write(temp.path().join("saved/x"), b"new").unwrap();
        let mut changed = root.open_file("saved/x").unwrap();
        assert!(exact_bytes(
            &mut changed,
            3,
            &crate::common::digest(b"old"),
            &mut Budget::new()
        )
        .is_err());
        std::fs::hard_link(temp.path().join("saved/x"), temp.path().join("hardlink")).unwrap();
        assert!(root.open_file("saved/x").is_err());
    }
    #[test]
    #[ignore = "bounded subprocess child only"]
    fn fifo_child() {
        let path = std::env::var_os("PARTIAL_FIFO_ROOT").expect("parent fixture");
        assert!(Root::open(Path::new(&path))
            .unwrap()
            .open_file("fifo")
            .is_err());
    }
    #[test]
    fn fifo_open_is_bounded_in_a_subprocess() {
        let temp = tempfile::tempdir().unwrap();
        let fifo = CString::new(temp.path().join("fifo").as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "partial::io::tests::fifo_child", "--ignored"])
            .env("PARTIAL_FIFO_ROOT", temp.path());
        let result = crate::transport::run_bounded(
            &mut command,
            std::time::Duration::from_secs(5),
            16384,
            None,
        )
        .unwrap();
        crate::transport::require_success(&result, "FIFO sentinel").unwrap();
    }
    #[test]
    fn partial_write_counts_only_accepted_bytes_and_refuses_before_write() {
        struct Broken(usize);
        impl Write for Broken {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if self.0 == 2 {
                    return Err(std::io::Error::other("injected write failure"));
                }
                let n = bytes.len().min(2);
                self.0 += n;
                Ok(n)
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut b = Budget::new();
        b.reserve_disk(4).unwrap();
        let mut output = Broken(0);
        assert!(counted_write(&mut output, b"four", &mut b).is_err());
        assert_eq!(b.written_bytes, 2);
        let mut b = Budget::new();
        let mut output = Broken(0);
        assert!(counted_write(&mut output, b"a", &mut b).is_err());
        assert_eq!(output.0, 0);
    }
}

#[cfg(test)]
thread_local! { static FAULT: std::cell::RefCell<Option<&'static str>> = const { std::cell::RefCell::new(None) }; }
#[cfg(test)]
thread_local! { static FORCE_LINK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
#[cfg(test)]
thread_local! { static LINK_HOOK: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const { std::cell::RefCell::new(None) }; }
#[cfg(test)]
pub(crate) fn on_link_source_open(hook: impl FnOnce() + 'static) {
    LINK_HOOK.with(|value| *value.borrow_mut() = Some(Box::new(hook)));
}
#[cfg(test)]
pub(crate) struct LinkGuard;
#[cfg(test)]
impl Drop for LinkGuard {
    fn drop(&mut self) {
        FORCE_LINK.with(|v| v.set(false));
        LINK_HOOK.with(|v| *v.borrow_mut() = None);
    }
}
#[cfg(test)]
pub(crate) fn force_link_install() -> LinkGuard {
    FORCE_LINK.with(|v| v.set(true));
    LinkGuard
}
#[cfg(test)]
pub(crate) struct FaultGuard;
#[cfg(test)]
impl Drop for FaultGuard {
    fn drop(&mut self) {
        FAULT.with(|f| *f.borrow_mut() = None);
    }
}
#[cfg(test)]
pub(crate) fn inject_fault(point: &'static str) -> FaultGuard {
    FAULT.with(|f| *f.borrow_mut() = Some(point));
    FaultGuard
}
#[cfg(test)]
fn fault(point: &str) -> Result<()> {
    ensure!(
        !FAULT.with(|f| f.borrow().is_some_and(|p| p == point)),
        "injected partial I/O failure: {point}"
    );
    Ok(())
}

fn counted_write(output: &mut dyn Write, mut bytes: &[u8], budget: &mut Budget) -> Result<()> {
    ensure!(
        budget
            .written_bytes
            .checked_add(bytes.len() as u64)
            .is_some_and(|n| n <= budget.disk_reserved),
        "consumer_resource_limit: destination reservation"
    );
    while !bytes.is_empty() {
        let count = match output.write(bytes) {
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        ensure!(count > 0, "partial destination write zero");
        budget.written(count as u64)?;
        bytes = &bytes[count..];
    }
    Ok(())
}
