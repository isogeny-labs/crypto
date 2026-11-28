//! Concrete segmented sequential bit tapes for the constructive algorithms.
//!
//! Logical lengths, positions and movement receipts are arbitrary-precision
//! integers. Fixed network role identifiers use `u128`; no vector indexed by
//! the manuscript's more-than-2^64 roles is allocated. Only one 64 KiB segment
//! is resident per open tape. Filesystem failures are reported as native
//! resource failures, without substituting a different permutation algorithm.
//!
//! Receipts count the represented one-dimensional tape operations. They are
//! not wall-clock measurements or a proof that this filesystem interpreter
//! has constant host overhead per represented tape step.

use crate::bigint::Big;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

const SEGMENT_BYTES: usize = 1 << 16;
const SEGMENT_BITS: u64 = (SEGMENT_BYTES as u64) * 8;
const HEADER_MAGIC: &[u8; 8] = b"SNLTAPE1";
static NEXT_WORKSPACE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TapeCost {
    pub read_bits: Big,
    pub written_bits: Big,
    pub head_moves: Big,
    pub erased_bits: Big,
    pub header_bits: Big,
    pub segment_loads: Big,
    pub segment_stores: Big,
}
impl Default for TapeCost {
    fn default() -> Self {
        Self {
            read_bits: Big::zero(),
            written_bits: Big::zero(),
            head_moves: Big::zero(),
            erased_bits: Big::zero(),
            header_bits: Big::zero(),
            segment_loads: Big::zero(),
            segment_stores: Big::zero(),
        }
    }
}

fn increment(value: &mut Big) {
    for limb in &mut value.0 {
        let (next, carry) = limb.overflowing_add(1);
        *limb = next;
        if !carry {
            return;
        }
    }
    value.0.push(1);
}
fn decrement(value: &mut Big) {
    debug_assert!(!value.is_zero());
    for limb in &mut value.0 {
        let (next, borrow) = limb.overflowing_sub(1);
        *limb = next;
        if !borrow {
            break;
        }
    }
    while value.0.last() == Some(&0) {
        value.0.pop();
    }
}

/// A private directory owned by this workspace. Dropping it does not erase
/// files: callers explicitly clean up only after all desired output is read.
pub struct TapeWorkspace {
    root: PathBuf,
    next_id: u128,
    free_ids: Vec<u128>,
    open: Rc<RefCell<BTreeSet<u128>>>,
    cost: Rc<RefCell<TapeCost>>,
}

