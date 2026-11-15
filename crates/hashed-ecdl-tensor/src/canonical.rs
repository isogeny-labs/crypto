//! Strict, versioned canonical snapshots of Boolean tensor networks.
//!
//! Construction-order identifiers are preserved deliberately: vertex, edge,
//! port and interface order are part of the encoded experiment instance. This
//! module does not claim graph-isomorphism canonical labelling.

use crate::network::{Bit, Network, Predicate, PredicateTag, Vertex, Word};
use crate::{integrity, invalid, overflow, BigUint, Error, Meter, Result};
use num_traits::{One, ToPrimitive, Zero};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::sync::Arc;

const MAGIC: &[u8; 8] = b"BTNIR001";

/// Current binary canonical-IR version.
pub const CANONICAL_IR_VERSION: u32 = 1;
/// Largest predicate arity admitted to exact truth-table canonicalization.
pub const MAX_CANONICAL_PREDICATE_ARITY: usize = 16;

/// Policy for predicates without a semantic [`PredicateTag`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportPolicy {
    /// Reject every untagged predicate. This is the production-encoding gate.
    RequireTagged,
    /// Admit an untagged predicate under a `truth-table` tag after exact bounded
    /// enumeration. This is intended for tiny custom fixtures and controls.
    AllowUntaggedTruthTables,
}

/// Provenance bound into canonical bytes and their SHA-256 identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IrProvenance {
    /// Logical source instance or fixture identifier.
    pub source: String,
    /// Generator or builder responsible for the network.
    pub generator: String,
    /// Source revision, protocol revision, or other immutable version label.
    pub revision: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ProgramBit {
    Constant(bool),
    Port(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ValueOperation {
    Constant(Vec<BigUint>),
    Not,
    And,
    Or,
    LessThanBit,
    IntegerAddBit,
    FourierColumn {
        omega: BigUint,
        modulus: BigUint,
        first: usize,
        second: usize,
        column: usize,
    },
    FourierRow {
        omega: BigUint,
        modulus: BigUint,
        first: usize,
        second: usize,
        scale: BigUint,
    },
    FourierPointwise {
        modulus: BigUint,
    },
    ProductCarry {
        limb_bits: usize,
    },
}

impl ValueOperation {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Constant(_) => "constant",
            Self::Not => "not",
            Self::And => "and",
            Self::Or => "or",
            Self::LessThanBit => "less-than-bit",
            Self::IntegerAddBit => "integer-add-bit",
            Self::FourierColumn { .. } => "fourier-column",
            Self::FourierRow { .. } => "fourier-row",
            Self::FourierPointwise { .. } => "fourier-pointwise",
            Self::ProductCarry { .. } => "product-carry",
        }
    }

    fn shape(&self) -> (usize, usize) {
        match self {
            Self::Constant(outputs) => (0, outputs.len()),
            Self::Not => (1, 1),
            Self::And | Self::Or => (2, 1),
            Self::LessThanBit => (3, 1),
            Self::IntegerAddBit => (3, 2),
            Self::FourierColumn { first, .. } => (*first, *first),
            Self::FourierRow { second, .. } => (*second, *second),
            Self::FourierPointwise { .. } => (2, 1),
            Self::ProductCarry { .. } => (2, 2),
        }
    }

    fn validate(&self) -> Result<()> {
        match self {
            Self::FourierColumn {
                modulus,
                first,
                second,
                column,
                ..
            } => {
                if modulus <= &BigUint::one() || *first == 0 || *second == 0 || column >= second {
                    return Err(invalid("canonical Fourier-column operation"));
                }
            }
            Self::FourierRow {
                modulus,
                first,
                second,
                scale,
                ..
            } => {
                if modulus <= &BigUint::one() || *first == 0 || *second == 0 || scale >= modulus {
                    return Err(invalid("canonical Fourier-row operation"));
                }
            }
            Self::FourierPointwise { modulus } if modulus <= &BigUint::one() => {
                return Err(invalid("canonical Fourier-pointwise modulus"));
            }
            Self::ProductCarry { limb_bits } if *limb_bits == 0 => {
                return Err(invalid("canonical product-carry limb width"));
            }
            _ => {}
        }
        Ok(())
    }

    fn evaluate(&self, values: &[BigUint], meter: &mut Meter) -> Result<Vec<BigUint>> {
        self.validate()?;
        let (inputs, _) = self.shape();
        if values.len() != inputs {
            return Err(invalid("canonical value-operation input count"));
        }
        match self {
            Self::Constant(outputs) => Ok(outputs.clone()),
            Self::Not => Ok(vec![BigUint::from(u8::from(values[0].is_zero()))]),
            Self::And => Ok(vec![&values[0] * &values[1]]),
            Self::Or => Ok(vec![BigUint::from(u8::from(
                !values[0].is_zero() || !values[1].is_zero(),
            ))]),
            Self::LessThanBit => Ok(vec![if values[0] == values[1] {
                values[2].clone()
            } else {
                BigUint::from(u8::from(values[0] < values[1]))
            }]),
            Self::IntegerAddBit => {
                let sum = values[0]
                    .to_u8()
                    .and_then(|a| values[1].to_u8().map(|b| (a, b)))
                    .and_then(|(a, b)| values[2].to_u8().map(|c| (a, b, c)))
                    .ok_or_else(|| integrity("canonical add-bit input"))?;
                let sum = sum.0 + sum.1 + sum.2;
                Ok(vec![BigUint::from(sum & 1), BigUint::from(sum >> 1)])
            }
            Self::FourierColumn {
                omega,
                modulus,
                first,
                second,
                column,
            } => {
                let mut out = Vec::with_capacity(*first);
                for t1 in 0..*first {
                    let mut sum = BigUint::zero();
                    for j1 in 0..*first {
                        meter.charge(1, "canonical Fourier column transform")?;
                        let exponent =
                            BigUint::from(*second) * BigUint::from(j1) * BigUint::from(t1);
                        sum = (sum + &values[j1] * omega.modpow(&exponent, modulus)) % modulus;
                    }
                    let twist = BigUint::from(*column) * BigUint::from(t1);
                    out.push(sum * omega.modpow(&twist, modulus) % modulus);
                }
                Ok(out)
            }
            Self::FourierRow {
                omega,
                modulus,
                first,
                second,
                scale,
            } => {
                let mut out = Vec::with_capacity(*second);
                for t2 in 0..*second {
                    let mut sum = BigUint::zero();
                    for j2 in 0..*second {
                        meter.charge(1, "canonical Fourier row transform")?;
                        let exponent =
                            BigUint::from(*first) * BigUint::from(j2) * BigUint::from(t2);
                        sum = (sum + &values[j2] * omega.modpow(&exponent, modulus)) % modulus;
                    }
                    out.push(sum * scale % modulus);
                }
                Ok(out)
            }
            Self::FourierPointwise { modulus } => {
                meter.charge(1, "canonical Fourier pointwise product")?;
                Ok(vec![&values[0] * &values[1] % modulus])
            }
            Self::ProductCarry { limb_bits } => {
                meter.charge(1, "canonical integer product carries")?;
                let sum = &values[0] + &values[1];
                Ok(vec![
                    &sum & ((BigUint::one() << limb_bits) - BigUint::one()),
                    sum >> limb_bits,
                ])
            }
        }
    }
}

