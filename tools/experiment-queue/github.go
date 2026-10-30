package main

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"sort"
	"strings"
	"time"
)

type githubError struct {
	Status int
	Text   string
}

func (e githubError) Error() string { return fmt.Sprintf("GitHub HTTP %d: %s", e.Status, e.Text) }

type GitHub struct {
	Base, Repo, Token string
	Client            *http.Client
}

func (g *GitHub) api(method, path string, body, out any) error {
	var input io.Reader
	if body != nil {
		b, err := json.Marshal(body)
		if err != nil {
			return err
		}
		input = bytes.NewReader(b)
	}
	req, err := http.NewRequest(method, strings.TrimSuffix(g.Base, "/")+"/repos/"+g.Repo+"/"+path, input)
	if err != nil {
		return err
	}
	req.Header.Set("Accept", "application/vnd.github+json")
	req.Header.Set("X-GitHub-Api-Version", "2022-11-28")
	req.Header.Set("Content-Type", "application/json")
	if g.Token != "" {
		req.Header.Set("Authorization", "Bearer "+g.Token)
	}
	client := g.Client
	if client == nil {
		client = &http.Client{Timeout: 30 * time.Second}
	}
	res, err := client.Do(req)
	if err != nil {
		return err
	}
	defer res.Body.Close()
	b, err := io.ReadAll(io.LimitReader(res.Body, 16<<20))
	if err != nil {
		return err
	}
	if res.StatusCode < 200 || res.StatusCode >= 300 {
		if len(b) > 2048 {
			b = b[:2048]
		}
		return githubError{res.StatusCode, string(b)}
	}
	if out != nil {
		return json.Unmarshal(b, out)
	}
	return nil
}
func httpStatus(err error, n int) bool { e, ok := err.(githubError); return ok && e.Status == n }

type gitRef struct {
	Ref    string                     `json:"ref"`
	Object struct{ SHA, Type string } `json:"object"`
}

func (g *GitHub) content(path, ref string) ([]byte, error) {
	if !relpath(path) {
		return nil, fmt.Errorf("invalid content path")
	}
	var v struct{ Content, Encoding string }
	parts := strings.Split(path, "/")
	for i := range parts {
		parts[i] = url.PathEscape(parts[i])
	}
	if err := g.api("GET", "contents/"+strings.Join(parts, "/")+"?ref="+url.QueryEscape(ref), nil, &v); err != nil {
		return nil, err
	}
	if v.Encoding != "base64" {
		return nil, fmt.Errorf("unsupported GitHub content encoding")
	}
	return base64.StdEncoding.DecodeString(strings.ReplaceAll(v.Content, "\n", ""))
}
func resultBranch(key string) string { return "automation/isolab-results-" + key }

// New work gets a submission slot first. Rotate existing claims hourly so one
// unresolved submission cannot indefinitely starve later receipt collection.
func selectionOrder(candidates []Candidate, claims map[string]gitRef, hour int64) []Candidate {
	fresh, existing := []Candidate{}, []Candidate{}
	for _, c := range candidates {
		if _, ok := claims[c.Key]; ok {
			existing = append(existing, c)
		} else {
			fresh = append(fresh, c)
		}
	}
	sort.Slice(existing, func(i, j int) bool { return existing[i].Key < existing[j].Key })
	if len(existing) > 0 {
		i := int(hour % int64(len(existing)))
		existing = append(existing[i:], existing[:i]...)
	}
	return append(fresh, existing...)
}
func (g *GitHub) restoreCandidate(ref gitRef) (Candidate, error) {
	parts := strings.Split(strings.TrimPrefix(ref.Ref, "refs/tags/isolab-queue/"), "/")
	if len(parts) != 2 || !identifier.MatchString(parts[0]) || !hex64.MatchString(parts[1]) || ref.Object.Type != "commit" || !hex40.MatchString(ref.Object.SHA) {
		return Candidate{}, fmt.Errorf("invalid permanent claim ref")
	}
	path := "experiments/queue/" + parts[0] + ".json"
	b, err := g.content(path, ref.Object.SHA)
	if err != nil {
		return Candidate{}, err
	}
	var e Entry
	if err = decode(b, &e); err != nil {
		return Candidate{}, err
	}
	if e.ID != parts[0] || e.key() != parts[1] {
		return Candidate{}, fmt.Errorf("claim/registration identity mismatch")
	}
	pb, err := g.content(e.Protocol.Path, ref.Object.SHA)
	if err != nil {
		return Candidate{}, err
	}
	id := strings.ReplaceAll(strings.ToLower(e.ID), "_", "-")
	if len(id) > 26 {
		id = id[:26]
	}
	c := Candidate{Entry: e, Key: parts[1], JobName: "q-" + id + "-" + parts[1][:20], RegistryPath: path, ControllerCommit: ref.Object.SHA, Protocol: pb, CollectOnly: true}
	if e.Kind == "benchmark" {
		c.Spec, err = g.content(e.WorkloadSpec, e.SourceCommit)
	}
	return c, err
}

