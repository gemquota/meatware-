import importlib.util
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "validate_docs.py"
SPEC = importlib.util.spec_from_file_location("validate_docs", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


class DocumentationValidationTests(unittest.TestCase):
    def test_repository_documents_are_valid(self):
        self.assertEqual(MODULE.validate(), [])

    def test_required_document_list_is_not_empty(self):
        self.assertGreaterEqual(len(MODULE.REQUIRED), 5)

    def test_readme_and_specification_exist(self):
        self.assertTrue((MODULE.ROOT / "README.md").is_file())
        self.assertTrue((MODULE.ROOT / "docs/SPECIFICATION.md").is_file())


if __name__ == "__main__":
    unittest.main()