/// Serializable live evaluator for a high-arity production predicate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredicateProgram {
    words: Vec<Vec<ProgramBit>>,
    input_words: usize,
    operation: ValueOperation,
}
impl PredicateProgram {
    pub(crate) fn define(
        words: &[Word],
        input_words: usize,
        operation: ValueOperation,
    ) -> Result<Self> {
        if input_words > words.len() {
            return Err(invalid("canonical predicate-program input words"));
        }
        let mut port = 0usize;
        let words = words
            .iter()
            .map(|word| {
                word.0
                    .iter()
                    .map(|bit| match bit {
                        Bit::Constant(value) => ProgramBit::Constant(*value),
                        Bit::Signal(_) => {
                            let current = port;
                            port += 1;
                            ProgramBit::Port(current)
                        }
                    })
                    .collect()
            })
            .collect();
        let program = Self {
            words,
            input_words,
            operation,
        };
        program.validate(port)?;
        Ok(program)
    }

    fn validate(&self, arity: usize) -> Result<()> {
        self.operation.validate()?;
        let (inputs, outputs) = self.operation.shape();
        if self.input_words != inputs || self.words.len() != inputs + outputs {
            return Err(invalid("canonical predicate-program word shape"));
        }
        let mut ports = vec![false; arity];
        for bit in self.words.iter().flatten() {
            if let ProgramBit::Port(port) = bit {
                if *port >= arity || ports[*port] {
                    return Err(invalid("canonical predicate-program port map"));
                }
                ports[*port] = true;
            }
        }
        if ports.iter().any(|used| !used) {
            return Err(invalid("incomplete canonical predicate-program ports"));
        }
        Ok(())
    }

    /// Evaluate the typed relation on an exact ordered port assignment.
    pub fn evaluate(&self, ports: &[bool], meter: &mut Meter) -> Result<bool> {
        self.validate(ports.len())?;
        let mut values = Vec::with_capacity(self.words.len());
        for word in &self.words {
            let mut value = BigUint::zero();
            for (position, bit) in word.iter().enumerate() {
                let set = match bit {
                    ProgramBit::Constant(value) => *value,
                    ProgramBit::Port(port) => ports[*port],
                };
                if set {
                    let position = u64::try_from(position)
                        .map_err(|_| overflow("canonical program bit position"))?;
                    value.set_bit(position, true);
                }
            }
            values.push(value);
        }
        let expected = self
            .operation
            .evaluate(&values[..self.input_words], meter)?;
        Ok(expected == values[self.input_words..])
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        write_len(&mut bytes, self.input_words)?;
        write_len(&mut bytes, self.words.len())?;
        for word in &self.words {
            write_len(&mut bytes, word.len())?;
            for bit in word {
                match bit {
                    ProgramBit::Constant(false) => bytes.push(0),
                    ProgramBit::Constant(true) => bytes.push(1),
                    ProgramBit::Port(port) => {
                        bytes.push(2);
                        write_len(&mut bytes, *port)?;
                    }
                }
            }
        }
        self.operation.write(&mut bytes)?;
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8], meter: &Meter) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let input_words = reader.index()?;
        let word_count = reader.length(meter, "canonical program words")?;
        let mut words = Vec::with_capacity(word_count);
        for _ in 0..word_count {
            let bit_count = reader.length(meter, "canonical program word bits")?;
            let mut word = Vec::with_capacity(bit_count);
            for _ in 0..bit_count {
                word.push(match reader.byte()? {
                    0 => ProgramBit::Constant(false),
                    1 => ProgramBit::Constant(true),
                    2 => ProgramBit::Port(reader.index()?),
                    _ => return Err(invalid("canonical program bit code")),
                });
            }
            words.push(word);
        }
        let operation = ValueOperation::read(&mut reader, meter)?;
        if !reader.is_empty() {
            return Err(invalid("trailing canonical predicate-program bytes"));
        }
        Ok(Self {
            words,
            input_words,
            operation,
        })
    }
}

impl ValueOperation {
    fn write(&self, bytes: &mut Vec<u8>) -> Result<()> {
        match self {
            Self::Constant(outputs) => {
                bytes.push(0);
                write_len(bytes, outputs.len())?;
                for output in outputs {
                    write_biguint(bytes, output)?;
                }
            }
            Self::Not => bytes.push(1),
            Self::And => bytes.push(2),
            Self::Or => bytes.push(3),
            Self::LessThanBit => bytes.push(4),
            Self::IntegerAddBit => bytes.push(5),
            Self::FourierColumn {
                omega,
                modulus,
                first,
                second,
                column,
            } => {
                bytes.push(6);
                write_biguint(bytes, omega)?;
                write_biguint(bytes, modulus)?;
                write_len(bytes, *first)?;
                write_len(bytes, *second)?;
                write_len(bytes, *column)?;
            }
            Self::FourierRow {
                omega,
                modulus,
                first,
                second,
                scale,
            } => {
                bytes.push(7);
                write_biguint(bytes, omega)?;
                write_biguint(bytes, modulus)?;
                write_len(bytes, *first)?;
                write_len(bytes, *second)?;
                write_biguint(bytes, scale)?;
            }
            Self::FourierPointwise { modulus } => {
                bytes.push(8);
                write_biguint(bytes, modulus)?;
            }
            Self::ProductCarry { limb_bits } => {
                bytes.push(9);
                write_len(bytes, *limb_bits)?;
            }
        }
        Ok(())
    }

