# Ground Truth: Rust observe -- tokio

Repository: tokio-rs/tokio
Commit: 467d614
Auditor: Human + AI (stratified audit)
Date: 2026-03-25

## Methodology

1. exspec observe output collected with and without fan-out filter
2. 277 external test files classified into 6 categories (A-F)
3. 53-file stratified sample selected (seed=42):
   - S1: 15 TP (import strategy) -- precision verification
   - S2: 5 TP (filename strategy) -- precision verification
   - S3: 13 FN (fan-out filtered) -- filter correctness assessment
   - S4: 10 FN (unmapped) -- root cause analysis
   - S5: 5 inline src/tests/ -- inline test assessment
   - S6: 5 cross-crate -- cross-crate mapping assessment
4. Each test file audited: use statements, test function names, assertion targets

## Scope Exclusions

- tests-build/ (23 files): compile tests (trybuild), not applicable for file-level mapping
- support/ helper files (8 files): test infrastructure, not test files

## Rust-Specific Decisions

- **Barrel import tests**: `use tokio::sync::broadcast` resolves to `tokio/src/sync/broadcast.rs`
- **Fan-out filtered tests**: True primary target recorded. Filter correctness noted.
- **Trait extension tests**: Primary target is the specific impl file (e.g., chain.rs), not the trait extension (async_read_ext.rs)
- **Inline src/tests/**: Recorded with true primary target for future inline test support
- **Trait-bound tests** (unwindsafe.rs): `*(none)*` -- not behavioral tests

## FN Root Cause Analysis

| Root Cause | Count | Description |
|-----------|-------|-------------|
| Barrel import (`tokio::`) | 10 | observe cannot resolve re-exports through crate barrel |
| No use statement | 4 | Fully-qualified inline paths or attribute macros only |
| Imports in macro body | 1 | `rt_test!` macro hides use statements from AST |
| Inline src/ tests | 5 | observe doesn't classify src/ files as test files |
| Fan-out filter false negative | 4 | Filter removed correct mapping (io_chain, io_read_exact, io_read_to_string, task_hooks) |

## Fan-out Filter Assessment

13 fan-out filtered files audited:
- **9 correctly filtered**: True target differs from what was mapped (e.g., runtime.rs -> specific subsystem)
- **4 incorrectly filtered**: True target matches or should have been retained
  - io_chain.rs, io_read_exact.rs, io_read_to_string.rs: filename clearly identifies specific io util
  - task_hooks.rs: builder.rs IS the SUT for task hook configuration

## Ground Truth

```json
{
  "metadata": {
    "repository": "tokio-rs/tokio",
    "commit": "467d614",
    "language": "rust",
    "auditor": "human+ai",
    "audit_coverage": "53-file stratified sample",
    "date": "2026-03-25"
  },
  "file_mappings": {
    "tokio-util/tests/context.rs": {
      "primary_targets": [
        "tokio-util/src/context.rs"
      ],
      "secondary_targets": [
        "tokio/src/runtime/builder.rs",
        "tokio/src/time/sleep.rs"
      ],
      "evidence": {
        "tokio-util/src/context.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio-util/tests/framed.rs": {
      "primary_targets": [
        "tokio-util/src/codec/framed.rs"
      ],
      "secondary_targets": [
        "tokio-util/src/codec/decoder.rs",
        "tokio-util/src/codec/encoder.rs"
      ],
      "evidence": {
        "tokio-util/src/codec/framed.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio-util/tests/framed_read.rs": {
      "primary_targets": [
        "tokio-util/src/codec/framed_read.rs"
      ],
      "secondary_targets": [
        "tokio-util/src/codec/decoder.rs"
      ],
      "evidence": {
        "tokio-util/src/codec/framed_read.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio-util/tests/sync_cancellation_token.rs": {
      "primary_targets": [
        "tokio-util/src/sync/cancellation_token.rs"
      ],
      "secondary_targets": [
        "tokio/src/sync/oneshot.rs"
      ],
      "evidence": {
        "tokio-util/src/sync/cancellation_token.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio-util/tests/task_join_map.rs": {
      "primary_targets": [
        "tokio-util/src/task/join_map.rs"
      ],
      "secondary_targets": [
        "tokio/src/sync/oneshot.rs",
        "tokio/src/time/sleep.rs"
      ],
      "evidence": {
        "tokio-util/src/task/join_map.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio-util/tests/udp.rs": {
      "primary_targets": [
        "tokio-util/src/udp/frame.rs"
      ],
      "secondary_targets": [
        "tokio/src/net/udp.rs"
      ],
      "evidence": {
        "tokio-util/src/udp/frame.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      }
    },
    "tokio/src/runtime/tests/loom_join_set.rs": {
      "primary_targets": [
        "tokio/src/task/join_set.rs"
      ],
      "secondary_targets": [
        "tokio/src/runtime/builder.rs"
      ],
      "evidence": {
        "tokio/src/task/join_set.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      }
    },
    "tokio/src/sync/tests/loom_semaphore_batch.rs": {
      "primary_targets": [
        "tokio/src/sync/batch_semaphore.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/sync/batch_semaphore.rs": [
          "direct_import",
          "module_path_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/fs_write.rs": {
      "primary_targets": [
        "tokio/src/fs/write.rs"
      ],
      "secondary_targets": [
        "tokio/src/fs/read_to_string.rs"
      ],
      "evidence": {
        "tokio/src/fs/write.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/io_async_read.rs": {
      "primary_targets": [
        "tokio/src/io/async_read.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/io/async_read.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/io_driver.rs": {
      "primary_targets": [
        "tokio/src/io/poll_evented.rs"
      ],
      "secondary_targets": [
        "tokio/src/net/tcp/listener.rs",
        "tokio/src/runtime/builder.rs"
      ],
      "evidence": {
        "tokio/src/io/poll_evented.rs": [
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/io_read_buf.rs": {
      "primary_targets": [
        "tokio/src/io/util/read_buf.rs",
        "tokio/src/io/read_buf.rs"
      ],
      "secondary_targets": [
        "tokio/src/io/util/async_read_ext.rs"
      ],
      "evidence": {
        "tokio/src/io/util/read_buf.rs": [
          "filename_match",
          "symbol_assertion"
        ],
        "tokio/src/io/read_buf.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/signal_drop_recv.rs": {
      "primary_targets": [
        "tokio/src/signal/unix.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/signal/unix.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/tcp_connect.rs": {
      "primary_targets": [
        "tokio/src/net/tcp/stream.rs"
      ],
      "secondary_targets": [
        "tokio/src/net/tcp/listener.rs"
      ],
      "evidence": {
        "tokio/src/net/tcp/stream.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/time_panic.rs": {
      "primary_targets": [
        "tokio/src/time/interval.rs",
        "tokio/src/time/timeout.rs"
      ],
      "secondary_targets": [
        "tokio/src/time/instant.rs",
        "tokio/src/runtime/builder.rs"
      ],
      "evidence": {
        "tokio/src/time/interval.rs": [
          "direct_import",
          "symbol_assertion"
        ],
        "tokio/src/time/timeout.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      }
    },
    "tokio-util/tests/abort_on_drop.rs": {
      "primary_targets": [
        "tokio-util/src/task/abort_on_drop.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio-util/src/task/abort_on_drop.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio-util/tests/task_join_queue.rs": {
      "primary_targets": [
        "tokio-util/src/task/join_queue.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio-util/src/task/join_queue.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/fs_open_options.rs": {
      "primary_targets": [
        "tokio/src/fs/open_options.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/fs/open_options.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/fs_open_options_windows.rs": {
      "primary_targets": [
        "tokio/src/fs/open_options.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/fs/open_options.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/tests/sync_broadcast_weak.rs": {
      "primary_targets": [
        "tokio/src/sync/broadcast.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/sync/broadcast.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      }
    },
    "tokio/src/runtime/tests/loom_blocking.rs": {
      "primary_targets": [
        "tokio/src/runtime/blocking/pool.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/runtime/blocking/pool.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed mapping to runtime.rs"
    },
    "tokio/src/runtime/tests/loom_current_thread/yield_now.rs": {
      "primary_targets": [
        "tokio/src/runtime/park.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/runtime/park.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed mapping to runtime.rs"
    },
    "tokio/src/runtime/tests/loom_multi_thread.rs": {
      "primary_targets": [
        "tokio/src/runtime/scheduler/multi_thread/worker.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/runtime/scheduler/multi_thread/worker.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed mapping to runtime.rs"
    },
    "tokio/src/runtime/tests/loom_multi_thread/yield_now.rs": {
      "primary_targets": [
        "tokio/src/runtime/park.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/runtime/park.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed mapping to runtime.rs"
    },
    "tokio/tests/duplex_stream.rs": {
      "primary_targets": [
        "tokio/src/io/util/mem.rs"
      ],
      "secondary_targets": [
        "tokio/src/io/util/async_read_ext.rs",
        "tokio/src/io/util/async_write_ext.rs"
      ],
      "evidence": {
        "tokio/src/io/util/mem.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed async_read_ext/async_write_ext mapping. True target is DuplexStream in mem.rs"
    },
    "tokio/tests/io_chain.rs": {
      "primary_targets": [
        "tokio/src/io/util/chain.rs"
      ],
      "secondary_targets": [
        "tokio/src/io/util/async_read_ext.rs"
      ],
      "evidence": {
        "tokio/src/io/util/chain.rs": [
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter removed async_read_ext mapping. True primary is chain.rs (specific io util)"
    },
    "tokio/tests/io_read_exact.rs": {
      "primary_targets": [
        "tokio/src/io/util/read_exact.rs"
      ],
      "secondary_targets": [
        "tokio/src/io/util/async_read_ext.rs"
      ],
      "evidence": {
        "tokio/src/io/util/read_exact.rs": [
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter removed async_read_ext mapping. True primary is read_exact.rs"
    },
    "tokio/tests/io_read_to_string.rs": {
      "primary_targets": [
        "tokio/src/io/util/read_to_string.rs"
      ],
      "secondary_targets": [
        "tokio/src/io/util/async_read_ext.rs"
      ],
      "evidence": {
        "tokio/src/io/util/read_to_string.rs": [
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter removed async_read_ext mapping. True primary is read_to_string.rs"
    },
    "tokio/tests/io_repeat.rs": {
      "primary_targets": [
        "tokio/src/io/util/repeat.rs"
      ],
      "secondary_targets": [
        "tokio/src/io/util/async_read_ext.rs"
      ],
      "evidence": {
        "tokio/src/io/util/repeat.rs": [
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed async_read_ext mapping. True primary is repeat.rs"
    },
    "tokio/tests/io_sink.rs": {
      "primary_targets": [
        "tokio/src/io/util/sink.rs"
      ],
      "secondary_targets": [
        "tokio/src/io/util/async_write_ext.rs"
      ],
      "evidence": {
        "tokio/src/io/util/sink.rs": [
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed async_write_ext mapping. True primary is sink.rs"
    },
    "tokio/tests/rt_worker_index.rs": {
      "primary_targets": [
        "tokio/src/runtime/mod.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/runtime/mod.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed mapping to runtime.rs (ambiguous target)"
    },
    "tokio/tests/task_abort.rs": {
      "primary_targets": [
        "tokio/src/runtime/task/join.rs"
      ],
      "secondary_targets": [
        "tokio/src/runtime/task/abort.rs",
        "tokio/src/runtime/builder.rs"
      ],
      "evidence": {
        "tokio/src/runtime/task/join.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter correctly removed builder.rs mapping. True primary is JoinHandle::abort in join.rs"
    },
    "tokio/tests/task_hooks.rs": {
      "primary_targets": [
        "tokio/src/runtime/builder.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/runtime/builder.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      },
      "notes": "fan-out filter INCORRECTLY removed builder.rs mapping. Builder IS the SUT (task hook config API)"
    },
    "tokio/tests/fs_uring.rs": {
      "primary_targets": [
        "tokio/src/fs/open_options.rs"
      ],
      "secondary_targets": [
        "tokio/src/runtime/builder.rs"
      ],
      "evidence": {
        "tokio/src/fs/open_options.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      },
      "notes": "FN: cfg-gated (io-uring). All imports through tokio:: barrel"
    },
    "tokio/tests/macros_select.rs": {
      "primary_targets": [
        "tokio/src/macros/select.rs"
      ],
      "secondary_targets": [
        "tokio/src/sync/oneshot.rs"
      ],
      "evidence": {
        "tokio/src/macros/select.rs": [
          "symbol_assertion",
          "filename_match"
        ]
      },
      "notes": "FN: tokio::select! macro invocation, not use-path import"
    },
    "tokio/tests/macros_test.rs": {
      "primary_targets": [
        "tokio-macros/src/entry.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio-macros/src/entry.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "FN: cross-crate proc macro (#[tokio::test]). No use-path import"
    },
    "tokio/tests/rt_common.rs": {
      "primary_targets": [
        "tokio/src/runtime/mod.rs",
        "tokio/src/runtime/builder.rs"
      ],
      "secondary_targets": [
        "tokio/src/sync/oneshot.rs",
        "tokio/src/task/mod.rs"
      ],
      "evidence": {
        "tokio/src/runtime/mod.rs": [
          "filename_match",
          "symbol_assertion"
        ],
        "tokio/src/runtime/builder.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "FN: imports inside macro_rules! body (rt_test! macro)"
    },
    "tokio/tests/sync_broadcast.rs": {
      "primary_targets": [
        "tokio/src/sync/broadcast.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/sync/broadcast.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "FN: import through tokio:: barrel (use tokio::sync::broadcast)"
    },
    "tokio/tests/sync_oneshot.rs": {
      "primary_targets": [
        "tokio/src/sync/oneshot.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/sync/oneshot.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "FN: import through tokio:: barrel (use tokio::sync::oneshot)"
    },
    "tokio/tests/sync_panic.rs": {
      "primary_targets": [
        "tokio/src/sync/broadcast.rs",
        "tokio/src/sync/mpsc/mod.rs",
        "tokio/src/sync/oneshot.rs",
        "tokio/src/sync/mutex.rs",
        "tokio/src/sync/rwlock.rs",
        "tokio/src/sync/semaphore.rs"
      ],
      "secondary_targets": [
        "tokio/src/runtime/builder.rs"
      ],
      "evidence": {
        "tokio/src/sync/broadcast.rs": [
          "direct_import",
          "symbol_assertion"
        ],
        "tokio/src/sync/mpsc/mod.rs": [
          "direct_import",
          "symbol_assertion"
        ],
        "tokio/src/sync/oneshot.rs": [
          "direct_import",
          "symbol_assertion"
        ],
        "tokio/src/sync/mutex.rs": [
          "direct_import",
          "symbol_assertion"
        ],
        "tokio/src/sync/rwlock.rs": [
          "direct_import",
          "symbol_assertion"
        ],
        "tokio/src/sync/semaphore.rs": [
          "direct_import",
          "symbol_assertion"
        ]
      },
      "notes": "FN: all imports through tokio:: barrel. Multi-target panic test"
    },
    "tokio/tests/task_blocking.rs": {
      "primary_targets": [
        "tokio/src/task/blocking.rs"
      ],
      "secondary_targets": [
        "tokio/src/runtime/mod.rs"
      ],
      "evidence": {
        "tokio/src/task/blocking.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "FN: import through tokio:: barrel (use tokio::task)"
    },
    "tokio/tests/time_wasm.rs": {
      "primary_targets": [
        "tokio/src/time/instant.rs"
      ],
      "secondary_targets": [
        "tokio/src/runtime/builder.rs"
      ],
      "evidence": {
        "tokio/src/time/instant.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "FN: no use statements, fully-qualified inline paths. cfg(wasm32)"
    },
    "tokio/tests/unwindsafe.rs": {
      "primary_targets": [
        "*(none)*"
      ],
      "secondary_targets": [],
      "evidence": {},
      "notes": "Trait-bound test (UnwindSafe). No behavioral assertions. Not a spec test."
    },
    "tokio-util/src/sync/tests/mod.rs": {
      "primary_targets": [
        "*(self)*"
      ],
      "secondary_targets": [],
      "evidence": {},
      "notes": "Inline test module organizer in src/. Not a standard test file"
    },
    "tokio/src/runtime/tests/loom_multi_thread/queue.rs": {
      "primary_targets": [
        "tokio/src/runtime/scheduler/multi_thread/queue.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/runtime/scheduler/multi_thread/queue.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "Inline loom test for multi_thread queue. src/ test dir"
    },
    "tokio/src/sync/tests/loom_atomic_waker.rs": {
      "primary_targets": [
        "tokio/src/sync/task/atomic_waker.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/sync/task/atomic_waker.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "Inline loom test. src/ test dir"
    },
    "tokio/src/sync/tests/loom_oneshot.rs": {
      "primary_targets": [
        "tokio/src/sync/oneshot.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/sync/oneshot.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "Inline loom test. src/ test dir"
    },
    "tokio/src/sync/tests/loom_watch.rs": {
      "primary_targets": [
        "tokio/src/sync/watch.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio/src/sync/watch.rs": [
          "direct_import",
          "filename_match",
          "symbol_assertion"
        ]
      },
      "notes": "Inline loom test. src/ test dir"
    },
    "tokio-stream/tests/stream_chain.rs": {
      "primary_targets": [
        "tokio-stream/src/stream_ext/chain.rs"
      ],
      "secondary_targets": [
        "tokio-stream/src/iter.rs"
      ],
      "evidence": {
        "tokio-stream/src/stream_ext/chain.rs": [
          "symbol_assertion",
          "filename_match"
        ]
      },
      "notes": "Same-crate (tokio-stream). Barrel import"
    },
    "tokio-stream/tests/stream_empty.rs": {
      "primary_targets": [
        "tokio-stream/src/empty.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio-stream/src/empty.rs": [
          "symbol_assertion",
          "filename_match"
        ]
      },
      "notes": "Same-crate (tokio-stream). Barrel import"
    },
    "tokio-stream/tests/stream_iter.rs": {
      "primary_targets": [
        "tokio-stream/src/iter.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio-stream/src/iter.rs": [
          "symbol_assertion",
          "filename_match"
        ]
      },
      "notes": "Same-crate (tokio-stream). Barrel import"
    },
    "tokio-stream/tests/stream_panic.rs": {
      "primary_targets": [
        "tokio-stream/src/stream_ext/chunks_timeout.rs"
      ],
      "secondary_targets": [
        "tokio-stream/src/iter.rs"
      ],
      "evidence": {
        "tokio-stream/src/stream_ext/chunks_timeout.rs": [
          "symbol_assertion"
        ]
      },
      "notes": "Same-crate. No filename match (stream_panic has no prod file)"
    },
    "tokio-util/tests/io_reader_stream.rs": {
      "primary_targets": [
        "tokio-util/src/io/reader_stream.rs"
      ],
      "secondary_targets": [],
      "evidence": {
        "tokio-util/src/io/reader_stream.rs": [
          "symbol_assertion",
          "filename_match"
        ]
      },
      "notes": "Same-crate (tokio-util). Inline import, no top-level use"
    }
  }
}
```