// A Git ref is an atomic permanent execution fence, independent of Kubernetes
// Job TTL and CodeBuild's short idempotency window. Existing claims only collect.
func (g *GitHub) reserve(p Plan, max int, dry bool) (Plan, error) {
	var refs []gitRef
	if err := g.api("GET", "git/matching-refs/tags/isolab-queue/", nil, &refs); err != nil && !httpStatus(err, 404) {
		return p, err
	}
	byKey := map[string]gitRef{}
	for _, r := range refs {
		parts := strings.Split(r.Ref, "/")
		if len(parts) > 0 {
			byKey[parts[len(parts)-1]] = r
		}
	}
	current := map[string]bool{}
	for _, c := range p.Candidates {
		current[c.Key] = true
	}
	for _, r := range refs {
		parts := strings.Split(r.Ref, "/")
		key := parts[len(parts)-1]
		if !current[key] {
			c, err := g.restoreCandidate(r)
			if err != nil {
				p.Blocked = append(p.Blocked, Blocked{r.Ref, "claim reconciliation: " + err.Error()})
				continue
			}
			p.Candidates = append(p.Candidates, c)
		}
	}
	selected := []Candidate{}
	for _, c := range selectionOrder(p.Candidates, byKey, time.Now().UTC().Unix()/3600) {
		var branch gitRef
		err := g.api("GET", "git/ref/heads/"+resultBranch(c.Key), nil, &branch)
		if err == nil {
			var prs []Publication
			if err = g.api("GET", "pulls?state=all&head="+url.QueryEscape(strings.Split(g.Repo, "/")[0]+":"+resultBranch(c.Key)), nil, &prs); err != nil {
				return p, err
			}
			if len(prs) > 0 {
				p.Blocked = append(p.Blocked, Blocked{c.RegistryPath, "results already returned on " + resultBranch(c.Key)})
				continue
			}
			// A branch without a PR means publication was interrupted. Collect
			// the existing execution and finish publication, never relaunch it.
			c.CollectOnly = true
		}
		if err != nil && !httpStatus(err, 404) {
			return p, err
		}
		if len(selected) >= max {
			p.Blocked = append(p.Blocked, Blocked{c.RegistryPath, "per-invocation execution/collection limit"})
			continue
		}
		if prior, ok := byKey[c.Key]; ok {
			var err error
			c, err = g.restoreCandidate(prior)
			if err != nil {
				return p, err
			}
		} else if !dry {
			err = g.api("POST", "git/refs", map[string]string{"ref": "refs/tags/isolab-queue/" + c.Entry.ID + "/" + c.Key, "sha": p.SourceCommit}, nil)
			if err != nil {
				if !httpStatus(err, 422) {
					return p, err
				}
				var prior gitRef
				if err = g.api("GET", "git/ref/tags/isolab-queue/"+c.Entry.ID+"/"+c.Key, nil, &prior); err != nil {
					return p, err
				}
				c, err = g.restoreCandidate(prior)
				if err != nil {
					return p, err
				}
			}
		}
		selected = append(selected, c)
	}
	p.Candidates = selected
	p.Claimed = !dry
	return p, nil
}

type ReturnedFile struct {
	Path   string `json:"path"`
	SHA256 string `json:"sha256"`
	Data   []byte `json:"data"`
}
type Result struct {
	Candidate     Candidate       `json:"candidate"`
	State         string          `json:"state"`
	Error         string          `json:"error,omitempty"`
	Receipt       json.RawMessage `json:"receipt,omitempty"`
	Files         []ReturnedFile  `json:"files,omitempty"`
	RemoteOutputs []RemoteOutput  `json:"remote_outputs,omitempty"`
}
type RemoteOutput struct {
	Path   string `json:"path"`
	URI    string `json:"uri"`
	SHA256 string `json:"sha256"`
	Bytes  int    `json:"bytes"`
}
type Publication struct {
	Number int    `json:"number"`
	URL    string `json:"html_url"`
}

