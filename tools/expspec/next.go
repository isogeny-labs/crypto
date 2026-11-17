package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"os"
	"sort"
	"strings"

	"gopkg.in/yaml.v3"
)

// Action is the one thing `next` tells the main session to do.
type Action map[string]any

func act(kind string, kv ...any) Action {
	a := Action{"action": kind}
	for i := 0; i+1 < len(kv); i += 2 {
		a[kv[i].(string)] = kv[i+1]
	}
	return a
}

func runCardAction(key, card string, background, assess bool) Action {
	return act("run_card", "key", key, "card", card, "relay", "python3 tools/task_card.py prompt "+card,
		"background", background, "assess", assess)
}

type runner struct {
	spec      *Spec
	specPath  string
	st        *State
	paths     map[string]string
	cells     []Cell
	cellsByID map[string]*Cell
	depth     int
}

// humanGate returns nil when answered yes, an ask (or stop on no) otherwise.
func (r *runner) humanGate(gid, question string) Action {
	if g := r.st.Gates[gid]; g != nil {
		switch g.Result {
		case "answered_yes":
			return nil
		case "answered_no":
			return act("stopped", "reason", "owner answered no to "+gid)
		}
	}
	r.st.Gates[gid] = &GateState{Result: "asked", At: now(), Detail: question}
	if r.st.Status != "stopped" {
		r.st.Status = "blocked"
	}
	r.st.BlockedOn = sp(gid)
	return act("ask", "gate", gid, "question", question,
		"record", "expspec answer <spec> --gate "+gid+" --yes|--no --by <owner>")
}

func (r *runner) escalation(gid, reason, question string) Action {
	a := r.humanGate(gid, question)
	if a != nil && a["action"] == "ask" {
		a["action"] = "escalate"
		a["reason"] = reason
	}
	return a
}

func (r *runner) stepOf(inst *Instance) *Step { return r.spec.step(inst.Step) }

func (r *runner) runsOnHost(inst *Instance) bool {
	s := r.stepOf(inst)
	return s != nil && (s.RunsOn == "host" || s.Scope == "per_host")
}

func (r *runner) ensureInstances() {
	hidx := r.st.hostIndex()
	for i := range r.spec.Steps {
		s := &r.spec.Steps[i]
		type k struct {
			key  string
			cell *string
			host *int
		}
		var keys []k
		switch s.Scope {
		case "once":
			keys = []k{{instanceKey(s.ID, "", -1), nil, nil}}
		case "per_cell":
			for _, c := range cellsForStep(s, r.cells) {
				keys = append(keys, k{instanceKey(s.ID, c.ID, -1), sp(c.ID), nil})
			}
		default:
			if hidx >= 0 {
				keys = []k{{instanceKey(s.ID, "", hidx), nil, ip(hidx)}}
			}
		}
		for _, x := range keys {
			if _, ok := r.st.Instances[x.key]; !ok {
				r.st.Instances[x.key] = &Instance{Step: s.ID, Cell: x.cell, Host: x.host, Status: "pending", Attempts: []Attempt{}}
			}
		}
	}
}

func (r *runner) needsMet(s *Step, cell *string, hidx int) bool {
	for _, n := range s.Needs {
		dep := r.spec.step(n)
		var keys []string
		switch dep.Scope {
		case "per_cell":
			if cell != nil {
				keys = []string{instanceKey(n, *cell, -1)}
			} else {
				for k, i := range r.st.Instances {
					if i.Step == n && !terminal[i.Status] {
						return false
					}
					_ = k
				}
				continue
			}
		case "per_host":
			keys = []string{instanceKey(n, "", hidx)}
		default:
			keys = []string{instanceKey(n, "", -1)}
		}
		for _, k := range keys {
			i := r.st.Instances[k]
			if i == nil || !okDone[i.Status] {
				return false
			}
		}
	}
	return true
}

