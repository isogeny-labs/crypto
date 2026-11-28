import Mathlib

/-!
# Fiat--Shamir transcript encoding is a separate proof obligation

This module isolates one representation boundary in Fiat--Shamir arguments.
Raw concatenation is not injective: two distinct typed field tuples can yield
the same hash input without finding a collision in the hash function.

The positive model uses natural numbers as *logical encoding tokens*.  Every
field has an ordered tag and an exact length, the decoder checks those tags in
order, and successful decoding requires full input consumption.  Natural
length tokens are canonical by construction; a byte codec still has to prove
that its concrete integer encoding is canonical and bounded.

Nothing below models a random oracle, hash collision resistance, protocol
reachability, or Fiat--Shamir security.  An exploitable ambiguity additionally
requires both typed interpretations to be admissible protocol transcripts.
-/

namespace FiatShamirEncoding

/-! ## Raw concatenation alias -/

/-- The unsafe representation of two typed fields by bare concatenation. -/
def rawEncode (fields : List Nat × List Nat) : List Nat :=
  fields.1 ++ fields.2

/-- First typed interpretation of the byte/token string `[1, 2, 3]`. -/
def rawLeft : List Nat × List Nat := ([1], [2, 3])

/-- Second typed interpretation of the same byte/token string. -/
def rawRight : List Nat × List Nat := ([1, 2], [3])

/-- The two typed transcripts are genuinely different. -/
theorem raw_transcripts_distinct : rawLeft ≠ rawRight := by
  decide

/-- Bare concatenation aliases the two distinct typed transcripts. -/
theorem raw_encoding_alias : rawEncode rawLeft = rawEncode rawRight := by
  decide

/-- Every hash function receives exactly the same input on the alias.

This is an encoding collision, not a collision in `H`: equality holds before
`H` is evaluated and no property of `H` is used.
-/
theorem every_hash_sees_raw_alias {Digest : Type*}
    (H : List Nat → Digest) :
    H (rawEncode rawLeft) = H (rawEncode rawRight) := by
  rw [raw_encoding_alias]

/-! ## A tagged, length-framed typed codec -/

/-- Domain separation is a typed field, not an unstructured prefix chosen by
the caller. -/
inductive Domain where
  | proof
  | signature
  deriving DecidableEq, Inhabited

/-- Minimal typed transcript used by the positive representation theorem. -/
structure Transcript where
  domain : Domain
  publicParams : List Nat
  statement : List Nat
  commitment : List Nat
  deriving DecidableEq, Inhabited

/-- A logical frame: tag, exact payload length, then payload.

The length is one natural-number token.  Refining this to bytes requires a
fixed-width or otherwise canonical natural-number codec plus a size cap.
-/
def frame (tag : Nat) (payload : List Nat) : List Nat :=
  [tag, payload.length] ++ payload

/-- Parse one expected frame and return its payload and unconsumed suffix.
The expected tag enforces field order. -/
def unframe (expected : Nat) : List Nat → Option (List Nat × List Nat)
  | tag :: count :: input =>
      if tag = expected ∧ count ≤ input.length then
        some (input.take count, input.drop count)
      else
        none
  | _ => none

/-- A frame parses to its original payload while leaving the following input
untouched. -/
@[simp] theorem unframe_frame_append
    (tag : Nat) (payload suffix : List Nat) :
    unframe tag (frame tag payload ++ suffix) = some (payload, suffix) := by
  simp [unframe, frame]

@[simp] theorem unframe_frame (tag : Nat) (payload : List Nat) :
    unframe tag (frame tag payload) = some (payload, []) := by
  simpa using unframe_frame_append tag payload []

/-- A wrong tag is rejected before any payload is interpreted. -/
theorem unframe_wrong_tag
    (expected actual : Nat) (payload suffix : List Nat)
    (hne : actual ≠ expected) :
    unframe expected (frame actual payload ++ suffix) = none := by
  simp [unframe, frame, hne]

/-- Canonical one-token encoding of the protocol domain. -/
def encodeDomain : Domain → List Nat
  | .proof => [10]
  | .signature => [11]

/-- Exact inverse on canonical domain tokens; extra tokens are rejected. -/
def decodeDomain : List Nat → Option Domain
  | [10] => some .proof
  | [11] => some .signature
  | _ => none

@[simp] theorem decodeDomain_encodeDomain (domain : Domain) :
    decodeDomain (encodeDomain domain) = some domain := by
  cases domain <;> rfl

/-- The canonical encoding fixes the domain and all public transcript fields,
with distinct ordered tags. -/
def encode (transcript : Transcript) : List Nat :=
  frame 0 (encodeDomain transcript.domain) ++
    (frame 1 transcript.publicParams ++
      (frame 2 transcript.statement ++
        frame 3 transcript.commitment))

/-- The partial inverse enforces tag order and rejects every trailing token. -/
def decode (input : List Nat) : Option Transcript := do
  let (domainTokens, rest1) ← unframe 0 input
  let domain ← decodeDomain domainTokens
  let (publicParams, rest2) ← unframe 1 rest1
  let (statement, rest3) ← unframe 2 rest2
  let (commitment, rest4) ← unframe 3 rest3
  if rest4 = [] then
    some {
      domain := domain
      publicParams := publicParams
      statement := statement
      commitment := commitment
    }
  else
    none

/-- Every typed transcript survives a canonical encode/decode round trip. -/
@[simp] theorem decode_encode (transcript : Transcript) :
    decode (encode transcript) = some transcript := by
  rcases transcript with ⟨domain, publicParams, statement, commitment⟩
  cases domain <;>
    simp [encode, decode, encodeDomain, decodeDomain]

/-- Full-consumption is semantic: appending a nonempty suffix makes an
otherwise canonical encoding invalid. -/
theorem decode_encode_append_rejected
    (transcript : Transcript) (suffix : List Nat) (hsuffix : suffix ≠ []) :
    decode (encode transcript ++ suffix) = none := by
  rcases transcript with ⟨domain, publicParams, statement, commitment⟩
  cases domain <;>
    simp [encode, decode, encodeDomain, decodeDomain, List.append_assoc,
      hsuffix]

/-- A total retraction used only to apply the standard left-inverse theorem.
Invalid encodings map to the default transcript; `decode` remains the
normative rejecting decoder. -/
def decodeTotal (input : List Nat) : Transcript :=
  (decode input).getD default

/-- The total retraction is a left inverse on canonical encodings. -/
theorem decodeTotal_encode : Function.LeftInverse decodeTotal encode := by
  intro transcript
  simp [decodeTotal]

/-- Tagged length framing makes the typed transcript encoding injective. -/
theorem encode_injective : Function.Injective encode :=
  decodeTotal_encode.injective

end FiatShamirEncoding