func (g *GitHub) publish(r Result) (Publication, error) {
	c := r.Candidate
	e := c.Entry
	if !identifier.MatchString(e.ID) || !hex64.MatchString(c.Key) || e.key() != c.Key {
		return Publication{}, fmt.Errorf("invalid result identity")
	}
	if r.State == "pending" || r.State == "submission_unknown" {
		return Publication{}, nil
	}
	branch := resultBranch(c.Key)
	var existing gitRef
	err := g.api("GET", "git/ref/heads/"+branch, nil, &existing)
	if err != nil && !httpStatus(err, 404) {
		return Publication{}, err
	}
	if httpStatus(err, 404) {
		var base gitRef
		if err = g.api("GET", "git/ref/heads/main", nil, &base); err != nil {
			return Publication{}, err
		}
		var commit struct{ Tree struct{ SHA string } }
		if err = g.api("GET", "git/commits/"+base.Object.SHA, nil, &commit); err != nil {
			return Publication{}, err
		}
		prefix := "research/experiment_queue/results/" + e.ID + "/" + c.Key + "/"
		files := map[string][]byte{}
		var provider map[string]any
		_ = json.Unmarshal(r.Receipt, &provider)
		files["README.md"] = []byte(fmt.Sprintf("# %s: %s\n\nQueue key: `%s`. Tested source: `%s`. Image: `%s`.\n\nThese are execution observations, not an automatic scientific verdict.\n\n%s\n\nPrivate launcher receipts: `%v`. Hashed raw/output locations are in receipt.json; fetch private session files before independent replay.\n", e.ID, r.State, c.Key, e.SourceCommit, e.Image.URI, r.Error, provider["receipt_uri"]))
		allowed := map[string]bool{}
		for _, p := range e.Outputs {
			allowed[p] = true
		}
		for _, f := range r.Files {
			if !allowed[f.Path] || !relpath(f.Path) || digest(f.Data) != f.SHA256 {
				return Publication{}, fmt.Errorf("unapproved or mismatched returned file")
			}
			limit := 64 << 10
			if strings.HasSuffix(f.Path, ".pdf") {
				limit = 10 << 20
			}
			if len(f.Data) > limit {
				return Publication{}, fmt.Errorf("returned file belongs in object storage")
			}
			if f.Path == "receipt.json" || f.Path == "README.md" {
				return Publication{}, fmt.Errorf("reserved result filename")
			}
			files["outputs/"+f.Path] = f.Data
		}
		// Keep the full request/protocol in private S3. Git gets compact receipts.
		manifest := []map[string]any{}
		for _, f := range r.Files {
			manifest = append(manifest, map[string]any{"path": f.Path, "sha256": f.SHA256, "bytes": len(f.Data)})
		}
		meta, err := json.MarshalIndent(map[string]any{"schema": "crypto.isolab-result-record/v1", "id": e.ID, "key": c.Key, "state": r.State, "error": r.Error, "source_commit": e.SourceCommit, "controller_commit": c.ControllerCommit, "image": e.Image, "protocol": e.Protocol, "job_receipt": r.Receipt, "returned_files": manifest, "remote_outputs": r.RemoteOutputs}, "", "  ")
		if err != nil {
			return Publication{}, err
		}
		if len(meta) > 64<<10 {
			return Publication{}, fmt.Errorf("compact receipt exceeds Git data policy")
		}
		files["receipt.json"] = append(meta, '\n')
		tree := []map[string]string{}
		for p, b := range files {
			var blob struct{ SHA string }
			if err = g.api("POST", "git/blobs", map[string]string{"encoding": "base64", "content": base64.StdEncoding.EncodeToString(b)}, &blob); err != nil {
				return Publication{}, err
			}
			tree = append(tree, map[string]string{"path": prefix + p, "mode": "100644", "type": "blob", "sha": blob.SHA})
		}
		var t struct{ SHA string }
		if err = g.api("POST", "git/trees", map[string]any{"base_tree": commit.Tree.SHA, "tree": tree}, &t); err != nil {
			return Publication{}, err
		}
		var co struct{ SHA string }
		if err = g.api("POST", "git/commits", map[string]any{"message": "Record Isolab execution " + e.ID + " (" + r.State + ")", "tree": t.SHA, "parents": []string{base.Object.SHA}}, &co); err != nil {
			return Publication{}, err
		}
		if err = g.api("POST", "git/refs", map[string]string{"ref": "refs/heads/" + branch, "sha": co.SHA}, nil); err != nil && !httpStatus(err, 422) {
			return Publication{}, err
		}
	}
	var prs []Publication
	if err = g.api("GET", "pulls?state=all&head="+url.QueryEscape(strings.Split(g.Repo, "/")[0]+":"+branch), nil, &prs); err != nil {
		return Publication{}, err
	}
	if len(prs) > 0 {
		return prs[0], nil
	}
	var pr Publication
	err = g.api("POST", "pulls", map[string]any{"title": "[experiments] " + e.ID + ": " + r.State, "head": branch, "base": "main", "draft": false, "body": "Returns the immutable Isolab execution records for `" + e.ID + "`.\n\nState: **" + r.State + "**. Tested source: `" + e.SourceCommit + "`. Queue key: `" + c.Key + "`.\n\nAll outputs remain observations pending the experiment's independent review and decision rules. Timeouts and failures are retained; no input protocol, leaderboard or scientific claim is changed.\n\n" + r.Error}, &pr)
	return pr, err
}