func (r *runner) teardownOr(fallback Action) (Action, error) {
	for hidx, h := range r.st.Hosts {
		if h.State != "up" && h.State != "provisioning" {
			continue
		}
		host := r.spec.host(h.Candidate)
		if host.Backend != "batch" {
			h.State, h.EndedAt = "torn_down", sp(now())
			continue
		}
		handle := strOr(h.Handle, "none")
		switch r.spec.Agent.permission("terminate_own_hosts") {
		case "never":
			return act("escalate", "reason", fmt.Sprintf("host %s (handle %s) needs teardown and terminate_own_hosts is never",
				h.Candidate, handle)), nil
		case "ask":
			if a := r.humanGate(fmt.Sprintf("teardown-%d", hidx),
				fmt.Sprintf("Terminate own host %s (handle %s).", h.Candidate, handle)); a != nil {
				if a["action"] == "stopped" {
					return act("escalate", "reason", fmt.Sprintf("teardown declined for %s; handle %s remains active",
						h.Candidate, handle)), nil
				}
				return a, nil
			}
		}
		key := fmt.Sprintf("_teardown#%d", hidx)
		if prev := r.st.Instances[key]; prev != nil {
			switch prev.Status {
			case "carded":
				card := prev.Attempts[len(prev.Attempts)-1].Card
				return runCardAction(key, card, host.Lifecycle.Teardown.TimeoutS > 600, false), nil
			case "failed":
				return act("escalate", "reason", fmt.Sprintf("teardown of %s failed; terminate handle %s by hand",
					h.Candidate, handle)), nil
			}
		}
		return r.lifecycleCard(hidx, host, "teardown")
	}
	return fallback, nil
}

func (r *runner) lifecycleCard(hidx int, host *Host, phase string) (Action, error) {
	key := fmt.Sprintf("_%s#%d", phase, hidx)
	if phase == "poll" {
		n := 0
		for k := range r.st.Instances {
			if strings.HasPrefix(k, fmt.Sprintf("_poll#%d.", hidx)) {
				n++
			}
		}
		key = fmt.Sprintf("_poll#%d.%d", hidx, n+1)
	}
	r.st.Instances[key] = &Instance{Step: "_" + phase, Host: ip(hidx), Status: "pending", Attempts: []Attempt{}}
	ctx := context(r.spec, r.paths, nil, host, hidx, r.st.Vars, false)
	c, err := lifecycleCommand(host, phase, ctx)
	if err != nil {
		return nil, err
	}
	card, err := writeCard(r.spec, r.st, r.paths, key, c)
	if err != nil {
		return nil, err
	}
	if r.st.Status != "stopped" {
		r.st.Status = "running"
	}
	return runCardAction(key, card, c.background, false), nil
}

func (r *runner) chooseHost(waiting []*Instance) *Host {
	bad := map[string]bool{}
	for _, h := range r.st.Hosts {
		if h.State == "unavailable" {
			bad[h.Candidate] = true
		}
	}
	var want []*Cell
	for _, i := range waiting {
		if i.Cell != nil {
			want = append(want, r.cellsByID[*i.Cell])
		}
	}
	sort.SliceStable(want, func(a, b int) bool { return want[a].Priority > want[b].Priority })
	noPaid := r.spec.Agent.permission("launch_paid_host") == "never"
	var ok []*Host
	for i := range r.spec.Hosts.Candidates {
		c := &r.spec.Hosts.Candidates[i]
		if bad[c.ID] || (noPaid && c.paid() && c.Backend != "local") {
			continue
		}
		if len(want) > 0 && !cellFitsHost(want[0], c) {
			continue
		}
		ok = append(ok, c)
	}
	if len(ok) == 0 {
		return nil
	}
	if r.spec.Hosts.Selection == "cheapest_fit" {
		sort.SliceStable(ok, func(a, b int) bool { return ok[a].EstHourlyUSD < ok[b].EstHourlyUSD })
	}
	return ok[0]
}

func (r *runner) missingArtifacts() []string {
	done := map[string]bool{}
	for _, i := range r.st.Instances {
		if i.Cell != nil && okDone[i.Status] {
			done[*i.Cell] = true
		}
	}
	var out []string
	for _, art := range r.spec.Reporting.RequiredArtifacts {
		perCell := strings.Contains(strings.ReplaceAll(art, " ", ""), "{{cell.")
		var targets []*Cell
		if perCell {
			for _, id := range sortedKeys(done) {
				targets = append(targets, r.cellsByID[id])
			}
		} else {
			targets = []*Cell{nil}
		}
		for _, c := range targets {
			p, _ := render(art, context(r.spec, r.paths, c, nil, -1, r.st.Vars, false))
			if _, err := os.Stat(repoPath(p)); err != nil {
				out = append(out, p)
			}
		}
	}
	return out
}

