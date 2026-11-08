import Lean.Elab.Tactic.Omega

/-!
Install layout used by `ic update` and `ic fixed`.

`update::layout` installs binaries into the selected directory and data into
`<prefix>/share/cryptocli` when the directory is called `bin`; otherwise data
lives in `<install>/.cryptocli`. This file proves the path identity that the
packaged binary relies on:

    layout (base ++ ["bin"]) = (base ++ ["bin"], base ++ ["share", "cryptocli"])

Rust tests cover concrete absolute paths and the non-`bin` fallback.
-/

namespace InstallLayout

/-- `(bin directory, share directory)` for a path represented as segments. -/
def layout (bin : List String) : List String × List String :=
  match bin.reverse with
  | "bin" :: rest => (bin, rest.reverse ++ ["share", "cryptocli"])
  | _ => (bin, bin ++ [".cryptocli"])

/-- The binary directory is never renamed. -/
theorem layout_bin (bin : List String) : (layout bin).1 = bin := by
  unfold layout
  split <;> rfl

/-- A standard `<base>/bin` install puts data under `<base>/share/cryptocli`. -/
theorem layout_base_bin (base : List String) :
    layout (base ++ ["bin"]) = (base ++ ["bin"], base ++ ["share", "cryptocli"]) := by
  unfold layout
  simp [List.reverse_append]

end InstallLayout

#print axioms InstallLayout.layout_bin
#print axioms InstallLayout.layout_base_bin
