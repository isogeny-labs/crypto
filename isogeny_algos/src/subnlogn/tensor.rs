//! Constructive binary phase frames at the manuscript's published `h = 100`.
//!
//! The residuals are the actual million-dimensional subspaces in Section 3,
//! not a smaller substitute network. Their bases are represented as orthogonal
//! sums of sparse tensor products. Every residual vector, its kernel sign,
//! and a complete binary change-of-basis program can be generated exactly.
//!
//! Source: `openai/math@4eaee87548250b2682affda7d05d947c05554bae`,
//! `03-motifs.tex`, `04-swap.tex`, and `05-layers.tex`. A compiled frame is a
//! concrete finite construction; it does not by itself execute the recursive
//! stream algorithm or establish its bit-operation cost.

use super::certified::{Complex, Signed};
use super::tape::{Tape, TapeCost, TapeWorkspace};
use crate::bigint::Big;
use std::fmt;
use std::io;

pub const H: usize = 100;
pub const M: usize = H * H * H;
pub const V: usize = H * (H - 1) * (H - 2) / 6;
pub const COMPLEX_NEIGHBORS: usize = 147_731;
pub const DATA_ROLES: u128 = (V as u128) * (V as u128) * (V as u128);
pub const COMPLEX_WIRES: u128 = 1_873_807_244_643_542_670_000;

/// Exact published Delta=C0*d^20, evaluated on parameter integers only.
/// This is the sufficient global bound from Section 5, not a fitted width.
pub fn published_guard(d: &Big) -> Big {
    let w = Big::from_u128(COMPLEX_WIRES).add_small((M + 1) as u64);
    let e = w.mul(&w).mul(&w).mul_small(64);
    let b = e.add(&Big::from_u128(1_873_807_244_636_671_267_308_000_000));
    let c0 = b.mul(&b).mul_small((128 * M) as u64);
    let mut power = Big::from_u64(1);
    for _ in 0..20 {
        power = power.mul(d);
    }
    c0.mul(&power)
}

pub fn published_row_axes(d: &Big) -> Big {
    let target = d.mul_small(2);
    let (mut power, mut exponent) = (Big::from_u64(1), 0u64);
    while power < target {
        power = power.mul_small(M as u64);
        exponent += 1;
    }
    Big::from_u64(exponent).mul_small((128 - (COMPLEX_WIRES - 1).leading_zeros()) as u64)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidTriple,
    InvalidStage,
    InvalidNeighborPair,
    IndexOutOfRange,
    InvalidBinaryBasis,
    SizeOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tensor frame: {self:?}")
    }
}
impl std::error::Error for Error {}

#[derive(Debug)]
pub enum TensorError {
    Frame(Error),
    Io(io::Error),
    InvalidShape,
    NativeSizeOverflow,
    InsufficientGuard,
    GuardAdmission { required: Big, available: usize },
    CoefficientOverflow,
    Swap(String),
}
impl fmt::Display for TensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "simultaneous layer: {self:?}")
    }
}
impl std::error::Error for TensorError {}
impl From<Error> for TensorError {
    fn from(value: Error) -> Self {
        Self::Frame(value)
    }
}
impl From<io::Error> for TensorError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Clone, Debug)]
pub struct LayerReceipt {
    pub individual_kernels: Big,
    pub network_nodes: Big,
    pub directional_children: Big,
    pub packed_repairs: Big,
    pub tape: TapeCost,
}
impl Default for LayerReceipt {
    fn default() -> Self {
        Self {
            individual_kernels: Big::zero(),
            network_nodes: Big::zero(),
            directional_children: Big::zero(),
            packed_repairs: Big::zero(),
            tape: TapeCost::default(),
        }
    }
}

/// Complete lexicographic array shape, excluding the coefficient record.
#[derive(Clone, Debug)]
pub struct LayerLayout {
    pub prefix: Big,
    pub suffix: Big,
    pub axes: usize,
    pub ambient_axes: usize,
    pub chunk_bits: usize,
    pub selected_bit: usize,
    pub record_coefficients: usize,
    pub grid_bits: usize,
    pub component_bits: usize,
}

fn bump(value: &mut Big) {
    for limb in &mut value.0 {
        let (next, carry) = limb.overflowing_add(1);
        *limb = next;
        if !carry {
            return;
        }
    }
    value.0.push(1);
}
fn down(value: &mut Big) {
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
fn xor_big(a: &Big, b: &Big) -> Big {
    Big::from_limbs(
        &(0..a.0.len().max(b.0.len()))
            .map(|i| a.0.get(i).copied().unwrap_or(0) ^ b.0.get(i).copied().unwrap_or(0))
            .collect::<Vec<_>>(),
    )
}
fn low_bits(value: &Big, width: usize) -> Big {
    if value.bits() <= width {
        return value.clone();
    }
    let mut limbs = value.0[..width.div_ceil(64)].to_vec();
    if !width.is_multiple_of(64) {
        if let Some(last) = limbs.last_mut() {
            *last &= (1u64 << (width % 64)) - 1;
        }
    }
    Big::from_limbs(&limbs)
}

/// The three current whole-chunk coordinates used by the packed primitive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedAddress {
    pub x: Big,
    pub y: Big,
    pub z: Big,
}

