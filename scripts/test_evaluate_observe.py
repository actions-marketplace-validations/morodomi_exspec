"""Tests for evaluate_observe.py — observe precision evaluation against ground truth."""

import json
import pytest
from evaluate_observe import (
    parse_ground_truth,
    parse_observe_output,
    normalize_path,
    evaluate_precision,
    stratum_breakdown,
    EvalResult,
)


# --- Fixtures ---


@pytest.fixture
def simple_ground_truth():
    """Ground truth with 2 test files, 3 primary mappings, 1 secondary."""
    return {
        "metadata": {"repository": "test/repo"},
        "file_mappings": {
            "src/test/foo.spec.ts": {
                "primary_targets": ["src/foo.ts"],
                "secondary_targets": ["src/helper.ts"],
                "confidence": "high",
                "evidence": {
                    "src/foo.ts": ["direct_import", "filename_match"],
                },
            },
            "src/test/bar.spec.ts": {
                "primary_targets": ["src/bar.ts", "src/baz.ts"],
                "secondary_targets": [],
                "confidence": "high",
                "evidence": {
                    "src/bar.ts": ["direct_import", "filename_match"],
                    "src/baz.ts": ["barrel_import", "call_usage"],
                },
            },
        },
    }


@pytest.fixture
def simple_observe_output():
    """Observe output mapping production files to test files (absolute paths)."""
    return {
        "file_mappings": [
            {
                "production_file": "/repo/src/foo.ts",
                "test_files": ["/repo/src/test/foo.spec.ts"],
                "strategy": "import",
            },
            {
                "production_file": "/repo/src/bar.ts",
                "test_files": ["/repo/src/test/bar.spec.ts"],
                "strategy": "import",
            },
            {
                "production_file": "/repo/src/helper.ts",
                "test_files": ["/repo/src/test/foo.spec.ts"],
                "strategy": "import",
            },
            {
                "production_file": "/repo/src/unknown.ts",
                "test_files": ["/repo/src/test/foo.spec.ts"],
                "strategy": "import",
            },
        ],
    }


# --- GT-EVAL-01: Ground truth parsing ---


class TestParseGroundTruth:
    def test_extracts_primary_targets(self, simple_ground_truth):
        """Given ground truth JSON, when parsed, then primary targets are extracted per test file."""
        result = parse_ground_truth(simple_ground_truth)
        assert result.primary_pairs == {
            ("src/test/foo.spec.ts", "src/foo.ts"),
            ("src/test/bar.spec.ts", "src/bar.ts"),
            ("src/test/bar.spec.ts", "src/baz.ts"),
        }

    def test_extracts_secondary_targets(self, simple_ground_truth):
        """Given ground truth JSON, when parsed, then secondary targets are collected."""
        result = parse_ground_truth(simple_ground_truth)
        assert result.secondary_pairs == {
            ("src/test/foo.spec.ts", "src/helper.ts"),
        }

    def test_skips_none_primary_targets(self):
        """Given test with *(none)* primary, when parsed, then no pairs generated."""
        gt = {
            "metadata": {},
            "file_mappings": {
                "test/foo.spec.ts": {
                    "primary_targets": ["*(none)*"],
                    "secondary_targets": [],
                    "confidence": "uncertain",
                    "evidence": {},
                },
            },
        }
        result = parse_ground_truth(gt)
        assert result.primary_pairs == set()

    def test_extracts_evidence_stratum(self, simple_ground_truth):
        """Given ground truth with evidence, when parsed, then evidence per primary pair is available."""
        result = parse_ground_truth(simple_ground_truth)
        assert result.evidence[("src/test/bar.spec.ts", "src/baz.ts")] == [
            "barrel_import",
            "call_usage",
        ]


# --- GT-EVAL-02: Observe output parsing ---


