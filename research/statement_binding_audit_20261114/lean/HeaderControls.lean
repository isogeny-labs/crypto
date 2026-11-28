module

public import Mathlib.Tactic

@[expose] public section

namespace StatementBindingAudit

/-- Genuine declarations with legal signature syntax that can contain a
separator before the declaration value. These are parser controls. -/
theorem bar_control (n : ℤ) : |n| ≥ 0 := abs_nonneg n

def namedArgument (n : ℕ) : ℕ := n

theorem named_argument_control (n : ℕ) : namedArgument (n := n)=n := rfl

theorem default_proof_control (n : ℕ) (h : n=n := by rfl) : n=n := h

theorem inline_type_control (n : (by exact ℕ)) : n=n := rfl

theorem comment_control (n : ℕ) : n /- | := by -/ = n := rfl

theorem string_control : ("a|:=" : String)="a|:=" := rfl

/-- A small delimiter-only model. Equivalence to the frozen Python regex is
not asserted; actual extractor behavior is separately observed. -/
def prefixChars (delimiter : List Char) : List Char → List Char
  | [] => []
  | c :: cs => if delimiter.isPrefixOf (c :: cs) then [] else c :: prefixChars delimiter cs

def prefixBefore (delimiter source : String) : List Char := prefixChars delimiter.toList source.toList

def barDelimiter : String := "|"
def barSourceA : String := "theorem t (n : Int) : |n| >= 0 := abs_nonneg n"
def barSourceB : String := "theorem t (n : Int) : |n| = |n| := rfl"
def namedDelimiter : String := ":="
def namedSourceA : String := "theorem t (n : Nat) : namedArgument (n := n)=n := rfl"
def namedSourceB : String := "theorem t (n : Nat) : namedArgument (n := n)+0=n := by simp [namedArgument]"
def proofDelimiter : String := "by"
def proofSourceA : String := "theorem t (n : Nat) (h : n=n := by rfl) : n=n := h"
def proofSourceB : String := "theorem t (n : Nat) (h : n=n := by rfl) : n+0=n := h"

theorem delimiter_model_not_injective :
    barSourceA ≠ barSourceB ∧ prefixBefore barDelimiter barSourceA=prefixBefore barDelimiter barSourceB ∧
    namedSourceA ≠ namedSourceB ∧ prefixBefore namedDelimiter namedSourceA=prefixBefore namedDelimiter namedSourceB ∧
    proofSourceA ≠ proofSourceB ∧ prefixBefore proofDelimiter proofSourceA=prefixBefore proofDelimiter proofSourceB := by
  decide +kernel

end StatementBindingAudit