#[derive(Clone, Copy, Debug)]
pub struct PackedParameters {
    pub chunks: usize,
    pub chunk_bits: usize,
    pub selected_bit: usize,
}
impl PackedParameters {
    pub fn width(self) -> Result<usize, TensorError> {
        if self.chunks == 0 || self.chunk_bits < 6 || self.selected_bit >= self.chunk_bits {
            return Err(TensorError::InvalidShape);
        }
        self.chunks
            .checked_mul(self.chunk_bits)
            .ok_or(TensorError::NativeSizeOverflow)
    }
    pub fn is_bad(self, address: &PackedAddress) -> Result<bool, TensorError> {
        self.width()?;
        let upper = Big::from_u64(1).shl(self.chunk_bits - 1).sub_small(11);
        for i in 0..self.chunks - 1 {
            let bit = self.selected_bit + i * self.chunk_bits;
            for coordinate in [&address.y, &address.z] {
                let guard = low_bits(&coordinate.shr(bit + 1), self.chunk_bits - 1);
                if guard < Big::from_u64(10) || guard > upper {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    fn offset(self, address: &PackedAddress, target_z: bool, odd: bool) -> Big {
        let control = if target_z { &address.y } else { &address.z };
        let mut offset = Big::zero();
        for i in 0..self.chunks - 1 {
            let bit = self.selected_bit + i * self.chunk_bits;
            if address.x.bit(bit) && control.bit(bit) == odd {
                offset = offset.add(&Big::from_u64(1).shl(bit));
            }
        }
        offset
    }
    /// Exact composition S0 of the eight whole-range modular rotations.
    pub fn eight_rotations(
        self,
        input: &PackedAddress,
        inverse: bool,
    ) -> Result<PackedAddress, TensorError> {
        let width = self.width()?;
        if [&input.x, &input.y, &input.z]
            .iter()
            .any(|v| v.bits() > width)
        {
            return Err(TensorError::InvalidShape);
        }
        let modulus = Big::from_u64(1).shl(width);
        let mut result = input.clone();
        for step in 0..8 {
            let index = if inverse { 7 - step } else { step };
            let (target_z, positive, odd) = PACKED_ROTATIONS[index];
            let positive = positive ^ inverse;
            let offset = self.offset(&result, target_z, odd);
            let target = if target_z {
                &mut result.z
            } else {
                &mut result.y
            };
            *target = if positive {
                low_bits(&target.add(&offset), width)
            } else if *target >= offset {
                target.sub(&offset)
            } else {
                modulus.sub(&offset.sub(target))
            };
        }
        Ok(result)
    }
    fn ideal_lower(self, input: &PackedAddress) -> PackedAddress {
        let mut result = input.clone();
        for i in 0..self.chunks - 1 {
            let bit = self.selected_bit + i * self.chunk_bits;
            if input.x.bit(bit) {
                result.y = xor_big(&result.y, &Big::from_u64(1).shl(bit));
            }
        }
        result
    }
    /// Destination T0(S0^-1(q)) for a CURRENT exceptional address q.
    pub fn repair_destination(self, current: &PackedAddress) -> Result<PackedAddress, TensorError> {
        Ok(self.ideal_lower(&self.eight_rotations(current, true)?))
    }
}

// (target is z, positive offset, tested current parity is odd)
const PACKED_ROTATIONS: [(bool, bool, bool); 8] = [
    (false, false, false),
    (true, false, false),
    (false, true, false),
    (true, true, false),
    (true, true, true),
    (false, true, true),
    (true, false, true),
    (false, false, true),
];

#[derive(Clone)]
struct StreamLayout {
    prefix: Big,
    rows: Big,
    chunks: usize,
    suffix: Big,
    chunk_bits: usize,
    selected_bit: usize,
    coefficients: usize,
    width: usize,
}
impl StreamLayout {
    fn address_bits(&self) -> Result<usize, TensorError> {
        self.chunks
            .checked_mul(self.chunk_bits)
            .ok_or(TensorError::NativeSizeOverflow)
    }
    fn record_bits(&self) -> Result<Big, TensorError> {
        Ok(Big::from_u128(
            (self.coefficients as u128)
                .checked_mul(self.width as u128)
                .and_then(|x| x.checked_mul(2))
                .ok_or(TensorError::NativeSizeOverflow)?,
        ))
    }
    fn records(&self) -> Result<Big, TensorError> {
        Ok(self
            .prefix
            .mul(&self.rows)
            .mul(&self.suffix)
            .shl(self.address_bits()?))
    }
    fn row_bits(&self) -> Result<Big, TensorError> {
        Ok(self
            .suffix
            .mul(&self.record_bits()?)
            .shl(self.address_bits()?))
    }
    fn volume(&self) -> Result<Big, TensorError> {
        Ok(self.records()?.mul(&self.record_bits()?))
    }
    fn selected_position(&self, axis: usize) -> Result<usize, TensorError> {
        if axis >= self.chunks {
            return Err(TensorError::InvalidShape);
        }
        (self.chunks - axis - 1)
            .checked_mul(self.chunk_bits)
            .and_then(|x| x.checked_add(self.selected_bit))
            .ok_or(TensorError::NativeSizeOverflow)
    }
}

fn write_signed(tape: &mut Tape, value: &Signed, width: usize) -> Result<(), TensorError> {
    if width == 0 {
        return Err(TensorError::InvalidShape);
    }
    let limit = Big::from_u64(1).shl(width - 1);
    if value.magnitude > limit || (!value.negative && value.magnitude == limit) {
        return Err(TensorError::CoefficientOverflow);
    }
    let encoded = if value.negative {
        Big::from_u64(1).shl(width).sub(&value.magnitude)
    } else {
        value.magnitude.clone()
    };
    tape.write_unsigned(&encoded, width)?;
    Ok(())
}
fn read_signed(tape: &mut Tape, width: usize) -> Result<Signed, TensorError> {
    if width == 0 {
        return Err(TensorError::InvalidShape);
    }
    let encoded = tape.read_unsigned(width)?;
    Ok(if encoded.bit(width - 1) {
        Signed::new(true, Big::from_u64(1).shl(width).sub(&encoded))
    } else {
        Signed::new(false, encoded)
    })
}
fn write_complex(tape: &mut Tape, value: &Complex, width: usize) -> Result<(), TensorError> {
    write_signed(tape, &value.re, width)?;
    write_signed(tape, &value.im, width)
}
fn read_complex(tape: &mut Tape, width: usize) -> Result<Complex, TensorError> {
    Ok(Complex {
        re: read_signed(tape, width)?,
        im: read_signed(tape, width)?,
    })
}
fn phase(value: &Complex, exponent: usize) -> Complex {
    match exponent % 4 {
        0 => value.clone(),
        1 => Complex {
            re: value.im.neg(),
            im: value.re.clone(),
        },
        2 => value.neg(),
        _ => Complex {
            re: value.im.clone(),
            im: value.re.neg(),
        },
    }
}
fn exact_half(value: &Complex) -> Result<Complex, TensorError> {
    if value.re.magnitude.bit(0) || value.im.magnitude.bit(0) {
        return Err(TensorError::InsufficientGuard);
    }
    Ok(value.shr_trunc(1))
}

struct LayerEngine<'a> {
    workspace: &'a mut TapeWorkspace,
    parking: Tape,
    receipt: LayerReceipt,
    stop_below: usize,
}

fn row_axis_count(d: usize) -> Result<usize, TensorError> {
    if d == 0 {
        return Err(TensorError::InvalidShape);
    }
    let target = 2 * d as u128;
    let (mut power, mut exponent) = (1u128, 0usize);
    while power < target {
        power = power
            .checked_mul(M as u128)
            .ok_or(TensorError::NativeSizeOverflow)?;
        exponent += 1;
    }
    let role_bits = 128 - (COMPLEX_WIRES - 1).leading_zeros() as usize;
    role_bits
        .checked_mul(exponent)
        .ok_or(TensorError::NativeSizeOverflow)
}

/// Check the published guard admission without opening or allocating tapes.
pub fn check_layer_admission(
    axes: usize,
    ambient_axes: usize,
    chunk_bits: usize,
    p: usize,
) -> Result<(), TensorError> {
    if axes == 0 || ambient_axes < axes || chunk_bits == 0 {
        return Err(TensorError::InvalidShape);
    }
    let row_axes = row_axis_count(ambient_axes)?;
    if chunk_bits >= 6 && axes.saturating_sub(row_axes) >= M {
        let required = published_guard(&Big::from_u128(ambient_axes as u128));
        if required > Big::from_u128(p as u128) {
            return Err(TensorError::GuardAdmission {
                required,
                available: p,
            });
        }
    }
    Ok(())
}

/// Execute the complete Section 5 C-network construction, outer phase
/// corrections, and a single final truncation on a sequential encoded tape.
///
/// Each component is an MSB-first two's-complement numerator over 2^p.
/// The large-node branch uses the actual published network, sparse residual
/// bases, packed rotations with exceptional repair, and recursively reused
/// physical role tapes. It includes arbitrary existing rows in scratch roles.
///
/// Extra fractional/integer guard space is at least p bits. Above the paper's
/// uniform cutoff its bound Delta<=p proves this sufficient. At other admitted
/// shapes every halving and encoded write is checked; an insufficient guard
/// returns an error, never a rounded intermediate result. Native filesystem
/// interpreter overhead is recorded separately from represented tape moves.
pub fn normalized_layer(
    tape: &mut Tape,
    layout: &LayerLayout,
    workspace: &mut TapeWorkspace,
) -> Result<LayerReceipt, TensorError> {
    if layout.axes == 0
        || layout.ambient_axes < layout.axes
        || layout.chunk_bits == 0
        || layout.selected_bit >= layout.chunk_bits
        || layout.record_coefficients == 0
        || layout.prefix.is_zero()
        || layout.suffix.is_zero()
        || layout.component_bits
            < layout
                .grid_bits
                .checked_add(2)
                .ok_or(TensorError::NativeSizeOverflow)?
    {
        return Err(TensorError::InvalidShape);
    }
    let original = StreamLayout {
        prefix: layout.prefix.clone(),
        rows: Big::from_u64(1),
        chunks: layout.axes,
        suffix: layout.suffix.clone(),
        chunk_bits: layout.chunk_bits,
        selected_bit: layout.selected_bit,
        coefficients: layout.record_coefficients,
        width: layout.component_bits,
    };
    if tape.len() != &original.volume()? {
        return Err(TensorError::InvalidShape);
    }
    // The actual network branch uses the manuscript's published admission,
    // rather than treating runtime overflow detection as its proof.
    check_layer_admission(
        layout.axes,
        layout.ambient_axes,
        layout.chunk_bits,
        layout.grid_bits,
    )?;
    let leaf_guard = layout
        .axes
        .checked_mul(9)
        .and_then(|x| x.checked_add(16))
        .ok_or(TensorError::NativeSizeOverflow)?;
    let guard = layout.grid_bits.max(leaf_guard);
    let width = guard
        .checked_mul(2)
        .and_then(|x| x.checked_add(layout.grid_bits))
        .and_then(|x| x.checked_add(3))
        .ok_or(TensorError::NativeSizeOverflow)?;
    let mut guarded_layout = original.clone();
    guarded_layout.width = width;
    let mut guarded = workspace.create_tape()?;
    tape.rewind()?;
    let mut records = original.records()?;
    while !records.is_zero() {
        for _ in 0..original.coefficients {
            let z = read_complex(tape, original.width)?;
            if !z.is_disk(layout.grid_bits) {
                return Err(TensorError::InvalidShape);
            }
            write_complex(&mut guarded, &z.shl(guard), width)?;
        }
        down(&mut records);
    }
    guarded.rewind()?;
    let parking = workspace.create_tape()?;
    let d = Big::from_u128(layout.ambient_axes as u128);
    let floor = d.isqrt().to_u128().ok_or(TensorError::NativeSizeOverflow)? as usize;
    let stop_below = if (floor as u128) * (floor as u128) == layout.ambient_axes as u128 {
        floor
    } else {
        floor + 1
    };
    let mut engine = LayerEngine {
        workspace,
        parking,
        receipt: LayerReceipt::default(),
        stop_below,
    };
    let axes: Vec<_> = (0..layout.axes).collect();
    engine.diagonal(&mut guarded, &guarded_layout, &axes, 1, 0)?;
    engine.all_c(&mut guarded, &guarded_layout, layout.ambient_axes)?;
    engine.diagonal(&mut guarded, &guarded_layout, &axes, 1, 0)?;
    let mut output = engine.workspace.create_tape()?;
    guarded.rewind()?;
    let mut records = original.records()?;
    while !records.is_zero() {
        for _ in 0..original.coefficients {
            let mut z = phase(
                &read_complex(&mut guarded, width)?,
                3 * ((layout.axes / 2) % 4),
            );
            let shift = layout.axes / 2;
            if !low_bits(&z.re.magnitude, shift).is_zero()
                || !low_bits(&z.im.magnitude, shift).is_zero()
            {
                return Err(TensorError::InsufficientGuard);
            }
            z = z.shr_trunc(shift);
            if !layout.axes.is_multiple_of(2) {
                z = exact_half(&z.sub(&phase(&z, 1)))?;
            }
            let result = z.shr_trunc(guard);
            if !result.is_disk(layout.grid_bits) {
                return Err(TensorError::CoefficientOverflow);
            }
            write_complex(&mut output, &result, original.width)?;
        }
        down(&mut records);
    }
    output.copy_to(tape)?;
    engine.workspace.release_tape(output)?;
    engine.workspace.release_tape(guarded)?;
    if !engine.parking.is_empty() {
        return Err(TensorError::InvalidShape);
    }
    engine.parking.flush()?;
    let parking = engine.parking;
    engine.workspace.release_tape(parking)?;
    engine.receipt.tape = engine.workspace.cost();
    Ok(engine.receipt)
}

/// Native assembly adapter: serialize the supplied finite array, invoke the
/// concrete tape algorithm above, then decode it. There is no alternate
/// direct-transform implementation hidden behind this wrapper.
#[allow(clippy::too_many_arguments)]
pub fn normalized_layer_values(
    values: &mut [Complex],
    prefix: usize,
    suffix: usize,
    axes: usize,
    chunk_bits: usize,
    selected_bit: usize,
    record_coefficients: usize,
    p: usize,
) -> Result<LayerReceipt, TensorError> {
    let address_bits = axes
        .checked_mul(chunk_bits)
        .ok_or(TensorError::NativeSizeOverflow)?;
    let range = 1usize
        .checked_shl(u32::try_from(address_bits).map_err(|_| TensorError::NativeSizeOverflow)?)
        .ok_or(TensorError::NativeSizeOverflow)?;
    let expected = prefix
        .checked_mul(suffix)
        .and_then(|x| x.checked_mul(range))
        .and_then(|x| x.checked_mul(record_coefficients))
        .ok_or(TensorError::NativeSizeOverflow)?;
    if expected != values.len() || prefix == 0 || suffix == 0 {
        return Err(TensorError::InvalidShape);
    }
    let width = p.checked_add(2).ok_or(TensorError::NativeSizeOverflow)?;
    let mut workspace = TapeWorkspace::create_in(std::env::temp_dir())?;
    let mut tape = workspace.create_tape()?;
    for value in values.iter() {
        write_complex(&mut tape, value, width)?;
    }
    tape.rewind()?;
    let layout = LayerLayout {
        prefix: Big::from_u128(prefix as u128),
        suffix: Big::from_u128(suffix as u128),
        axes,
        ambient_axes: axes.checked_add(1).ok_or(TensorError::NativeSizeOverflow)?,
        chunk_bits,
        selected_bit,
        record_coefficients,
        grid_bits: p,
        component_bits: width,
    };
    let result = normalized_layer(&mut tape, &layout, &mut workspace);
    match result {
        Ok(mut receipt) => {
            tape.rewind()?;
            for value in values {
                *value = read_complex(&mut tape, width)?;
            }
            tape.flush()?;
            receipt.tape = workspace.cost();
            drop(tape);
            workspace.cleanup()?;
            Ok(receipt)
        }
        Err(error) => {
            drop(tape);
            let _ = workspace.cleanup();
            Err(error)
        }
    }
}

impl LayerEngine<'_> {
    fn add_roles(
        &mut self,
        target: u128,
        source: u128,
        negative: bool,
        half: bool,
        layout: &StreamLayout,
    ) -> Result<(), TensorError> {
        if target == source {
            return Err(TensorError::InvalidShape);
        }
        let mut target_tape = self.workspace.complex_role(target)?;
        let mut source_tape = self.workspace.complex_role(source)?;
        let mut output = self.workspace.create_tape()?;
        target_tape.rewind()?;
        source_tape.rewind()?;
        let mut records = layout.records()?;
        while !records.is_zero() {
            for _ in 0..layout.coefficients {
                let y = read_complex(&mut target_tape, layout.width)?;
                let mut x = read_complex(&mut source_tape, layout.width)?;
                if half {
                    x = exact_half(&x)?;
                }
                if negative {
                    x = x.neg();
                }
                write_complex(&mut output, &y.add(&x), layout.width)?;
            }
            down(&mut records);
        }
        output.copy_to(&mut target_tape)?;
        source_tape.rewind()?;
        source_tape.flush()?;
        target_tape.flush()?;
        self.workspace.release_tape(output)?;
        Ok(())
    }

    fn scalar_gate(
        &mut self,
        invocation: PublishedInvocation,
        time: usize,
        triple: Option<Triple>,
        layout: &StreamLayout,
    ) -> Result<(), TensorError> {
        // (kind: copy/inject/gather/scatter, physical bank is X, subtract)
        let operation = if invocation.stage == 1 {
            [
                (0, false, false),
                (2, false, false),
                (1, true, true),
                (3, true, true),
                (2, false, true),
                (0, false, true),
                (3, true, false),
                (1, true, false),
            ][time]
        } else {
            [
                (1, false, true),
                (3, false, true),
                (0, true, false),
                (2, true, false),
                (3, false, false),
                (1, false, false),
                (2, true, true),
                (0, true, true),
            ][time]
        };
        let (kind, bank_x, negative) = operation;
        if kind <= 1 {
            let t = triple.ok_or(TensorError::InvalidShape)?;
            let bank = if bank_x {
                invocation.x_role(t)
            } else {
                invocation.y_role(t)
            };
            for n in 0..COMPLEX_NEIGHBORS {
                let other = t.neighbor(n)?;
                let (x, y) = if bank_x { (t, other) } else { (other, t) };
                let side = invocation.side_role(x, y)?;
                if kind == 0 {
                    self.add_roles(side, bank, negative, false, layout)?;
                } else {
                    self.add_roles(
                        bank,
                        side,
                        (t.intersection(other) == 2) ^ negative,
                        true,
                        layout,
                    )?;
                }
            }
        } else {
            for rank in 0..V {
                let t = Triple::from_rank(rank)?;
                let bank = if bank_x {
                    invocation.x_role(t)
                } else {
                    invocation.y_role(t)
                };
                for center in t.entries().into_iter().chain(std::iter::once(H)) {
                    let role = invocation.center_role(center)?;
                    if kind == 2 {
                        self.add_roles(role, bank, negative, false, layout)?;
                    } else {
                        self.add_roles(bank, role, negative ^ (center == H), true, layout)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn child(
        &mut self,
        io: &mut Tape,
        role: u128,
        layout: &StreamLayout,
        start: usize,
        e: usize,
    ) -> Result<(), TensorError> {
        bump(&mut self.receipt.directional_children);
        if e < M || e < self.stop_below || layout.chunk_bits < 6 {
            let mut tape = self.workspace.complex_role(role)?;
            self.c_power(&mut tape, layout, start, e)?;
            tape.flush()?;
            return Ok(());
        }
        let role_volume = layout.volume()?;
        // Save just this parent's inactive role streams at the current stack
        // top. Descendants never scan through these parked bits.
        for other in 0..COMPLEX_WIRES {
            let mut stream = self.workspace.complex_role(other)?;
            stream.rewind()?;
            if other == role {
                stream.copy_to(io)?;
            } else {
                stream.copy_bits_to(&mut self.parking, &role_volume)?;
            }
            stream.clear()?;
            stream.flush()?;
        }
        self.c_power(io, layout, start, e)?;
        {
            let mut active = self.workspace.complex_role(role)?;
            io.copy_to(&mut active)?;
            active.flush()?;
        }
        io.clear()?;
        let mut other = COMPLEX_WIRES;
        while other > 0 {
            other -= 1;
            if other == role {
                continue;
            }
            let mut stream = self.workspace.complex_role(other)?;
            stream.clear()?;
            stream.write_zero_bits(&role_volume)?;
            let mut remaining = role_volume.clone();
            while !remaining.is_zero() {
                stream.write_previous_bit(self.parking.pop_bit()?)?;
                down(&mut remaining);
            }
            stream.flush()?;
        }
        Ok(())
    }

    fn edge(
        &mut self,
        io: &mut Tape,
        role: u128,
        invocation: PublishedInvocation,
        edge: EdgeKind,
        layout: &StreamLayout,
        start: usize,
        f: usize,
    ) -> Result<(), TensorError> {
        let basis = ResidualBasis::new(invocation, edge)?;
        if basis.rank() == 0 {
            return Ok(());
        }
        {
            let mut stream = self.workspace.complex_role(role)?;
            for operation in basis.coordinate_additions(true)? {
                self.packed_addition(&mut stream, layout, start, f, operation)?;
            }
            stream.flush()?;
        }
        for direction in 0..basis.rank() {
            let child_start = start + direction * f;
            let inverse = basis.kernel_is_inverse(direction)?;
            let axes: Vec<_> = (child_start..child_start + f).collect();
            if inverse {
                let mut stream = self.workspace.complex_role(role)?;
                self.diagonal(&mut stream, layout, &axes, 2, 0)?;
                stream.flush()?;
            }
            self.child(io, role, layout, child_start, f)?;
            if inverse {
                let mut stream = self.workspace.complex_role(role)?;
                self.diagonal(&mut stream, layout, &axes, 2, (3 * (f % 4)) % 4)?;
                stream.flush()?;
            }
        }
        {
            let mut stream = self.workspace.complex_role(role)?;
            for operation in basis.coordinate_additions(false)? {
                self.packed_addition(&mut stream, layout, start, f, operation)?;
            }
            stream.flush()?;
        }
        Ok(())
    }

    fn side_gate(
        &mut self,
        io: &mut Tape,
        invocation: PublishedInvocation,
        time: usize,
        triple: Triple,
        layout: &StreamLayout,
        start: usize,
        f: usize,
    ) -> Result<(), TensorError> {
        match time {
            0 => {
                for n in 0..COMPLEX_NEIGHBORS {
                    let x = triple.neighbor(n)?;
                    self.edge(
                        io,
                        invocation.side_role(x, triple)?,
                        invocation,
                        EdgeKind::SideSourceTo0 { y: triple },
                        layout,
                        start,
                        f,
                    )?;
                }
            }
            2 => {
                self.edge(
                    io,
                    invocation.x_role(triple),
                    invocation,
                    EdgeKind::XInTo2(triple),
                    layout,
                    start,
                    f,
                )?;
                for n in 0..COMPLEX_NEIGHBORS {
                    let y = triple.neighbor(n)?;
                    self.edge(
                        io,
                        invocation.side_role(triple, y)?,
                        invocation,
                        EdgeKind::Side0To2 { x: triple, y },
                        layout,
                        start,
                        f,
                    )?;
                }
            }
            5 => {
                self.edge(
                    io,
                    invocation.y_role(triple),
                    invocation,
                    EdgeKind::Y4To5(triple),
                    layout,
                    start,
                    f,
                )?;
                for n in 0..COMPLEX_NEIGHBORS {
                    let x = triple.neighbor(n)?;
                    self.edge(
                        io,
                        invocation.side_role(x, triple)?,
                        invocation,
                        EdgeKind::Side2To5 { x, y: triple },
                        layout,
                        start,
                        f,
                    )?;
                }
            }
            7 => {
                for n in 0..COMPLEX_NEIGHBORS {
                    let y = triple.neighbor(n)?;
                    self.edge(
                        io,
                        invocation.side_role(triple, y)?,
                        invocation,
                        EdgeKind::Side5To7 { y },
                        layout,
                        start,
                        f,
                    )?;
                }
            }
            _ => return Err(TensorError::InvalidShape),
        }
        self.scalar_gate(invocation, time, Some(triple), layout)
    }

    fn central_gate(
        &mut self,
        io: &mut Tape,
        invocation: PublishedInvocation,
        time: usize,
        layout: &StreamLayout,
        start: usize,
        f: usize,
    ) -> Result<(), TensorError> {
        if time == 1 || time == 3 {
            for rank in 0..V {
                let triple = Triple::from_rank(rank)?;
                let (role, edge) = if time == 1 {
                    (invocation.y_role(triple), EdgeKind::Y0To1(triple))
                } else {
                    (invocation.x_role(triple), EdgeKind::X2To3(triple))
                };
                self.edge(io, role, invocation, edge, layout, start, f)?;
            }
        }
        let edge = match time {
            1 => EdgeKind::CenterSourceTo1,
            3 => EdgeKind::Center1To3,
            4 => EdgeKind::Center3To4,
            6 => EdgeKind::Center4To6,
            _ => return Err(TensorError::InvalidShape),
        };
        for center in 0..=H {
            self.edge(
                io,
                invocation.center_role(center)?,
                invocation,
                edge,
                layout,
                start,
                f,
            )?;
        }
        self.scalar_gate(invocation, time, None, layout)
    }

    fn endpoint(
        &mut self,
        layout: &StreamLayout,
        start: usize,
        f: usize,
        source: bool,
    ) -> Result<(), TensorError> {
        for a0 in 0..V {
            for a1 in 0..V {
                for a2 in 0..V {
                    let triples = [
                        Triple::from_rank(a0)?,
                        Triple::from_rank(a1)?,
                        Triple::from_rank(a2)?,
                    ];
                    let index = (a0 as u128 * V as u128 + a1 as u128) * V as u128 + a2 as u128;
                    let axes: Vec<_> = tensor_support(&triples)
                        .into_iter()
                        .flat_map(|slot| start + slot * f..start + (slot + 1) * f)
                        .collect();
                    let role = if source { index } else { DATA_ROLES + index };
                    let mut stream = self.workspace.complex_role(role)?;
                    self.diagonal(
                        &mut stream,
                        layout,
                        &axes,
                        2,
                        if source { 0 } else { (27 * (f % 4)) % 4 },
                    )?;
                    stream.flush()?;
                    drop(stream);
                    if !source {
                        let mut stream = self.workspace.complex_role(index)?;
                        self.diagonal(&mut stream, layout, &[], 0, 2)?;
                        stream.flush()?;
                    }
                }
            }
        }
        Ok(())
    }

    fn split_roles(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
    ) -> Result<StreamLayout, TensorError> {
        let (groups, remainder) = layout.rows.divrem(&Big::from_u128(COMPLEX_WIRES));
        if !remainder.is_zero() || groups.is_zero() {
            return Err(TensorError::InvalidShape);
        }
        tape.rewind()?;
        let row_bits = layout.row_bits()?;
        let mut prefix = Big::zero();
        while prefix < layout.prefix {
            let mut group = Big::zero();
            while group < groups {
                for role in 0..COMPLEX_WIRES {
                    let mut stream = self.workspace.complex_role(role)?;
                    if prefix.is_zero() && group.is_zero() {
                        stream.clear()?;
                    }
                    tape.copy_bits_to(&mut stream, &row_bits)?;
                    stream.flush()?;
                }
                bump(&mut group);
            }
            bump(&mut prefix);
        }
        tape.clear()?;
        for role in 0..COMPLEX_WIRES {
            let mut stream = self.workspace.complex_role(role)?;
            stream.rewind()?;
            stream.flush()?;
        }
        let mut result = layout.clone();
        result.rows = groups;
        Ok(result)
    }

    fn merge_roles(&mut self, tape: &mut Tape, layout: &StreamLayout) -> Result<(), TensorError> {
        for role in 0..COMPLEX_WIRES {
            let mut stream = self.workspace.complex_role(role)?;
            stream.rewind()?;
            stream.flush()?;
        }
        tape.clear()?;
        let row_bits = layout.row_bits()?;
        let mut prefix = Big::zero();
        while prefix < layout.prefix {
            let mut group = Big::zero();
            while group < layout.rows {
                for role in 0..COMPLEX_WIRES {
                    let physical = if role < DATA_ROLES {
                        role + DATA_ROLES
                    } else if role < 2 * DATA_ROLES {
                        role - DATA_ROLES
                    } else {
                        role
                    };
                    let mut stream = self.workspace.complex_role(physical)?;
                    stream.copy_bits_to(tape, &row_bits)?;
                    stream.flush()?;
                }
                bump(&mut group);
            }
            bump(&mut prefix);
        }
        tape.rewind()?;
        tape.flush()?;
        for role in 0..COMPLEX_WIRES {
            let mut stream = self.workspace.complex_role(role)?;
            stream.clear()?;
            stream.flush()?;
        }
        Ok(())
    }

    fn c_power(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        start: usize,
        e: usize,
    ) -> Result<(), TensorError> {
        if e == 0 || start.checked_add(e).is_none_or(|x| x > layout.chunks) {
            return Err(TensorError::InvalidShape);
        }
        if e < M || e < self.stop_below || layout.chunk_bits < 6 {
            for axis in start..start + e {
                self.individual_c(tape, layout, axis)?;
            }
            return Ok(());
        }
        if !e.is_multiple_of(M) {
            return Err(TensorError::InvalidShape);
        }
        bump(&mut self.receipt.network_nodes);
        let f = e / M;
        let role_layout = self.split_roles(tape, layout)?;
        self.endpoint(&role_layout, start, f, true)?;
        for stage in 0..3 {
            for a in 0..V {
                for b in 0..V {
                    let invocation = PublishedInvocation::new(
                        stage,
                        [Triple::from_rank(a)?, Triple::from_rank(b)?],
                    )?;
                    for time in 0..8 {
                        if [0, 2, 5, 7].contains(&time) {
                            for rank in 0..V {
                                self.side_gate(
                                    tape,
                                    invocation,
                                    time,
                                    Triple::from_rank(rank)?,
                                    &role_layout,
                                    start,
                                    f,
                                )?;
                            }
                        } else {
                            self.central_gate(tape, invocation, time, &role_layout, start, f)?;
                        }
                    }
                    for y_rank in 0..V {
                        let y = Triple::from_rank(y_rank)?;
                        for n in 0..COMPLEX_NEIGHBORS {
                            let x = y.neighbor(n)?;
                            self.edge(
                                tape,
                                invocation.side_role(x, y)?,
                                invocation,
                                EdgeKind::ScratchLastToSink,
                                &role_layout,
                                start,
                                f,
                            )?;
                        }
                    }
                    for center in 0..=H {
                        self.edge(
                            tape,
                            invocation.center_role(center)?,
                            invocation,
                            EdgeKind::ScratchLastToSink,
                            &role_layout,
                            start,
                            f,
                        )?;
                    }
                }
            }
        }
        self.endpoint(&role_layout, start, f, false)?;
        self.merge_roles(tape, &role_layout)
    }

    fn change_rows(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        rows: &Big,
    ) -> Result<(), TensorError> {
        let mut output = self.workspace.create_tape()?;
        let row_bits = layout.row_bits()?;
        let common = if rows < &layout.rows {
            rows
        } else {
            &layout.rows
        };
        let copy_bits = common.mul(&row_bits);
        let mut prefix = Big::zero();
        tape.rewind()?;
        while prefix < layout.prefix {
            tape.copy_bits_to(&mut output, &copy_bits)?;
            if rows > &layout.rows {
                output.write_zero_bits(&rows.sub(&layout.rows).mul(&row_bits))?;
            } else {
                tape.skip_bits(&layout.rows.sub(rows).mul(&row_bits))?;
            }
            bump(&mut prefix);
        }
        output.copy_to(tape)?;
        self.workspace.release_tape(output)?;
        Ok(())
    }

    fn all_c(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        d: usize,
    ) -> Result<(), TensorError> {
        let q0 = row_axis_count(d)?;
        if layout.chunks <= q0 || layout.chunk_bits < 6 {
            for axis in 0..layout.chunks {
                self.individual_c(tape, layout, axis)?;
            }
            return Ok(());
        }
        for axis in 0..q0 {
            self.individual_c(tape, layout, axis)?;
        }
        let mut inner = layout.clone();
        inner.rows = Big::from_u64(1).shl(
            q0.checked_mul(layout.chunk_bits)
                .ok_or(TensorError::NativeSizeOverflow)?,
        );
        inner.chunks -= q0;
        let original_rows = inner.rows.clone();
        let mut multiple = Big::from_u64(1);
        let mut capacity = 1usize;
        while capacity < d {
            multiple = multiple.mul(&Big::from_u128(COMPLEX_WIRES));
            capacity = capacity
                .checked_mul(M)
                .ok_or(TensorError::NativeSizeOverflow)?;
        }
        let (quotient, remainder) = original_rows.divrem(&multiple);
        let padded = if remainder.is_zero() {
            original_rows.clone()
        } else {
            quotient.add_small(1).mul(&multiple)
        };
        self.change_rows(tape, &inner, &padded)?;
        inner.rows = padded;
        let mut remaining = inner.chunks;
        let mut start = 0;
        let mut power = 1usize;
        while power <= remaining / M {
            power *= M;
        }
        while power > 0 {
            let count = remaining / power;
            for _ in 0..count {
                self.c_power(tape, &inner, start, power)?;
                start += power;
            }
            remaining %= power;
            power /= M;
        }
        self.change_rows(tape, &inner, &original_rows)
    }

    fn individual_c(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        axis: usize,
    ) -> Result<(), TensorError> {
        let bit = layout.selected_position(axis)?;
        let block = layout.suffix.mul(&layout.record_bits()?).shl(bit);
        let groups = layout
            .prefix
            .mul(&layout.rows)
            .shl(layout.address_bits()? - bit - 1);
        let mut zero = self.workspace.create_tape()?;
        let mut one = self.workspace.create_tape()?;
        let mut out_zero = self.workspace.create_tape()?;
        let mut out_one = self.workspace.create_tape()?;
        tape.rewind()?;
        let mut remaining = groups.clone();
        while !remaining.is_zero() {
            tape.copy_bits_to(&mut zero, &block)?;
            tape.copy_bits_to(&mut one, &block)?;
            down(&mut remaining);
        }
        zero.rewind()?;
        one.rewind()?;
        let mut records = layout.records()?.shr(1);
        while !records.is_zero() {
            for _ in 0..layout.coefficients {
                let u = read_complex(&mut zero, layout.width)?;
                let v = read_complex(&mut one, layout.width)?;
                let sum = u.add(&v);
                let idifference = phase(&u.sub(&v), 1);
                write_complex(
                    &mut out_zero,
                    &exact_half(&sum.add(&idifference))?,
                    layout.width,
                )?;
                write_complex(
                    &mut out_one,
                    &exact_half(&sum.sub(&idifference))?,
                    layout.width,
                )?;
            }
            down(&mut records);
        }
        tape.clear()?;
        out_zero.rewind()?;
        out_one.rewind()?;
        let mut remaining = groups;
        while !remaining.is_zero() {
            out_zero.copy_bits_to(tape, &block)?;
            out_one.copy_bits_to(tape, &block)?;
            down(&mut remaining);
        }
        tape.rewind()?;
        tape.flush()?;
        for temporary in [zero, one, out_zero, out_one] {
            self.workspace.release_tape(temporary)?;
        }
        bump(&mut self.receipt.individual_kernels);
        Ok(())
    }

    /// Apply fourth-root phases from selected chunk bits. `weight` is one
    /// for S, two for Z; `constant` is applied once per coefficient.
    fn diagonal(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        axes: &[usize],
        weight: usize,
        constant: usize,
    ) -> Result<(), TensorError> {
        let positions: Vec<_> = axes
            .iter()
            .map(|&axis| layout.selected_position(axis))
            .collect::<Result<_, _>>()?;
        let mut output = self.workspace.create_tape()?;
        tape.rewind()?;
        let records = layout.records()?;
        let mut rank = Big::zero();
        while rank < records {
            let address = rank.divrem(&layout.suffix).0;
            let exponent =
                (constant + weight * positions.iter().filter(|&&bit| address.bit(bit)).count()) % 4;
            for _ in 0..layout.coefficients {
                let z = read_complex(tape, layout.width)?;
                write_complex(&mut output, &phase(&z, exponent), layout.width)?;
            }
            bump(&mut rank);
        }
        output.copy_to(tape)?;
        self.workspace.release_tape(output)?;
        Ok(())
    }

    fn swap_slots(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        left: usize,
        right: usize,
        f: usize,
    ) -> Result<(), TensorError> {
        let (left, right) = (left.min(right), left.max(right));
        if left == right {
            return Ok(());
        }
        if left.checked_add(f).is_none_or(|x| x > right)
            || right.checked_add(f).is_none_or(|x| x > layout.chunks)
        {
            return Err(TensorError::InvalidShape);
        }
        let width = f
            .checked_mul(layout.chunk_bits)
            .ok_or(TensorError::NativeSizeOverflow)?;
        let prefix = layout
            .prefix
            .mul(&layout.rows)
            .shl(left * layout.chunk_bits);
        let gap = Big::from_u64(1).shl((right - left - f) * layout.chunk_bits);
        let suffix = layout
            .suffix
            .mul(&layout.record_bits()?)
            .shl((layout.chunks - right - f) * layout.chunk_bits);
        super::swap::swap_chunks(tape, &prefix, width, &gap, &suffix, self.workspace)
            .map(|_| ())
            .map_err(|e| TensorError::Swap(e.to_string()))
    }

    fn rotation(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        positions: [usize; 3],
        f: usize,
        target_z: bool,
        positive: bool,
        odd: bool,
    ) -> Result<(), TensorError> {
        let parameters = PackedParameters {
            chunks: f,
            chunk_bits: layout.chunk_bits,
            selected_bit: layout.selected_bit,
        };
        let width = parameters.width()?;
        let logical_target = if target_z { 2 } else { 1 };
        let rightmost = *positions.iter().max().ok_or(TensorError::InvalidShape)?;
        let mut physical = positions;
        let old_target = physical[logical_target];
        if old_target != rightmost {
            self.swap_slots(tape, layout, old_target, rightmost, f)?;
            let other = physical.iter().position(|&x| x == rightmost).unwrap();
            physical.swap(logical_target, other);
        }
        let target = physical[logical_target];
        let groups = layout
            .prefix
            .mul(&layout.rows)
            .shl(target * layout.chunk_bits);
        let range = Big::from_u64(1).shl(width);
        let suffix = layout
            .suffix
            .mul(&layout.record_bits()?)
            .shl((layout.chunks - target - f) * layout.chunk_bits);
        let mut first = self.workspace.create_tape()?;
        let mut second = self.workspace.create_tape()?;
        tape.rewind()?;
        for pass in 0..2 {
            if pass == 1 {
                first.rewind()?;
                second.rewind()?;
                tape.clear()?;
            }
            let mut prefix = Big::zero();
            while prefix < groups {
                let extract = |name: usize| -> Big {
                    if name == logical_target {
                        Big::zero()
                    } else {
                        low_bits(
                            &prefix.shr((target - physical[name] - f) * layout.chunk_bits),
                            width,
                        )
                    }
                };
                let address = PackedAddress {
                    x: extract(0),
                    y: extract(1),
                    z: extract(2),
                };
                let raw = parameters.offset(&address, target_z, odd);
                let offset = if positive || raw.is_zero() {
                    raw
                } else {
                    range.sub(&raw)
                };
                let first_bits = range.sub(&offset).mul(&suffix);
                let second_bits = offset.mul(&suffix);
                if pass == 0 {
                    tape.copy_bits_to(&mut first, &first_bits)?;
                    tape.copy_bits_to(&mut second, &second_bits)?;
                } else {
                    second.copy_bits_to(tape, &second_bits)?;
                    first.copy_bits_to(tape, &first_bits)?;
                }
                bump(&mut prefix);
            }
        }
        tape.rewind()?;
        tape.flush()?;
        self.workspace.release_tape(first)?;
        self.workspace.release_tape(second)?;
        if old_target != rightmost {
            self.swap_slots(tape, layout, old_target, rightmost, f)?;
        }
        Ok(())
    }

    fn packed_address(
        &self,
        rank: &Big,
        layout: &StreamLayout,
        positions: [usize; 3],
        f: usize,
    ) -> Result<PackedAddress, TensorError> {
        let width = f
            .checked_mul(layout.chunk_bits)
            .ok_or(TensorError::NativeSizeOverflow)?;
        let address = rank.divrem(&layout.suffix).0;
        let coordinate = |position: usize| {
            low_bits(
                &address.shr((layout.chunks - position - f) * layout.chunk_bits),
                width,
            )
        };
        Ok(PackedAddress {
            x: coordinate(positions[0]),
            y: coordinate(positions[1]),
            z: coordinate(positions[2]),
        })
    }

    fn replaced_rank(
        &self,
        rank: &Big,
        current: &PackedAddress,
        destination: &PackedAddress,
        layout: &StreamLayout,
        positions: [usize; 3],
        f: usize,
    ) -> Big {
        let (mut address, suffix) = rank.divrem(&layout.suffix);
        for ((old, new), position) in [&current.x, &current.y, &current.z]
            .into_iter()
            .zip([&destination.x, &destination.y, &destination.z])
            .zip(positions)
        {
            address = xor_big(
                &address,
                &xor_big(old, new).shl((layout.chunks - position - f) * layout.chunk_bits),
            );
        }
        address.mul(&layout.suffix).add(&suffix)
    }

    fn repair(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        positions: [usize; 3],
        f: usize,
    ) -> Result<(), TensorError> {
        let parameters = PackedParameters {
            chunks: f,
            chunk_bits: layout.chunk_bits,
            selected_bit: layout.selected_bit,
        };
        let records = layout.records()?;
        let payload = layout.record_bits()?;
        let key_width = records.sub_small(1).bits();
        let mut extracted = self.workspace.create_tape()?;
        let mut bad_count = Big::zero();
        tape.rewind()?;
        let mut rank = Big::zero();
        while rank < records {
            let current = self.packed_address(&rank, layout, positions, f)?;
            if parameters.is_bad(&current)? {
                let destination = parameters.repair_destination(&current)?;
                let key = self.replaced_rank(&rank, &current, &destination, layout, positions, f);
                extracted.write_unsigned(&key, key_width)?;
                tape.copy_bits_to(&mut extracted, &payload)?;
                bump(&mut bad_count);
            } else {
                tape.skip_bits(&payload)?;
            }
            bump(&mut rank);
        }
        if !bad_count.is_zero() {
            let mut zero = self.workspace.create_tape()?;
            let mut one = self.workspace.create_tape()?;
            for key_bit in 0..key_width {
                extracted.rewind()?;
                let mut remaining = bad_count.clone();
                while !remaining.is_zero() {
                    let key = extracted.read_unsigned(key_width)?;
                    let output = if key.bit(key_bit) {
                        &mut one
                    } else {
                        &mut zero
                    };
                    output.write_unsigned(&key, key_width)?;
                    extracted.copy_bits_to(output, &payload)?;
                    down(&mut remaining);
                }
                extracted.clear()?;
                zero.rewind()?;
                one.rewind()?;
                let n0 = zero.len().clone();
                let n1 = one.len().clone();
                zero.copy_bits_to(&mut extracted, &n0)?;
                one.copy_bits_to(&mut extracted, &n1)?;
                zero.clear()?;
                one.clear()?;
            }
            self.workspace.release_tape(zero)?;
            self.workspace.release_tape(one)?;
            extracted.rewind()?;
            tape.rewind()?;
            let mut output = self.workspace.create_tape()?;
            let mut rank = Big::zero();
            while rank < records {
                let current = self.packed_address(&rank, layout, positions, f)?;
                if parameters.is_bad(&current)? {
                    let key = extracted.read_unsigned(key_width)?;
                    if key != rank {
                        return Err(TensorError::InvalidShape);
                    }
                    tape.skip_bits(&payload)?;
                    extracted.copy_bits_to(&mut output, &payload)?;
                } else {
                    tape.copy_bits_to(&mut output, &payload)?;
                }
                bump(&mut rank);
            }
            output.copy_to(tape)?;
            self.workspace.release_tape(output)?;
            self.receipt.packed_repairs = self.receipt.packed_repairs.add(&bad_count);
        } else {
            tape.rewind()?;
        }
        self.workspace.release_tape(extracted)?;
        Ok(())
    }

    /// Exact fixed permutation of two selected address bits, by four ordered
    /// streams. This is Lemma 2.1's fixed-digit primitive, not a global sort.
    fn individual_xor(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        control: usize,
        target: usize,
    ) -> Result<(), TensorError> {
        if control == target {
            return Err(TensorError::InvalidShape);
        }
        let bit_control = layout.selected_position(control)?;
        let bit_target = layout.selected_position(target)?;
        let records = layout.records()?;
        let payload = layout.record_bits()?;
        let mut streams = [
            self.workspace.create_tape()?,
            self.workspace.create_tape()?,
            self.workspace.create_tape()?,
            self.workspace.create_tape()?,
        ];
        tape.rewind()?;
        let mut rank = Big::zero();
        while rank < records {
            let address = rank.divrem(&layout.suffix).0;
            let class =
                2 * usize::from(address.bit(bit_control)) + usize::from(address.bit(bit_target));
            tape.copy_bits_to(&mut streams[class], &payload)?;
            bump(&mut rank);
        }
        for stream in &mut streams {
            stream.rewind()?;
        }
        tape.clear()?;
        let mut rank = Big::zero();
        while rank < records {
            let address = rank.divrem(&layout.suffix).0;
            let control = address.bit(bit_control);
            let original_target = address.bit(bit_target) ^ control;
            let class = 2 * usize::from(control) + usize::from(original_target);
            streams[class].copy_bits_to(tape, &payload)?;
            bump(&mut rank);
        }
        tape.rewind()?;
        tape.flush()?;
        for stream in streams {
            self.workspace.release_tape(stream)?;
        }
        Ok(())
    }

    fn packed_addition(
        &mut self,
        tape: &mut Tape,
        layout: &StreamLayout,
        start: usize,
        f: usize,
        addition: RowAddition,
    ) -> Result<(), TensorError> {
        let third = (0..M)
            .find(|&i| i != addition.control && i != addition.target)
            .ok_or(TensorError::InvalidShape)?;
        let positions = [
            start + addition.control * f,
            start + addition.target * f,
            start + third * f,
        ];
        if f > 1 {
            for (target_z, positive, odd) in PACKED_ROTATIONS {
                self.rotation(tape, layout, positions, f, target_z, positive, odd)?;
            }
            self.repair(tape, layout, positions, f)?;
        }
        // Chunk indices within a slot are numbered from its least significant
        // end. The omitted top selected position is the first physical chunk.
        self.individual_xor(tape, layout, positions[0], positions[1])
    }
}

/// A triple in lexicographic order on the published ground set `[100]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Triple([usize; 3]);

impl Triple {
    pub fn new(entries: [usize; 3]) -> Result<Self, Error> {
        if entries[0] < entries[1] && entries[1] < entries[2] && entries[2] < H {
            Ok(Self(entries))
        } else {
            Err(Error::InvalidTriple)
        }
    }

    pub fn entries(self) -> [usize; 3] {
        self.0
    }

    pub fn rank(self) -> usize {
        combination_rank(&self.0, H)
    }

    pub fn from_rank(rank: usize) -> Result<Self, Error> {
        Ok(Self(combination_unrank(rank, H)?))
    }

    pub fn intersection(self, other: Self) -> usize {
        self.0.iter().filter(|x| other.0.contains(x)).count()
    }

    pub fn is_complex_neighbor(self, other: Self) -> bool {
        self != other && self.intersection(other).is_multiple_of(2)
    }

    /// Neighbors are ordered by disjoint triples first, then by the common
    /// pair and the remaining point. This gives a canonical side-wire order.
    pub fn neighbor(self, index: usize) -> Result<Self, Error> {
        let disjoint = choose3(H - 3);
        if index >= COMPLEX_NEIGHBORS {
            return Err(Error::IndexOutOfRange);
        }
        let outside = |rank: usize| -> usize {
            let mut value = rank;
            for &excluded in &self.0 {
                if value >= excluded {
                    value += 1;
                }
            }
            value
        };
        if index < disjoint {
            let local = combination_unrank(index, H - 3)?;
            Self::new(local.map(outside))
        } else {
            let index = index - disjoint;
            let pairs = [(0, 1), (0, 2), (1, 2)];
            let (a, b) = pairs[index / (H - 3)];
            let mut entries = [self.0[a], self.0[b], outside(index % (H - 3))];
            entries.sort_unstable();
            Self::new(entries)
        }
    }

    pub fn neighbor_rank(self, other: Self) -> Result<usize, Error> {
        if !self.is_complex_neighbor(other) {
            return Err(Error::InvalidNeighborPair);
        }
        let outside_rank =
            |value: usize| value - self.0.iter().filter(|&&excluded| excluded < value).count();
        if self.intersection(other) == 0 {
            Ok(combination_rank(&other.0.map(outside_rank), H - 3))
        } else {
            let common: Vec<_> = self
                .0
                .iter()
                .enumerate()
                .filter(|(_, x)| other.0.contains(x))
                .map(|(i, _)| i)
                .collect();
            let pair = match common.as_slice() {
                [0, 1] => 0,
                [0, 2] => 1,
                [1, 2] => 2,
                _ => return Err(Error::InvalidNeighborPair),
            };
            let outside = *other
                .0
                .iter()
                .find(|x| !self.0.contains(x))
                .ok_or(Error::InvalidNeighborPair)?;
            Ok(choose3(H - 3) + pair * (H - 3) + outside_rank(outside))
        }
    }
}

fn choose2(n: usize) -> usize {
    if n < 2 {
        0
    } else {
        n * (n - 1) / 2
    }
}
fn choose3(n: usize) -> usize {
    if n < 3 {
        0
    } else {
        n * (n - 1) * (n - 2) / 6
    }
}
fn combination_rank(entries: &[usize; 3], n: usize) -> usize {
    let mut result = 0;
    let mut first = 0;
    for (position, &entry) in entries.iter().enumerate() {
        for value in first..entry {
            result += match position {
                0 => choose2(n - value - 1),
                1 => n - value - 1,
                _ => 1,
            };
        }
        first = entry + 1;
    }
    result
}
fn combination_unrank(mut rank: usize, n: usize) -> Result<[usize; 3], Error> {
    if rank >= choose3(n) {
        return Err(Error::IndexOutOfRange);
    }
    let mut result = [0; 3];
    let mut first = 0;
    for (position, entry) in result.iter_mut().enumerate() {
        for value in first..n {
            let count = match position {
                0 => choose2(n - value - 1),
                1 => n - value - 1,
                _ => 1,
            };
            if rank < count {
                *entry = value;
                first = value + 1;
                break;
            }
            rank -= count;
        }
    }
    Ok(result)
}

/// The stage is zero-based. `fixed` lists the other two triples in physical
/// coordinate order: `(a2,a3)`, `(a1,a3)`, or `(a1,a2)` respectively.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublishedInvocation {
    pub stage: usize,
    pub fixed: [Triple; 2],
}

impl PublishedInvocation {
    pub fn new(stage: usize, fixed: [Triple; 2]) -> Result<Self, Error> {
        if stage >= 3 {
            return Err(Error::InvalidStage);
        }
        Ok(Self { stage, fixed })
    }

    pub fn index(self) -> u128 {
        self.stage as u128 * (V as u128).pow(2)
            + self.fixed[0].rank() as u128 * V as u128
            + self.fixed[1].rank() as u128
    }

    fn data_index(self, triple: Triple) -> u128 {
        let coordinates = match self.stage {
            0 => [triple, self.fixed[0], self.fixed[1]],
            1 => [self.fixed[0], triple, self.fixed[1]],
            _ => [self.fixed[0], self.fixed[1], triple],
        };
        coordinates
            .iter()
            .fold(0, |n, t| n * V as u128 + t.rank() as u128)
    }

    pub fn x_role(self, triple: Triple) -> u128 {
        self.data_index(triple)
    }
    pub fn y_role(self, triple: Triple) -> u128 {
        DATA_ROLES + self.data_index(triple)
    }
    pub fn side_role(self, x: Triple, y: Triple) -> Result<u128, Error> {
        let offset = y.rank() * COMPLEX_NEIGHBORS + y.neighbor_rank(x)?;
        Ok(self.scratch_start() + offset as u128)
    }
    /// Center `H` is the star wire; `0..H` are the ordinary centers.
    pub fn center_role(self, center: usize) -> Result<u128, Error> {
        if center > H {
            return Err(Error::IndexOutOfRange);
        }
        Ok(self.scratch_start() + (V * COMPLEX_NEIGHBORS + center) as u128)
    }
    fn scratch_start(self) -> u128 {
        2 * DATA_ROLES + self.index() * (V as u128 * COMPLEX_NEIGHBORS as u128 + H as u128 + 1)
    }
}

/// The complete nonzero edge table in Lemma 3.2, plus its zero edges.
/// Physical X/Y naming remains unchanged in the inverse second stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeKind {
    Unchanged,
    XInTo2(Triple),
    X2To3(Triple),
    Y0To1(Triple),
    Y4To5(Triple),
    CenterSourceTo1,
    Center1To3,
    Center3To4,
    Center4To6,
    SideSourceTo0 { y: Triple },
    Side0To2 { x: Triple, y: Triple },
    Side2To5 { x: Triple, y: Triple },
    Side5To7 { y: Triple },
    ScratchLastToSink,
}

/// A sorted, duplicate-free sparse binary vector. The ambient dimension may
/// be one million even when this vector has only one nonzero coordinate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SparseVector {
    pub dimension: usize,
    pub support: Vec<usize>,
}

impl SparseVector {
    pub fn dot(&self, other: &Self) -> Result<bool, Error> {
        if self.dimension != other.dimension {
            return Err(Error::InvalidBinaryBasis);
        }
        let (mut i, mut j, mut result) = (0, 0, false);
        while i < self.support.len() && j < other.support.len() {
            match self.support[i].cmp(&other.support[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => {
                    result = !result;
                    i += 1;
                    j += 1;
                }
            }
        }
        Ok(result)
    }
}

#[derive(Clone, Debug)]
struct FactorPartition {
    dimension: usize,
    lines: Vec<Vec<usize>>,
    region: Vec<usize>,
    local_complement: Vec<u128>,
}

impl FactorPartition {
    fn full(dimension: usize) -> Self {
        Self {
            dimension,
            lines: vec![],
            region: vec![],
            local_complement: vec![],
        }
    }

    /// Complete the supplied mutually orthonormal sparse lines. Only their
    /// union and one extra unit coordinate participate in elimination.
    fn with_lines(dimension: usize, lines: Vec<Vec<usize>>) -> Result<Self, Error> {
        if lines.is_empty() {
            return Ok(Self::full(dimension));
        }
        let mut region: Vec<_> = lines.iter().flatten().copied().collect();
        region.sort_unstable();
        region.dedup();
        if region.last().is_some_and(|&i| i >= dimension) || region.len() >= 128 {
            return Err(Error::InvalidBinaryBasis);
        }
        if let Some(outside) = (0..dimension).find(|i| region.binary_search(i).is_err()) {
            region.push(outside);
            region.sort_unstable();
        }
        let masks: Vec<u128> = lines
            .iter()
            .map(|line| {
                line.iter().fold(0, |v, i| {
                    v ^ (1u128 << region.binary_search(i).expect("line coordinate"))
                })
            })
            .collect();
        for (i, &x) in masks.iter().enumerate() {
            for (j, &y) in masks.iter().enumerate() {
                if parity(x & y) != (i == j) {
                    return Err(Error::InvalidBinaryBasis);
                }
            }
        }
        let mut independent = Vec::new();
        let mut echelon = [0u128; 128];
        for i in 0..region.len() {
            let mut projected = 1u128 << i;
            for &line in &masks {
                if line & (1u128 << i) != 0 {
                    projected ^= line;
                }
            }
            let mut reduced = projected;
            while reduced != 0 {
                let pivot = reduced.trailing_zeros() as usize;
                if echelon[pivot] == 0 {
                    echelon[pivot] = reduced;
                    independent.push(projected);
                    break;
                }
                reduced ^= echelon[pivot];
            }
        }
        let local_complement = orthonormalize(independent)?;
        if local_complement.len() + lines.len() != region.len() {
            return Err(Error::InvalidBinaryBasis);
        }
        Ok(Self {
            dimension,
            lines,
            region,
            local_complement,
        })
    }

    fn components(&self) -> usize {
        self.lines.len() + 1
    }

    fn component_len(&self, component: usize) -> usize {
        if component < self.lines.len() {
            1
        } else {
            self.dimension - self.lines.len()
        }
    }

    fn component_offset(&self, component: usize) -> usize {
        component.min(self.lines.len())
    }

    fn column(&self, component: usize, index: usize) -> Vec<usize> {
        if component < self.lines.len() {
            return self.lines[component].clone();
        }
        let outside_count = self.dimension - self.region.len();
        if index < outside_count {
            let mut coordinate = index;
            for &excluded in &self.region {
                if coordinate >= excluded {
                    coordinate += 1;
                }
            }
            return vec![coordinate];
        }
        let mask = self.local_complement[index - outside_count];
        self.region
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1u128 << i) != 0)
            .map(|(_, &coordinate)| coordinate)
            .collect()
    }

    /// Exact row-addition program for the matrix whose columns are the
    /// component bases, in the same order used by `column`.
    fn row_additions(&self) -> Result<Vec<RowAddition>, Error> {
        if self.lines.is_empty() {
            return Ok(vec![]);
        }
        let n = self.dimension;
        let outside = n - self.region.len();
        let mut permutation = vec![0; n];
        for (i, slot) in permutation.iter_mut().take(self.lines.len()).enumerate() {
            *slot = self.region[i];
        }
        for i in 0..outside {
            permutation[self.lines.len() + i] = self.column(self.lines.len(), i)[0];
        }
        for i in 0..self.local_complement.len() {
            permutation[self.lines.len() + outside + i] = self.region[self.lines.len() + i];
        }
        let mut operations: Vec<_> = PermutationAdditions::new(permutation).collect();
        let mut rows = vec![0u128; self.region.len()];
        let mut columns = Vec::new();
        for line in &self.lines {
            columns.push(line.iter().fold(0u128, |mask, coordinate| {
                mask | (1u128 << self.region.binary_search(coordinate).unwrap())
            }));
        }
        columns.extend(&self.local_complement);
        for (j, &column) in columns.iter().enumerate() {
            for (i, row) in rows.iter_mut().enumerate() {
                if column & (1u128 << i) != 0 {
                    *row |= 1u128 << j;
                }
            }
        }
        let mut elimination = Vec::new();
        for pivot in 0..rows.len() {
            if rows[pivot] & (1u128 << pivot) == 0 {
                let next = (pivot + 1..rows.len())
                    .find(|&i| rows[i] & (1u128 << pivot) != 0)
                    .ok_or(Error::InvalidBinaryBasis)?;
                rows.swap(pivot, next);
                append_swap(&mut elimination, self.region[pivot], self.region[next]);
            }
            for i in 0..rows.len() {
                if i != pivot && rows[i] & (1u128 << pivot) != 0 {
                    rows[i] ^= rows[pivot];
                    elimination.push(RowAddition {
                        target: self.region[i],
                        control: self.region[pivot],
                    });
                }
            }
        }
        // Elimination acts on the left: E * B = I. Reverse its involutions.
        operations.extend(elimination.into_iter().rev());
        Ok(operations)
    }
}

fn parity(value: u128) -> bool {
    !value.count_ones().is_multiple_of(2)
}

fn orthonormalize(mut work: Vec<u128>) -> Result<Vec<u128>, Error> {
    let mut units = Vec::new();
    while let Some(index) = work.iter().position(|&v| parity(v)) {
        let w = work.remove(index);
        for v in &mut work {
            if parity(*v & w) {
                *v ^= w;
            }
        }
        units.push(w);
    }
    while !work.is_empty() {
        let a = work.remove(0);
        let b_index = work
            .iter()
            .position(|&b| parity(a & b))
            .ok_or(Error::InvalidBinaryBasis)?;
        let b = work.remove(b_index);
        for v in &mut work {
            let (va, vb) = (parity(*v & a), parity(*v & b));
            if vb {
                *v ^= a;
            }
            if va {
                *v ^= b;
            }
        }
        let w = units.pop().ok_or(Error::InvalidBinaryBasis)?;
        units.extend([w ^ a, w ^ b, w ^ a ^ b]);
    }
    Ok(units)
}

fn tensor_support(triples: &[Triple]) -> Vec<usize> {
    let mut support = vec![0];
    for triple in triples {
        support = support
            .iter()
            .flat_map(|&x| triple.0.iter().map(move |&y| x * H + y))
            .collect();
    }
    support
}

#[derive(Clone, Debug)]
struct Sector {
    components: [usize; 3],
    lengths: [usize; 3],
    count: usize,
    active: bool,
}

/// A full orthonormal completion M, with its first `rank()` columns equal to
/// a basis of the selected edge residual. No dense M-by-M matrix is stored.
#[derive(Clone, Debug)]
pub struct ResidualBasis {
    invocation: PublishedInvocation,
    edge: EdgeKind,
    factors: [FactorPartition; 3],
    sectors: Vec<Sector>,
    rank: usize,
    decreasing: bool,
}

impl ResidualBasis {
    pub fn new(invocation: PublishedInvocation, edge: EdgeKind) -> Result<Self, Error> {
        if invocation.stage >= 3 {
            return Err(Error::InvalidStage);
        }
        let j = invocation.stage;
        let a = H.pow(j as u32);
        let q = H.pow((2 - j) as u32);
        let before = FactorPartition::with_lines(a, vec![tensor_support(&invocation.fixed[..j])])?;
        let after = FactorPartition::with_lines(q, vec![tensor_support(&invocation.fixed[j..])])?;
        let middle = match edge {
            EdgeKind::XInTo2(t) | EdgeKind::X2To3(t) | EdgeKind::Y0To1(t) | EdgeKind::Y4To5(t) => {
                FactorPartition::with_lines(H, vec![t.0.to_vec()])?
            }
            EdgeKind::SideSourceTo0 { y } | EdgeKind::Side5To7 { y } => {
                FactorPartition::with_lines(H, vec![y.0.to_vec()])?
            }
            EdgeKind::Side0To2 { x, y } | EdgeKind::Side2To5 { x, y } => {
                if !x.is_complex_neighbor(y) {
                    return Err(Error::InvalidNeighborPair);
                }
                FactorPartition::with_lines(H, vec![y.0.to_vec(), x.0.to_vec()])?
            }
            _ => FactorPartition::full(H),
        };
        let factors = [before, middle, after];
        let mut sectors = Vec::new();
        for ca in 0..factors[0].components() {
            for cf in 0..factors[1].components() {
                for cq in 0..factors[2].components() {
                    let active = if edge == EdgeKind::ScratchLastToSink {
                        cq == 1
                    } else if cq != 0 {
                        false
                    } else {
                        match edge {
                            EdgeKind::Unchanged => false,
                            EdgeKind::XInTo2(_) | EdgeKind::Y0To1(_) => ca == 1 && cf == 1,
                            EdgeKind::X2To3(_) | EdgeKind::Y4To5(_) => ca == 0 && cf == 1,
                            EdgeKind::CenterSourceTo1 => ca == 1,
                            EdgeKind::Center1To3 | EdgeKind::Center3To4 | EdgeKind::Center4To6 => {
                                ca == 0
                            }
                            EdgeKind::SideSourceTo0 { .. } => ca == 1 && cf == 0,
                            EdgeKind::Side0To2 { .. } => {
                                (ca == 1 && cf != 0) || (ca == 0 && cf == 1)
                            }
                            EdgeKind::Side2To5 { .. } => ca == 0 && cf == 2,
                            EdgeKind::Side5To7 { .. } => ca == 0 && cf == 0,
                            EdgeKind::ScratchLastToSink => unreachable!(),
                        }
                    };
                    let components = [ca, cf, cq];
                    let lengths = std::array::from_fn(|i| factors[i].component_len(components[i]));
                    let count = lengths.iter().product();
                    if count != 0 {
                        sectors.push(Sector {
                            components,
                            lengths,
                            count,
                            active,
                        });
                    }
                }
            }
        }
        sectors.sort_by_key(|s| !s.active);
        let rank = sectors.iter().filter(|s| s.active).map(|s| s.count).sum();
        Ok(Self {
            invocation,
            edge,
            factors,
            sectors,
            rank,
            decreasing: edge == EdgeKind::Center3To4,
        })
    }

    pub fn rank(&self) -> usize {
        self.rank
    }
    pub fn invocation(&self) -> PublishedInvocation {
        self.invocation
    }
    pub fn edge(&self) -> EdgeKind {
        self.edge
    }
    pub fn decreasing(&self) -> bool {
        self.decreasing
    }

    fn locate(&self, mut index: usize) -> Result<(&Sector, [usize; 3]), Error> {
        if index >= M {
            return Err(Error::IndexOutOfRange);
        }
        for sector in &self.sectors {
            if index < sector.count {
                let last = index % sector.lengths[2];
                index /= sector.lengths[2];
                return Ok((
                    sector,
                    [index / sector.lengths[1], index % sector.lengths[1], last],
                ));
            }
            index -= sector.count;
        }
        Err(Error::InvalidBinaryBasis)
    }

    pub fn column(&self, index: usize) -> Result<SparseVector, Error> {
        let (sector, indices) = self.locate(index)?;
        let columns: [Vec<usize>; 3] =
            std::array::from_fn(|i| self.factors[i].column(sector.components[i], indices[i]));
        let h = self.factors[1].dimension;
        let q = self.factors[2].dimension;
        let mut support = Vec::new();
        for &a in &columns[0] {
            for &b in &columns[1] {
                for &c in &columns[2] {
                    support.push((a * h + b) * q + c);
                }
            }
        }
        Ok(SparseVector {
            dimension: M,
            support,
        })
    }

    /// The phase coefficient is +/- wt(v) mod 4, not just the edge direction.
    pub fn kernel_is_inverse(&self, index: usize) -> Result<bool, Error> {
        if index >= self.rank {
            return Err(Error::IndexOutOfRange);
        }
        let weight = self.column(index)?.support.len();
        if weight % 2 == 0 {
            return Err(Error::InvalidBinaryBasis);
        }
        Ok((weight % 4 == 3) ^ self.decreasing)
    }

    fn product_position(&self, index: usize) -> Result<usize, Error> {
        let (sector, indices) = self.locate(index)?;
        let coordinates: [usize; 3] = std::array::from_fn(|i| {
            self.factors[i].component_offset(sector.components[i]) + indices[i]
        });
        Ok(
            (coordinates[0] * self.factors[1].dimension + coordinates[1])
                * self.factors[2].dimension
                + coordinates[2],
        )
    }

    /// Generate the complete row-addition implementation of M or M^-1.
    /// The iterator uses O(m) index storage, never O(m^2) matrix storage.
    pub fn coordinate_additions(&self, inverse: bool) -> Result<CoordinateAdditions, Error> {
        let mut permutation = vec![0; M];
        for i in 0..M {
            let position = self.product_position(i)?;
            if inverse {
                permutation[position] = i;
            } else {
                permutation[i] = position;
            }
        }
        let mut factor_programs = Vec::new();
        for i in 0..3 {
            let operations = self.factors[i].row_additions()?;
            factor_programs.push(FactorAdditions::new(
                operations,
                self.factors[..i].iter().map(|f| f.dimension).product(),
                self.factors[i].dimension,
                self.factors[i + 1..].iter().map(|f| f.dimension).product(),
                inverse,
            ));
        }
        if inverse {
            factor_programs.reverse();
        }
        Ok(CoordinateAdditions {
            permutation: PermutationAdditions::new(permutation),
            factors: factor_programs,
            factor_index: 0,
            inverse,
            permutation_done: false,
        })
    }
}

/// An elementary binary row addition, an involution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowAddition {
    pub target: usize,
    pub control: usize,
}

fn append_swap(out: &mut Vec<RowAddition>, a: usize, b: usize) {
    out.extend([
        RowAddition {
            target: a,
            control: b,
        },
        RowAddition {
            target: b,
            control: a,
        },
        RowAddition {
            target: a,
            control: b,
        },
    ]);
}

struct PermutationAdditions {
    permutation: Vec<usize>,
    position: usize,
    pending: Option<(usize, usize, u8)>,
}
impl PermutationAdditions {
    fn new(permutation: Vec<usize>) -> Self {
        Self {
            permutation,
            position: 0,
            pending: None,
        }
    }
}
impl Iterator for PermutationAdditions {
    type Item = RowAddition;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((a, b, stage)) = self.pending {
            self.pending = if stage == 2 {
                None
            } else {
                Some((a, b, stage + 1))
            };
            return Some(if stage == 1 {
                RowAddition {
                    target: b,
                    control: a,
                }
            } else {
                RowAddition {
                    target: a,
                    control: b,
                }
            });
        }
        while self.position < self.permutation.len()
            && self.permutation[self.position] == self.position
        {
            self.position += 1;
        }
        if self.position == self.permutation.len() {
            return None;
        }
        let (a, b) = (self.position, self.permutation[self.position]);
        self.permutation.swap(a, b);
        self.pending = Some((a, b, 1));
        Some(RowAddition {
            target: a,
            control: b,
        })
    }
}

struct FactorAdditions {
    operations: Vec<RowAddition>,
    prefix: usize,
    dimension: usize,
    suffix: usize,
    operation: usize,
    replica: usize,
    inverse: bool,
}
impl FactorAdditions {
    fn new(
        operations: Vec<RowAddition>,
        prefix: usize,
        dimension: usize,
        suffix: usize,
        inverse: bool,
    ) -> Self {
        Self {
            operations,
            prefix,
            dimension,
            suffix,
            operation: 0,
            replica: 0,
            inverse,
        }
    }
}
impl Iterator for FactorAdditions {
    type Item = RowAddition;
    fn next(&mut self) -> Option<Self::Item> {
        if self.operation == self.operations.len() {
            return None;
        }
        let index = if self.inverse {
            self.operations.len() - 1 - self.operation
        } else {
            self.operation
        };
        let row = self.operations[index];
        let (prefix, suffix) = (self.replica / self.suffix, self.replica % self.suffix);
        let result = RowAddition {
            target: (prefix * self.dimension + row.target) * self.suffix + suffix,
            control: (prefix * self.dimension + row.control) * self.suffix + suffix,
        };
        self.replica += 1;
        if self.replica == self.prefix * self.suffix {
            self.replica = 0;
            self.operation += 1;
        }
        Some(result)
    }
}

/// Fully specified CNOT program for a published phase frame's coordinate
/// change. Each instruction acts on all columns by the packed primitive.
pub struct CoordinateAdditions {
    permutation: PermutationAdditions,
    factors: Vec<FactorAdditions>,
    factor_index: usize,
    inverse: bool,
    permutation_done: bool,
}
impl Iterator for CoordinateAdditions {
    type Item = RowAddition;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.inverse && !self.permutation_done {
            if let Some(operation) = self.permutation.next() {
                return Some(operation);
            }
            self.permutation_done = true;
        }
        while self.factor_index < self.factors.len() {
            if let Some(operation) = self.factors[self.factor_index].next() {
                return Some(operation);
            }
            self.factor_index += 1;
        }
        if self.inverse {
            self.permutation.next()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(entries: [usize; 3]) -> Triple {
        Triple::new(entries).unwrap()
    }

    #[test]
    fn actual_published_neighbor_indexing_and_wide_roles() {
        let y = t([0, 51, 99]);
        for index in [0, 1, choose3(97) - 1, choose3(97), COMPLEX_NEIGHBORS - 1] {
            let x = y.neighbor(index).unwrap();
            assert!(y.is_complex_neighbor(x));
            assert_eq!(y.neighbor_rank(x).unwrap(), index);
        }
        for rank in [0, 1, 99, 12_345, V - 1] {
            assert_eq!(Triple::from_rank(rank).unwrap().rank(), rank);
        }
        let last = Triple::from_rank(V - 1).unwrap();
        let invocation = PublishedInvocation::new(2, [last, last]).unwrap();
        assert_eq!(invocation.center_role(H).unwrap() + 1, COMPLEX_WIRES);
        assert!(invocation.center_role(H).unwrap() > u64::MAX as u128);
    }

    #[test]
    fn sparse_complement_is_orthonormal_and_orthogonal_to_lines() {
        for lines in [vec![vec![0, 1, 2]], vec![vec![0, 1, 2], vec![0, 1, 3]]] {
            let basis = FactorPartition::with_lines(H, lines.clone()).unwrap();
            let columns: Vec<_> = (0..basis.components())
                .flat_map(|c| (0..basis.component_len(c)).map(move |i| (c, i)))
                .map(|(c, i)| SparseVector {
                    dimension: H,
                    support: basis.column(c, i),
                })
                .collect();
            assert_eq!(columns.len(), H);
            for (i, x) in columns.iter().enumerate() {
                for (j, y) in columns.iter().enumerate() {
                    assert_eq!(x.dot(y).unwrap(), i == j);
                }
            }
        }
    }

    #[test]
    fn factor_row_program_matches_independent_column_matrix() {
        let factor = FactorPartition::with_lines(H, vec![vec![0, 1, 2], vec![0, 1, 3]]).unwrap();
        let operations = factor.row_additions().unwrap();
        for index in 0..H {
            let mut data = vec![false; H];
            data[index] = true;
            for op in &operations {
                data[op.target] ^= data[op.control];
            }
            let (component, local) = if index < 2 {
                (index, 0)
            } else {
                (2, index - 2)
            };
            let expected = factor.column(component, local);
            assert_eq!(
                data.iter()
                    .enumerate()
                    .filter_map(|(i, &v)| v.then_some(i))
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }

    #[test]
    fn published_residual_ranks_and_decreasing_kernel_signs() {
        let (x, y) = (t([0, 1, 2]), t([0, 1, 3]));
        for stage in 0..3 {
            let invocation = PublishedInvocation::new(stage, [t([4, 5, 6]), t([7, 8, 9])]).unwrap();
            let a = H.pow(stage as u32);
            let q = H.pow((2 - stage) as u32);
            for (edge, rank) in [
                (EdgeKind::Unchanged, 0),
                (EdgeKind::XInTo2(x), (a - 1) * (H - 1)),
                (EdgeKind::X2To3(x), H - 1),
                (EdgeKind::Y0To1(y), (a - 1) * (H - 1)),
                (EdgeKind::Y4To5(y), H - 1),
                (EdgeKind::CenterSourceTo1, (a - 1) * H),
                (EdgeKind::Center1To3, H),
                (EdgeKind::Center3To4, H),
                (EdgeKind::SideSourceTo0 { y }, a - 1),
                (EdgeKind::Side0To2 { x, y }, (a - 1) * (H - 1) + 1),
                (EdgeKind::Side2To5 { x, y }, H - 2),
                (EdgeKind::Side5To7 { y }, 1),
                (EdgeKind::ScratchLastToSink, a * H * (q - 1)),
            ] {
                let basis = ResidualBasis::new(invocation, edge).unwrap();
                assert_eq!(basis.rank(), rank, "stage={stage}, edge={edge:?}");
                assert_eq!(basis.sectors.iter().map(|s| s.count).sum::<usize>(), M);
                for i in [0, rank.saturating_sub(1), M / 2, M - 1] {
                    assert!(basis.column(i).unwrap().support.len() % 2 == 1);
                }
            }
            let forward = ResidualBasis::new(invocation, EdgeKind::Center1To3).unwrap();
            let inverse = ResidualBasis::new(invocation, EdgeKind::Center3To4).unwrap();
            for i in 0..H {
                assert_eq!(forward.column(i).unwrap(), inverse.column(i).unwrap());
                assert_ne!(
                    forward.kernel_is_inverse(i).unwrap(),
                    inverse.kernel_is_inverse(i).unwrap()
                );
            }
        }
    }

    #[test]
    fn million_dimensional_coordinate_program_has_correct_effect_and_inverse() {
        let invocation = PublishedInvocation::new(1, [t([0, 1, 2]), t([4, 5, 6])]).unwrap();
        let basis = ResidualBasis::new(
            invocation,
            EdgeKind::Side0To2 {
                x: t([7, 8, 9]),
                y: t([7, 8, 10]),
            },
        )
        .unwrap();
        let chosen = [0, 1, basis.rank() - 1, basis.rank(), M - 1];
        // Five independent basis vectors in the bits of each byte.
        let mut data = vec![0u8; M];
        let mut expected = vec![0u8; M];
        for (bit, &index) in chosen.iter().enumerate() {
            data[index] |= 1 << bit;
            for coordinate in basis.column(index).unwrap().support {
                expected[coordinate] |= 1 << bit;
            }
        }
        let original = data.clone();
        for operation in basis.coordinate_additions(false).unwrap() {
            data[operation.target] ^= data[operation.control];
        }
        assert_eq!(data, expected);
        for operation in basis.coordinate_additions(true).unwrap() {
            data[operation.target] ^= data[operation.control];
        }
        assert_eq!(data, original);
    }

    #[test]
    fn packed_rotations_repair_current_bad_addresses_exactly() {
        for selected_bit in [0, 1, 5] {
            let parameters = PackedParameters {
                chunks: 2,
                chunk_bits: 6,
                selected_bit,
            };
            for y in 0u64..64 {
                for z in 0u64..64 {
                    let original = PackedAddress {
                        x: Big::from_u64(1)
                            .shl(selected_bit)
                            .add(&Big::from_u64(1).shl(selected_bit + 6)),
                        y: Big::from_u64(y).shl(selected_bit),
                        z: Big::from_u64(z).shl(selected_bit),
                    };
                    let current = parameters.eight_rotations(&original, false).unwrap();
                    assert_eq!(
                        parameters.eight_rotations(&current, true).unwrap(),
                        original
                    );
                    assert_eq!(
                        parameters.is_bad(&current).unwrap(),
                        parameters.is_bad(&original).unwrap()
                    );
                    let ideal = parameters.ideal_lower(&original);
                    if !parameters.is_bad(&original).unwrap() {
                        assert_eq!(current, ideal);
                    }
                    assert_eq!(parameters.repair_destination(&current).unwrap(), ideal);
                }
            }
        }
    }

    #[test]
    fn encoded_layer_matches_direct_walsh_sum_with_spectators() {
        let (prefix, suffix, axes, chunk_bits, selected_bit, coefficients, p) =
            (2usize, 3usize, 3usize, 2usize, 1usize, 2usize, 12usize);
        let range = 1usize << (axes * chunk_bits);
        let records = prefix * range * suffix;
        let fixture = |index: usize| ((index * 7 % 41) as i64 - 20, (index * 11 % 37) as i64 - 18);
        let mut values: Vec<_> = (0..records * coefficients)
            .map(|i| {
                let (re, im) = fixture(i);
                Complex {
                    re: Signed::from_i64(re),
                    im: Signed::from_i64(im),
                }
            })
            .collect();
        let selected: Vec<_> = (0..axes)
            .map(|axis| (axes - axis - 1) * chunk_bits + selected_bit)
            .collect();
        let mask = selected.iter().fold(0, |mask, &bit| mask | (1usize << bit));
        let mut expected = Vec::new();
        for rank in 0..records {
            let address = rank / suffix;
            for coefficient in 0..coefficients {
                let (mut re, mut im) = (0i64, 0i64);
                for combination in 0..1usize << axes {
                    let mut source_address = address & !mask;
                    let mut odd = false;
                    for (axis, &bit) in selected.iter().enumerate() {
                        if combination & (1 << axis) != 0 {
                            source_address |= 1 << bit;
                            odd ^= address & (1 << bit) != 0;
                        }
                    }
                    let source =
                        (source_address * suffix + rank % suffix) * coefficients + coefficient;
                    let (r, i) = fixture(source);
                    if odd {
                        re -= r;
                        im -= i;
                    } else {
                        re += r;
                        im += i;
                    }
                }
                expected.push(Complex {
                    re: Signed::from_i64(re / (1 << axes)),
                    im: Signed::from_i64(im / (1 << axes)),
                });
            }
        }
        let receipt = normalized_layer_values(
            &mut values,
            prefix,
            suffix,
            axes,
            chunk_bits,
            selected_bit,
            coefficients,
            p,
        )
        .unwrap();
        assert_eq!(values, expected);
        assert_eq!(receipt.individual_kernels, Big::from_u128(axes as u128));
        assert!(receipt.network_nodes.is_zero());
        assert!(receipt.tape.read_bits > Big::zero());
    }

    #[test]
    fn published_admission_is_checked_before_any_tape_work() {
        assert!(check_layer_admission(3, 4, 2, 32).is_ok());
        assert!(matches!(
            check_layer_admission(M + 1000, M + 1001, 6, 1024),
            Err(TensorError::GuardAdmission { .. })
        ));
        let delta = published_guard(&Big::from_u64(1));
        assert!(delta > Big::from_u128(u128::MAX));
        assert_eq!(published_guard(&Big::from_u64(2)), delta.shl(20));
        assert_eq!(published_row_axes(&Big::from_u64(1)), Big::from_u64(71));
    }
}