    fn read(reader: &mut Reader<'_>, meter: &Meter) -> Result<Self> {
        let operation = match reader.byte()? {
            0 => {
                let count = reader.length(meter, "canonical constant outputs")?;
                let mut outputs = Vec::with_capacity(count);
                for _ in 0..count {
                    outputs.push(reader.biguint(meter)?);
                }
                Self::Constant(outputs)
            }
            1 => Self::Not,
            2 => Self::And,
            3 => Self::Or,
            4 => Self::LessThanBit,
            5 => Self::IntegerAddBit,
            6 => Self::FourierColumn {
                omega: reader.biguint(meter)?,
                modulus: reader.biguint(meter)?,
                first: reader.index()?,
                second: reader.index()?,
                column: reader.index()?,
            },
            7 => Self::FourierRow {
                omega: reader.biguint(meter)?,
                modulus: reader.biguint(meter)?,
                first: reader.index()?,
                second: reader.index()?,
                scale: reader.biguint(meter)?,
            },
            8 => Self::FourierPointwise {
                modulus: reader.biguint(meter)?,
            },
            9 => Self::ProductCarry {
                limb_bits: reader.index()?,
            },
            _ => return Err(invalid("canonical value-operation code")),
        };
        operation.validate()?;
        Ok(operation)
    }
}

/// One exact predicate in canonical form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalPredicate {
    /// Semantic family and sorted parameters.
    pub tag: PredicateTag,
    /// Ordered port count.
    pub arity: usize,
    /// Packed truth table; assignment bit `i` supplies port `i`.
    pub accepted: Vec<u8>,
    /// Typed live evaluator when the predicate exceeds the truth-table cap.
    pub program: Option<PredicateProgram>,
    /// SHA-256 binding the tag, parameters, arity, and semantic body.
    pub semantic_sha256: [u8; 32],
}

/// One vertex with its predicate and edge identifiers in exact port order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalVertex {
    /// Exact predicate.
    pub predicate: CanonicalPredicate,
    /// Edge identifier at each ordered predicate port.
    pub edges: Vec<usize>,
}

/// One undirected non-loop edge in construction order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalEdge {
    /// First endpoint vertex.
    pub left: usize,
    /// Second endpoint vertex.
    pub right: usize,
}

/// Complete canonical experiment instance, including interface order and pins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalNetwork {
    /// Binary format version.
    pub version: u32,
    /// Instance provenance included in the identity hash.
    pub provenance: IrProvenance,
    /// Vertices in construction order.
    pub vertices: Vec<CanonicalVertex>,
    /// Edges in construction order.
    pub edges: Vec<CanonicalEdge>,
    /// Interface edge identifiers in language-word order.
    pub interface: Vec<usize>,
    /// Exact edge pins: `None` is free, `Some(false/true)` is fixed.
    pub pins: Vec<Option<bool>>,
}

fn checked_add(total: &mut usize, value: usize, phase: &str) -> Result<()> {
    *total = total.checked_add(value).ok_or_else(|| overflow(phase))?;
    Ok(())
}

fn table_shape(arity: usize) -> Result<(usize, usize)> {
    if arity > MAX_CANONICAL_PREDICATE_ARITY {
        return Err(invalid("canonical predicate arity exceeds exact cap"));
    }
    let assignments = 1usize
        .checked_shl(arity as u32)
        .ok_or_else(|| overflow("canonical truth-table assignments"))?;
    Ok((assignments, assignments.div_ceil(8)))
}

fn hash_len(hasher: &mut Sha256, value: usize) -> Result<()> {
    let value = u64::try_from(value).map_err(|_| overflow("canonical hash length"))?;
    hasher.update(value.to_le_bytes());
    Ok(())
}

fn hash_bytes(hasher: &mut Sha256, bytes: &[u8]) -> Result<()> {
    hash_len(hasher, bytes.len())?;
    hasher.update(bytes);
    Ok(())
}

fn predicate_digest(
    tag: &PredicateTag,
    arity: usize,
    accepted: &[u8],
    program: Option<&PredicateProgram>,
) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    hasher.update(b"hashed-ecdl-tensor canonical predicate v1\0");
    hash_bytes(&mut hasher, tag.kind.as_bytes())?;
    hash_len(&mut hasher, tag.parameters.len())?;
    for (key, value) in &tag.parameters {
        hash_bytes(&mut hasher, key.as_bytes())?;
        hash_bytes(&mut hasher, value.as_bytes())?;
    }
    hash_len(&mut hasher, arity)?;
    if let Some(program) = program {
        hasher.update([1]);
        hash_bytes(&mut hasher, &program.to_bytes()?)?;
    } else {
        hasher.update([0]);
        hash_bytes(&mut hasher, accepted)?;
    }
    Ok(hasher.finalize().into())
}

fn validate_tag(tag: &PredicateTag) -> Result<()> {
    if tag.kind.is_empty() {
        return Err(invalid("empty canonical predicate kind"));
    }
    let mut previous: Option<&(String, String)> = None;
    for parameter in &tag.parameters {
        if parameter.0.is_empty() {
            return Err(invalid("empty canonical predicate parameter key"));
        }
        if let Some(old) = previous {
            if old.0 >= parameter.0 {
                return Err(invalid(
                    "canonical predicate parameters must be strictly sorted",
                ));
            }
        }
        previous = Some(parameter);
    }
    Ok(())
}

impl CanonicalPredicate {
    /// Evaluate this exact canonical predicate on one ordered port assignment.
    ///
    /// Bounded predicates use their packed truth table. Larger production
    /// predicates use the typed program that is part of the canonical digest.
    pub fn evaluate(&self, ports: &[bool], meter: &mut Meter) -> Result<bool> {
        self.validate()?;
        meter.charge(1, "canonical predicate evaluation")?;
        if let Some(program) = &self.program {
            program.evaluate(ports, meter)
        } else {
            self.accepts(ports)
        }
    }

