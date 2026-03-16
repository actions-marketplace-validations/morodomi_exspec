# observe: Failure Boundaries

This document catalogs the known failure boundaries of `exspec observe` -- cases where the static test-to-code mapping produces false negatives (FN). Each boundary is verified by a boundary specification test in `observe.rs` (`boundary_b{N}_*` tests).

## Summary

| # | Boundary | Root Cause | Impact | Fixability |
|---|----------|-----------|--------|------------|
| B1 | Namespace re-export | `re_export.scm` lacks `namespace_export` pattern | FN | Medium (query addition) |
| B2 | Cross-package barrel import | Non-relative paths excluded from import tracing | FN | Hard (requires tsconfig/node_modules resolution) |
| B3 | tsconfig path alias | ~~Same as B2~~ **Resolved in 8c-3** | ~~FN~~ Resolved | tsconfig.json paths parsing |
| B4 | Interface/enum filter side-effect | `is_non_sut_helper` filters primary test targets | FN | Medium (context-aware filtering) |
| B5 | Dynamic import | `import_mapping.scm` only captures static `import` statements | FN | Low (rare in test code) |
| B6 | Monorepo scan_root boundary | Resolved paths outside scan_root have no production file match | FN | By design |

## B1: Namespace re-export (`export * as Ns from`)

**Syntax**: `export * as Validators from './validators'`

**Why it fails**: `re_export.scm` handles two patterns:
1. Named re-export: `export { Foo } from './module'`
2. Wildcard re-export: `export * from './module'`

Namespace re-export (`export * as Ns from`) produces a `namespace_export` AST node, which neither pattern matches.

**Impact**: When a barrel file uses namespace grouping, all symbols behind that namespace become invisible to import tracing. This is uncommon in NestJS but appears in some utility packages.

**Tests**: `boundary_b1_ns_reexport_not_captured`, `boundary_b1_ns_reexport_mapping_miss`

**Fix path**: Add a third pattern to `re_export.scm` targeting `namespace_export` nodes. Requires deciding whether to resolve the namespace (Ns.Foo -> Foo) or treat the entire namespace as an opaque symbol.

## B2: Cross-package barrel import (non-relative path)

**Syntax**: `import { Foo } from '@org/common'`

**Why it fails**: `extract_imports` filters out any module specifier that does not start with `./` or `../`. Package-scoped imports (`@org/common`, `@nestjs/common`) are indistinguishable from third-party dependencies without `node_modules` resolution.

**Impact**: This is the primary FN source (7/11 FN in NestJS eval). Monorepo packages that import from sibling packages via package names lose all import-tracing signal.

**Tests**: `boundary_b2_non_relative_import_skipped`, `boundary_b2_cross_pkg_barrel_unresolvable`

**Fix path**: Parse `tsconfig.json` paths or resolve `node_modules` symlinks (common in Yarn/pnpm workspaces). High complexity, high reward.

## B3: tsconfig path alias (`@app/*`) -- Resolved in 8c-3

**Syntax**: `import { FooService } from '@app/services/foo.service'`

**Root cause**: `@app/` does not start with `./` or `../`, so `extract_imports` skips it. The path alias is defined in `tsconfig.json` (e.g., `"@app/*": ["src/*"]`).

**Resolution (Phase 8c-3)**: `tsconfig.rs` module parses `tsconfig.json` `compilerOptions.paths` + `baseUrl`, resolves aliases to absolute paths, and feeds them into the existing file resolution pipeline. Supports `extends` chains (relative paths only, max 3 levels) and auto-discovers `tsconfig.json` by walking up from scan_root.

**Remaining limitations**:
- JSON5 tsconfig (comments, trailing commas) not supported -- standard JSON only
- `extends` referencing npm packages (`@tsconfig/node18`) ignored
- `baseUrl`-only resolution (without `paths`) not supported
- B2 (cross-package barrel via `node_modules`) remains unresolved

**Tests**: `boundary_b3_tsconfig_alias_not_resolved` (without tsconfig -- FN by design), `boundary_b3_tsconfig_alias_resolved` (with tsconfig -- resolved)

## B4: Interface/enum filter side-effect

**Syntax**: `import { RouteParamtypes } from './route-paramtypes.enum'`

**Why it fails**: `is_non_sut_helper` filters files matching `*.enum.*`, `*.interface.*`, and `*.exception.*`. This is an intentional design choice -- these files are typically type definitions, not testable units. However, when a test directly imports an enum or interface as its **primary test target**, the filter creates a false negative.

**Impact**: 4/11 FN in NestJS eval. The trade-off is deliberate: filtering these reduces noise (many tests import enums as incidental dependencies), but the minority case of "testing the enum itself" is lost.

**Tests**: `boundary_b4_enum_primary_target_filtered`, `boundary_b4_interface_primary_target_filtered`

**Fix path**: Context-aware filtering that distinguishes "incidental import" from "primary test target" (e.g., if the test name or describe block references the enum/interface name). Medium complexity.

## B5: Dynamic import (`import()`)

**Syntax**: `const m = await import('./user.service')`

**Why it fails**: `import_mapping.scm` only captures static `import { ... } from '...'` statements. Dynamic `import()` expressions produce a `call_expression` AST node with `import` as the function, which the query does not match.

**Impact**: Rare in test files. Most test frameworks use static imports. Dynamic imports appear occasionally in lazy-loading tests or module isolation patterns.

**Tests**: `boundary_b5_dynamic_import_not_extracted`

**Fix path**: Add a `call_expression` pattern to `import_mapping.scm` targeting `import(specifier)`. Low priority due to rarity.

## B6: Monorepo scan_root boundary

**Syntax**: `import { Shared } from '../../common/src/shared'` (where `shared.ts` is outside scan_root)

**Why it fails**: `map_test_files_with_imports` only considers files within `production_files` (which are collected from scan_root). A relative import that resolves to a file outside scan_root will resolve successfully at the filesystem level, but there is no matching entry in `canonical_to_idx` to map it to.

**Impact**: By design. scan_root defines the analysis boundary. Files outside it are not part of the production codebase being analyzed.

**Tests**: `boundary_b6_import_outside_scan_root`

**Fix path**: None needed. This is intentional scoping. Users should set scan_root to the monorepo root if they want cross-package visibility (at the cost of including all packages).

## Applicability Scope

Based on these boundaries, observe is most reliable for:

- **Single-package TypeScript projects** (no B2/B3/B6 impact)
- **Projects using relative imports or tsconfig path aliases** (no B2 impact, B3 resolved)
- **Projects with standard barrel patterns** (`export { X } from` or `export * from`, no B1 impact)

Observe is **less reliable** for:
- **Monorepo workspaces** with cross-package imports via node_modules (B2, B6)
- **Projects heavy on namespace re-exports** (B1)
- **Projects where enums/interfaces are primary test targets** (B4)