impl TapeWorkspace {
    pub fn create_in(parent: impl AsRef<Path>) -> io::Result<Self> {
        fs::create_dir_all(parent.as_ref())?;
        let serial = NEXT_WORKSPACE.fetch_add(1, Ordering::Relaxed);
        let root = parent
            .as_ref()
            .join(format!("subnlogn-tapes-{}-{serial}", std::process::id()));
        // create_dir, rather than create_dir_all, prevents adopting old data.
        fs::create_dir(&root)?;
        Ok(Self {
            root,
            next_id: 0,
            free_ids: Vec::new(),
            open: Rc::new(RefCell::new(BTreeSet::new())),
            cost: Rc::new(RefCell::new(TapeCost::default())),
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn cost(&self) -> TapeCost {
        self.cost.borrow().clone()
    }

    pub fn create_tape(&mut self) -> io::Result<Tape> {
        if let Some(id) = self.free_ids.pop() {
            return self.open_tape(id);
        }
        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| io::Error::other("tape identifier overflow"))?;
        if id >= (1u128 << 120) {
            return Err(io::Error::other("temporary tape identifier exhausted"));
        }
        self.open_tape(id)
    }

    /// Recycle a temporary work tape after erasing its visited region.
    pub fn release_tape(&mut self, mut tape: Tape) -> io::Result<()> {
        let id = tape.id();
        if id >= (1u128 << 120) {
            return Err(io::Error::other("a fixed role tape is not a temporary"));
        }
        tape.clear()?;
        tape.flush()?;
        drop(tape);
        self.free_ids.push(id);
        Ok(())
    }

    /// Open a fixed physical complex-network role. The network's W role
    /// tapes are reused at all recursion depths, as in Section 5.
    pub fn complex_role(&mut self, role: u128) -> io::Result<Tape> {
        if role >= super::tensor::COMPLEX_WIRES {
            return Err(io::Error::other("complex role out of range"));
        }
        self.open_tape((1u128 << 120) + role)
    }

    pub fn bit_role(&mut self, role: u128) -> io::Result<Tape> {
        const W: u128 = 177_176_569_091_445_000_000;
        if role >= W {
            return Err(io::Error::other("bit role out of range"));
        }
        self.open_tape((1u128 << 121) + role)
    }

    pub fn reopen(&mut self, id: u128) -> io::Result<Tape> {
        self.open_tape(id)
    }

    fn open_tape(&mut self, id: u128) -> io::Result<Tape> {
        if !self.open.borrow_mut().insert(id) {
            return Err(io::Error::other("the same tape cannot be opened twice"));
        }
        let directory = self.root.join(format!("tape-{id:032x}"));
        let result = (|| {
            fs::create_dir_all(&directory)?;
            let header = directory.join("length.bin");
            let length = match File::open(&header) {
                Ok(mut file) => {
                    let mut bytes = Vec::new();
                    file.read_to_end(&mut bytes)?;
                    let mut cost = self.cost.borrow_mut();
                    cost.header_bits = cost
                        .header_bits
                        .add(&Big::from_u128(bytes.len() as u128 * 8));
                    decode_length(&bytes)?
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => Big::zero(),
                Err(e) => return Err(e),
            };
            let position = match File::open(directory.join("position.bin")) {
                Ok(mut file) => {
                    let mut bytes = Vec::new();
                    file.read_to_end(&mut bytes)?;
                    let mut cost = self.cost.borrow_mut();
                    cost.header_bits = cost
                        .header_bits
                        .add(&Big::from_u128(bytes.len() as u128 * 8));
                    decode_length(&bytes)?
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => Big::zero(),
                Err(e) => return Err(e),
            };
            if position > length {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "tape head past its logical end",
                ));
            }
            let remaining = length.sub(&position);
            let (segment, offset) = position.divrem_small(SEGMENT_BITS);
            Ok(Tape {
                id,
                directory,
                position,
                remaining,
                length,
                segment,
                offset,
                loaded: false,
                dirty: false,
                metadata_dirty: true,
                buffer: vec![0; SEGMENT_BYTES].into_boxed_slice(),
                cost: self.cost.clone(),
                open: self.open.clone(),
            })
        })();
        if result.is_err() {
            self.open.borrow_mut().remove(&id);
        }
        result
    }

    pub fn cleanup(self) -> io::Result<()> {
        if !self.open.borrow().is_empty() {
            return Err(io::Error::other(
                "cannot remove a workspace with open tapes",
            ));
        }
        fs::remove_dir_all(self.root)
    }
}

fn encode_length(length: &Big) -> io::Result<Vec<u8>> {
    let limbs = u64::try_from(length.0.len())
        .map_err(|_| io::Error::other("descriptor length overflow"))?;
    let mut result = Vec::with_capacity(16 + length.0.len() * 8);
    result.extend(HEADER_MAGIC);
    result.extend(limbs.to_le_bytes());
    for &limb in &length.0 {
        result.extend(limb.to_le_bytes());
    }
    Ok(result)
}
fn decode_length(bytes: &[u8]) -> io::Result<Big> {
    if bytes.len() < 16 || &bytes[..8] != HEADER_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid tape descriptor",
        ));
    }
    let limbs = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
    let limbs =
        usize::try_from(limbs).map_err(|_| io::Error::other("native descriptor size overflow"))?;
    if limbs.checked_mul(8).and_then(|n| n.checked_add(16)) != Some(bytes.len()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "truncated tape descriptor",
        ));
    }
    let result = Big::from_limbs(
        &bytes[16..]
            .as_chunks::<8>()
            .0
            .iter()
            .map(|chunk| u64::from_le_bytes(*chunk))
            .collect::<Vec<_>>(),
    );
    if result.0.len() != limbs {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "noncanonical tape descriptor",
        ));
    }
    Ok(result)
}

pub struct Tape {
    id: u128,
    directory: PathBuf,
    length: Big,
    position: Big,
    remaining: Big,
    segment: Big,
    offset: u64,
    loaded: bool,
    dirty: bool,
    metadata_dirty: bool,
    buffer: Box<[u8]>,
    cost: Rc<RefCell<TapeCost>>,
    open: Rc<RefCell<BTreeSet<u128>>>,
}