    fn accepts(&self, ports: &[bool]) -> Result<bool> {
        if ports.len() != self.arity {
            return Err(invalid("canonical predicate port arity"));
        }
        if self.program.is_some() {
            return Err(integrity(
                "programmed predicate routed through truth-table lookup",
            ));
        }
        let mut assignment = 0usize;
        for (port, &bit) in ports.iter().enumerate() {
            if bit {
                assignment |= 1usize << port;
            }
        }
        Ok((self.accepted[assignment / 8] >> (assignment % 8)) & 1 == 1)
    }

    fn validate(&self) -> Result<()> {
        validate_tag(&self.tag)?;
        if let Some(program) = &self.program {
            if !self.accepted.is_empty() {
                return Err(invalid("canonical programmed predicate has truth table"));
            }
            program.validate(self.arity)?;
        } else {
            let (assignments, bytes) = table_shape(self.arity)?;
            if self.accepted.len() != bytes {
                return Err(invalid("canonical predicate truth-table length"));
            }
            if assignments % 8 != 0 {
                let used = assignments % 8;
                let mask = !((1u8 << used) - 1);
                if self.accepted.last().copied().unwrap_or(0) & mask != 0 {
                    return Err(invalid("nonzero unused canonical truth-table bits"));
                }
            }
        }
        if predicate_digest(&self.tag, self.arity, &self.accepted, self.program.as_ref())?
            != self.semantic_sha256
        {
            return Err(invalid("canonical predicate semantic digest"));
        }
        Ok(())
    }
}

#[derive(Clone)]
struct TruthTablePredicate {
    canonical: CanonicalPredicate,
}
impl Predicate for TruthTablePredicate {
    fn arity(&self) -> usize {
        self.canonical.arity
    }

    fn evaluate(&self, ports: &[bool], meter: &mut Meter) -> Result<bool> {
        self.canonical.evaluate(ports, meter)
    }

    fn canonical_tag(&self) -> Option<&PredicateTag> {
        Some(&self.canonical.tag)
    }

    fn canonical_program(&self) -> Option<&PredicateProgram> {
        self.canonical.program.as_ref()
    }
}

impl CanonicalNetwork {
    /// Capture a network and pin vector as exact canonical bounded predicates.
    pub fn capture(
        network: &Network,
        pins: &[Option<bool>],
        provenance: IrProvenance,
        policy: ExportPolicy,
        meter: &mut Meter,
    ) -> Result<Self> {
        network.check_pins(pins)?;
        meter.vector(network.vertex_count(), "canonical vertices")?;
        meter.vector(network.edge_count(), "canonical edges")?;
        let mut vertices = Vec::with_capacity(network.vertex_count());
        for vertex in &network.vertices {
            let arity = vertex.predicate.arity();
            let tag = match vertex.predicate.canonical_tag() {
                Some(tag) => tag.clone(),
                None if policy == ExportPolicy::AllowUntaggedTruthTables => {
                    PredicateTag::new("truth-table", vec![("arity".into(), arity.to_string())])
                }
                None => return Err(invalid("untagged production predicate")),
            };
            validate_tag(&tag)?;
            let (accepted, program) = if arity <= MAX_CANONICAL_PREDICATE_ARITY {
                let (assignments, bytes) = table_shape(arity)?;
                meter.vector(assignments, "canonical truth-table assignments")?;
                meter.vector(bytes, "canonical truth-table bytes")?;
                let mut accepted = vec![0u8; bytes];
                let mut ports = vec![false; arity];
                for assignment in 0..assignments {
                    meter.charge(1, "canonical predicate enumeration")?;
                    for (port, bit) in ports.iter_mut().enumerate() {
                        *bit = assignment & (1usize << port) != 0;
                    }
                    if vertex.predicate.evaluate(&ports, meter)? {
                        accepted[assignment / 8] |= 1u8 << (assignment % 8);
                    }
                }
                (accepted, None)
            } else {
                let program = vertex
                    .predicate
                    .canonical_program()
                    .cloned()
                    .ok_or_else(|| invalid("high-arity predicate lacks typed program"))?;
                program.validate(arity)?;
                (Vec::new(), Some(program))
            };
            let semantic_sha256 = predicate_digest(&tag, arity, &accepted, program.as_ref())?;
            vertices.push(CanonicalVertex {
                predicate: CanonicalPredicate {
                    tag,
                    arity,
                    accepted,
                    program,
                    semantic_sha256,
                },
                edges: vertex.edges.clone(),
            });
        }
        let snapshot = Self {
            version: CANONICAL_IR_VERSION,
            provenance,
            vertices,
            edges: network
                .edges
                .iter()
                .map(|&(left, right)| CanonicalEdge { left, right })
                .collect(),
            interface: network.interface.clone(),
            pins: pins.to_vec(),
        };
        snapshot.validate(meter)?;
        Ok(snapshot)
    }

    /// Validate version, provenance, predicate bindings, graph incidence,
    /// interface order, and pin shape without rebuilding a network.
    pub fn validate(&self, meter: &mut Meter) -> Result<()> {
        if self.version != CANONICAL_IR_VERSION {
            return Err(invalid("unsupported canonical IR version"));
        }
        if self.provenance.source.is_empty()
            || self.provenance.generator.is_empty()
            || self.provenance.revision.is_empty()
        {
            return Err(invalid("canonical IR provenance fields must be nonempty"));
        }
        meter.vector(self.vertices.len(), "canonical validation vertices")?;
        meter.vector(self.edges.len(), "canonical validation edges")?;
        if self.pins.len() != self.edges.len() {
            return Err(invalid("canonical pin vector width"));
        }
        let mut left_seen = vec![false; self.edges.len()];
        let mut right_seen = vec![false; self.edges.len()];
        for edge in &self.edges {
            if edge.left == edge.right
                || edge.left >= self.vertices.len()
                || edge.right >= self.vertices.len()
            {
                return Err(invalid("canonical edge endpoint or self-loop"));
            }
        }
        for (vertex_id, vertex) in self.vertices.iter().enumerate() {
            meter.charge(1, "canonical vertex validation")?;
            vertex.predicate.validate()?;
            if vertex.edges.len() != vertex.predicate.arity {
                return Err(invalid("canonical vertex arity"));
            }
            let mut local = BTreeSet::new();
            for &edge_id in &vertex.edges {
                if edge_id >= self.edges.len() || !local.insert(edge_id) {
                    return Err(invalid("canonical vertex edge incidence"));
                }
                let edge = self.edges[edge_id];
                if edge.left == vertex_id {
                    if left_seen[edge_id] {
                        return Err(invalid("duplicate canonical left incidence"));
                    }
                    left_seen[edge_id] = true;
                } else if edge.right == vertex_id {
                    if right_seen[edge_id] {
                        return Err(invalid("duplicate canonical right incidence"));
                    }
                    right_seen[edge_id] = true;
                } else {
                    return Err(invalid("canonical edge absent from named endpoint"));
                }
            }
        }
        if left_seen.iter().any(|seen| !seen) || right_seen.iter().any(|seen| !seen) {
            return Err(invalid("incomplete canonical edge incidence"));
        }
        let mut interface = BTreeSet::new();
        if self
            .interface
            .iter()
            .any(|&edge| edge >= self.edges.len() || !interface.insert(edge))
        {
            return Err(invalid("canonical interface edges"));
        }
        Ok(())
    }

