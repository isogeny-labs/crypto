#!/usr/bin/env python3

import copy
import json
import pathlib
import unittest

from validate_population import ContractError, specification_digest, validate


ROOT = pathlib.Path(__file__).parent


class ContractTests(unittest.TestCase):
    def setUp(self) -> None:
        self.valid = json.loads((ROOT / "fixtures" / "valid.json").read_text())

    def test_valid_fixture(self) -> None:
        validate(self.valid)

    def test_missing_terminal_target_fails(self) -> None:
        document = copy.deepcopy(self.valid)
        document["targets"].pop()
        with self.assertRaisesRegex(ContractError, "exactly cover manifest"):
            validate(document)

    def test_rank_decrease_fails(self) -> None:
        document = copy.deepcopy(self.valid)
        document["rank_trace"][1]["rank"] = 1
        document["rank_trace"][2]["rank"] = 0
        with self.assertRaisesRegex(ContractError, "must be monotone"):
            validate(document)

    def test_spec_mutation_fails_digest(self) -> None:
        document = copy.deepcopy(self.valid)
        document["identity"]["arity"] = 3
        self.assertNotEqual(document["spec_digest"], specification_digest(document))
        with self.assertRaisesRegex(ContractError, "spec_digest"):
            validate(document)


if __name__ == "__main__":
    unittest.main()