impl Tape {
    pub fn id(&self) -> u128 {
        self.id
    }
    pub fn len(&self) -> &Big {
        &self.length
    }
    pub fn position(&self) -> &Big {
        &self.position
    }
    pub fn is_empty(&self) -> bool {
        self.length.is_zero()
    }

    fn segment_path(&self) -> PathBuf {
        self.directory
            .join(format!("segment-{}.bin", self.segment.to_dec()))
    }

    fn load(&mut self) -> io::Result<()> {
        if self.loaded {
            return Ok(());
        }
        self.buffer.fill(0);
        match File::open(self.segment_path()) {
            Ok(mut file) => {
                let mut filled = 0;
                while filled < self.buffer.len() {
                    let n = file.read(&mut self.buffer[filled..])?;
                    if n == 0 {
                        break;
                    }
                    filled += n;
                }
                if filled != self.buffer.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "truncated tape segment",
                    ));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound && self.position >= self.length => {}
            Err(e) => return Err(e),
        }
        self.loaded = true;
        increment(&mut self.cost.borrow_mut().segment_loads);
        Ok(())
    }

    fn flush_segment(&mut self) -> io::Result<()> {
        if self.dirty {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.segment_path())?;
            file.write_all(&self.buffer)?;
            self.dirty = false;
            increment(&mut self.cost.borrow_mut().segment_stores);
        }
        Ok(())
    }

    fn advance(&mut self) -> io::Result<()> {
        increment(&mut self.position);
        self.metadata_dirty = true;
        self.offset += 1;
        increment(&mut self.cost.borrow_mut().head_moves);
        if self.offset == SEGMENT_BITS {
            self.flush_segment()?;
            increment(&mut self.segment);
            self.offset = 0;
            self.loaded = false;
        }
        Ok(())
    }

    pub fn read_bit(&mut self) -> io::Result<Option<bool>> {
        if self.remaining.is_zero() {
            return Ok(None);
        }
        self.load()?;
        let bit = (self.buffer[(self.offset / 8) as usize] >> (7 - self.offset % 8)) & 1 != 0;
        decrement(&mut self.remaining);
        increment(&mut self.cost.borrow_mut().read_bits);
        self.advance()?;
        Ok(Some(bit))
    }

    pub fn require_bit(&mut self) -> io::Result<bool> {
        self.read_bit()?
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "short logical tape"))
    }

    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.load()?;
        let byte = &mut self.buffer[(self.offset / 8) as usize];
        let mask = 1u8 << (7 - self.offset % 8);
        if bit {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
        self.dirty = true;
        if self.remaining.is_zero() {
            increment(&mut self.length);
            self.metadata_dirty = true;
        } else {
            decrement(&mut self.remaining);
        }
        increment(&mut self.cost.borrow_mut().written_bits);
        self.advance()
    }

    fn retreat(&mut self) -> io::Result<()> {
        if self.position.is_zero() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "tape head at origin",
            ));
        }
        if self.offset == 0 {
            self.flush_segment()?;
            decrement(&mut self.segment);
            self.offset = SEGMENT_BITS;
            self.loaded = false;
        }
        self.offset -= 1;
        decrement(&mut self.position);
        increment(&mut self.remaining);
        increment(&mut self.cost.borrow_mut().head_moves);
        self.metadata_dirty = true;
        self.load()
    }

    /// Move left and overwrite that cell. This is used to restore a parked
    /// stream in forward order while popping the parking stack backwards.
    pub fn write_previous_bit(&mut self, bit: bool) -> io::Result<()> {
        self.retreat()?;
        let byte = &mut self.buffer[(self.offset / 8) as usize];
        let mask = 1u8 << (7 - self.offset % 8);
        if bit {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
        self.dirty = true;
        increment(&mut self.cost.borrow_mut().written_bits);
        Ok(())
    }

    /// Pop and erase one cell at the current logical stack top. It never
    /// scans or rewinds through a parked ancestor.
    pub fn pop_bit(&mut self) -> io::Result<bool> {
        if !self.remaining.is_zero() {
            return Err(io::Error::other("pop requires the head at the stack top"));
        }
        self.retreat()?;
        let byte = &mut self.buffer[(self.offset / 8) as usize];
        let mask = 1u8 << (7 - self.offset % 8);
        let bit = *byte & mask != 0;
        *byte &= !mask;
        self.dirty = true;
        decrement(&mut self.length);
        self.remaining = Big::zero();
        self.metadata_dirty = true;
        let mut cost = self.cost.borrow_mut();
        increment(&mut cost.read_bits);
        increment(&mut cost.written_bits);
        increment(&mut cost.erased_bits);
        Ok(bit)
    }

    /// Return the represented tape head to its origin. The full traversed
    /// distance is charged; this is not an uncharged random seek primitive.
    pub fn rewind(&mut self) -> io::Result<()> {
        self.flush_segment()?;
        let mut cost = self.cost.borrow_mut();
        cost.head_moves = cost.head_moves.add(&self.position);
        drop(cost);
        self.position = Big::zero();
        self.remaining = self.length.clone();
        self.segment = Big::zero();
        self.offset = 0;
        self.loaded = false;
        self.metadata_dirty = true;
        Ok(())
    }

    /// Visit and erase only this tape's current logical region, then rewind.
    /// A reusable work tape never scans a parked ancestor's data.
    pub fn clear(&mut self) -> io::Result<()> {
        self.rewind()?;
        let mut count = self.length.clone();
        while !count.is_zero() {
            self.write_bit(false)?;
            decrement(&mut count);
        }
        {
            let mut cost = self.cost.borrow_mut();
            cost.erased_bits = cost.erased_bits.add(&self.length);
        }
        self.rewind()?;
        self.length = Big::zero();
        self.remaining = Big::zero();
        self.metadata_dirty = true;
        self.flush()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.flush_segment()?;
        if self.metadata_dirty {
            let bytes = encode_length(&self.length)?;
            let mut file = File::create(self.directory.join("length.bin"))?;
            file.write_all(&bytes)?;
            let position = encode_length(&self.position)?;
            let mut head_file = File::create(self.directory.join("position.bin"))?;
            head_file.write_all(&position)?;
            let mut cost = self.cost.borrow_mut();
            cost.header_bits = cost
                .header_bits
                .add(&Big::from_u128((bytes.len() + position.len()) as u128 * 8));
            self.metadata_dirty = false;
        }
        Ok(())
    }

    pub fn copy_to(&mut self, output: &mut Tape) -> io::Result<()> {
        if self.id == output.id {
            return Err(io::Error::other("copy source and target must differ"));
        }
        self.rewind()?;
        output.clear()?;
        while let Some(bit) = self.read_bit()? {
            output.write_bit(bit)?;
        }
        self.rewind()?;
        output.rewind()?;
        output.flush()
    }

    pub fn copy_bits_to(&mut self, output: &mut Tape, count: &Big) -> io::Result<()> {
        let mut remaining = count.clone();
        while !remaining.is_zero() {
            output.write_bit(self.require_bit()?)?;
            decrement(&mut remaining);
        }
        Ok(())
    }

    pub fn skip_bits(&mut self, count: &Big) -> io::Result<()> {
        let mut remaining = count.clone();
        while !remaining.is_zero() {
            self.require_bit()?;
            decrement(&mut remaining);
        }
        Ok(())
    }

    pub fn write_zero_bits(&mut self, count: &Big) -> io::Result<()> {
        let mut remaining = count.clone();
        while !remaining.is_zero() {
            self.write_bit(false)?;
            decrement(&mut remaining);
        }
        Ok(())
    }

    /// Fixed-width unsigned MSB-first integer record, checked before writing.
    pub fn write_unsigned(&mut self, value: &Big, width: usize) -> io::Result<()> {
        if value.bits() > width {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unsigned record overflow",
            ));
        }
        for bit in (0..width).rev() {
            self.write_bit(value.bit(bit))?;
        }
        Ok(())
    }

    pub fn read_unsigned(&mut self, width: usize) -> io::Result<Big> {
        let mut limbs = vec![0u64; width.div_ceil(64)];
        for bit in (0..width).rev() {
            if self.require_bit()? {
                limbs[bit / 64] |= 1u64 << (bit % 64);
            }
        }
        Ok(Big::from_limbs(&limbs))
    }
}

