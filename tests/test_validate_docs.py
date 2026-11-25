import importlib.util
import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "validate_docs.py"
SPEC = importlib.util.spec_from_file_location("validate_docs", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


class DocumentationValidationTests(unittest.TestCase):
    def test_repository_documents_are_valid(self):
        self.assertEqual(MODULE.validate(), [])

    def test_required_document_list_covers_phase_two(self):
        self.assertGreaterEqual(len(MODULE.REQUIRED), 9)
        self.assertIn("docs/SUBSTRATE_MODEL.md", MODULE.REQUIRED)
        self.assertIn("docs/TRANSITION_CONTRACT.md", MODULE.REQUIRED)

    def test_transition_fixtures_are_machine_readable(self):
        fixture = ROOT / "tests" / "fixtures" / "transition_cases.json"
        data = json.loads(fixture.read_text(encoding="utf-8"))
        self.assertGreaterEqual(len(data["cases"]), 10)
        self.assertTrue(all("id" in case and "expected" in case for case in data["cases"]))

    def test_model_contracts_exclude_domain_primitives(self):
        model = (ROOT / "docs" / "SUBSTRATE_MODEL.md").read_text(encoding="utf-8").lower()
        self.assertIn("no built-in organisms", model)
        self.assertIn("no privileged", model)
        self.assertIn("relation-to-relation", model)

    def test_readme_links_to_authoritative_contracts(self):
        readme = (ROOT / "README.md").read_text(encoding="utf-8")
        self.assertIn("docs/SUBSTRATE_MODEL.md", readme)
        self.assertIn("docs/TRANSITION_CONTRACT.md", readme)


if __name__ == "__main__":
    unittest.main()
