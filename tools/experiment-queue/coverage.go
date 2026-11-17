package main

import (
	"encoding/json"
	"fmt"
)

func integerValue(v any) int {
	n, ok := v.(json.Number)
	if !ok {
		return -1
	}
	x, e := n.Int64()
	if e != nil || x < 0 || x > 1000000 {
		return -1
	}
	return int(x)
}
func checkCoverage(protocol json.RawMessage, export map[string]any) error {
	p, err := object(protocol)
	if err != nil {
		return err
	}
	sample := asMap(p["sample_size"])
	instances, replicates, warmups := integerValue(sample["instances"]), integerValue(sample["replicates"]), integerValue(sample["warmups"])
	if warmups < 0 {
		warmups = integerValue(asMap(p["measurement"])["warmups"])
	}
	if instances < 1 || replicates < 1 || warmups < 0 {
		return fmt.Errorf("invalid frozen coverage definition")
	}
	arms := map[string]bool{}
	for _, a := range array(p["arms"]) {
		name, _ := asMap(a)["id"].(string)
		arms[name] = true
	}
	cells := map[string]map[string][2]int{}
	seen := map[string]bool{}
	for _, v := range array(export["runs"]) {
		r := asMap(v)
		arm, _ := r["arm"].(string)
		instance, _ := r["instance_id"].(string)
		if !arms[arm] || instance == "" {
			return fmt.Errorf("export has an unregistered arm/instance")
		}
		key := fmt.Sprintf("%s/%s/%v/%v", arm, instance, r["replicate"], r["warmup"])
		if seen[key] {
			return fmt.Errorf("duplicate exported execution cell")
		}
		seen[key] = true
		if cells[arm] == nil {
			cells[arm] = map[string][2]int{}
		}
		count := cells[arm][instance]
		if r["warmup"] == true {
			count[1]++
		} else {
			count[0]++
		}
		cells[arm][instance] = count
	}
	for arm := range arms {
		if len(cells[arm]) != instances {
			return fmt.Errorf("incomplete instance coverage for %s", arm)
		}
		for _, counts := range cells[arm] {
			if counts[0] != replicates || counts[1] != warmups {
				return fmt.Errorf("incomplete replicate/warmup coverage for %s", arm)
			}
		}
	}
	return nil
}