impl Drop for Tape {
    fn drop(&mut self) {
        // Algorithms call flush explicitly before treating an output as done.
        // Drop is a last-resort persistence attempt, never a success receipt.
        let _ = self.flush();
        self.open.borrow_mut().remove(&self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_segments_reopen_and_do_not_alias_wide_roles() {
        let mut workspace = TapeWorkspace::create_in(std::env::temp_dir()).unwrap();
        let mut tape = workspace
            .complex_role(super::super::tensor::COMPLEX_WIRES - 1)
            .unwrap();
        let id = tape.id();
        for i in 0..SEGMENT_BITS + 19 {
            tape.write_bit(i % 7 == 0).unwrap();
        }
        tape.flush().unwrap();
        assert_eq!(tape.len(), &Big::from_u64(SEGMENT_BITS + 19));
        drop(tape);
        let mut tape = workspace.reopen(id).unwrap();
        assert_eq!(tape.position(), &Big::from_u64(SEGMENT_BITS + 19));
        tape.rewind().unwrap();
        for i in 0..SEGMENT_BITS + 19 {
            assert_eq!(tape.read_bit().unwrap(), Some(i % 7 == 0));
        }
        assert_eq!(tape.read_bit().unwrap(), None);
        tape.rewind().unwrap();
        assert_eq!(tape.require_bit().unwrap(), true);
        let mut copy = workspace.create_tape().unwrap();
        tape.copy_to(&mut copy).unwrap();
        assert_eq!(copy.len(), tape.len());
        let copy_id = copy.id();
        copy.clear().unwrap();
        assert!(copy.is_empty());
        assert!(workspace.reopen(copy_id).is_err());
        drop(copy);
        drop(tape);
        let cost = workspace.cost();
        assert!(cost.read_bits > Big::zero());
        assert!(cost.head_moves > cost.read_bits);
        workspace.cleanup().unwrap();
    }

    #[test]
    fn descriptors_and_fixed_width_records_are_exact() {
        let huge = Big::from_u64(1).shl(333).add_small(71);
        assert_eq!(decode_length(&encode_length(&huge).unwrap()).unwrap(), huge);
        let mut workspace = TapeWorkspace::create_in(std::env::temp_dir()).unwrap();
        let mut tape = workspace.create_tape().unwrap();
        tape.write_unsigned(&huge, 340).unwrap();
        assert!(tape.write_unsigned(&huge, 333).is_err());
        tape.rewind().unwrap();
        assert_eq!(tape.read_unsigned(340).unwrap(), huge);
        drop(tape);
        workspace.cleanup().unwrap();
    }

    #[test]
    fn stack_restore_does_not_scan_parked_ancestors() {
        let mut workspace = TapeWorkspace::create_in(std::env::temp_dir()).unwrap();
        let mut parking = workspace.create_tape().unwrap();
        for bit in [true, false, true] {
            parking.write_bit(bit).unwrap();
        }
        let payload: Vec<_> = (0..31).map(|i| i % 3 == 0).collect();
        for &bit in &payload {
            parking.write_bit(bit).unwrap();
        }
        let mut restored = workspace.create_tape().unwrap();
        restored.write_zero_bits(&Big::from_u64(31)).unwrap();
        let before = workspace.cost().head_moves;
        for _ in 0..31 {
            restored
                .write_previous_bit(parking.pop_bit().unwrap())
                .unwrap();
        }
        assert_eq!(workspace.cost().head_moves.sub(&before), Big::from_u64(62));
        assert_eq!(parking.len(), &Big::from_u64(3));
        assert_eq!(parking.position(), &Big::from_u64(3));
        for bit in payload {
            assert_eq!(restored.require_bit().unwrap(), bit);
        }
        assert_eq!(parking.pop_bit().unwrap(), true);
        assert_eq!(parking.pop_bit().unwrap(), false);
        assert_eq!(parking.pop_bit().unwrap(), true);
        workspace.release_tape(parking).unwrap();
        workspace.release_tape(restored).unwrap();
        let reused = workspace.create_tape().unwrap();
        assert!(reused.is_empty());
        drop(reused);
        workspace.cleanup().unwrap();
    }
}