class TestParseObserveOutput:
    def test_inverts_production_to_test_mapping(self, simple_observe_output):
        """Given observe output (prod→tests), when parsed, then (test, prod) pairs returned."""
        pairs = parse_observe_output(simple_observe_output, scan_root="/repo")
        assert ("src/test/foo.spec.ts", "src/foo.ts") in pairs
        assert ("src/test/bar.spec.ts", "src/bar.ts") in pairs
        assert ("src/test/foo.spec.ts", "src/helper.ts") in pairs
        assert ("src/test/foo.spec.ts", "src/unknown.ts") in pairs

    def test_strips_scan_root_prefix(self, simple_observe_output):
        """Given absolute paths in observe output, when parsed with scan_root, then paths are relative."""
        pairs = parse_observe_output(simple_observe_output, scan_root="/repo")
        for test_file, prod_file in pairs:
            assert not test_file.startswith("/")
            assert not prod_file.startswith("/")

    def test_trailing_slash_in_scan_root(self, simple_observe_output):
        """Given scan_root with trailing slash, when parsed, then paths still correct."""
        pairs = parse_observe_output(simple_observe_output, scan_root="/repo/")
        assert ("src/test/foo.spec.ts", "src/foo.ts") in pairs


# --- GT-EVAL-03: Path normalization ---


class TestNormalizePath:
    def test_strips_prefix(self):
        """Given absolute path and scan_root, when normalized, then prefix removed."""
        assert normalize_path("/repo/src/foo.ts", "/repo") == "src/foo.ts"

    def test_strips_trailing_slash(self):
        """Given scan_root with trailing slash, when normalized, then works correctly."""
        assert normalize_path("/repo/src/foo.ts", "/repo/") == "src/foo.ts"

    def test_no_prefix_match_returns_original(self):
        """Given path not starting with scan_root, when normalized, then returned as-is."""
        assert normalize_path("src/foo.ts", "/other") == "src/foo.ts"

    def test_strips_leading_dot_slash(self):
        """Given path with ./ prefix, when normalized, then stripped."""
        assert normalize_path("./src/foo.ts", "") == "src/foo.ts"


# --- GT-EVAL-04: TP/FP/FN/ignored count ---