func (r *runner) summary() map[string]int {
	out := map[string]int{}
	for _, i := range r.st.Instances {
		if !strings.HasPrefix(i.Step, "_") {
			out[i.Status]++
		}
	}
	return out
}

func shortHash(s string) string {
	h := sha256.Sum256([]byte(s))
	return hex.EncodeToString(h[:])[:10]
}

func (r *runner) step() (Action, error) {
	r.depth++
	if r.depth > 50 {
		return act("escalate", "reason", "internal: next did not settle"), nil
	}
	spec, st := r.spec, r.st
	problems := ingestPending(spec, st, r.paths, r.cellsByID)
	if st.Status == "stopped" {
		return act("stopped", "reason", strOr(st.BlockedOn, "")), nil
	}
	for _, p := range problems {
		if a := r.escalation("esc-"+shortHash(p), p,
			"ESCALATION: "+p+". Answer yes to continue past it, no to stop the experiment."); a != nil {
			return a, nil
		}
	}

	// Complete after-gates before scheduling dependent work. A receipt says
	// the command finished; it does not answer a human gate.
	for _, key := range sortedInstanceKeys(st) {
		inst := st.Instances[key]
		s := r.stepOf(inst)
		if !okDone[inst.Status] || s == nil || s.Gates == nil || len(s.Gates.After) == 0 {
			continue
		}
		var cell *Cell
		if inst.Cell != nil {
			cell = r.cellsByID[*inst.Cell]
		}
		hidx, host := -1, (*Host)(nil)
		if inst.Host != nil {
			hidx = *inst.Host
			host = spec.host(st.Hosts[hidx].Candidate)
		}
		ctx := context(spec, r.paths, cell, host, hidx, st.Vars, false)
		if a, err := r.evaluateGates(s.Gates.After, "after", key, inst, ctx); a != nil || err != nil {
			return a, err
		}
	}

	for _, key := range sortedInstanceKeys(st) {
		inst := st.Instances[key]
		if inst.Status == "carded" {
			att := inst.Attempts[len(inst.Attempts)-1]
			var card Card
			raw, _ := os.ReadFile(repoPath(att.Card))
			_ = yaml.Unmarshal(raw, &card)
			s := r.stepOf(inst)
			return runCardAction(key, att.Card, card.Background, s != nil && s.Evidence), nil
		}
	}

	// Budget.
	used, capUSD := spend(spec, st), spec.Budget.MaxCostUSD
	if used > capUSD || (capUSD > 0 && used >= capUSD) {
		return act("stopped", "reason", fmt.Sprintf("budget cap reached: $%g of $%g", used, capUSD)), nil
	}
	frac := spec.Budget.EscalateAtFraction
	if frac == 0 {
		frac = 0.8
	}
	if capUSD > 0 && used >= frac*capUSD {
		if a := r.humanGate(fmt.Sprintf("budget-%d", int(frac*100)),
			fmt.Sprintf("Spend estimate $%g has passed %d%% of the $%g budget. Continue.", used, int(frac*100), capUSD)); a != nil {
			return a, nil
		}
	}
	var first *string
	for _, h := range st.Hosts {
		if h.StartedAt != nil && (first == nil || parseTS(*h.StartedAt).Before(parseTS(*first))) {
			first = h.StartedAt
		}
	}
	if first != nil {
		if hours := clock().Sub(parseTS(*first)).Hours(); hours >= spec.Budget.MaxWallHours {
			return act("stopped", "reason", fmt.Sprintf("max_wall_hours reached (%.1f h)", hours)), nil
		}
	}

	hidx := st.hostIndex()
	var host *Host
	if hidx >= 0 {
		host = spec.host(st.Hosts[hidx].Candidate)
	}
	r.ensureInstances()

	// Cells no remaining candidate can ever run.
	unavailable := map[string]bool{}
	for _, h := range st.Hosts {
		if h.State == "unavailable" {
			unavailable[h.Candidate] = true
		}
	}
	noPaid := spec.Agent.permission("launch_paid_host") == "never"
	for i := range r.cells {
		c := &r.cells[i]
		ok := false
		for j := range spec.Hosts.Candidates {
			h := &spec.Hosts.Candidates[j]
			if unavailable[h.ID] || (noPaid && h.paid() && h.Backend != "local") {
				continue
			}
			ok = ok || cellFitsHost(c, h)
		}
		if !ok {
			markCell(st, c.ID, "not_attempted", "no available host candidate fits this cell")
		}
	}
	cellOKHere := func(cell *string) bool {
		return cell == nil || host == nil || cellFitsHost(r.cellsByID[*cell], host)
	}

	remoteRunning := false
	for si := range spec.Steps {
		s := &spec.Steps[si]
		var keys []string
		for _, k := range sortedInstanceKeys(st) {
			i := st.Instances[k]
			if i.Step != s.ID {
				continue
			}
			if s.Scope == "per_host" && (i.Host == nil || *i.Host != hidx) {
				continue
			}
			keys = append(keys, k)
		}
		if s.Scope == "per_cell" {
			order := map[string]int{}
			for n, c := range cellsForStep(s, r.cells) {
				order[c.ID] = n
			}
			sort.SliceStable(keys, func(a, b int) bool {
				return order[*st.Instances[keys[a]].Cell] < order[*st.Instances[keys[b]].Cell]
			})
		}
		for _, key := range keys {
			inst := st.Instances[key]
			if inst.Status == "running_remote" {
				remoteRunning = true
				continue
			}
			if inst.Status != "pending" {
				continue
			}
			if s.RunsOn == "host" && (host == nil || !cellOKHere(inst.Cell)) {
				continue
			}
			if !r.needsMet(s, inst.Cell, hidx) {
				continue
			}
			var cell *Cell
			if inst.Cell != nil {
				cell = r.cellsByID[*inst.Cell]
			}
			ctx := context(spec, r.paths, cell, host, hidx, st.Vars, false)
			if s.Gates != nil {
				if a, err := r.evaluateGates(s.Gates.Before, "before", key, inst, ctx); a != nil || err != nil {
					return a, err
				}
			}
			if inst.Status != "pending" {
				continue
			}
			if len(s.DoneWhen) > 0 {
				all := true
				for _, c := range s.DoneWhen {
					if ok, _ := runCheck(c, ctx, r.paths); !ok {
						all = false
					}
				}
				if all {
					inst.Status, inst.Reason = "skipped_done", sp("done_when passed")
					st.event("skip", key+": done_when passed")
					continue
				}
			}
			if s.RunsOn == "host" && host.Backend == "batch" {
				inst.Status = "running_remote"
				remoteRunning = true
				continue
			}
			c, err := stepCommand(spec, s, ctx, host)
			if err != nil {
				return nil, err
			}
			card, err := writeCard(spec, st, r.paths, key, c)
			if err != nil {
				return nil, err
			}
			if hidx >= 0 {
				inst.Host = ip(hidx)
			}
			st.Status, st.BlockedOn = "running", nil
			return runCardAction(key, card, c.background, s.Evidence), nil
		}
	}

	if host != nil && host.Backend == "batch" {
		hs := st.Hosts[hidx]
		if remoteRunning {
			raw, _ := os.ReadFile(repoPath(r.paths["collected"] + "/STATUS"))
			status := string(raw)
			for _, key := range sortedInstanceKeys(st) {
				inst := st.Instances[key]
				if inst.Status != "running_remote" {
					continue
				}
				s := r.stepOf(inst)
				var cell *Cell
				if inst.Cell != nil {
					cell = r.cellsByID[*inst.Cell]
				}
				ctx := context(spec, r.paths, cell, host, hidx, st.Vars, false)
				done, _ := render(s.StatusDone, ctx)
				if statusHas(status, done) {
					inst.Status = "done"
					continue
				}
				for _, t := range s.StatusFailed {
					if tok, _ := render(t, ctx); statusHas(status, tok) {
						inst.Status, inst.Reason = "failed", sp("STATUS reports failure: "+tok)
						if p := applyOnFailure(st, s, inst, key); p != "" {
							if a := r.escalation("esc-"+shortHash(p), p,
								"ESCALATION: "+p+". Answer yes to continue past it, no to stop the experiment."); a != nil {
								return a, nil
							}
						}
						break
					}
				}
			}
			still := false
			for _, i := range st.Instances {
				still = still || i.Status == "running_remote"
			}
			if still {
				every := host.Lifecycle.PollEveryS
				if every == 0 {
					every = 1800
				}
				last := hs.LastPollAt
				if last == nil {
					last = hs.StartedAt
				}
				if last != nil && clock().Sub(parseTS(*last)).Seconds() < float64(every) {
					st.Status = "waiting"
					return act("wait", "until", after(every, last), "reason", "remote steps running on "+host.ID), nil
				}
				return r.lifecycleCard(hidx, host, "poll")
			}
			return r.step()
		}
		pendingHere := false
		for _, i := range st.Instances {
			if i.Status == "pending" && r.runsOnHost(i) && cellOKHere(i.Cell) {
				pendingHere = true
			}
		}
		if !pendingHere {
			if c := st.Instances[fmt.Sprintf("_collect#%d", hidx)]; c == nil || (!okDone[c.Status] && c.Status != "failed") {
				return r.lifecycleCard(hidx, host, "collect")
			}
			a, err := r.teardownOr(nil)
			if a != nil || err != nil {
				return a, err
			}
			return r.step()
		}
	} else if host != nil {
		pendingHere := false
		for _, i := range st.Instances {
			if i.Status == "pending" && r.runsOnHost(i) && cellOKHere(i.Cell) {
				pendingHere = true
			}
		}
		if !pendingHere {
			st.Hosts[hidx].State, st.Hosts[hidx].EndedAt = "torn_down", sp(now())
			st.event("host", "released "+host.ID)
			return r.step()
		}
	}

	var waiting []*Instance
	for _, k := range sortedInstanceKeys(st) {
		i := st.Instances[k]
		if i.Status == "pending" && r.runsOnHost(i) {
			waiting = append(waiting, i)
		}
	}
	if host == nil && len(waiting) > 0 {
		cand := r.chooseHost(waiting)
		if cand == nil {
			for _, i := range waiting {
				i.Status, i.Reason = "not_attempted", sp("no host candidate available")
			}
			return r.step()
		}
		if cand.paid() && cand.Backend != "local" {
			life := 0.0
			if cand.Lifecycle != nil {
				life = float64(cand.Lifecycle.LifetimeS)
			}
			projected := cand.EstHourlyUSD * life / 3600
			if spec.Agent.permission("launch_paid_host") == "ask" || projected > spec.Agent.spendWithoutAsking() {
				what := cand.Note
				if what == "" {
					what = cand.Backend
				}
				if a := r.humanGate(fmt.Sprintf("launch-%s-%d", cand.ID, len(st.Hosts)),
					fmt.Sprintf("Launch host %s (%s) at about $%g/h, lifetime cap %d h (up to $%.2f). Spent so far $%g of $%g.",
						cand.ID, what, cand.EstHourlyUSD, int(life/3600), projected, used, capUSD)); a != nil {
					return a, nil
				}
			}
		}
		st.Hosts = append(st.Hosts, &HostState{Candidate: cand.ID, State: "provisioning", HourlyUSD: cand.EstHourlyUSD,
			StartedAt: sp(now())})
		hidx = len(st.Hosts) - 1
		st.event("host", "selected "+cand.ID)
		if cand.Backend == "local" || cand.Backend == "ssh" {
			st.Hosts[hidx].State = "up"
			return r.step()
		}
		return r.lifecycleCard(hidx, cand, "provision")
	}

	allTerminal := true
	var blocked []string
	for _, k := range sortedInstanceKeys(st) {
		i := st.Instances[k]
		if !terminal[i.Status] {
			allTerminal = false
			if i.Status == "pending" {
				blocked = append(blocked, k)
			}
		}
	}
	if allTerminal {
		if missing := r.missingArtifacts(); len(missing) > 0 && st.Status != "completed" {
			if len(missing) > 8 {
				missing = missing[:8]
			}
			if a := r.escalation("esc-missing-artifacts", "missing_output: "+strings.Join(missing, ", "),
				"ESCALATION: required artifacts missing: "+strings.Join(missing, ", ")+
					". Answer yes to mark the run completed anyway, no to stop."); a != nil {
				return a, nil
			}
		}
		st.Status, st.BlockedOn = "completed", nil
		return act("done", "summary", r.summary(),
			"next", "Reasoning tier: expspec facts <spec>, then apply outcome.decision_rules and write reporting.results_file."), nil
	}
	if len(blocked) > 10 {
		blocked = blocked[:10]
	}
	return act("escalate", "reason", fmt.Sprintf("no runnable instance; pending with unmet needs: %v", blocked)), nil
}