    fn encoded_len(&self) -> Result<usize> {
        fn string_len(total: &mut usize, value: &str) -> Result<()> {
            checked_add(total, 8, "canonical string length")?;
            checked_add(total, value.len(), "canonical string bytes")
        }
        let mut total = MAGIC.len() + 4;
        string_len(&mut total, &self.provenance.source)?;
        string_len(&mut total, &self.provenance.generator)?;
        string_len(&mut total, &self.provenance.revision)?;
        checked_add(&mut total, 8, "canonical vertex count")?;
        for vertex in &self.vertices {
            string_len(&mut total, &vertex.predicate.tag.kind)?;
            checked_add(&mut total, 8, "canonical parameter count")?;
            for (key, value) in &vertex.predicate.tag.parameters {
                string_len(&mut total, key)?;
                string_len(&mut total, value)?;
            }
            checked_add(&mut total, 8, "canonical predicate arity")?;
            let body_len = if let Some(program) = &vertex.predicate.program {
                program.to_bytes()?.len()
            } else {
                vertex.predicate.accepted.len()
            };
            checked_add(&mut total, 1, "canonical predicate body code")?;
            checked_add(&mut total, 8, "canonical predicate body length")?;
            checked_add(&mut total, body_len, "canonical predicate body bytes")?;
            checked_add(&mut total, 32, "canonical predicate digest")?;
            checked_add(&mut total, 8, "canonical port count")?;
            checked_add(
                &mut total,
                vertex
                    .edges
                    .len()
                    .checked_mul(8)
                    .ok_or_else(|| overflow("canonical port identifiers"))?,
                "canonical port identifiers",
            )?;
        }
        checked_add(&mut total, 8, "canonical edge count")?;
        checked_add(
            &mut total,
            self.edges
                .len()
                .checked_mul(16)
                .ok_or_else(|| overflow("canonical edge endpoints"))?,
            "canonical edge endpoints",
        )?;
        checked_add(&mut total, 8, "canonical interface count")?;
        checked_add(
            &mut total,
            self.interface
                .len()
                .checked_mul(8)
                .ok_or_else(|| overflow("canonical interface identifiers"))?,
            "canonical interface identifiers",
        )?;
        checked_add(&mut total, 8, "canonical pin count")?;
        checked_add(&mut total, self.pins.len(), "canonical pin bytes")?;
        Ok(total)
    }

