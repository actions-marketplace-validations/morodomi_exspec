# Ground Truth: Rust observe -- tower

Repository: tower-rs/tower
Commit: 251296d
Auditor: Human + AI (full audit)
Date: 2026-03-25

## Methodology

1. exspec observe output collected (`observe --lang rust --format json`)
2. 18-file full audit of tower/tests/ directory (small enough for complete coverage)
3. Each test file audited: use statements, test function names, observe mapping result
4. Inline self-match files (src/ with #[cfg(test)]) recorded separately

## Scope Exclusions

- tower/tests/support.rs: helper file (no test functions, only utility structs/functions used by other test files)
- tower/tests/limit/main.rs: module entry file only (mod concurrency; mod rate;), no own test functions
- tower/tests/util/main.rs: module entry file only (mod call_all; mod oneshot; mod service_fn;), no own test functions

## GT Scope Summary

| Stratum | Files | Description |
|---------|-------|-------------|
| tower/tests/ external (real tests) | 15 | Test files with at least one test function |
| tower/src/ inline tests (self-match) | 8 | Source files with #[cfg(test)] modules |
| **Total** | **24** | **Note**: scope revised to 24 post-#199 (tower-test/tests/mock.rs added); effective audit scope used for P/R is 24 |

## Rust-Specific Decisions

- **Submodule direct import**: tower tests use `use tower::util::ServiceExt`, `use tower::retry::Policy`, `use tower::buffer::error::*`. These submodule imports are the pattern Rust observe is designed to handle. This is the "normal-case" pattern.
- **mod.rs barrel FN**: `use tower::filter::AsyncFilter` resolves through `tower/src/filter/mod.rs`. The mod.rs file is not recognized as a production file by observe (it exports symbols via `pub use self::...` making it a barrel, but the fan-out suppression or barrel detection prevents mapping). This is the dominant FN cause for tower.
- **Module entry files**: limit/main.rs and util/main.rs are `mod file1; mod file2;` entry points with no own test functions. They are excluded from GT scope (not meaningful test files).
- **Helper file**: support.rs contains only utility code (trace_init, AssertSpanSvc). No test functions. Excluded from GT scope.
- **Inline self-matches**: 8 src/ files contain #[cfg(test)] modules and map to themselves via filename strategy. Recorded as TP by definition.

## FN Root Cause Analysis

Post-#199 (barrel self-match fix, 2026-03-25): 4 of the original 5 FN were resolved. 2 FN remain.

| Root Cause | Count | Example Files |
|-----------|-------|---------------|
| Cross-crate (tower-test crate) | 1 | tower-test/tests/mock.rs |
| Deep mod hierarchy (concurrency) | 1 | tests/limit/concurrency.rs |

**Detail (remaining FN)**:
- `tower-test/tests/mock.rs`: lives in a separate `tower-test` crate. observe maps within each crate; cross-crate test-to-code mapping is not supported.
- `tests/limit/concurrency.rs`: `use tower::limit::concurrency::ConcurrencyLimitLayer` → resolves through deep `limit/concurrency/mod.rs` barrel hierarchy, not directly mapped.

**Resolved FN (fixed by #199 barrel self-match)**:
- `tests/filter/async_filter.rs`: `use tower::filter::{error::Error, AsyncFilter}` → now correctly maps to `tower/src/filter/mod.rs` via barrel self-match
- `tests/hedge/main.rs`: `use tower::hedge::{Hedge, Policy}` → maps to `tower/src/hedge/mod.rs`
- `tests/steer/main.rs`: `use tower::steer::Steer` → maps to `tower/src/steer/mod.rs`
- `tests/util/call_all.rs`: `use tower::util::ServiceExt` → now mapped via barrel self-match

**Note on "normal-case" classification**: tower avoids crate-root barrel re-export (`use tower::Service` routes to tower_service crate, not through barrel). Most tests use submodule paths. Post-#199, tower now meets R>=90% ship criteria.

## P/R Metrics

### Post-#199 (barrel self-match fix, 2026-03-25)

Based on GT scope of 24 files (15 external + 8 inline + 1 cross-crate tower-test/tests/mock.rs):

- **TP (correctly mapped)**: 22 (14 external + 8 inline self-matches)
- **FP**: 0
- **FN**: 2 (tower-test/tests/mock.rs cross-crate, limit/concurrency.rs deep mod hierarchy)
- **Precision** = 22 / (22 + 0) = **100%** (meets >= 98% ship criterion -- PASS)
- **Recall** = 22 / (22 + 2) = **91.7%** (meets >= 90% ship criterion -- PASS)

**#199 improvement**: 4 FN resolved (filter/async_filter, hedge/main, steer/main, util/call_all). All were mod.rs barrel files where types are defined in the module root. Barrel self-match fix allows observe to recognize mod.rs as a mappable production file target.

### Pre-#199 (for reference)

- TP: 18 (10 external + 8 inline), FP: 0, FN: 5
- Precision: 100%, Recall: 78.3% (18/23, scope=23)

**Note on cycle doc R=94.7%**: The cycle doc recorded R=94.7% (18/19) based on misreading the observe summary field `test_files: 19, mapped_files: 19`. That field counts production files that have associated test file mappings (production-centric view), not the recall of external test files. The correct GT-based pre-#199 recall was 78.3%.

**Conclusion**: Post-#199, tower meets both ship criteria: P=100% (PASS) and R=91.7% (PASS). Rust observe achieves ship criteria on tower, a normal-case library using submodule direct imports. Remaining 2 FN are structurally out of scope (cross-crate) or require deep mod hierarchy tracing (deep concurrency path).

**17-library survey context**: tower was the highest-recall library in the 17-library survey. Pre-#199 R=78.3% meant no library achieved R>=90%. Post-#199, tower achieves R=91.7%, confirming that Rust observe ship criteria are met for normal-case libraries.

## Ground Truth

```json
{
  "metadata": {
    "repository": "tower-rs/tower",
    "commit": "251296d",
    "language": "rust",
    "auditor": "human+ai",
    "audit_coverage": "18-file full audit (tower/tests/) + 8 inline self-matches",
    "date": "2026-03-25"
  },
  "file_mappings": {
    "tower/tests/balance/main.rs": {
      "primary_targets": [
        "tower/tower/src/balance/p2c/service.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/balance/p2c/service.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/buffer/main.rs": {
      "primary_targets": [
        "tower/tower/src/buffer/service.rs",
        "tower/tower/src/buffer/error.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/buffer/service.rs": [
          "direct_import"
        ],
        "tower/tower/src/buffer/error.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/builder.rs": {
      "primary_targets": [
        "tower/tower/src/retry/policy.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/retry/policy.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/limit/rate.rs": {
      "primary_targets": [
        "tower/tower/src/limit/rate/rate.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/limit/rate/rate.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/load_shed/main.rs": {
      "primary_targets": [
        "tower/tower/src/load_shed/layer.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/load_shed/layer.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/ready_cache/main.rs": {
      "primary_targets": [
        "tower/tower/src/ready_cache/cache.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/ready_cache/cache.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/retry/main.rs": {
      "primary_targets": [
        "tower/tower/src/retry/policy.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/retry/policy.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/spawn_ready/main.rs": {
      "primary_targets": [
        "tower/tower/src/spawn_ready/layer.rs",
        "tower/tower/src/spawn_ready/service.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/spawn_ready/layer.rs": [
          "direct_import"
        ],
        "tower/tower/src/spawn_ready/service.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/util/oneshot.rs": {
      "primary_targets": [
        "tower/tower/src/util/oneshot.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/util/oneshot.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower/tests/util/service_fn.rs": {
      "primary_targets": [
        "tower/tower/src/util/service_fn.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/util/service_fn.rs": [
          "direct_import"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import"
    },
    "tower-layer/src/layer_fn.rs": {
      "primary_targets": [
        "tower-layer/src/layer_fn.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower-layer/src/layer_fn.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match: #[cfg(test)] module in production file"
    },
    "tower/src/load/peak_ewma.rs": {
      "primary_targets": [
        "tower/tower/src/load/peak_ewma.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/load/peak_ewma.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match"
    },
    "tower/src/load/pending_requests.rs": {
      "primary_targets": [
        "tower/tower/src/load/pending_requests.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/load/pending_requests.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match"
    },
    "tower/src/make/make_service/shared.rs": {
      "primary_targets": [
        "tower/tower/src/make/make_service/shared.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/make/make_service/shared.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match"
    },
    "tower/src/retry/backoff.rs": {
      "primary_targets": [
        "tower/tower/src/retry/backoff.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/retry/backoff.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match"
    },
    "tower/src/retry/budget/tps_budget.rs": {
      "primary_targets": [
        "tower/tower/src/retry/budget/tps_budget.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/retry/budget/tps_budget.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match"
    },
    "tower/src/util/future_service.rs": {
      "primary_targets": [
        "tower/tower/src/util/future_service.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/util/future_service.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match"
    },
    "tower/src/util/rng.rs": {
      "primary_targets": [
        "tower/tower/src/util/rng.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/util/rng.rs": [
          "filename_match"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "filename",
      "note": "Inline self-match"
    },
    "tower/tests/filter/async_filter.rs": {
      "primary_targets": [
        "tower/tower/src/filter/mod.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/filter/mod.rs": [
          "symbol_assertion"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import",
      "note": "Resolved by #199 barrel self-match fix. `use tower::filter::{error::Error, AsyncFilter}` -- AsyncFilter defined in filter/mod.rs. mod.rs now recognized as mappable production file."
    },
    "tower/tests/hedge/main.rs": {
      "primary_targets": [
        "tower/tower/src/hedge/mod.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/hedge/mod.rs": [
          "symbol_assertion"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import",
      "note": "Resolved by #199 barrel self-match fix. `use tower::hedge::{Hedge, Policy}` -- both defined in hedge/mod.rs."
    },
    "tower/tests/steer/main.rs": {
      "primary_targets": [
        "tower/tower/src/steer/mod.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/steer/mod.rs": [
          "symbol_assertion"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import",
      "note": "Resolved by #199 barrel self-match fix. `use tower::steer::Steer` -- Steer defined in steer/mod.rs (only file in steer/)."
    },
    "tower-test/tests/mock.rs": {
      "primary_targets": [],
      "secondary_targets": [],
      "evidence": {},
      "observe_result": "FN",
      "root_cause": "cross_crate",
      "note": "tower-test is a separate crate. observe does not map across crate boundaries."
    },
    "tower/tests/limit/concurrency.rs": {
      "primary_targets": [
        "tower/tower/src/limit/concurrency/service.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/limit/concurrency/service.rs": [
          "symbol_assertion"
        ]
      },
      "observe_result": "FN",
      "root_cause": "mod_rs_barrel",
      "note": "`use tower::limit::concurrency::ConcurrencyLimitLayer` -- resolves through limit/concurrency/mod.rs barrel"
    },
    "tower/tests/util/call_all.rs": {
      "primary_targets": [
        "tower/tower/src/util/call_all/ordered.rs",
        "tower/tower/src/util/call_all/unordered.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tower/tower/src/util/call_all/ordered.rs": [
          "symbol_assertion"
        ]
      },
      "observe_result": "TP",
      "observe_strategy": "import",
      "note": "Resolved by #199 barrel self-match fix. `use tower::util::ServiceExt` -- routes through util/mod.rs. Now mapped via barrel self-match."
    }
  }
}
```
