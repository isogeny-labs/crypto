module
public section
namespace CoverGraph
variable {U X : Type*}

/-- Original variables, with all residuals evaluated at the prescribed image. -/
@[expose] def Original (phi : U → X) (residual : U → X → Prop) :=
  {u : U // residual u (phi u)}

/-- Added image coordinates constrained to be the actual image. -/
@[expose] def Lifted (phi : U → X) (residual : U → X → Prop) :=
  {v : U × X // v.2=phi v.1 ∧ residual v.1 v.2}

def lift (phi : U → X) (residual : U → X → Prop) :
    Original phi residual → Lifted phi residual :=
  fun u => ⟨(u.val,phi u.val),rfl,u.property⟩

def project (phi : U → X) (residual : U → X → Prop) :
    Lifted phi residual → Original phi residual :=
  fun v => ⟨v.val.1,by rw [← v.property.1]; exact v.property.2⟩

/-- The actual original-to-graph map has the actual projection as left inverse. -/
theorem projectLift (phi : U → X) (residual : U → X → Prop)
    (u : Original phi residual) : project phi residual (lift phi residual u)=u := by
  cases u
  rfl

/-- The same projection is a right inverse, so no image-coordinate solutions are added. -/
theorem liftProject (phi : U → X) (residual : U → X → Prop)
    (v : Lifted phi residual) : lift phi residual (project phi residual v)=v := by
  apply Subtype.ext
  apply Prod.ext
  · rfl
  · exact v.property.1.symm
end CoverGraph
#print axioms CoverGraph.projectLift
#print axioms CoverGraph.liftProject