    /// Serialize to the strict little-endian `BTNIR001` binary format.
    pub fn to_bytes(&self, meter: &mut Meter) -> Result<Vec<u8>> {
        self.validate(meter)?;
        let length = self.encoded_len()?;
        meter.vector(length, "canonical serialized bytes")?;
        let mut bytes = Vec::with_capacity(length);
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&self.version.to_le_bytes());
        write_string(&mut bytes, &self.provenance.source)?;
        write_string(&mut bytes, &self.provenance.generator)?;
        write_string(&mut bytes, &self.provenance.revision)?;
        write_len(&mut bytes, self.vertices.len())?;
        for vertex in &self.vertices {
            write_string(&mut bytes, &vertex.predicate.tag.kind)?;
            write_len(&mut bytes, vertex.predicate.tag.parameters.len())?;
            for (key, value) in &vertex.predicate.tag.parameters {
                write_string(&mut bytes, key)?;
                write_string(&mut bytes, value)?;
            }
            write_len(&mut bytes, vertex.predicate.arity)?;
            if let Some(program) = &vertex.predicate.program {
                bytes.push(1);
                let program = program.to_bytes()?;
                write_len(&mut bytes, program.len())?;
                bytes.extend_from_slice(&program);
            } else {
                bytes.push(0);
                write_len(&mut bytes, vertex.predicate.accepted.len())?;
                bytes.extend_from_slice(&vertex.predicate.accepted);
            }
            bytes.extend_from_slice(&vertex.predicate.semantic_sha256);
            write_len(&mut bytes, vertex.edges.len())?;
            for &edge in &vertex.edges {
                write_len(&mut bytes, edge)?;
            }
        }
        write_len(&mut bytes, self.edges.len())?;
        for edge in &self.edges {
            write_len(&mut bytes, edge.left)?;
            write_len(&mut bytes, edge.right)?;
        }
        write_len(&mut bytes, self.interface.len())?;
        for &edge in &self.interface {
            write_len(&mut bytes, edge)?;
        }
        write_len(&mut bytes, self.pins.len())?;
        for pin in &self.pins {
            bytes.push(match pin {
                None => 0,
                Some(false) => 1,
                Some(true) => 2,
            });
        }
        if bytes.len() != length {
            return Err(integrity("canonical encoded length disagreement"));
        }
        Ok(bytes)
    }

    /// Parse strict canonical bytes. Trailing bytes and malformed incidence are
    /// rejected before a network is returned.
    pub fn from_bytes(bytes: &[u8], meter: &mut Meter) -> Result<Self> {
        meter.vector(bytes.len(), "canonical input bytes")?;
        let mut reader = Reader::new(bytes);
        if reader.take(MAGIC.len())? != MAGIC {
            return Err(invalid("canonical IR magic"));
        }
        let version = reader.u32()?;
        let provenance = IrProvenance {
            source: reader.string(meter)?,
            generator: reader.string(meter)?,
            revision: reader.string(meter)?,
        };
        let vertex_count = reader.length(meter, "canonical parsed vertices")?;
        let mut vertices = Vec::with_capacity(vertex_count);
        for _ in 0..vertex_count {
            let kind = reader.string(meter)?;
            let parameter_count = reader.length(meter, "canonical parsed parameters")?;
            let mut parameters = Vec::with_capacity(parameter_count);
            for _ in 0..parameter_count {
                parameters.push((reader.string(meter)?, reader.string(meter)?));
            }
            let arity = reader.index()?;
            let body_code = reader.byte()?;
            let body_len = reader.length(meter, "canonical parsed predicate body")?;
            let body = reader.take(body_len)?;
            let (accepted, program) = match body_code {
                0 => (body.to_vec(), None),
                1 => (Vec::new(), Some(PredicateProgram::from_bytes(body, meter)?)),
                _ => return Err(invalid("canonical predicate body code")),
            };
            let semantic_sha256: [u8; 32] = reader
                .take(32)?
                .try_into()
                .map_err(|_| integrity("canonical predicate digest width"))?;
            let port_count = reader.length(meter, "canonical parsed ports")?;
            let mut edges = Vec::with_capacity(port_count);
            for _ in 0..port_count {
                edges.push(reader.index()?);
            }
            vertices.push(CanonicalVertex {
                predicate: CanonicalPredicate {
                    tag: PredicateTag { kind, parameters },
                    arity,
                    accepted,
                    program,
                    semantic_sha256,
                },
                edges,
            });
        }
        let edge_count = reader.length(meter, "canonical parsed edges")?;
        let mut edges = Vec::with_capacity(edge_count);
        for _ in 0..edge_count {
            edges.push(CanonicalEdge {
                left: reader.index()?,
                right: reader.index()?,
            });
        }
        let interface_count = reader.length(meter, "canonical parsed interface")?;
        let mut interface = Vec::with_capacity(interface_count);
        for _ in 0..interface_count {
            interface.push(reader.index()?);
        }
        let pin_count = reader.length(meter, "canonical parsed pins")?;
        let mut pins = Vec::with_capacity(pin_count);
        for &pin in reader.take(pin_count)? {
            pins.push(match pin {
                0 => None,
                1 => Some(false),
                2 => Some(true),
                _ => return Err(invalid("canonical pin code")),
            });
        }
        if !reader.is_empty() {
            return Err(invalid("trailing canonical IR bytes"));
        }
        let snapshot = Self {
            version,
            provenance,
            vertices,
            edges,
            interface,
            pins,
        };
        snapshot.validate(meter)?;
        Ok(snapshot)
    }

    /// Return SHA-256 of the strict canonical bytes.
    pub fn sha256(&self, meter: &mut Meter) -> Result<[u8; 32]> {
        let bytes = self.to_bytes(meter)?;
        let blocks = bytes.len().div_ceil(64);
        meter.charge(
            u64::try_from(blocks).map_err(|_| overflow("canonical SHA-256 blocks"))?,
            "canonical SHA-256",
        )?;
        Ok(Sha256::digest(bytes).into())
    }

    /// Return lowercase hexadecimal SHA-256 of the strict canonical bytes.
    pub fn sha256_hex(&self, meter: &mut Meter) -> Result<String> {
        let digest = self.sha256(meter)?;
        let mut out = String::with_capacity(64);
        for byte in digest {
            use std::fmt::Write;
            write!(&mut out, "{byte:02x}")
                .map_err(|_| integrity("canonical SHA-256 formatting"))?;
        }
        Ok(out)
    }

    /// Rebuild an exact predicate graph and return its bound pin vector.
    /// Parsimonious program hints are intentionally not reconstructed; the
    /// rebuilt graph is an independent direct-count replay instrument.
    pub fn rebuild(&self, meter: &mut Meter) -> Result<(Network, Vec<Option<bool>>)> {
        self.validate(meter)?;
        let vertices = self
            .vertices
            .iter()
            .map(|vertex| Vertex {
                predicate: Arc::new(TruthTablePredicate {
                    canonical: vertex.predicate.clone(),
                }) as Arc<dyn Predicate>,
                edges: vertex.edges.clone(),
            })
            .collect();
        let network = Network {
            vertices,
            edges: self
                .edges
                .iter()
                .map(|edge| (edge.left, edge.right))
                .collect(),
            interface: self.interface.clone(),
            model: None,
        };
        Ok((network, self.pins.clone()))
    }
}