// evaluateGates applies the same human, polling and failure rules on either
// side of a step. A non-nil action goes back to the caller; nil with the
// instance status unchanged means every gate passed.
func (r *runner) evaluateGates(ids []string, side, key string, inst *Instance, ctx ctxMap) (Action, error) {
	before := inst.Status
	for _, gid := range ids {
		a, err := r.gateOne(r.spec.gate(gid), side, key, inst, ctx)
		if a != nil || err != nil {
			return a, err
		}
		if inst.Status != before {
			break
		}
	}
	return nil, nil
}

// gateOne evaluates one gate for one instance.
func (r *runner) gateOne(g *Gate, side, key string, inst *Instance, ctx ctxMap) (Action, error) {
	st := r.st
	gkey := g.ID + ":" + key
	if g.Kind == "human" {
		q, _ := render(g.Ask, ctx)
		return r.humanGate(gkey, q), nil
	}
	prior := st.Gates[gkey]
	if prior != nil && prior.Result == "pass" {
		return nil, nil
	}
	passed, detail := allChecks(g, ctx, r.paths)
	firstAt := now()
	if prior != nil && prior.FirstAt != "" {
		firstAt = prior.FirstAt
	}
	if passed {
		st.Gates[gkey] = &GateState{Result: "pass", FirstAt: firstAt, At: now(), Detail: detail}
		st.event("gate", gkey+" pass")
		return nil, nil
	}
	if g.Kind == "wait_until" && clock().Sub(parseTS(firstAt)).Seconds() < float64(g.MaxWaitS) {
		st.Gates[gkey] = &GateState{Result: "waiting", FirstAt: firstAt, At: now(), Detail: detail}
		if len(g.Refresh) > 0 {
			rkey := "_refresh-" + g.ID
			var last *string
			if ri := st.Instances[rkey]; ri != nil && len(ri.Attempts) > 0 {
				last = &ri.Attempts[len(ri.Attempts)-1].CardedAt
			}
			if last == nil || clock().Sub(parseTS(*last)).Seconds() >= float64(g.EveryS) {
				if st.Instances[rkey] == nil {
					st.Instances[rkey] = &Instance{Step: rkey, Status: "pending", Attempts: []Attempt{}}
				}
				argv, err := renderAll(g.Refresh, ctx)
				if err != nil {
					return nil, err
				}
				card, err := writeCard(r.spec, st, r.paths, rkey, cmdSpec{CardCommand: CardCommand{Argv: argv,
					TimeoutSeconds: 600, SuccessExitCodes: []int{0}}})
				if err != nil {
					return nil, err
				}
				return runCardAction(rkey, card, false, false), nil
			}
		}
		st.Status = "waiting"
		return act("wait", "until", after(g.EveryS, nil), "reason", "gate "+g.ID+": "+detail), nil
	}
	st.Gates[gkey] = &GateState{Result: "fail", FirstAt: firstAt, At: now(), Detail: detail}
	msg := applyFail(st, g, inst, fmt.Sprintf("gate %s %s %s: %s", g.ID, side, key, detail))
	if strings.HasPrefix(msg, "stopped") {
		return act("stopped", "reason", msg), nil
	}
	if msg != "" {
		if a := r.escalation("esc-"+gkey, msg, "ESCALATION: "+msg+". Answer yes to skip this instance and continue, no to stop."); a != nil {
			return a, nil
		}
		inst.Status, inst.Reason = "not_attempted", sp(msg)
	}
	return nil, nil
}

// next is one `expspec next`: whatever step decides, a stop is recorded and
// every host still up is torn down before the stop is reported.
func (r *runner) next() (Action, error) {
	a, err := r.step()
	if err != nil || a == nil || a["action"] != "stopped" {
		return a, err
	}
	r.st.Status, r.st.BlockedOn = "stopped", sp(fmt.Sprint(a["reason"]))
	return r.teardownOr(a)
}