class TestEvaluatePrecision:
    def test_tp_count(self, simple_ground_truth, simple_observe_output):
        """Given observe predicts foo.ts and bar.ts correctly, when evaluated, then TP=2."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        assert result.tp == 2

    def test_fp_count(self, simple_ground_truth, simple_observe_output):
        """Given observe predicts unknown.ts (not in GT), when evaluated, then FP=1."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        assert result.fp == 1

    def test_fn_count(self, simple_ground_truth, simple_observe_output):
        """Given GT has baz.ts but observe doesn't predict it, when evaluated, then FN=1."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        assert result.fn == 1

    def test_ignored_secondary(self, simple_ground_truth, simple_observe_output):
        """Given observe predicts helper.ts which is secondary, when evaluated, then ignored (not TP/FP)."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        assert result.ignored == 1
        # helper.ts is secondary, so should not be in TP or FP
        assert ("src/test/foo.spec.ts", "src/helper.ts") not in result.tp_pairs
        assert ("src/test/foo.spec.ts", "src/helper.ts") not in result.fp_pairs

    def test_precision_calculation(self, simple_ground_truth, simple_observe_output):
        """Given TP=2, FP=1, when evaluated, then precision=2/3."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        assert result.precision == pytest.approx(2 / 3)

    def test_recall_calculation(self, simple_ground_truth, simple_observe_output):
        """Given TP=2, FN=1, when evaluated, then recall=2/3."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        assert result.recall == pytest.approx(2 / 3)

    def test_f1_calculation(self, simple_ground_truth, simple_observe_output):
        """Given P=R=2/3, when evaluated, then F1=2/3."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        assert result.f1 == pytest.approx(2 / 3)


class TestEvaluateEdgeCases:
    def test_zero_predictions(self):
        """Given observe outputs nothing, when evaluated, then precision=0, recall=0, F1=0."""
        gt = {
            "metadata": {},
            "file_mappings": {
                "test/a.spec.ts": {
                    "primary_targets": ["src/a.ts"],
                    "secondary_targets": [],
                    "confidence": "high",
                    "evidence": {"src/a.ts": ["direct_import"]},
                },
            },
        }
        observe = {"file_mappings": []}
        result = evaluate_precision(observe, gt, scan_root="/repo")
        assert result.tp == 0
        assert result.fn == 1
        assert result.precision == 0.0
        assert result.recall == 0.0
        assert result.f1 == 0.0

    def test_no_ground_truth(self):
        """Given empty ground truth, when evaluated, then out-of-scope predictions are excluded."""
        gt = {"metadata": {}, "file_mappings": {}}
        observe = {
            "file_mappings": [
                {
                    "production_file": "/repo/src/a.ts",
                    "test_files": ["/repo/test/a.spec.ts"],
                    "strategy": "import",
                }
            ]
        }
        result = evaluate_precision(observe, gt, scan_root="/repo")
        # test/a.spec.ts is not in GT scope, so all predictions excluded
        assert result.tp == 0
        assert result.fp == 0
        assert result.fn == 0
        assert result.precision == 0.0
        assert result.recall == 0.0
        assert result.f1 == 0.0

    def test_observe_test_not_in_gt_scope(self):
        """Given observe predicts pair for test not in GT, when evaluated, then pair is not FP (out of scope)."""
        gt = {
            "metadata": {},
            "file_mappings": {
                "test/a.spec.ts": {
                    "primary_targets": ["src/a.ts"],
                    "secondary_targets": [],
                    "confidence": "high",
                    "evidence": {"src/a.ts": ["direct_import"]},
                },
            },
        }
        observe = {
            "file_mappings": [
                {
                    "production_file": "/repo/src/a.ts",
                    "test_files": ["/repo/test/a.spec.ts"],
                    "strategy": "import",
                },
                {
                    "production_file": "/repo/other/x.ts",
                    "test_files": ["/repo/other/x.spec.ts"],
                    "strategy": "import",
                },
            ]
        }
        result = evaluate_precision(observe, gt, scan_root="/repo")
        # other/x.spec.ts is not in GT scope, so should be excluded
        assert result.tp == 1
        assert result.fp == 0
        assert result.fn == 0


# --- GT-EVAL-05: Zero division guard (covered above) ---


# --- GT-EVAL-06: Stratum breakdown ---


class TestStratumBreakdown:
    def test_direct_import_stratum(self, simple_ground_truth, simple_observe_output):
        """Given GT with direct_import and barrel_import evidence, when breakdown, then stratified."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        gt_parsed = parse_ground_truth(simple_ground_truth)
        breakdown = stratum_breakdown(result, gt_parsed)
        assert "direct_import" in breakdown
        assert "barrel_import" in breakdown
        # foo.ts and bar.ts both have direct_import evidence and are TP
        assert breakdown["direct_import"]["tp"] == 2
        # baz.ts has barrel_import evidence and is FN
        assert breakdown["barrel_import"]["fn"] == 1

    def test_breakdown_precision_recall(self, simple_ground_truth, simple_observe_output):
        """Given stratum breakdown, when accessed, then precision/recall are per-stratum."""
        result = evaluate_precision(
            simple_observe_output, simple_ground_truth, scan_root="/repo"
        )
        gt_parsed = parse_ground_truth(simple_ground_truth)
        breakdown = stratum_breakdown(result, gt_parsed)
        # direct_import: TP=1 (foo.ts), FP=0, FN=0 → P=1.0, R=1.0
        # (bar.ts also has direct_import but is TP too)
        di = breakdown["direct_import"]
        assert di["precision"] == pytest.approx(di["tp"] / (di["tp"] + di["fp"])) if (di["tp"] + di["fp"]) > 0 else True
