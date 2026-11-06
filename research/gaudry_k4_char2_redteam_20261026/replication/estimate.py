# Wall-time estimates for 8 targets per arm at m = 16, 17 on 4 cores (one job per core).
# All inputs are measured in the study sessions; scaling is an estimate.
import math
ic_ms_per_trial = [2.9, 3.0, 3.0, 3.30, 3.29, 3.17]   # m=10,12,12,15,15,14 (sessions d-ladder, e-m15, f-lp)
ic_ms = sum(ic_ms_per_trial) / len(ic_ms_per_trial)
trials_per_rel = {15: (1545102/8020 + 1572023/8004)/2}
rel_target = {16: 2**16/4, 17: 2**17/4}
tpr = {16: 196.0, 17: 198.0}                           # trials per relation: 178-196 measured m=10..15, slight trend (estimate)
ic_hours = {}
for m in (16, 17):
    trials = rel_target[m] * tpr[m]
    ic_hours[m] = trials * ic_ms / 1000 / 3600
# rho: model steps = expected_steps (detail.expected_steps), scales 4x per m (sqrt(r)); 0.56 us/step at one-word (m=15), ~0.97 us/step at u128 (m=17, L0 host)
exp_steps_15 = 672867904
rho_steps = {16: 4*exp_steps_15, 17: 16*exp_steps_15}
rho_hours_model = {16: rho_steps[16]*0.56e-6/3600, 17: rho_steps[17]*0.97e-6/3600}
rho_hours_measured17 = (15986242844+11722773009)/2*0.97e-6/3600
def wall_4core(job_hours, cores=4):
    # 8 identical jobs on 4 cores: two waves
    waves = math.ceil(len(job_hours)/cores)
    return waves * max(job_hours)
out = {}
for m in (16, 17):
    ic = [ic_hours[m]]*8
    rho = [rho_hours_model[m]]*8 if m == 16 else [rho_hours_measured17]*8
    out[m] = dict(ic_per_target_h=round(ic_hours[m],2), rho_per_target_h=round(rho[0],2),
                  ic_arm_wall_h=round(wall_4core(ic),2), rho_arm_wall_h=round(wall_4core(rho),2),
                  both_arms_total_core_h=round(sum(ic)+sum(rho),1),
                  both_arms_4core_lower_bound_h=round((sum(ic)+sum(rho))/4,1))
for m,v in out.items(): print(m, v)
print('ic m15 check: ms/trial', round(ic_ms,2), 'rho m17 model hours', rho_hours_model[17], 'measured-steps', rho_hours_measured17)
