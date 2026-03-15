#!/usr/bin/env python3
"""Evaluate exspec observe output against hand-audited ground truth.

Usage:
    python3 evaluate_observe.py \
        --observe-json /tmp/observe-output.json \
        --ground-truth docs/observe-ground-truth.md \
        --scan-root /tmp/nestjs-nest

Outputs Markdown-formatted precision/recall/F1 with stratum breakdown.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field


@dataclass
class GroundTruthParsed:
    """Parsed ground truth with primary/secondary pairs and evidence."""

    primary_pairs: set[tuple[str, str]] = field(default_factory=set)
    secondary_pairs: set[tuple[str, str]] = field(default_factory=set)
    evidence: dict[tuple[str, str], list[str]] = field(default_factory=dict)
    test_files_in_scope: set[str] = field(default_factory=set)


@dataclass
class EvalResult:
    """Evaluation result with TP/FP/FN counts and pair lists."""

    tp: int = 0
    fp: int = 0
    fn: int = 0
    ignored: int = 0
    precision: float = 0.0
    recall: float = 0.0
    f1: float = 0.0
    tp_pairs: set[tuple[str, str]] = field(default_factory=set)
    fp_pairs: set[tuple[str, str]] = field(default_factory=set)
    fn_pairs: set[tuple[str, str]] = field(default_factory=set)
    ignored_pairs: set[tuple[str, str]] = field(default_factory=set)


def normalize_path(path: str, scan_root: str) -> str:
    """Strip scan_root prefix and leading ./ from path."""
    scan_root = scan_root.rstrip("/")
    if scan_root and path.startswith(scan_root):
        path = path[len(scan_root) :].lstrip("/")
    if path.startswith("./"):
        path = path[2:]
    return path


def parse_ground_truth(gt_data: dict) -> GroundTruthParsed:
    """Parse ground truth JSON into primary/secondary pair sets."""
    result = GroundTruthParsed()

    for test_file, mapping in gt_data.get("file_mappings", {}).items():
        result.test_files_in_scope.add(test_file)

        for target in mapping.get("primary_targets", []):
            if target == "*(none)*":
                continue
            pair = (test_file, target)
            result.primary_pairs.add(pair)

            evidence = mapping.get("evidence", {}).get(target, [])
            if evidence:
                result.evidence[pair] = evidence

        for target in mapping.get("secondary_targets", []):
            result.secondary_pairs.add((test_file, target))

    return result


def parse_observe_output(
    observe_data: dict, scan_root: str
) -> set[tuple[str, str]]:
    """Parse observe JSON output into (test_file, production_file) pairs with relative paths."""
    pairs: set[tuple[str, str]] = set()

    for mapping in observe_data.get("file_mappings", []):
        prod_file = normalize_path(mapping["production_file"], scan_root)
        for test_file in mapping.get("test_files", []):
            test_rel = normalize_path(test_file, scan_root)
            pairs.add((test_rel, prod_file))

    return pairs


def evaluate_precision(
    observe_data: dict, gt_data: dict, scan_root: str
) -> EvalResult:
    """Compare observe output against ground truth. Returns TP/FP/FN/precision/recall/F1."""
    gt = parse_ground_truth(gt_data)
    observe_pairs = parse_observe_output(observe_data, scan_root)

    result = EvalResult()

    # Only evaluate pairs where test_file is in GT scope
    scoped_observe_pairs = {
        (t, p) for t, p in observe_pairs if t in gt.test_files_in_scope
    }

    for pair in scoped_observe_pairs:
        if pair in gt.primary_pairs:
            result.tp += 1
            result.tp_pairs.add(pair)
        elif pair in gt.secondary_pairs:
            result.ignored += 1
            result.ignored_pairs.add(pair)
        else:
            result.fp += 1
            result.fp_pairs.add(pair)

    # FN: primary pairs not predicted by observe
    for pair in gt.primary_pairs:
        if pair not in scoped_observe_pairs:
            result.fn += 1
            result.fn_pairs.add(pair)

    # Calculate metrics with zero-division guard
    if result.tp + result.fp > 0:
        result.precision = result.tp / (result.tp + result.fp)
    if result.tp + result.fn > 0:
        result.recall = result.tp / (result.tp + result.fn)
    if result.precision + result.recall > 0:
        result.f1 = (
            2 * result.precision * result.recall / (result.precision + result.recall)
        )

    return result


def stratum_breakdown(
    eval_result: EvalResult, gt_parsed: GroundTruthParsed
) -> dict[str, dict]:
    """Break down evaluation results by evidence stratum (e.g., direct_import, barrel_import)."""
    strata: dict[str, dict] = {}

    # Collect all evidence types
    all_evidence_types: set[str] = set()
    for evs in gt_parsed.evidence.values():
        all_evidence_types.update(evs)

    for ev_type in sorted(all_evidence_types):
        # Find GT pairs that have this evidence type
        gt_pairs_with_ev = {
            pair
            for pair, evs in gt_parsed.evidence.items()
            if ev_type in evs
        }

        tp = len(eval_result.tp_pairs & gt_pairs_with_ev)
        fn = len(eval_result.fn_pairs & gt_pairs_with_ev)
        # FP: observe pairs that are FP and the GT pair they "should" have matched
        # has this evidence type — not directly applicable. For stratum FP, count 0.
        fp = 0

        precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
        recall = tp / (tp + fn) if (tp + fn) > 0 else 0.0
        f1 = (
            2 * precision * recall / (precision + recall)
            if (precision + recall) > 0
            else 0.0
        )

        strata[ev_type] = {
            "tp": tp,
            "fp": fp,
            "fn": fn,
            "total_gt": len(gt_pairs_with_ev),
            "precision": precision,
            "recall": recall,
            "f1": f1,
        }

    return strata


def extract_json_from_markdown(md_content: str) -> dict:
    """Extract the JSON block from ground truth Markdown file."""
    match = re.search(r"```json\n(.*?)\n```", md_content, re.DOTALL)
    if not match:
        raise ValueError("No JSON block found in ground truth Markdown")
    return json.loads(match.group(1))


def format_results_markdown(
    eval_result: EvalResult,
    strata: dict[str, dict],
    gt_parsed: GroundTruthParsed,
) -> str:
    """Format evaluation results as Markdown."""
    lines = [
        "# Observe Precision Evaluation Results",
        "",
        "## Summary",
        "",
        f"| Metric | Value |",
        f"|--------|-------|",
        f"| TP (correct predictions) | {eval_result.tp} |",
        f"| FP (incorrect predictions) | {eval_result.fp} |",
        f"| FN (missed ground truth) | {eval_result.fn} |",
        f"| Ignored (secondary targets) | {eval_result.ignored} |",
        f"| Precision | {eval_result.precision:.1%} |",
        f"| Recall | {eval_result.recall:.1%} |",
        f"| F1 Score | {eval_result.f1:.1%} |",
        "",
        "## Stratum Breakdown",
        "",
        "| Evidence Type | GT Pairs | TP | FN | Recall |",
        "|---------------|----------|----|----|--------|",
    ]

    for ev_type, data in sorted(strata.items()):
        lines.append(
            f"| {ev_type} | {data['total_gt']} | {data['tp']} | {data['fn']} | {data['recall']:.1%} |"
        )

    lines.extend(
        [
            "",
            "## True Positives",
            "",
            "| Test File | Production File |",
            "|-----------|-----------------|",
        ]
    )
    for test_f, prod_f in sorted(eval_result.tp_pairs):
        lines.append(f"| {test_f} | {prod_f} |")

    lines.extend(
        [
            "",
            "## False Positives",
            "",
            "| Test File | Production File |",
            "|-----------|-----------------|",
        ]
    )
    for test_f, prod_f in sorted(eval_result.fp_pairs):
        lines.append(f"| {test_f} | {prod_f} |")

    lines.extend(
        [
            "",
            "## False Negatives",
            "",
            "| Test File | Production File | Evidence |",
            "|-----------|-----------------|----------|",
        ]
    )
    for test_f, prod_f in sorted(eval_result.fn_pairs):
        evidence = gt_parsed.evidence.get((test_f, prod_f), [])
        lines.append(f"| {test_f} | {prod_f} | {', '.join(evidence)} |")

    lines.append("")
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(
        description="Evaluate exspec observe output against ground truth"
    )
    parser.add_argument(
        "--observe-json", required=True, help="Path to observe JSON output"
    )
    parser.add_argument(
        "--ground-truth", required=True, help="Path to ground truth Markdown"
    )
    parser.add_argument(
        "--scan-root", required=True, help="Repository root used for observe scan"
    )
    args = parser.parse_args()

    with open(args.observe_json) as f:
        observe_data = json.load(f)

    with open(args.ground_truth) as f:
        gt_data = extract_json_from_markdown(f.read())

    gt_parsed = parse_ground_truth(gt_data)
    eval_result = evaluate_precision(observe_data, gt_data, scan_root=args.scan_root)
    strata = stratum_breakdown(eval_result, gt_parsed)

    print(format_results_markdown(eval_result, strata, gt_parsed))


if __name__ == "__main__":
    main()
