package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"strings"
	"time"
)

func main() {
	if err := mainErr(os.Args[1:]); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
func mainErr(args []string) error {
	if len(args) == 0 {
		return fmt.Errorf("usage: experiment-queue plan|run|publish [flags]")
	}
	flags := flag.NewFlagSet(args[0], flag.ContinueOnError)
	root := flags.String("root", ".", "repository root")
	schemas := flags.String("schemas", "schemas", "canonical protocol/export schema directory")
	revision := flags.String("revision", os.Getenv("GITHUB_SHA"), "trusted main commit")
	repo := flags.String("repo", os.Getenv("GITHUB_REPOSITORY"), "owner/repository")
	file := flags.String("file", "queue-plan.json", "plan/results input")
	out := flags.String("out", "queue-results.json", "output JSON")
	max := flags.Int("max-jobs", 1, "maximum jobs/collections, 1..4")
	dry := flags.Bool("dry-run", false, "discover without permanent claims or AWS")
	toolDir := flags.String("tool-dir", "tools/experiment-queue", "trusted controller scripts")
	wait := flags.Duration("wait", 75*time.Minute, "bounded wait for a new execution; existing claims only collect")
	requestFile := flags.String("request", "", "immutable dispatch request for native metadata export")
	session := flags.String("session", "", "native ecbench session directory")
	uri := flags.String("uri", "", "exact private result prefix")
	admitted := flags.Bool("admitted", false, "native benchmark replay/L2/A-A admission passed")
	if err := flags.Parse(args[1:]); err != nil {
		return err
	}
	if args[0] == "export-ecbench" {
		return exportEcbench(*requestFile, *session, *out, *admitted)
	}
	if args[0] == "fetch-verified" {
		x := Execution{AWS: AWSCLI{}}
		return x.fetchManifest(context.Background(), *uri, *out)
	}
	base := os.Getenv("GITHUB_API_URL")
	if base == "" {
		base = "https://api.github.com"
	}
	g := GitHub{Base: base, Repo: *repo, Token: os.Getenv("GH_TOKEN")}
	if !strings.Contains(*repo, "/") {
		return fmt.Errorf("repository owner/name is required")
	}
	switch args[0] {
	case "plan":
		if !hex40.MatchString(*revision) || *max < 1 || *max > 4 {
			return fmt.Errorf("exact main revision and max-jobs 1..4 required")
		}
		hydration := g.hydrate(*root, *revision)
		p := discover(*root, *schemas, *repo, *revision, *max)
		p.Blocked = append(p.Blocked, hydration...)
		if len(p.Candidates) > 0 && !*dry {
			for _, name := range []string{"ISOLAB_TRIGGER_ROLE_ARN", "ISOLAB_RESULTS_BUCKET", "ISOLAB_CODEBUILD_PROJECT"} {
				if os.Getenv(name) == "" {
					return fmt.Errorf("%s must be configured before creating permanent claims", name)
				}
			}
		}
		var err error
		p, err = g.reserve(p, *max, *dry)
		if err != nil {
			return err
		}
		if err = writeJSON(*out, p); err != nil {
			return err
		}
		if summary := os.Getenv("GITHUB_STEP_SUMMARY"); summary != "" {
			f, e := os.OpenFile(summary, os.O_APPEND|os.O_WRONLY, 0600)
			if e == nil {
				defer f.Close()
				fmt.Fprintf(f, "## Isolab experiment queue\n\nSelected: %d. Dry run: %t.\n\n", len(p.Candidates), *dry)
				for _, b := range p.Blocked {
					fmt.Fprintf(f, "- `%s`: %s\n", b.Record, b.Reason)
				}
			}
		}
		fmt.Printf("selected=%d\n", len(p.Candidates))
		return nil
	case "run":
		b, err := os.ReadFile(*file)
		if err != nil {
			return err
		}
		var p Plan
		if err = json.Unmarshal(b, &p); err != nil {
			return err
		}
		if p.Schema != "crypto.isolab-plan/v1" || p.Repository != *repo || !p.Claimed || len(p.Candidates) > 4 {
			return fmt.Errorf("invalid plan identity")
		}
		bucket, project := os.Getenv("ISOLAB_BUCKET"), os.Getenv("ISOLAB_CODEBUILD_PROJECT")
		if bucket == "" || project == "" {
			return fmt.Errorf("Isolab bucket/project configuration is missing")
		}
		x := Execution{AWS: AWSCLI{}, Bucket: bucket, Project: project, Schemas: *schemas, ToolDir: *toolDir, Poll: 30 * time.Second, Wait: *wait}
		results := []Result{}
		for _, c := range p.Candidates {
			if !hex64.MatchString(c.Key) || c.Entry.key() != c.Key {
				return fmt.Errorf("altered plan identity")
			}
			results = append(results, x.execute(context.Background(), c))
			if err = writeJSON(*out, results); err != nil {
				return err
			}
		}
		return writeJSON(*out, results)
	case "publish":
		b, err := os.ReadFile(*file)
		if err != nil {
			return err
		}
		if len(b) > 64<<20 {
			return fmt.Errorf("result transfer exceeds 64 MiB")
		}
		var results []Result
		if err = json.Unmarshal(b, &results); err != nil {
			return err
		}
		prs := []Publication{}
		for _, r := range results {
			p, err := g.publish(r)
			if err != nil {
				return err
			}
			if p.Number != 0 {
				prs = append(prs, p)
			}
		}
		return writeJSON(*out, map[string]any{"pull_requests": prs})
	default:
		return fmt.Errorf("unknown command %q", args[0])
	}
}