fn write_len(bytes: &mut Vec<u8>, value: usize) -> Result<()> {
    let value = u64::try_from(value).map_err(|_| overflow("canonical serialized length"))?;
    bytes.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

fn write_string(bytes: &mut Vec<u8>, value: &str) -> Result<()> {
    write_len(bytes, value.len())?;
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

fn write_biguint(bytes: &mut Vec<u8>, value: &BigUint) -> Result<()> {
    let limbs = value.to_bytes_le();
    write_len(bytes, limbs.len())?;
    bytes.extend_from_slice(&limbs);
    Ok(())
}

struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8]> {
        let end = self
            .position
            .checked_add(length)
            .ok_or_else(|| overflow("canonical parser position"))?;
        let slice = self
            .bytes
            .get(self.position..end)
            .ok_or_else(|| invalid("truncated canonical IR"))?;
        self.position = end;
        Ok(slice)
    }

    fn u32(&mut self) -> Result<u32> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| integrity("canonical u32 width"))?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn byte(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn u64(&mut self) -> Result<u64> {
        let bytes: [u8; 8] = self
            .take(8)?
            .try_into()
            .map_err(|_| integrity("canonical u64 width"))?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn index(&mut self) -> Result<usize> {
        usize::try_from(self.u64()?).map_err(|_| invalid("canonical index exceeds usize"))
    }

    fn length(&mut self, meter: &Meter, phase: &str) -> Result<usize> {
        let length = self.index()?;
        meter.vector(length, phase)?;
        Ok(length)
    }

    fn string(&mut self, meter: &Meter) -> Result<String> {
        let length = self.length(meter, "canonical parsed string")?;
        String::from_utf8(self.take(length)?.to_vec())
            .map_err(|_| Error::Invalid("canonical string is not UTF-8".into()))
    }

    fn biguint(&mut self, meter: &Meter) -> Result<BigUint> {
        let length = self.length(meter, "canonical parsed integer")?;
        let bytes = self.take(length)?;
        if bytes.last() == Some(&0) {
            return Err(invalid("noncanonical little-endian integer"));
        }
        Ok(BigUint::from_bytes_le(bytes))
    }

    fn is_empty(&self) -> bool {
        self.position == self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contraction::{Backend, Counter};
    use crate::encoding::Encoder;
    use crate::network::{direct_count, FnPredicate, GraphBuilder};
    use crate::Limits;
    use num_traits::Zero;

    fn meter() -> Meter {
        Meter::new(Limits {
            steps: 20_000_000,
            vector: 1_000_000,
            bits: 1_000_000,
        })
    }

    fn fixture() -> (Network, Vec<Option<bool>>, IrProvenance) {
        let work = meter();
        let mut encoder = Encoder::new();
        let a = encoder.input_word(1, &work).unwrap();
        let b = encoder.input_word(1, &work).unwrap();
        let c = encoder.and(&a, &b, &work).unwrap();
        encoder.mark_interface(&a, &work).unwrap();
        encoder.mark_interface(&b, &work).unwrap();
        encoder.mark_interface(&c, &work).unwrap();
        let network = encoder.finish(&work).unwrap();
        let mut pins = vec![None; network.edge_count()];
        pins[network.interface()[0]] = Some(true);
        (
            network,
            pins,
            IrProvenance {
                source: "canonical-ir-and-fixture".into(),
                generator: "hashed-ecdl-tensor::encoding::Encoder".into(),
                revision: "test-v1".into(),
            },
        )
    }

    fn prefix_counts(
        network: &Network,
        pins: &[Option<bool>],
        work: &mut Meter,
    ) -> Vec<Vec<crate::BigUint>> {
        let mut profile = Vec::new();
        for length in 0..=network.interface().len() {
            let mut level = Vec::new();
            for assignment in 0..(1usize << length) {
                let mut conditioned = pins.to_vec();
                let mut conflict = false;
                for position in 0..length {
                    let edge = network.interface()[position];
                    let bit = assignment & (1usize << position) != 0;
                    if conditioned[edge].is_some_and(|old| old != bit) {
                        conflict = true;
                        break;
                    }
                    conditioned[edge] = Some(bit);
                }
                level.push(if conflict {
                    crate::BigUint::zero()
                } else {
                    direct_count(network, &conditioned, work).unwrap()
                });
            }
            profile.push(level);
        }
        profile
    }

    #[test]
    fn tagged_ir_round_trip_preserves_counts_prefixes_witnesses_and_bytes() {
        let (network, pins, provenance) = fixture();
        let mut work = meter();
        let snapshot = CanonicalNetwork::capture(
            &network,
            &pins,
            provenance,
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .unwrap();
        let bytes = snapshot.to_bytes(&mut work).unwrap();
        let digest = snapshot.sha256_hex(&mut work).unwrap();
        assert_eq!(
            digest,
            "bb17ed447ff1f7585d22abf0cadd914e0da988cac3e4308fe0a451465e70106c"
        );
        let parsed = CanonicalNetwork::from_bytes(&bytes, &mut work).unwrap();
        assert_eq!(parsed.to_bytes(&mut work).unwrap(), bytes);
        assert_eq!(parsed.sha256_hex(&mut work).unwrap(), digest);
        let (rebuilt, rebuilt_pins) = parsed.rebuild(&mut work).unwrap();
        let recaptured = CanonicalNetwork::capture(
            &rebuilt,
            &rebuilt_pins,
            parsed.provenance.clone(),
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .unwrap();
        assert_eq!(recaptured.to_bytes(&mut work).unwrap(), bytes);

        let direct = direct_count(&network, &pins, &mut work).unwrap();
        let rebuilt_direct = direct_count(&rebuilt, &rebuilt_pins, &mut work).unwrap();
        assert_eq!(direct, rebuilt_direct);
        let mut functional = Counter::default();
        assert_eq!(
            functional.count(&network, &pins, &mut work).unwrap(),
            direct
        );
        assert_eq!(
            functional.statistics.last_backend,
            Some(Backend::Functional)
        );
        assert_eq!(
            prefix_counts(&network, &pins, &mut work),
            prefix_counts(&rebuilt, &rebuilt_pins, &mut work)
        );

        let mut original_witness = Counter::default();
        let mut replay_witness = Counter::default();
        assert_eq!(
            original_witness
                .witness(&network, &pins, &mut work)
                .unwrap(),
            replay_witness
                .witness(&rebuilt, &rebuilt_pins, &mut work)
                .unwrap()
        );
        assert_eq!(
            replay_witness.statistics.last_backend,
            Some(Backend::Direct)
        );
    }

    #[test]
    fn all_five_registered_mutations_are_detected_or_change_semantics() {
        let (network, pins, provenance) = fixture();
        let mut work = meter();
        let snapshot = CanonicalNetwork::capture(
            &network,
            &pins,
            provenance,
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .unwrap();
        let original_digest = snapshot.sha256_hex(&mut work).unwrap();
        let original_count = direct_count(&network, &pins, &mut work).unwrap();

        let mut endpoints = snapshot.clone();
        endpoints.edges.swap(0, 1);
        assert!(endpoints.validate(&mut work).is_err());

        let mut interface = snapshot.clone();
        interface.interface.swap(0, 1);
        assert_ne!(interface.sha256_hex(&mut work).unwrap(), original_digest);
        let (interface_network, interface_pins) = interface.rebuild(&mut work).unwrap();
        let mut original_witness = Counter::default();
        let mut mutated_witness = Counter::default();
        assert_ne!(
            original_witness
                .witness(&network, &pins, &mut work)
                .unwrap(),
            mutated_witness
                .witness(&interface_network, &interface_pins, &mut work)
                .unwrap()
        );

        let mut parameter = snapshot.clone();
        parameter.vertices[0].predicate.tag.parameters[0]
            .1
            .push('x');
        assert!(parameter.validate(&mut work).is_err());

        let mut ports = snapshot.clone();
        ports.vertices[0].edges.reverse();
        assert_ne!(ports.sha256_hex(&mut work).unwrap(), original_digest);
        let (port_network, port_pins) = ports.rebuild(&mut work).unwrap();
        assert_ne!(
            direct_count(&port_network, &port_pins, &mut work).unwrap(),
            original_count
        );

        let mut dropped_pin = snapshot.clone();
        dropped_pin.pins.pop();
        assert!(dropped_pin.validate(&mut work).is_err());
    }

    #[test]
    fn untagged_predicates_require_the_explicit_fixture_policy() {
        let mut work = meter();
        let mut builder = GraphBuilder::new();
        let left = builder.vertex(FnPredicate::new(1, |ports, _| Ok(ports[0])));
        let right = builder.vertex(FnPredicate::new(1, |_, _| Ok(true)));
        let edge = builder.edge(left, right, &mut work).unwrap();
        let network = builder.finish(vec![edge]).unwrap();
        let provenance = IrProvenance {
            source: "untagged-fixture".into(),
            generator: "GraphBuilder".into(),
            revision: "test-v1".into(),
        };
        assert!(CanonicalNetwork::capture(
            &network,
            &[None],
            provenance.clone(),
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .is_err());
        assert!(CanonicalNetwork::capture(
            &network,
            &[None],
            provenance,
            ExportPolicy::AllowUntaggedTruthTables,
            &mut work,
        )
        .is_ok());
    }

    #[test]
    fn high_arity_closures_without_typed_programs_are_rejected() {
        let mut work = meter();
        let mut builder = GraphBuilder::new();
        let left = builder.vertex(FnPredicate::tagged(
            MAX_CANONICAL_PREDICATE_ARITY + 1,
            PredicateTag::new("test.high-arity-left", Vec::new()),
            |_, _| Ok(true),
        ));
        let right = builder.vertex(FnPredicate::tagged(
            MAX_CANONICAL_PREDICATE_ARITY + 1,
            PredicateTag::new("test.high-arity-right", Vec::new()),
            |_, _| Ok(true),
        ));
        for _ in 0..=MAX_CANONICAL_PREDICATE_ARITY {
            builder.edge(left, right, &mut work).unwrap();
        }
        let network = builder.finish(Vec::new()).unwrap();
        assert!(CanonicalNetwork::capture(
            &network,
            &vec![None; network.edge_count()],
            IrProvenance {
                source: "high-arity-closure".into(),
                generator: "GraphBuilder".into(),
                revision: "test-v1".into(),
            },
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .is_err());
    }

    #[test]
    fn representative_decomposition_network_uses_typed_programs_above_the_exact_cap() {
        let mut work = Meter::new(Limits {
            steps: 500_000_000,
            vector: 20_000_000,
            bits: 1_000_000,
        });
        let curve = crate::curve::tiny(&mut work);
        let bases = [(7u8, 3u8), (2u8, 1u8)]
            .into_iter()
            .map(|(alpha, beta)| {
                crate::reduction::FactorBase::new(
                    &curve,
                    11u8.into(),
                    5u8.into(),
                    crate::reduction::HashSeed {
                        alpha: alpha.into(),
                        beta: beta.into(),
                    },
                    &mut work,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let network =
            crate::reduction::decomposition_network(&curve, &bases, curve.generator(), &mut work)
                .unwrap();
        let pins = vec![None; network.edge_count()];
        let snapshot = CanonicalNetwork::capture(
            &network,
            &pins,
            IrProvenance {
                source: "tiny-decomposition-network".into(),
                generator: "hashed-ecdl-tensor::reduction::decomposition_network".into(),
                revision: "test-v1".into(),
            },
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .unwrap();
        assert!(snapshot
            .vertices
            .iter()
            .all(|vertex| vertex.predicate.tag.kind != "truth-table"));
        assert!(snapshot
            .vertices
            .iter()
            .any(|vertex| vertex.predicate.arity > MAX_CANONICAL_PREDICATE_ARITY));
        assert!(snapshot.vertices.iter().all(|vertex| {
            vertex.predicate.arity <= MAX_CANONICAL_PREDICATE_ARITY
                || vertex.predicate.program.is_some()
        }));
        let bytes = snapshot.to_bytes(&mut work).unwrap();
        let parsed = CanonicalNetwork::from_bytes(&bytes, &mut work).unwrap();
        assert_eq!(parsed.to_bytes(&mut work).unwrap(), bytes);
        let mut corrupted_program = parsed.clone();
        let program = corrupted_program
            .vertices
            .iter_mut()
            .find_map(|vertex| vertex.predicate.program.as_mut())
            .unwrap();
        program.words[0].reverse();
        assert!(corrupted_program.validate(&mut work).is_err());
        let (rebuilt, rebuilt_pins) = parsed.rebuild(&mut work).unwrap();
        let recaptured = CanonicalNetwork::capture(
            &rebuilt,
            &rebuilt_pins,
            parsed.provenance.clone(),
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .unwrap();
        assert_eq!(recaptured.to_bytes(&mut work).unwrap(), bytes);
        for (original, replay) in network.vertices.iter().zip(&rebuilt.vertices) {
            if original.predicate.arity() <= MAX_CANONICAL_PREDICATE_ARITY {
                continue;
            }
            for bit in [false, true] {
                let ports = vec![bit; original.predicate.arity()];
                assert_eq!(
                    original.predicate.evaluate(&ports, &mut work).unwrap(),
                    replay.predicate.evaluate(&ports, &mut work).unwrap()
                );
            }
        }
    }

    #[test]
    fn parser_rejects_truncation_trailing_bytes_and_bad_magic() {
        let (network, pins, provenance) = fixture();
        let mut work = meter();
        let snapshot = CanonicalNetwork::capture(
            &network,
            &pins,
            provenance,
            ExportPolicy::RequireTagged,
            &mut work,
        )
        .unwrap();
        let bytes = snapshot.to_bytes(&mut work).unwrap();
        assert!(CanonicalNetwork::from_bytes(&bytes[..bytes.len() - 1], &mut work).is_err());
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(CanonicalNetwork::from_bytes(&trailing, &mut work).is_err());
        let mut bad_magic = bytes;
        bad_magic[0] ^= 1;
        assert!(CanonicalNetwork::from_bytes(&bad_magic, &mut work).is_err());
    }
}
