// Administrative metadata validation using the repository queue's native
// jsonschema dependency and canonical JSON convention; no cryptographic work.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"time"

	"github.com/santhosh-tekuri/jsonschema/v6"
)

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func run() error {
	if len(os.Args) != 4 {
		return fmt.Errorf("usage: validate-protocol SCHEMA PROTOCOL check|freeze")
	}
	mode := os.Args[3]
	if mode != "check" && mode != "freeze" {
		return fmt.Errorf("unsupported mode")
	}
	b, err := os.ReadFile(os.Args[2])
	if err != nil {
		return err
	}
	var p map[string]any
	dec := json.NewDecoder(bytes.NewReader(b))
	dec.UseNumber()
	if err = dec.Decode(&p); err != nil {
		return err
	}
	freeze, ok := p["freeze"].(map[string]any)
	if !ok {
		return fmt.Errorf("missing freeze record")
	}
	delete(p, "freeze")
	var canon bytes.Buffer
	enc := json.NewEncoder(&canon)
	enc.SetEscapeHTML(false)
	if err = enc.Encode(p); err != nil {
		return err
	}
	hash := sha256.Sum256(bytes.TrimSuffix(canon.Bytes(), []byte{'\n'}))
	digest := hex.EncodeToString(hash[:])
	if mode == "freeze" {
		freeze["status"] = "frozen"
		freeze["protocol_sha256"] = digest
		freeze["frozen_at"] = time.Now().UTC().Format(time.RFC3339)
		freeze["filed_before_affected_data"] = true
	} else if freeze["status"] != "frozen" || freeze["protocol_sha256"] != digest || freeze["filed_before_affected_data"] != true {
		return fmt.Errorf("prospective freeze/hash mismatch")
	}
	p["freeze"] = freeze
	b, err = json.MarshalIndent(p, "", "  ")
	if err != nil {
		return err
	}
	c := jsonschema.NewCompiler()
	schema, err := c.Compile(os.Args[1])
	if err != nil {
		return err
	}
	v, err := jsonschema.UnmarshalJSON(bytes.NewReader(b))
	if err != nil {
		return err
	}
	if err = schema.Validate(v); err != nil {
		return err
	}
	if mode == "freeze" {
		if err = os.WriteFile(os.Args[2], append(b, '\n'), 0644); err != nil {
			return err
		}
	}
	fmt.Printf("protocol schema and prospective canonical hash verified: %s\n", digest)
	return nil
}
