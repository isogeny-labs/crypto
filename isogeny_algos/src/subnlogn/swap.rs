//! Constructive address-chunk interchanges from sections 2--4 of the paper.
//!
//! The fixed-width base case moves individual address bits on sequential
//! streams.  The large-width path compiles the published rational bit motif,
//! supplies rows from high digits, and recursively executes one smaller
//! interchange per rational edge rank.  Neither path uses random scatter.
//! Tape receipts describe the represented machine, separately from the host
//! filesystem interpreter's overhead.

use super::certified::{Complex, Signed};
use super::tape::{Tape, TapeCost, TapeWorkspace};
use super::tensor::{PublishedInvocation, Triple, H, M, V};
use crate::bigint::Big;
use std::{fmt, io};

const BIT_NEIGHBORS: usize = 13_968;
const DATA: u128 = (V as u128) * (V as u128) * (V as u128);
const WIRES: u128 = 177_176_569_091_445_000_000;
const EDGE_RANKS: u128 = 177_176_569_088_785_861_287_000_000;
// A fixed finite base-case interval is permitted by Lemma 4.4.  The
// recursive branch is independently defined for all larger widths.
const DIRECT_BITS: usize = M;

#[derive(Debug)]
pub enum SwapError {
    Io(io::Error),
    InvalidShape,
    SizeOverflow,
    Allocation,
    Construction(&'static str),
}
impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "chunk-swap tape: {e}"),
            Self::InvalidShape => f.write_str("invalid chunk-swap shape or payload"),
            Self::SizeOverflow => f.write_str("chunk-swap native descriptor size overflow"),
            Self::Allocation => f.write_str("chunk-swap fixed construction exceeds native memory"),
            Self::Construction(s) => write!(f, "chunk-swap construction invariant: {s}"),
        }
    }
}
impl std::error::Error for SwapError {}
impl From<io::Error> for SwapError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapWork {
    pub individual_bit_swaps: Big,
    pub recursive_nodes: Big,
    pub rank_children: Big,
}
impl Default for SwapWork {
    fn default() -> Self {
        Self {
            individual_bit_swaps: Big::zero(),
            recursive_nodes: Big::zero(),
            rank_children: Big::zero(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapReceipt {
    pub tape: TapeCost,
    pub work: SwapWork,
    pub input_bits: Big,
    pub component_bits: usize,
}

fn mul(a: &Big, b: &Big) -> Big {
    super::fastint::multiply(a, b)
}
fn div(a: &Big, b: &Big) -> (Big, Big) {
    super::fastdiv::divrem(a, b)
}
fn one() -> Big {
    Big::from_u64(1)
}
fn pow2(n: usize) -> Big {
    one().shl(n)
}
fn product(values: &[Big]) -> Big {
    values.iter().fold(one(), |a, b| mul(&a, b))
}
fn power(base: &Big, mut exponent: usize) -> Big {
    let mut result = one();
    let mut base = base.clone();
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul(&result, &base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = mul(&base, &base);
        }
    }
    result
}
fn increment(value: &mut Big) {
    for limb in &mut value.0 {
        let (v, carry) = limb.overflowing_add(1);
        *limb = v;
        if !carry {
            return;
        }
    }
    value.0.push(1);
}
fn decrement(value: &mut Big) {
    assert!(!value.is_zero());
    for limb in &mut value.0 {
        let (v, borrow) = limb.overflowing_sub(1);
        *limb = v;
        if !borrow {
            break;
        }
    }
    while value.0.last() == Some(&0) {
        value.0.pop();
    }
}
fn repeat(mut count: Big, mut f: impl FnMut() -> Result<(), SwapError>) -> Result<(), SwapError> {
    while !count.is_zero() {
        f()?;
        decrement(&mut count);
    }
    Ok(())
}
fn replace(tape: &mut Tape, mut output: Tape, ws: &mut TapeWorkspace) -> Result<(), SwapError> {
    tape.clear()?;
    output.rewind()?;
    output.copy_to(tape)?;
    tape.rewind()?;
    ws.release_tape(output)?;
    Ok(())
}

/// Interchange the two binary chunks in `[P][2^u][G][2^u][B]`.
/// `B` counts payload bits, including every trailing spectator field.
pub fn swap_chunks(
    tape: &mut Tape,
    prefix: &Big,
    width: usize,
    gap: &Big,
    suffix: &Big,
    workspace: &mut TapeWorkspace,
) -> Result<SwapWork, SwapError> {
    if prefix.is_zero() || gap.is_zero() || suffix.is_zero() {
        return Err(SwapError::InvalidShape);
    }
    let expected = product(&[
        prefix.clone(),
        pow2(width),
        gap.clone(),
        pow2(width),
        suffix.clone(),
    ]);
    if tape.len() != &expected {
        return Err(SwapError::InvalidShape);
    }
    let mut work = SwapWork::default();
    if width <= DIRECT_BITS {
        direct_binary(tape, prefix, width, gap, suffix, workspace, &mut work)?;
    } else {
        let prime = compile_prime()?;
        let target = pow2(width);
        let mut e = 0usize;
        let mut range = one();
        while range < target {
            range = mul(&range, &prime);
            e = e.checked_add(1).ok_or(SwapError::SizeOverflow)?;
        }
        pad_pair(tape, prefix, &target, &range, gap, suffix, true, workspace)?;
        let parking = workspace.create_tape()?;
        let descriptors = workspace.create_tape()?;
        let mut context = Context {
            workspace,
            parking,
            descriptors,
            prime,
            work,
        };
        arbitrary_radix(tape, prefix, e, gap, suffix, &mut context)?;
        work = context.work.clone();
        context.workspace.release_tape(context.parking)?;
        context.workspace.release_tape(context.descriptors)?;
        pad_pair(
            tape,
            prefix,
            &target,
            &range,
            gap,
            suffix,
            false,
            context.workspace,
        )?;
    }
    tape.rewind()?;
    Ok(work)
}

/// Serialize exact complex numerators, execute the physical permutation, and
/// restore the same values.  Precision `p` does not bound the integer part:
/// wider CRT/recovery numerators determine a larger common component width.
pub fn swap_record_chunks(
    values: &mut [Complex],
    prefix_bits: usize,
    chunk_bits: usize,
    gap_bits: usize,
    suffix_bits: usize,
    record_coefficients: usize,
    p: usize,
) -> Result<SwapReceipt, SwapError> {
    if record_coefficients == 0 {
        return Err(SwapError::InvalidShape);
    }
    let address_bits = prefix_bits
        .checked_add(chunk_bits)
        .and_then(|n| n.checked_add(gap_bits))
        .and_then(|n| n.checked_add(chunk_bits))
        .and_then(|n| n.checked_add(suffix_bits))
        .ok_or(SwapError::SizeOverflow)?;
    let records = 1usize
        .checked_shl(u32::try_from(address_bits).map_err(|_| SwapError::SizeOverflow)?)
        .ok_or(SwapError::SizeOverflow)?;
    if records.checked_mul(record_coefficients) != Some(values.len()) {
        return Err(SwapError::InvalidShape);
    }
    let mut width = p.checked_add(2).ok_or(SwapError::SizeOverflow)?;
    for value in values.iter() {
        width = width.max(
            value
                .re
                .magnitude
                .bits()
                .checked_add(1)
                .ok_or(SwapError::SizeOverflow)?,
        );
        width = width.max(
            value
                .im
                .magnitude
                .bits()
                .checked_add(1)
                .ok_or(SwapError::SizeOverflow)?,
        );
    }
    let mut workspace = TapeWorkspace::create_in(std::env::temp_dir())?;
    let mut tape = workspace.create_tape()?;
    let result = (|| {
        for value in values.iter() {
            write_signed(&mut tape, &value.re, width)?;
            write_signed(&mut tape, &value.im, width)?;
        }
        let input_bits = tape.len().clone();
        let payload = Big::from_u128(record_coefficients as u128)
            .mul_small(2)
            .mul_small(width as u64)
            .shl(suffix_bits);
        let work = swap_chunks(
            &mut tape,
            &pow2(prefix_bits),
            chunk_bits,
            &pow2(gap_bits),
            &payload,
            &mut workspace,
        )?;
        for value in values.iter_mut() {
            value.re = read_signed(&mut tape, width)?;
            value.im = read_signed(&mut tape, width)?;
        }
        tape.clear()?;
        Ok(SwapReceipt {
            tape: workspace.cost(),
            work,
            input_bits,
            component_bits: width,
        })
    })();
    drop(tape);
    let cleanup = workspace.cleanup();
    match result {
        Ok(receipt) => {
            cleanup?;
            Ok(receipt)
        }
        Err(e) => Err(e),
    }
}
fn write_signed(tape: &mut Tape, value: &Signed, width: usize) -> Result<(), SwapError> {
    if value.magnitude.bits() >= width {
        return Err(SwapError::InvalidShape);
    }
    let bits = if value.negative && !value.magnitude.is_zero() {
        pow2(width).sub(&value.magnitude)
    } else {
        value.magnitude.clone()
    };
    tape.write_unsigned(&bits, width)?;
    Ok(())
}
fn read_signed(tape: &mut Tape, width: usize) -> Result<Signed, SwapError> {
    let bits = tape.read_unsigned(width)?;
    Ok(if bits.bit(width - 1) {
        Signed::new(true, pow2(width).sub(&bits))
    } else {
        Signed::from_big(bits)
    })
}

fn direct_binary(
    tape: &mut Tape,
    p: &Big,
    u: usize,
    g: &Big,
    b: &Big,
    ws: &mut TapeWorkspace,
    work: &mut SwapWork,
) -> Result<(), SwapError> {
    for position in 0..u {
        swap_bit(
            tape,
            &p.shl(position),
            &g.shl(u - 1),
            &b.shl(u - position - 1),
            ws,
        )?;
        increment(&mut work.individual_bit_swaps);
    }
    Ok(())
}
fn swap_bit(
    tape: &mut Tape,
    p: &Big,
    g: &Big,
    b: &Big,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    let mut streams = [
        ws.create_tape()?,
        ws.create_tape()?,
        ws.create_tape()?,
        ws.create_tape()?,
    ];
    tape.rewind()?;
    repeat(p.clone(), || {
        for h in 0..2 {
            repeat(g.clone(), || {
                for d in 0..2 {
                    tape.copy_bits_to(&mut streams[2 * h + d], b)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    })?;
    for stream in &mut streams {
        stream.rewind()?;
    }
    tape.clear()?;
    repeat(p.clone(), || {
        for d in 0..2 {
            repeat(g.clone(), || {
                for h in 0..2 {
                    streams[2 * h + d].copy_bits_to(tape, b)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    })?;
    for stream in streams {
        ws.release_tape(stream)?;
    }
    tape.rewind()?;
    Ok(())
}

// Move a binary field across an arbitrary complete middle interval.  All
// inner loops consume contiguous blocks; no payload rank is used as a seek.
fn move_bit(
    tape: &mut Tape,
    p: &Big,
    middle: &Big,
    b: &Big,
    right: bool,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    let mut streams = [ws.create_tape()?, ws.create_tape()?];
    tape.rewind()?;
    repeat(p.clone(), || {
        if right {
            for stream in &mut streams {
                repeat(middle.clone(), || {
                    tape.copy_bits_to(stream, b)?;
                    Ok(())
                })?;
            }
        } else {
            repeat(middle.clone(), || {
                for stream in &mut streams {
                    tape.copy_bits_to(stream, b)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    })?;
    for stream in &mut streams {
        stream.rewind()?;
    }
    tape.clear()?;
    repeat(p.clone(), || {
        if right {
            repeat(middle.clone(), || {
                for stream in &mut streams {
                    stream.copy_bits_to(tape, b)?;
                }
                Ok(())
            })?;
        } else {
            for stream in &mut streams {
                repeat(middle.clone(), || {
                    stream.copy_bits_to(tape, b)?;
                    Ok(())
                })?;
            }
        }
        Ok(())
    })?;
    for stream in streams {
        ws.release_tape(stream)?;
    }
    tape.rewind()?;
    Ok(())
}

fn pad_field(
    tape: &mut Tape,
    p: &Big,
    old: &Big,
    new: &Big,
    b: &Big,
    insert: bool,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    if old > new {
        return Err(SwapError::InvalidShape);
    }
    let valid = mul(old, b);
    let extra = mul(&new.sub(old), b);
    let mut output = ws.create_tape()?;
    tape.rewind()?;
    repeat(p.clone(), || {
        tape.copy_bits_to(&mut output, &valid)?;
        if insert {
            output.write_zero_bits(&extra)?;
        } else {
            repeat(extra.clone(), || {
                tape.require_bit()?;
                Ok(())
            })?;
        }
        Ok(())
    })?;
    replace(tape, output, ws)
}
fn pad_pair(
    tape: &mut Tape,
    p: &Big,
    old: &Big,
    new: &Big,
    g: &Big,
    b: &Big,
    insert: bool,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    if insert {
        pad_field(
            tape,
            &product(&[p.clone(), old.clone(), g.clone()]),
            old,
            new,
            b,
            true,
            ws,
        )?;
        pad_field(
            tape,
            p,
            old,
            new,
            &product(&[g.clone(), new.clone(), b.clone()]),
            true,
            ws,
        )
    } else {
        pad_field(
            tape,
            p,
            old,
            new,
            &product(&[g.clone(), new.clone(), b.clone()]),
            false,
            ws,
        )?;
        pad_field(
            tape,
            &product(&[p.clone(), old.clone(), g.clone()]),
            old,
            new,
            b,
            false,
            ws,
        )
    }
}
fn radix_digit_swap(
    tape: &mut Tape,
    p: &Big,
    q: &Big,
    g: &Big,
    b: &Big,
    ws: &mut TapeWorkspace,
    work: &mut SwapWork,
) -> Result<(), SwapError> {
    let bits = q.bits();
    let padded = pow2(bits);
    pad_pair(tape, p, q, &padded, g, b, true, ws)?;
    direct_binary(tape, p, bits, g, b, ws, work)?;
    pad_pair(tape, p, q, &padded, g, b, false, ws)
}
fn move_radix_digit(
    tape: &mut Tape,
    p: &Big,
    q: &Big,
    middle: &Big,
    b: &Big,
    right: bool,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    let bits = q.bits();
    let padded = pow2(bits);
    if right {
        pad_field(tape, p, q, &padded, &mul(middle, b), true, ws)?;
        for done in 0..bits {
            move_bit(
                tape,
                &p.shl(bits - done - 1),
                middle,
                &b.shl(done),
                true,
                ws,
            )?;
        }
        pad_field(tape, &mul(p, middle), q, &padded, b, false, ws)
    } else {
        pad_field(tape, &mul(p, middle), q, &padded, b, true, ws)?;
        for done in 0..bits {
            move_bit(
                tape,
                &p.shl(done),
                middle,
                &b.shl(bits - done - 1),
                false,
                ws,
            )?;
        }
        pad_field(tape, p, q, &padded, &mul(middle, b), false, ws)
    }
}

// Exact fixed rational compiler.  Its dimension is a fixed part of the
// published machine, independent of the input width.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Rational {
    numerator: Signed,
    denominator: Big,
}
impl Rational {
    fn new(numerator: Signed, denominator: Big) -> Self {
        assert!(!denominator.is_zero());
        if numerator.is_zero() {
            return Self::integer(0);
        }
        let mut a = numerator.magnitude.clone();
        let mut b = denominator.clone();
        while !b.is_zero() {
            let r = div(&a, &b).1;
            a = b;
            b = r;
        }
        Self {
            numerator: Signed::new(numerator.negative, div(&numerator.magnitude, &a).0),
            denominator: div(&denominator, &a).0,
        }
    }
    fn integer(n: i64) -> Self {
        Self {
            numerator: Signed::from_i64(n),
            denominator: one(),
        }
    }
    fn zero(&self) -> bool {
        self.numerator.is_zero()
    }
    fn neg(&self) -> Self {
        Self {
            numerator: self.numerator.neg(),
            denominator: self.denominator.clone(),
        }
    }
    fn add(&self, rhs: &Self) -> Self {
        Self::new(
            self.numerator
                .mul_big(&rhs.denominator)
                .add(&rhs.numerator.mul_big(&self.denominator)),
            mul(&self.denominator, &rhs.denominator),
        )
    }
    fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }
    fn mul(&self, rhs: &Self) -> Self {
        Self::new(
            self.numerator.mul(&rhs.numerator),
            mul(&self.denominator, &rhs.denominator),
        )
    }
    fn inv(&self) -> Self {
        assert!(!self.zero());
        Self::new(
            Signed::new(self.numerator.negative, self.denominator.clone()),
            self.numerator.magnitude.clone(),
        )
    }
    fn rem(&self, modulus: &Big) -> Big {
        let n = div(&self.numerator.magnitude, modulus).1;
        let n = if self.numerator.negative && !n.is_zero() {
            modulus.sub(&n)
        } else {
            n
        };
        div(&mul(&n, &inverse_mod(&self.denominator, modulus)), modulus).1
    }
}
fn inverse_mod(value: &Big, modulus: &Big) -> Big {
    let mut a = modulus.clone();
    let mut b = div(value, modulus).1;
    let mut x = Signed::zero();
    let mut y = Signed::from_i64(1);
    while !b.is_zero() {
        let (q, r) = div(&a, &b);
        let next = x.sub(&y.mul_big(&q));
        a = b;
        b = r;
        x = y;
        y = next;
    }
    assert_eq!(a, one(), "nonunit in compiled rational shear");
    let xmod = div(&x.magnitude, modulus).1;
    if x.negative && !xmod.is_zero() {
        modulus.sub(&xmod)
    } else {
        xmod
    }
}
#[derive(Clone, Debug)]
struct Matrix {
    n: usize,
    entries: Vec<Rational>,
}
impl Matrix {
    fn zeros(n: usize) -> Result<Self, SwapError> {
        let count = n.checked_mul(n).ok_or(SwapError::SizeOverflow)?;
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(count)
            .map_err(|_| SwapError::Allocation)?;
        entries.resize_with(count, || Rational::integer(0));
        Ok(Self { n, entries })
    }
    fn identity(n: usize) -> Result<Self, SwapError> {
        let mut a = Self::zeros(n)?;
        for i in 0..n {
            a.set(i, i, Rational::integer(1));
        }
        Ok(a)
    }
    fn get(&self, i: usize, j: usize) -> &Rational {
        &self.entries[i * self.n + j]
    }
    fn set(&mut self, i: usize, j: usize, x: Rational) {
        self.entries[i * self.n + j] = x;
    }
    fn lower_inverse(&self) -> Result<Self, SwapError> {
        let mut inv = Self::zeros(self.n)?;
        for i in 0..self.n {
            if self.get(i, i).zero() {
                return Err(SwapError::Construction("zero triangular diagonal"));
            }
            let diagonal = self.get(i, i).inv();
            inv.set(i, i, diagonal.clone());
            for j in 0..i {
                let mut sum = Rational::integer(0);
                for k in j..i {
                    sum = sum.add(&self.get(i, k).mul(inv.get(k, j)));
                }
                inv.set(i, j, sum.neg().mul(&diagonal));
            }
        }
        Ok(inv)
    }
    fn admissible(&self, q: &Big) -> bool {
        self.entries
            .iter()
            .all(|x| !div(&x.denominator, q).1.is_zero())
            && (0..self.n).all(|i| !div(&self.get(i, i).numerator.magnitude, q).1.is_zero())
    }
}
struct Factors {
    left: Matrix,
    right: Matrix,
    left_inverse: Matrix,
    right_inverse: Matrix,
    pivots: Vec<(usize, usize)>,
}
fn factor(mut a: Matrix) -> Result<Factors, SwapError> {
    let n = a.n;
    let mut left = Matrix::identity(n)?;
    let mut right = Matrix::identity(n)?;
    let mut rows = vec![true; n];
    let mut cols = vec![true; n];
    let mut pivots = Vec::new();
    loop {
        let mut pivot = None;
        'search: for i in 0..n {
            if rows[i] {
                for j in (0..n).rev() {
                    if cols[j] && !a.get(i, j).zero() {
                        pivot = Some((i, j));
                        break 'search;
                    }
                }
            }
        }
        let Some((i, j)) = pivot else { break };
        let value = a.get(i, j).clone();
        for k in 0..j {
            if cols[k] && !a.get(i, k).zero() {
                let c = a.get(i, k).mul(&value.inv());
                for r in 0..n {
                    a.set(r, k, a.get(r, k).sub(&a.get(r, j).mul(&c)));
                    right.set(r, k, right.get(r, k).sub(&right.get(r, j).mul(&c)));
                }
            }
        }
        for k in i + 1..n {
            if rows[k] && !a.get(k, j).zero() {
                let c = a.get(k, j).mul(&value.inv());
                for col in 0..n {
                    a.set(k, col, a.get(k, col).sub(&a.get(i, col).mul(&c)));
                    left.set(k, col, left.get(k, col).sub(&left.get(i, col).mul(&c)));
                }
            }
        }
        let scale = value.inv();
        for col in 0..n {
            a.set(i, col, a.get(i, col).mul(&scale));
            left.set(i, col, left.get(i, col).mul(&scale));
        }
        rows[i] = false;
        cols[j] = false;
        pivots.push((i, j));
    }
    let left_inverse = left.lower_inverse()?;
    let right_inverse = right.lower_inverse()?;
    Ok(Factors {
        left,
        right,
        left_inverse,
        right_inverse,
        pivots,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StageLabel {
    IncomingX,
    IncomingY,
    YCenter,
    XSide,
    XCenter,
    YSide,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Label {
    Zero,
    Full,
    Line([Triple; 3]),
    NegativeLine([Triple; 3]),
    Stage {
        stage: usize,
        triples: [Triple; 3],
        kind: StageLabel,
    },
}
fn triple(index: usize) -> Result<Triple, SwapError> {
    Triple::from_rank(index).map_err(|_| SwapError::Construction("triple index"))
}
fn triples(inv: PublishedInvocation, t: Triple) -> [Triple; 3] {
    match inv.stage {
        0 => [t, inv.fixed[0], inv.fixed[1]],
        1 => [inv.fixed[0], t, inv.fixed[1]],
        _ => [inv.fixed[0], inv.fixed[1], t],
    }
}
fn stage_label(inv: PublishedInvocation, t: Triple, kind: StageLabel) -> Label {
    if inv.stage == 0 && matches!(kind, StageLabel::IncomingY | StageLabel::YCenter) {
        return Label::Zero;
    }
    if inv.stage == 2 && kind == StageLabel::XCenter {
        return Label::Full;
    }
    let ts = triples(inv, t);
    if inv.stage == 0 && matches!(kind, StageLabel::IncomingX | StageLabel::XSide) {
        return Label::Line(ts);
    }
    Label::Stage {
        stage: inv.stage,
        triples: ts,
        kind,
    }
}
fn line_entry(t: Triple, row: usize, col: usize) -> Rational {
    if !t.entries().contains(&row) {
        return Rational::integer(0);
    }
    Rational::new(
        Signed::from_i64(if t.entries().contains(&col) { 2 } else { -1 }),
        Big::from_u64(6),
    )
}
impl Label {
    fn entry(self, row: usize, col: usize) -> Rational {
        let rr = [row / (H * H), (row / H) % H, row % H];
        let cc = [col / (H * H), (col / H) % H, col % H];
        let identity = |a: usize, b: usize| Rational::integer(i64::from(a == b));
        match self {
            Self::Zero => Rational::integer(0),
            Self::Full => identity(row, col),
            Self::Line(ts) | Self::NegativeLine(ts) => {
                let mut value = Rational::integer(1);
                for k in 0..3 {
                    value = value.mul(&line_entry(ts[k], rr[k], cc[k]));
                }
                if matches!(self, Self::NegativeLine(_)) {
                    value.neg()
                } else {
                    value
                }
            }
            Self::Stage {
                stage,
                triples: ts,
                kind,
            } => {
                let mut before = Rational::integer(1);
                let mut ibefore = Rational::integer(1);
                let mut after = Rational::integer(1);
                for k in 0..stage {
                    before = before.mul(&line_entry(ts[k], rr[k], cc[k]));
                    ibefore = ibefore.mul(&identity(rr[k], cc[k]));
                }
                for k in stage + 1..3 {
                    after = after.mul(&line_entry(ts[k], rr[k], cc[k]));
                }
                let pt = line_entry(ts[stage], rr[stage], cc[stage]);
                let it = identity(rr[stage], cc[stage]);
                let b = ibefore.sub(&before);
                let core = match kind {
                    StageLabel::IncomingX => ibefore.mul(&pt),
                    StageLabel::IncomingY => b.mul(&pt),
                    StageLabel::YCenter => b.mul(&it),
                    StageLabel::XSide => b.mul(&it).add(&before.mul(&pt)),
                    StageLabel::XCenter => ibefore.mul(&it),
                    StageLabel::YSide => ibefore.mul(&it).sub(&before.mul(&pt)),
                };
                core.mul(&after)
            }
        }
    }
}
fn edge_factors(from: Label, to: Label) -> Result<Factors, SwapError> {
    let mut matrix = Matrix::zeros(M)?;
    for row in 0..M {
        for col in 0..M {
            matrix.set(row, col, to.entry(row, col).sub(&from.entry(row, col)));
        }
    }
    factor(matrix)
}
fn bit_neighbor(y: Triple, index: usize) -> Result<Triple, SwapError> {
    if index >= BIT_NEIGHBORS {
        return Err(SwapError::Construction("bit neighbor index"));
    }
    let entries = y.entries();
    let outside = (0..H).filter(|x| !entries.contains(x)).collect::<Vec<_>>();
    let selected = index / 4656;
    let mut pair = index % 4656;
    for first in 0..outside.len() - 1 {
        let count = outside.len() - first - 1;
        if pair < count {
            let mut result = [entries[selected], outside[first], outside[first + 1 + pair]];
            result.sort_unstable();
            return Triple::new(result).map_err(|_| SwapError::Construction("bit neighbor"));
        }
        pair -= count;
    }
    Err(SwapError::Construction("bit neighbor unranking"))
}
fn bit_neighbor_rank(y: Triple, x: Triple) -> Result<usize, SwapError> {
    if y.intersection(x) != 1 {
        return Err(SwapError::Construction("nonneighbor side wire"));
    }
    let yy = y.entries();
    let xx = x.entries();
    let selected = yy.iter().position(|t| xx.contains(t)).unwrap();
    let outside = (0..H).filter(|t| !yy.contains(t)).collect::<Vec<_>>();
    let pair = xx
        .iter()
        .filter(|t| !yy.contains(t))
        .map(|t| outside.iter().position(|a| *a == *t).unwrap())
        .collect::<Vec<_>>();
    let before = pair[0] * (2 * outside.len() - pair[0] - 1) / 2;
    Ok(selected * 4656 + before + pair[1] - pair[0] - 1)
}
fn scratch_start(inv: PublishedInvocation) -> u128 {
    2 * DATA + inv.index() * (V as u128 * BIT_NEIGHBORS as u128 + H as u128)
}
fn side_role(inv: PublishedInvocation, x: Triple, y: Triple) -> Result<u128, SwapError> {
    Ok(scratch_start(inv) + (y.rank() * BIT_NEIGHBORS + bit_neighbor_rank(y, x)?) as u128)
}
fn center_role(inv: PublishedInvocation, i: usize) -> u128 {
    scratch_start(inv) + (V * BIT_NEIGHBORS + i) as u128
}
#[derive(Clone, Copy)]
enum Event {
    Edge { role: u128, from: Label, to: Label },
    Xor { target: u128, control: u128 },
}
fn emit_edge(
    emit: &mut impl FnMut(Event) -> Result<bool, SwapError>,
    role: u128,
    from: Label,
    to: Label,
) -> Result<bool, SwapError> {
    if from == to {
        Ok(true)
    } else {
        emit(Event::Edge { role, from, to })
    }
}
// This emits every common gate's incoming edges before any of its scalar
// updates.  It is a generator for the explicit h=100 network, not a supplied
// circuit or an unimplemented arithmetic callback.
fn invocation_events(
    inv: PublishedInvocation,
    emit: &mut impl FnMut(Event) -> Result<bool, SwapError>,
) -> Result<bool, SwapError> {
    let inverse = inv.stage == 1;
    for time in 0..8 {
        if matches!(time, 0 | 2 | 5 | 7) {
            let y_gate = matches!(time, 0 | 5);
            for tr in 0..V {
                let t = triple(tr)?;
                let target_label = stage_label(
                    inv,
                    t,
                    match time {
                        0 => StageLabel::IncomingY,
                        2 => StageLabel::XSide,
                        5 => StageLabel::YSide,
                        _ => StageLabel::XCenter,
                    },
                );
                let data_role = if y_gate { inv.y_role(t) } else { inv.x_role(t) };
                let data_previous = match time {
                    0 => stage_label(inv, t, StageLabel::IncomingY),
                    2 => {
                        if inv.stage == 0 {
                            Label::NegativeLine(triples(inv, t))
                        } else {
                            stage_label(inv, t, StageLabel::IncomingX)
                        }
                    }
                    5 => stage_label(inv, t, StageLabel::YCenter),
                    _ => stage_label(inv, t, StageLabel::XCenter),
                };
                if !emit_edge(emit, data_role, data_previous, target_label)? {
                    return Ok(false);
                }
                for ni in 0..BIT_NEIGHBORS {
                    let other = bit_neighbor(t, ni)?;
                    let (x, y) = if y_gate { (other, t) } else { (t, other) };
                    let previous = match time {
                        0 => Label::Zero,
                        2 => stage_label(inv, y, StageLabel::IncomingY),
                        5 => stage_label(inv, x, StageLabel::XSide),
                        _ => stage_label(inv, y, StageLabel::YSide),
                    };
                    if !emit_edge(emit, side_role(inv, x, y)?, previous, target_label)? {
                        return Ok(false);
                    }
                }
                let data_is_target = y_gate != inverse;
                for ni in 0..BIT_NEIGHBORS {
                    let other = bit_neighbor(t, ni)?;
                    let (x, y) = if y_gate { (other, t) } else { (t, other) };
                    let side = side_role(inv, x, y)?;
                    let event = if data_is_target {
                        Event::Xor {
                            target: data_role,
                            control: side,
                        }
                    } else {
                        Event::Xor {
                            target: side,
                            control: data_role,
                        }
                    };
                    if !emit(event)? {
                        return Ok(false);
                    }
                }
            }
        } else {
            let y_gate = matches!(time, 1 | 4);
            let dummy = triple(0)?;
            let target_label = stage_label(
                inv,
                dummy,
                if y_gate {
                    StageLabel::YCenter
                } else {
                    StageLabel::XCenter
                },
            );
            for tr in 0..V {
                let t = triple(tr)?;
                let previous = stage_label(
                    inv,
                    t,
                    match time {
                        1 => StageLabel::IncomingY,
                        3 => StageLabel::XSide,
                        4 => StageLabel::YCenter,
                        _ => StageLabel::XCenter,
                    },
                );
                if !emit_edge(
                    emit,
                    if y_gate { inv.y_role(t) } else { inv.x_role(t) },
                    previous,
                    target_label,
                )? {
                    return Ok(false);
                }
            }
            for i in 0..H {
                let previous = match time {
                    1 => Label::Zero,
                    3 | 6 => stage_label(inv, dummy, StageLabel::YCenter),
                    _ => stage_label(inv, dummy, StageLabel::XCenter),
                };
                if !emit_edge(emit, center_role(inv, i), previous, target_label)? {
                    return Ok(false);
                }
            }
            let data_is_target = y_gate != inverse;
            for tr in 0..V {
                let t = triple(tr)?;
                let data = if y_gate { inv.y_role(t) } else { inv.x_role(t) };
                for i in t.entries() {
                    let center = center_role(inv, i);
                    let event = if data_is_target {
                        Event::Xor {
                            target: data,
                            control: center,
                        }
                    } else {
                        Event::Xor {
                            target: center,
                            control: data,
                        }
                    };
                    if !emit(event)? {
                        return Ok(false);
                    }
                }
            }
        }
    }
    let last = stage_label(inv, triple(0)?, StageLabel::XCenter);
    for y_rank in 0..V {
        let y = triple(y_rank)?;
        for ni in 0..BIT_NEIGHBORS {
            if !emit_edge(
                emit,
                side_role(inv, bit_neighbor(y, ni)?, y)?,
                last,
                Label::Full,
            )? {
                return Ok(false);
            }
        }
    }
    for i in 0..H {
        if !emit_edge(emit, center_role(inv, i), last, Label::Full)? {
            return Ok(false);
        }
    }
    Ok(true)
}
fn published_events(
    mut emit: impl FnMut(Event) -> Result<bool, SwapError>,
) -> Result<bool, SwapError> {
    for stage in 0..3 {
        for a in 0..V {
            for b in 0..V {
                let inv = PublishedInvocation::new(stage, [triple(a)?, triple(b)?])
                    .map_err(|_| SwapError::Construction("invocation"))?;
                if !invocation_events(inv, &mut emit)? {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}
fn prime(value: &Big) -> bool {
    if value < &Big::from_u64(2) {
        return false;
    }
    if value == &Big::from_u64(2) {
        return true;
    }
    if !value.bit(0) {
        return false;
    }
    let mut trial = Big::from_u64(3);
    while mul(&trial, &trial) <= *value {
        if div(value, &trial).1.is_zero() {
            return false;
        }
        trial = trial.add_small(2);
    }
    true
}
fn compile_prime() -> Result<Big, SwapError> {
    // The projector entries have denominators dividing 6^3, so start above
    // 2 and 3.  All factors and inverses are then checked explicitly; there
    // is no assumed lucky prime or probabilistic primality test.
    let mut q = Big::from_u64(5);
    loop {
        if prime(&q) {
            let mut ranks = 0u128;
            let accepted = published_events(|event| {
                if let Event::Edge { from, to, .. } = event {
                    let factors = edge_factors(from, to)?;
                    ranks = ranks
                        .checked_add(factors.pivots.len() as u128)
                        .ok_or(SwapError::SizeOverflow)?;
                    return Ok([
                        &factors.left,
                        &factors.right,
                        &factors.left_inverse,
                        &factors.right_inverse,
                    ]
                    .iter()
                    .all(|m| m.admissible(&q)));
                }
                Ok(true)
            })?;
            if accepted {
                if ranks != EDGE_RANKS {
                    return Err(SwapError::Construction(
                        "published rational edge-rank total",
                    ));
                }
                return Ok(q);
            }
        }
        q = q.add_small(2);
    }
}

struct Odometer {
    values: Vec<Big>,
    remaining: Vec<Big>,
    limits: Vec<Big>,
}
impl Odometer {
    fn new(limits: &[Big]) -> Self {
        Self {
            values: vec![Big::zero(); limits.len()],
            remaining: limits.to_vec(),
            limits: limits.to_vec(),
        }
    }
    fn advance(&mut self) {
        for i in (0..self.values.len()).rev() {
            decrement(&mut self.remaining[i]);
            if self.remaining[i].is_zero() {
                self.remaining[i] = self.limits[i].clone();
                self.values[i] = Big::zero();
            } else {
                increment(&mut self.values[i]);
                break;
            }
        }
    }
}
fn rotate(
    tape: &mut Tape,
    fields: &[Big],
    target: usize,
    offset: impl Fn(&[Big]) -> Big,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    let prefixes = product(&fields[..target]);
    let q = &fields[target];
    let suffix = product(&fields[target + 1..]);
    let mut left = ws.create_tape()?;
    let mut right = ws.create_tape()?;
    let mut cursor = Odometer::new(&fields[..target]);
    tape.rewind()?;
    repeat(prefixes.clone(), || {
        let amount = offset(&cursor.values);
        if &amount >= q {
            return Err(SwapError::Construction("rotation offset"));
        }
        tape.copy_bits_to(&mut left, &mul(&q.sub(&amount), &suffix))?;
        tape.copy_bits_to(&mut right, &mul(&amount, &suffix))?;
        cursor.advance();
        Ok(())
    })?;
    left.rewind()?;
    right.rewind()?;
    tape.clear()?;
    cursor = Odometer::new(&fields[..target]);
    repeat(prefixes, || {
        let amount = offset(&cursor.values);
        right.copy_bits_to(tape, &mul(&amount, &suffix))?;
        left.copy_bits_to(tape, &mul(&q.sub(&amount), &suffix))?;
        cursor.advance();
        Ok(())
    })?;
    ws.release_tape(left)?;
    ws.release_tape(right)?;
    tape.rewind()?;
    Ok(())
}
fn ordered_add(
    tape: &mut Tape,
    fields: &[Big],
    target: usize,
    control: usize,
    negative: bool,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    if control >= target || fields[control] != fields[target] {
        return Err(SwapError::InvalidShape);
    }
    let q = fields[target].clone();
    rotate(
        tape,
        fields,
        target,
        |address| {
            let x = address[control].clone();
            if negative && !x.is_zero() {
                q.sub(&x)
            } else {
                x
            }
        },
        ws,
    )
}
fn negate_field(
    tape: &mut Tape,
    fields: &[Big],
    target: usize,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    let p = product(&fields[..target]);
    let count = fields[target].sub_small(1);
    let b = product(&fields[target + 1..]);
    let mut output = ws.create_tape()?;
    let mut stack = ws.create_tape()?;
    let mut record = ws.create_tape()?;
    tape.rewind()?;
    repeat(p, || {
        tape.copy_bits_to(&mut output, &b)?;
        tape.copy_bits_to(&mut stack, &mul(&count, &b))?;
        repeat(count.clone(), || {
            repeat(b.clone(), || {
                record.write_bit(stack.pop_bit()?)?;
                Ok(())
            })?;
            repeat(b.clone(), || {
                output.write_bit(record.pop_bit()?)?;
                Ok(())
            })
        })
    })?;
    ws.release_tape(stack)?;
    ws.release_tape(record)?;
    replace(tape, output, ws)
}
fn integer_unit(
    tape: &mut Tape,
    fields: &[Big],
    target: usize,
    c: &Big,
    inverse: bool,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    if c == &one() {
        return Ok(());
    }
    if c.is_zero() {
        return Err(SwapError::Construction("zero unit"));
    }
    let count = usize::try_from(c.to_u128().ok_or(SwapError::SizeOverflow)?)
        .map_err(|_| SwapError::SizeOverflow)?;
    let q = &fields[target];
    let p = product(&fields[..target]);
    let b = product(&fields[target + 1..]);
    let mut streams = Vec::new();
    streams
        .try_reserve_exact(count)
        .map_err(|_| SwapError::Allocation)?;
    let mut lengths = Vec::new();
    lengths
        .try_reserve_exact(count)
        .map_err(|_| SwapError::Allocation)?;
    let mut previous = Big::zero();
    for j in 0..count {
        streams.push(ws.create_tape()?);
        let (mut end, remainder) = div(&mul(&Big::from_u128((j + 1) as u128), q), c);
        if !remainder.is_zero() {
            end = end.add_small(1);
        }
        lengths.push(mul(&end.sub(&previous), &b));
        previous = end;
    }
    let inverse_q = inverse_mod(q, c);
    let next_class = |current: &Big| {
        if current >= &inverse_q {
            current.sub(&inverse_q)
        } else {
            c.sub(&inverse_q.sub(current))
        }
    };
    tape.rewind()?;
    repeat(p.clone(), || {
        if inverse {
            let mut class = Big::zero();
            repeat(q.clone(), || {
                let j = class.to_u128().unwrap() as usize;
                tape.copy_bits_to(&mut streams[j], &b)?;
                class = next_class(&class);
                Ok(())
            })?;
        } else {
            for j in 0..count {
                tape.copy_bits_to(&mut streams[j], &lengths[j])?;
            }
        }
        Ok(())
    })?;
    for stream in &mut streams {
        stream.rewind()?;
    }
    tape.clear()?;
    repeat(p, || {
        if inverse {
            for j in 0..count {
                streams[j].copy_bits_to(tape, &lengths[j])?;
            }
        } else {
            let mut class = Big::zero();
            repeat(q.clone(), || {
                let j = class.to_u128().unwrap() as usize;
                streams[j].copy_bits_to(tape, &b)?;
                class = next_class(&class);
                Ok(())
            })?;
        }
        Ok(())
    })?;
    for stream in streams {
        ws.release_tape(stream)?;
    }
    tape.rewind()?;
    Ok(())
}
fn triangular(
    tape: &mut Tape,
    fields: &[Big],
    start: usize,
    a: &Matrix,
    ws: &mut TapeWorkspace,
) -> Result<(), SwapError> {
    let q = fields[start].clone();
    for i in (0..a.n).rev() {
        let diagonal = a.get(i, i);
        integer_unit(
            tape,
            fields,
            start + i,
            &diagonal.numerator.magnitude,
            false,
            ws,
        )?;
        integer_unit(tape, fields, start + i, &diagonal.denominator, true, ws)?;
        if diagonal.numerator.negative {
            negate_field(tape, fields, start + i, ws)?;
        }
        let coefficients = (0..i).map(|j| a.get(i, j).rem(&q)).collect::<Vec<_>>();
        if coefficients.iter().any(|x| !x.is_zero()) {
            rotate(
                tape,
                fields,
                start + i,
                |address| {
                    let mut sum = Big::zero();
                    for j in 0..i {
                        sum = sum.add(&mul(&coefficients[j], &address[start + j]));
                    }
                    div(&sum, &q).1
                },
                ws,
            )?;
        }
    }
    Ok(())
}

struct Context<'a> {
    workspace: &'a mut TapeWorkspace,
    parking: Tape,
    descriptors: Tape,
    prime: Big,
    work: SwapWork,
}
#[derive(Clone)]
struct RowShape {
    prefix: Big,
    rows: Big,
    before: Big,
    gap: Big,
    suffix: Big,
}
impl RowShape {
    fn fields(&self, range: &Big) -> Vec<Big> {
        vec![
            self.prefix.clone(),
            self.rows.clone(),
            self.before.clone(),
            range.clone(),
            self.gap.clone(),
            range.clone(),
            self.suffix.clone(),
        ]
    }
    fn row_bits(&self, range: &Big) -> Big {
        product(&[
            self.before.clone(),
            range.clone(),
            self.gap.clone(),
            range.clone(),
            self.suffix.clone(),
        ])
    }
}
fn component_fields(shape: &RowShape, range: &Big, count: usize) -> Vec<Big> {
    let mut fields = vec![
        shape.prefix.clone(),
        shape.rows.clone(),
        shape.before.clone(),
    ];
    fields.extend((0..count).map(|_| range.clone()));
    fields.push(shape.gap.clone());
    fields.extend((0..count).map(|_| range.clone()));
    fields.push(shape.suffix.clone());
    fields
}
fn descriptor_push(tape: &mut Tape, values: &[Big]) -> Result<usize, SwapError> {
    let mut length = 0usize;
    for value in values {
        let x = value.add_small(1);
        let bits = x.bits();
        for _ in 1..bits {
            tape.write_bit(false)?;
        }
        tape.write_unsigned(&x, bits)?;
        length = length
            .checked_add(2 * bits - 1)
            .ok_or(SwapError::SizeOverflow)?;
    }
    Ok(length)
}
fn descriptor_pop(tape: &mut Tape, bits: usize) -> Result<(), SwapError> {
    for _ in 0..bits {
        tape.pop_bit()?;
    }
    Ok(())
}
fn park_stream(stream: &mut Tape, parking: &mut Tape, bits: &Big) -> Result<(), SwapError> {
    stream.rewind()?;
    if stream.len() != bits {
        return Err(SwapError::Construction("parked role length"));
    }
    // copy_to replaces its destination.  A stack must instead append and
    // retain the cursor at its current top, including every ancestor.
    stream.copy_bits_to(parking, bits)?;
    stream.clear()?;
    stream.flush()?;
    Ok(())
}
fn restore_stream(stream: &mut Tape, parking: &mut Tape, bits: &Big) -> Result<(), SwapError> {
    stream.clear()?;
    stream.write_zero_bits(bits)?;
    repeat(bits.clone(), || {
        stream.write_previous_bit(parking.pop_bit()?)?;
        Ok(())
    })?;
    stream.flush()?;
    Ok(())
}
fn parked_child(
    tape: &mut Tape,
    shape: &RowShape,
    e: usize,
    active: u128,
    ctx: &mut Context<'_>,
) -> Result<(), SwapError> {
    let parked_start = ctx.parking.len().clone();
    let range = power(&ctx.prime, e);
    let role_bits = product(&shape.fields(&range));
    let frame = descriptor_push(
        &mut ctx.descriptors,
        &[
            shape.prefix.clone(),
            shape.rows.clone(),
            shape.before.clone(),
            shape.gap.clone(),
            shape.suffix.clone(),
            Big::from_u128(active),
            Big::from_u128(e as u128),
            role_bits.clone(),
        ],
    )?;
    for role in 0..WIRES {
        if role != active {
            let mut stream = ctx.workspace.bit_role(role)?;
            park_stream(&mut stream, &mut ctx.parking, &role_bits)?;
        }
    }
    power_swap(tape, shape, e, ctx)?;
    for role in (0..WIRES).rev() {
        if role != active {
            let mut stream = ctx.workspace.bit_role(role)?;
            restore_stream(&mut stream, &mut ctx.parking, &role_bits)?;
        }
    }
    if ctx.parking.len() != &parked_start {
        return Err(SwapError::Construction("parking stack boundary"));
    }
    descriptor_pop(&mut ctx.descriptors, frame)?;
    Ok(())
}
fn xor_role(target: u128, control: u128, ctx: &mut Context<'_>) -> Result<(), SwapError> {
    if target == control {
        return Err(SwapError::Construction("self-XOR gate"));
    }
    let mut a = ctx.workspace.bit_role(target)?;
    let mut b = ctx.workspace.bit_role(control)?;
    if a.len() != b.len() {
        return Err(SwapError::Construction("unaligned role gate"));
    }
    let len = a.len().clone();
    a.rewind()?;
    b.rewind()?;
    let mut output = ctx.workspace.create_tape()?;
    repeat(len, || {
        output.write_bit(a.require_bit()? ^ b.require_bit()?)?;
        Ok(())
    })?;
    replace(&mut a, output, ctx.workspace)?;
    a.flush()?;
    b.flush()?;
    Ok(())
}
fn edge(
    tape: &mut Tape,
    role: u128,
    from: Label,
    to: Label,
    shape: &RowShape,
    e: usize,
    ctx: &mut Context<'_>,
) -> Result<(), SwapError> {
    let factors = edge_factors(from, to)?;
    execute_factored_edge(tape, role, &factors, shape, e, ctx)
}
fn execute_factored_edge(
    tape: &mut Tape,
    role: u128,
    factors: &Factors,
    shape: &RowShape,
    e: usize,
    ctx: &mut Context<'_>,
) -> Result<(), SwapError> {
    if factors.pivots.is_empty() {
        return Ok(());
    }
    if ![
        &factors.left,
        &factors.right,
        &factors.left_inverse,
        &factors.right_inverse,
    ]
    .iter()
    .all(|m| m.admissible(&ctx.prime))
    {
        return Err(SwapError::Construction("inadmissible fixed prime"));
    }
    let dimension = factors.left.n;
    if dimension == 0 || !e.is_multiple_of(dimension) {
        return Err(SwapError::Construction("factored edge dimension"));
    }
    let small = e / dimension;
    let q = power(&ctx.prime, small);
    let fields = component_fields(shape, &q, dimension);
    let h_start = 3;
    let d_start = dimension + 4;
    {
        let mut role_tape = ctx.workspace.bit_role(role)?;
        role_tape.rewind()?;
        tape.clear()?;
        role_tape.copy_to(tape)?;
        role_tape.clear()?;
        role_tape.flush()?;
    }
    tape.rewind()?;
    // L A R = Pi, hence A=L^-1 Pi R^-1.
    triangular(
        tape,
        &fields,
        d_start,
        &factors.right_inverse,
        ctx.workspace,
    )?;
    triangular(tape, &fields, h_start, &factors.left, ctx.workspace)?;
    for &(i, j) in &factors.pivots {
        let hi = h_start + i;
        let dj = d_start + j;
        ordered_add(tape, &fields, dj, hi, false, ctx.workspace)?;
        let child = RowShape {
            prefix: shape.prefix.clone(),
            rows: shape.rows.clone(),
            before: product(&fields[2..hi]),
            gap: product(&fields[hi + 1..dj]),
            suffix: product(&fields[dj + 1..]),
        };
        increment(&mut ctx.work.rank_children);
        // A true digit leaf never opens any network role.  Its active
        // stream can be transformed directly without parking other roles.
        if small == 1 {
            power_swap(tape, &child, small, ctx)?;
        } else {
            parked_child(tape, &child, small, role, ctx)?;
        }
        negate_field(tape, &fields, dj, ctx.workspace)?;
        ordered_add(tape, &fields, dj, hi, false, ctx.workspace)?;
    }
    triangular(tape, &fields, h_start, &factors.left_inverse, ctx.workspace)?;
    triangular(tape, &fields, d_start, &factors.right, ctx.workspace)?;
    {
        let mut role_tape = ctx.workspace.bit_role(role)?;
        role_tape.clear()?;
        tape.rewind()?;
        tape.copy_to(&mut role_tape)?;
        role_tape.rewind()?;
        role_tape.flush()?;
    }
    tape.clear()?;
    Ok(())
}
fn power_swap(
    tape: &mut Tape,
    shape: &RowShape,
    e: usize,
    ctx: &mut Context<'_>,
) -> Result<(), SwapError> {
    if e == 1 {
        return radix_digit_swap(
            tape,
            &product(&[
                shape.prefix.clone(),
                shape.rows.clone(),
                shape.before.clone(),
            ]),
            &ctx.prime,
            &shape.gap,
            &shape.suffix,
            ctx.workspace,
            &mut ctx.work,
        );
    }
    if e == 0 || !e.is_multiple_of(M) {
        return Err(SwapError::Construction("non-power recursive width"));
    }
    increment(&mut ctx.work.recursive_nodes);
    let range = power(&ctx.prime, e);
    let fields = component_fields(shape, &power(&ctx.prime, e / M), M);
    for j in 0..M {
        ordered_add(tape, &fields, M + 4 + j, 3 + j, true, ctx.workspace)?;
    }
    let (child_rows, remainder) = div(&shape.rows, &Big::from_u128(WIRES));
    if !remainder.is_zero() {
        return Err(SwapError::Construction("row divisibility"));
    }
    let row_bits = shape.row_bits(&range);
    for role in 0..WIRES {
        let mut stream = ctx.workspace.bit_role(role)?;
        stream.clear()?;
        stream.flush()?;
    }
    tape.rewind()?;
    repeat(shape.prefix.clone(), || {
        repeat(child_rows.clone(), || {
            for role in 0..WIRES {
                let mut stream = ctx.workspace.bit_role(role)?;
                tape.copy_bits_to(&mut stream, &row_bits)?;
                stream.flush()?;
            }
            Ok(())
        })
    })?;
    tape.clear()?;
    let child_shape = RowShape {
        rows: child_rows.clone(),
        ..shape.clone()
    };
    published_events(|event| {
        match event {
            Event::Edge { role, from, to } => edge(tape, role, from, to, &child_shape, e, ctx)?,
            Event::Xor { target, control } => xor_role(target, control, ctx)?,
        };
        Ok(true)
    })?;
    for role in 0..WIRES {
        let mut stream = ctx.workspace.bit_role(role)?;
        stream.rewind()?;
        stream.flush()?;
    }
    repeat(shape.prefix.clone(), || {
        repeat(child_rows.clone(), || {
            for input_role in 0..WIRES {
                let output_role = if input_role < DATA {
                    input_role + DATA
                } else if input_role < 2 * DATA {
                    input_role - DATA
                } else {
                    input_role
                };
                let mut stream = ctx.workspace.bit_role(output_role)?;
                stream.copy_bits_to(tape, &row_bits)?;
                stream.flush()?;
            }
            Ok(())
        })
    })?;
    for role in 0..WIRES {
        let mut stream = ctx.workspace.bit_role(role)?;
        stream.clear()?;
        stream.flush()?;
    }
    tape.rewind()?;
    for j in 0..M {
        negate_field(tape, &fields, M + 4 + j, ctx.workspace)?;
        ordered_add(tape, &fields, M + 4 + j, 3 + j, false, ctx.workspace)?;
    }
    Ok(())
}
fn arbitrary_radix(
    tape: &mut Tape,
    p: &Big,
    e: usize,
    g: &Big,
    b: &Big,
    ctx: &mut Context<'_>,
) -> Result<(), SwapError> {
    if e == 0 {
        return Ok(());
    }
    let q = ctx.prime.clone();
    let mut k = 0usize;
    let mut bound = one();
    let e_big = Big::from_u128(e as u128);
    while bound < e_big {
        bound = bound.mul_small(M as u64);
        k += 1;
    }
    let divisor = power(&Big::from_u128(WIRES), k);
    let mut rho = 0usize;
    let mut rows = one();
    let q2 = mul(&q, &q);
    while rows < divisor {
        rows = mul(&rows, &q2);
        rho = rho.checked_add(1).ok_or(SwapError::SizeOverflow)?;
    }
    if rho == 0 || rho >= e {
        for position in 0..e {
            radix_digit_swap(
                tape,
                &mul(p, &power(&q, position)),
                &q,
                &mul(g, &power(&q, e - 1)),
                &mul(b, &power(&q, e - position - 1)),
                ctx.workspace,
                &mut ctx.work,
            )?;
        }
        return Ok(());
    }
    let remaining = e - rho;
    let high = power(&q, rho);
    let low = power(&q, remaining);
    for position in 0..rho {
        radix_digit_swap(
            tape,
            &mul(p, &power(&q, position)),
            &q,
            &mul(g, &power(&q, e - 1)),
            &mul(b, &power(&q, e - position - 1)),
            ctx.workspace,
            &mut ctx.work,
        )?;
    }
    for done in 0..rho {
        move_radix_digit(
            tape,
            &product(&[p.clone(), high.clone(), power(&q, done)]),
            &q,
            &mul(&low, g),
            &product(&[power(&q, rho - done - 1), low.clone(), b.clone()]),
            false,
            ctx.workspace,
        )?;
    }
    let (mut row_units, rem) = div(&rows, &divisor);
    if !rem.is_zero() {
        row_units = row_units.add_small(1);
    }
    let padded_rows = mul(&row_units, &divisor);
    let row_suffix = product(&[low.clone(), g.clone(), low.clone(), b.clone()]);
    pad_field(
        tape,
        p,
        &rows,
        &padded_rows,
        &row_suffix,
        true,
        ctx.workspace,
    )?;
    let mut pieces = Vec::new();
    let mut left = remaining;
    let mut width = 1usize;
    while left != 0 {
        let digit = left % M;
        for _ in 0..digit {
            pieces.push(width);
        }
        left /= M;
        if left != 0 {
            width = width.checked_mul(M).ok_or(SwapError::SizeOverflow)?;
        }
    }
    let mut offset = 0usize;
    for width in pieces.into_iter().rev() {
        let shape = RowShape {
            prefix: p.clone(),
            rows: padded_rows.clone(),
            before: power(&q, offset),
            gap: mul(g, &power(&q, remaining - width)),
            suffix: mul(b, &power(&q, remaining - offset - width)),
        };
        power_swap(tape, &shape, width, ctx)?;
        offset += width;
    }
    if offset != remaining {
        return Err(SwapError::Construction("piece partition"));
    }
    pad_field(
        tape,
        p,
        &rows,
        &padded_rows,
        &row_suffix,
        false,
        ctx.workspace,
    )?;
    for done in 0..rho {
        move_radix_digit(
            tape,
            &product(&[p.clone(), high.clone(), power(&q, rho - done - 1)]),
            &q,
            &mul(&low, g),
            &product(&[power(&q, done), low.clone(), b.clone()]),
            true,
            ctx.workspace,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composed_rank_shear_executes_exactly_one_child_per_pivot() {
        for nonzero in [false, true] {
            let mut a = Matrix::zeros(2).unwrap();
            if nonzero {
                a.set(0, 0, Rational::new(Signed::from_i64(-1), Big::from_u64(2)));
                a.set(0, 1, Rational::new(Signed::from_i64(1), Big::from_u64(3)));
                a.set(1, 0, Rational::integer(-1));
                a.set(1, 1, Rational::new(Signed::from_i64(2), Big::from_u64(3)));
            }
            let factors = factor(a).unwrap();
            let rank = usize::from(nonzero);
            assert_eq!(factors.pivots.len(), rank);
            let mut expected = vec![0u64; 625];
            for h0 in 0..5 {
                for h1 in 0..5 {
                    for d0 in 0..5 {
                        for d1 in 0..5 {
                            let source = ((h0 * 5 + h1) * 5 + d0) * 5 + d1;
                            let x = if nonzero {
                                (h0 + 2 * d0 + 2 * d1) % 5
                            } else {
                                h0
                            };
                            let y = if nonzero {
                                (h1 + 4 * d0 + 4 * d1) % 5
                            } else {
                                h1
                            };
                            expected[((x * 5 + y) * 5 + d0) * 5 + d1] = source as u64;
                        }
                    }
                }
            }
            let mut ws = TapeWorkspace::create_in(std::env::temp_dir()).unwrap();
            let mut io = ws.create_tape().unwrap();
            {
                let mut input = ws.bit_role(0).unwrap();
                for value in 0..625 {
                    input.write_unsigned(&Big::from_u64(value), 16).unwrap();
                }
                input.flush().unwrap();
            }
            let parking = ws.create_tape().unwrap();
            let descriptors = ws.create_tape().unwrap();
            let shape = RowShape {
                prefix: one(),
                rows: one(),
                before: one(),
                gap: one(),
                suffix: Big::from_u64(16),
            };
            {
                let mut ctx = Context {
                    workspace: &mut ws,
                    parking,
                    descriptors,
                    prime: Big::from_u64(5),
                    work: SwapWork::default(),
                };
                execute_factored_edge(&mut io, 0, &factors, &shape, 2, &mut ctx).unwrap();
                assert_eq!(ctx.work.rank_children, Big::from_u64(rank as u64));
                assert!(ctx.parking.is_empty());
                assert!(ctx.descriptors.is_empty());
                let mut output = ctx.workspace.bit_role(0).unwrap();
                output.rewind().unwrap();
                let actual = (0..625)
                    .map(|_| output.read_unsigned(16).unwrap().to_u128().unwrap() as u64)
                    .collect::<Vec<_>>();
                assert_eq!(actual, expected);
                drop(output);
                ctx.workspace.release_tape(ctx.parking).unwrap();
                ctx.workspace.release_tape(ctx.descriptors).unwrap();
            }
            drop(io);
            ws.cleanup().unwrap();
        }
    }

    #[test]
    fn parking_appends_multiple_roles_without_crossing_an_ancestor() {
        let mut ws = TapeWorkspace::create_in(std::env::temp_dir()).unwrap();
        let mut stack = ws.create_tape().unwrap();
        let mut a = ws.create_tape().unwrap();
        let mut b = ws.create_tape().unwrap();
        stack.write_unsigned(&Big::from_u64(0xa6), 8).unwrap();
        a.write_unsigned(&Big::from_u64(0x9317), 16).unwrap();
        b.write_unsigned(&Big::from_u64(0xe45a0d), 24).unwrap();
        park_stream(&mut a, &mut stack, &Big::from_u64(16)).unwrap();
        park_stream(&mut b, &mut stack, &Big::from_u64(24)).unwrap();
        assert_eq!(stack.len(), &Big::from_u64(48));
        assert_eq!(stack.position(), stack.len());
        restore_stream(&mut b, &mut stack, &Big::from_u64(24)).unwrap();
        restore_stream(&mut a, &mut stack, &Big::from_u64(16)).unwrap();
        assert_eq!(stack.len(), &Big::from_u64(8));
        assert_eq!(stack.position(), stack.len());
        a.rewind().unwrap();
        b.rewind().unwrap();
        stack.rewind().unwrap();
        assert_eq!(a.read_unsigned(16).unwrap(), Big::from_u64(0x9317));
        assert_eq!(b.read_unsigned(24).unwrap(), Big::from_u64(0xe45a0d));
        assert_eq!(stack.read_unsigned(8).unwrap(), Big::from_u64(0xa6));
        drop(a);
        drop(b);
        drop(stack);
        ws.cleanup().unwrap();
    }

    fn records(
        input: &[u64],
        width: usize,
        operation: impl FnOnce(&mut Tape, &mut TapeWorkspace) -> Result<(), SwapError>,
    ) -> Vec<u64> {
        let mut ws = TapeWorkspace::create_in(std::env::temp_dir()).unwrap();
        let mut tape = ws.create_tape().unwrap();
        for &value in input {
            tape.write_unsigned(&Big::from_u64(value), width).unwrap();
        }
        operation(&mut tape, &mut ws).unwrap();
        tape.rewind().unwrap();
        let output = (0..input.len())
            .map(|_| tape.read_unsigned(width).unwrap().to_u128().unwrap() as u64)
            .collect();
        assert!(tape.read_bit().unwrap().is_none());
        drop(tape);
        ws.cleanup().unwrap();
        output
    }

    #[test]
    fn sequential_binary_wrapper_preserves_wide_signed_payloads() {
        let mut input = (0..256)
            .map(|i| Complex {
                re: Signed::new(i % 3 == 0, Big::from_u64(i as u64 + 1).shl(300)),
                im: Signed::from_i64(i as i64 - 128),
            })
            .collect::<Vec<_>>();
        let original = input.clone();
        let mut expected = input.clone();
        for p in 0..2 {
            for h in 0..4 {
                for g in 0..2 {
                    for d in 0..4 {
                        for s in 0..2 {
                            for c in 0..2 {
                                let source = (((((p * 4 + h) * 2 + g) * 4 + d) * 2 + s) * 2) + c;
                                let destination =
                                    (((((p * 4 + d) * 2 + g) * 4 + h) * 2 + s) * 2) + c;
                                expected[destination] = original[source].clone();
                            }
                        }
                    }
                }
            }
        }
        let receipt = swap_record_chunks(&mut input, 1, 2, 1, 1, 2, 128).unwrap();
        assert_eq!(input, expected);
        assert!(receipt.component_bits > 300);
        assert_eq!(receipt.work.individual_bit_swaps, Big::from_u64(2));
        assert!(receipt.work.recursive_nodes.is_zero());
        assert!(!receipt.tape.read_bits.is_zero());
        assert!(!receipt.tape.head_moves.is_zero());
        swap_record_chunks(&mut input, 1, 2, 1, 1, 2, 128).unwrap();
        assert_eq!(input, original);
    }

    #[test]
    fn odd_radix_padding_and_digit_moves_match_numeric_addresses() {
        let input = (0..36u64).collect::<Vec<_>>();
        let mut expected = vec![0; 36];
        for p in 0..2 {
            for h in 0..3 {
                for g in 0..2 {
                    for d in 0..3 {
                        expected[((p * 3 + d) * 2 + g) * 3 + h] =
                            input[((p * 3 + h) * 2 + g) * 3 + d];
                    }
                }
            }
        }
        let output = records(&input, 8, |t, ws| {
            radix_digit_swap(
                t,
                &Big::from_u64(2),
                &Big::from_u64(3),
                &Big::from_u64(2),
                &Big::from_u64(8),
                ws,
                &mut SwapWork::default(),
            )
        });
        assert_eq!(output, expected);
        let input = (0..30u64).collect::<Vec<_>>();
        let mut expected = vec![0; 30];
        for p in 0..2 {
            for digit in 0..3 {
                for middle in 0..5 {
                    expected[(p * 5 + middle) * 3 + digit] = input[(p * 3 + digit) * 5 + middle];
                }
            }
        }
        let moved = records(&input, 8, |t, ws| {
            move_radix_digit(
                t,
                &Big::from_u64(2),
                &Big::from_u64(3),
                &Big::from_u64(5),
                &Big::from_u64(8),
                true,
                ws,
            )
        });
        assert_eq!(moved, expected);
        let restored = records(&moved, 8, |t, ws| {
            move_radix_digit(
                t,
                &Big::from_u64(2),
                &Big::from_u64(3),
                &Big::from_u64(5),
                &Big::from_u64(8),
                false,
                ws,
            )
        });
        assert_eq!(restored, input);
    }

    #[test]
    fn fixed_integer_units_inverse_and_negative_target_are_exact() {
        let input = (0..10u64).collect::<Vec<_>>();
        let fields = [Big::from_u64(2), Big::from_u64(5), Big::from_u64(8)];
        for c in [2u64, 7] {
            let mut expected = vec![0; 10];
            for p in 0..2 {
                for x in 0..5 {
                    expected[p * 5 + (x * c as usize) % 5] = input[p * 5 + x];
                }
            }
            let output = records(&input, 8, |t, ws| {
                integer_unit(t, &fields, 1, &Big::from_u64(c), false, ws)
            });
            assert_eq!(output, expected);
            let restored = records(&output, 8, |t, ws| {
                integer_unit(t, &fields, 1, &Big::from_u64(c), true, ws)
            });
            assert_eq!(restored, input);
        }
        let mut expected = vec![0; 10];
        for p in 0..2 {
            for x in 0..5 {
                expected[p * 5 + (5 - x) % 5] = input[p * 5 + x];
            }
        }
        assert_eq!(
            records(&input, 8, |t, ws| negate_field(t, &fields, 1, ws)),
            expected
        );
    }

    #[test]
    fn rational_triangular_shear_preserves_earlier_controls() {
        let mut a = Matrix::zeros(2).unwrap();
        a.set(0, 0, Rational::integer(2));
        a.set(1, 0, Rational::integer(-3));
        a.set(1, 1, Rational::new(Signed::from_i64(1), Big::from_u64(2)));
        let input = (0..25u64).collect::<Vec<_>>();
        let fields = [one(), Big::from_u64(5), Big::from_u64(5), Big::from_u64(8)];
        let mut expected = vec![0; 25];
        for x in 0..5 {
            for y in 0..5 {
                let xx = 2 * x % 5;
                let yy = (3 * y + 10 - 3 * x % 5) % 5;
                expected[xx * 5 + yy] = input[x * 5 + y];
            }
        }
        let output = records(&input, 8, |t, ws| triangular(t, &fields, 1, &a, ws));
        assert_eq!(output, expected);
        let inverse = a.lower_inverse().unwrap();
        let restored = records(&output, 8, |t, ws| triangular(t, &fields, 1, &inverse, ws));
        assert_eq!(restored, input);
    }

    fn matrix_product(a: &Matrix, b: &Matrix) -> Matrix {
        assert_eq!(a.n, b.n);
        let mut out = Matrix::zeros(a.n).unwrap();
        for i in 0..a.n {
            for j in 0..a.n {
                let mut sum = Rational::integer(0);
                for k in 0..a.n {
                    sum = sum.add(&a.get(i, k).mul(b.get(k, j)));
                }
                out.set(i, j, sum);
            }
        }
        out
    }
    #[test]
    fn exact_lower_lower_compiler_reconstructs_rank_deficient_matrices() {
        let examples = [
            ([[0, 0, 0], [2, 4, 0], [1, 2, 0]], 1usize),
            ([[1, 2, -1], [0, 1, 3], [2, 5, 1]], 2),
            ([[0, 1, 0], [1, 0, 2], [2, 0, 1]], 3),
        ];
        for (entries, rank) in examples {
            let mut a = Matrix::zeros(3).unwrap();
            for i in 0..3 {
                for j in 0..3 {
                    a.set(
                        i,
                        j,
                        Rational::new(Signed::from_i64(entries[i][j]), Big::from_u64(3)),
                    );
                }
            }
            let f = factor(a.clone()).unwrap();
            assert_eq!(f.pivots.len(), rank);
            let mut pi = Matrix::zeros(3).unwrap();
            for &(i, j) in &f.pivots {
                pi.set(i, j, Rational::integer(1));
            }
            assert_eq!(
                matrix_product(&matrix_product(&f.left, &a), &f.right).entries,
                pi.entries
            );
            assert_eq!(
                matrix_product(&matrix_product(&f.left_inverse, &pi), &f.right_inverse).entries,
                a.entries
            );
            for m in [&f.left, &f.right, &f.left_inverse, &f.right_inverse] {
                for i in 0..3 {
                    for j in i + 1..3 {
                        assert!(m.get(i, j).zero());
                    }
                }
            }
        }
    }

    #[test]
    fn published_bit_neighbors_roles_and_rational_projector_are_constructive() {
        for y in [
            Triple::new([0, 1, 2]).unwrap(),
            Triple::new([4, 50, 99]).unwrap(),
            Triple::new([97, 98, 99]).unwrap(),
        ] {
            for index in [0, 4655, 4656, 9311, 13967] {
                let x = bit_neighbor(y, index).unwrap();
                assert_eq!(x.intersection(y), 1);
                assert_eq!(bit_neighbor_rank(y, x).unwrap(), index);
            }
        }
        let last = triple(V - 1).unwrap();
        let inv = PublishedInvocation::new(2, [last, last]).unwrap();
        assert_eq!(center_role(inv, H - 1) + 1, WIRES);
        assert!(center_role(inv, H - 1) > u64::MAX as u128);
        let ts = [
            Triple::new([0, 3, 9]).unwrap(),
            Triple::new([2, 7, 45]).unwrap(),
            Triple::new([6, 51, 99]).unwrap(),
        ];
        let label = Label::Line(ts);
        let mut support = Vec::new();
        for a in ts[0].entries() {
            for b in ts[1].entries() {
                for c in ts[2].entries() {
                    support.push((a * H + b) * H + c);
                }
            }
        }
        for i in [0, 6, 351, 30_799, 99_999, 999_999] {
            for j in [0, 9, 99, 20_706, 90_699, 999_999] {
                let mut sum = Rational::integer(0);
                for &k in &support {
                    sum = sum.add(&label.entry(i, k).mul(&label.entry(k, j)));
                }
                assert_eq!(sum, label.entry(i, j));
            }
        }
        let mut trace = Rational::integer(0);
        for &i in &support {
            trace = trace.add(&label.entry(i, i));
        }
        assert_eq!(trace, Rational::integer(1));
        assert!(prime(&Big::from_u64(5)));
        assert!(!prime(&Big::from_u64(341)));
    }
}
