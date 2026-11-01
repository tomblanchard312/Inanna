"""Check OCI entry-point safety without building or publishing an image."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


class OciTests(unittest.TestCase):
    def test_clean_checkout_help_and_push_require_an_explicit_registry(self):
        script = Path(__file__).resolve().parent / "linux/flatpak-oci.sh"
        with tempfile.TemporaryDirectory() as tmp:
            env = {k: v for k, v in os.environ.items() if k not in ("REGISTRY", "GITHUB_TOKEN")}
            env["DIST"] = tmp
            help_result = subprocess.run(["bash", str(script), "--help"], env=env, capture_output=True, text=True)
            self.assertEqual(help_result.returncode, 0, help_result.stderr)
            result = subprocess.run(["bash", str(script), "--push"], env=env, capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("REGISTRY is required", result.stderr)
            self.assertNotIn("Building Flatpak", result.stdout)


if __name__ == "__main__":
    unittest.main()
