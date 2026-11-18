# Private runtime orchestration

Implemented on ac1102e9b6abc40ccb3e2b1ec721f56b4ee06e37. Compilation and focused controls are pending. No DatasetPlan or dispatch registration.

Sources: dynamic prepare_original, prepare_step4, invoke_original, retain_linear, run_group_step4, confirm_step4_witness, retain_step4_terminal, step4_outcome, take_step4_artifacts and borrowed failure visitors; solvers dynamic_original/parent flush/verify_dynamic_claims; execution original limits; curves public and subgroup factor-base verification; contracts/STAGE3_ADAPTER_DESIGN.md. This follows existing algorithms and capture rules.

The composition requires a Bound sink and unexpired supplied original worker deadline; reconstructs public/domain inputs without private witness; prepares the persistent publisher before native work; performs linear/group stages; confirms once; retains terminal; reads sealed facts before moving owners; then flushes ordinary artifacts. Early preparation/native admission failures borrow complete retained slots into live custody and return explicit partial labels, not completion. Missing terminal evidence retains admitted artifacts and conservative false labels.

The proposed integration control uses n47/m4 both twists with a fixed known positive target equal to four copies of the first verified subgroup factor-base point. This is an explicit software control, no resampling, general recovery, or new scientific measurement. It exercises real capture, independent original-input replay, outer labels and snapshot restore; missing Bound sink and expired deadline reject. Attempt identity/context are synthetic fixture objects: production original binding and central admission remain required.

Outstanding before registration: independently verify original config/context against bound capture and run; unique manifest public/factor-base/solver-domain roles; reserved identity trigger; supervisor/pre-common-stage censored policy; parent return events and capture-finish failures; original-run CLI controls. No new spend or jobs.

Predecessor validation:585 focused passed,0 failed,7 ignored; strict release Clippy passed. PR2993 delivers the separately validated earlier3adc85e source; this runtime increment is not in that PR.
