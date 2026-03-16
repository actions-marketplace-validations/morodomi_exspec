use std::path::{Path, PathBuf};

/// A single path alias entry from tsconfig paths.
///
/// For `"@app/*": ["src/*"]`, the split is:
///   prefix = "@app/"  (everything before the `*`)
///   suffix = ""       (everything after the `*`, often empty)
///   targets = [("src/", "")]  (prefix / suffix pairs for each target)
#[derive(Debug, Clone)]
pub struct PathAlias {
    pub prefix: String,
    pub suffix: String,
    pub targets: Vec<(String, String)>,
}

/// Parsed tsconfig paths configuration.
#[derive(Debug, Clone)]
pub struct TsconfigPaths {
    pub base_url: PathBuf,
    pub aliases: Vec<PathAlias>,
}

impl TsconfigPaths {
    /// Parse tsconfig from JSON string with the directory containing the tsconfig.
    ///
    /// Returns `None` when:
    /// - JSON is invalid (includes JSON5 / comments)
    /// - `compilerOptions` is absent
    /// - `paths` is absent or empty
    pub fn from_str(json: &str, tsconfig_dir: &Path) -> Option<Self> {
        Self::from_str_depth(json, tsconfig_dir, 0)
    }

    fn from_str_depth(json: &str, tsconfig_dir: &Path, depth: usize) -> Option<Self> {
        let value: serde_json::Value = serde_json::from_str(json).ok()?;

        // Parse base from extends (max 3 levels)
        let parent_paths: Vec<PathAlias> = if depth < 3 {
            if let Some(extends_val) = value.get("extends").and_then(|v| v.as_str()) {
                // Only handle relative paths; skip npm packages
                if extends_val.starts_with("./") || extends_val.starts_with("../") {
                    let extends_path = tsconfig_dir.join(extends_val);
                    if let Ok(content) = std::fs::read_to_string(&extends_path) {
                        let parent_dir = extends_path.parent().unwrap_or(tsconfig_dir);
                        if let Some(parent) = Self::from_str_depth(&content, parent_dir, depth + 1)
                        {
                            parent.aliases
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let compiler_options = value.get("compilerOptions")?;

        // Determine base_url
        let base_url =
            if let Some(base_url_str) = compiler_options.get("baseUrl").and_then(|v| v.as_str()) {
                tsconfig_dir.join(base_url_str)
            } else {
                tsconfig_dir.to_path_buf()
            };

        // Parse paths
        let paths_obj = compiler_options.get("paths").and_then(|v| v.as_object());

        let mut child_aliases: Vec<PathAlias> = if let Some(paths) = paths_obj {
            paths
                .iter()
                .map(|(pattern, targets_val)| {
                    let (prefix, suffix) = split_wildcard(pattern);
                    let targets = targets_val
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|t| t.as_str())
                                .map(split_wildcard)
                                .collect()
                        })
                        .unwrap_or_default();
                    PathAlias {
                        prefix,
                        suffix,
                        targets,
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        // Merge: start from parent, then child overrides by prefix key
        // Child paths override parent paths with the same pattern key
        let mut merged: Vec<PathAlias> = Vec::new();
        let child_prefixes: std::collections::HashSet<String> =
            child_aliases.iter().map(|a| a.prefix.clone()).collect();
        for parent_alias in parent_paths {
            if !child_prefixes.contains(&parent_alias.prefix) {
                merged.push(parent_alias);
            }
        }
        merged.append(&mut child_aliases);

        // Return None only if there are no aliases at all and no parent aliases
        if merged.is_empty() {
            return None;
        }

        Some(TsconfigPaths {
            base_url,
            aliases: merged,
        })
    }

    /// Resolve an import specifier against the path aliases.
    ///
    /// Returns the first matching resolved `PathBuf`, or `None` if no alias matches.
    pub fn resolve_alias(&self, specifier: &str) -> Option<PathBuf> {
        for alias in &self.aliases {
            if alias.suffix.is_empty() && alias.prefix == specifier {
                // Exact match (no wildcard in pattern)
                // Use first target
                if let Some((target_prefix, target_suffix)) = alias.targets.first() {
                    if target_suffix.is_empty() {
                        return Some(self.base_url.join(target_prefix));
                    } else {
                        // target has wildcard but pattern doesn't — unusual, skip
                        continue;
                    }
                }
            } else if !alias.prefix.is_empty()
                && specifier.starts_with(&alias.prefix)
                && specifier.ends_with(&alias.suffix)
                && specifier.len() >= alias.prefix.len() + alias.suffix.len()
            {
                // Wildcard match
                let wildcard_start = alias.prefix.len();
                let wildcard_end = if alias.suffix.is_empty() {
                    specifier.len()
                } else {
                    specifier.len() - alias.suffix.len()
                };
                if wildcard_start > wildcard_end {
                    continue;
                }
                let wildcard = &specifier[wildcard_start..wildcard_end];

                // Return first matching target
                if let Some((target_prefix, target_suffix)) = alias.targets.first() {
                    let resolved = format!("{target_prefix}{wildcard}{target_suffix}");
                    return Some(self.base_url.join(&resolved));
                }
            }
        }
        None
    }
}

/// Split a pattern on the first `*` into (prefix, suffix).
/// If no `*`, returns (pattern, "").
fn split_wildcard(pattern: &str) -> (String, String) {
    if let Some(idx) = pattern.find('*') {
        let prefix = pattern[..idx].to_string();
        let suffix = pattern[idx + 1..].to_string();
        (prefix, suffix)
    } else {
        (pattern.to_string(), String::new())
    }
}

/// Discover `tsconfig.json` by walking up from the given directory.
///
/// Returns the path of the first `tsconfig.json` found, or `None` if the
/// filesystem root is reached without finding one.
pub fn discover_tsconfig(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();
    for _ in 0..10 {
        let candidate = current.join("tsconfig.json");
        if candidate.exists() {
            return Some(candidate);
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break,
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // TP-01: basic alias parse
    #[test]
    fn test_parse_basic_alias() {
        // Given: tsconfig JSON with one alias @app/* -> src/*
        let json = r#"{"compilerOptions":{"baseUrl":".","paths":{"@app/*":["src/*"]}}}"#;
        let dir = TempDir::new().unwrap();

        // When: parse
        let result = TsconfigPaths::from_str(json, dir.path());

        // Then: 1 alias, prefix="@app/", targets=[("src/","")]
        let tc = result.expect("expected Some(TsconfigPaths)");
        assert_eq!(tc.aliases.len(), 1, "expected 1 alias");
        let alias = &tc.aliases[0];
        assert_eq!(alias.prefix, "@app/");
        assert_eq!(
            alias.targets,
            vec![("src/".to_string(), "".to_string())],
            "expected targets=[('src/', '')]"
        );
    }

    // TP-02: multiple targets
    #[test]
    fn test_parse_multiple_targets() {
        // Given: paths with two targets for the same alias
        let json = r#"{"compilerOptions":{"baseUrl":".","paths":{"@app/*":["src/*","lib/*"]}}}"#;
        let dir = TempDir::new().unwrap();

        // When: parse
        let result = TsconfigPaths::from_str(json, dir.path());

        // Then: targets.len() == 2
        let tc = result.expect("expected Some(TsconfigPaths)");
        let alias = &tc.aliases[0];
        assert_eq!(alias.targets.len(), 2, "expected 2 targets");
    }

    // TP-03: baseUrl defaults to tsconfig_dir when omitted
    #[test]
    fn test_base_url_defaults_to_tsconfig_dir() {
        // Given: JSON with no baseUrl
        let json = r#"{"compilerOptions":{"paths":{"@app/*":["src/*"]}}}"#;
        let dir = TempDir::new().unwrap();
        let tsconfig_dir = dir.path();

        // When: parse
        let result = TsconfigPaths::from_str(json, tsconfig_dir);

        // Then: base_url == tsconfig_dir (canonicalized)
        let tc = result.expect("expected Some(TsconfigPaths)");
        assert_eq!(
            tc.base_url, tsconfig_dir,
            "expected base_url to equal tsconfig_dir"
        );
    }

    // TP-04: exact match (no wildcard) resolves correctly
    #[test]
    fn test_exact_match_no_wildcard() {
        // Given: alias without wildcard @config -> src/config/index
        let dir = TempDir::new().unwrap();
        let json =
            r#"{"compilerOptions":{"baseUrl":".","paths":{"@config":["src/config/index"]}}}"#;
        let tc = TsconfigPaths::from_str(json, dir.path()).expect("expected Some");

        // When: resolve exact specifier
        let result = tc.resolve_alias("@config");

        // Then: resolves to base_url/src/config/index
        let expected = dir.path().join("src/config/index");
        assert_eq!(result, Some(expected), "expected exact match resolution");
    }

    // TP-05: extends chain inherits paths from base
    #[test]
    fn test_extends_chain_inherits_paths() {
        // Given: base tsconfig with @base/* -> base_src/*
        //        child tsconfig extends base, has no paths of its own
        let dir = TempDir::new().unwrap();

        let base_json = r#"{"compilerOptions":{"baseUrl":".","paths":{"@base/*":["base_src/*"]}}}"#;
        let base_path = dir.path().join("tsconfig.base.json");
        fs::write(&base_path, base_json).unwrap();

        let child_json = r#"{"extends":"./tsconfig.base.json","compilerOptions":{"baseUrl":"."}}"#;
        let child_path = dir.path().join("tsconfig.json");
        fs::write(&child_path, child_json).unwrap();

        // When: parse child
        let child_source = fs::read_to_string(&child_path).unwrap();
        let result = TsconfigPaths::from_str(&child_source, dir.path());

        // Then: child inherits @base/* alias from base
        let tc = result.expect("expected Some(TsconfigPaths) with inherited paths");
        assert!(
            tc.aliases.iter().any(|a| a.prefix == "@base/"),
            "expected @base/ alias inherited from base, got {:?}",
            tc.aliases
        );
    }

    // TP-06: extends child overrides base paths
    #[test]
    fn test_extends_child_overrides() {
        // Given: base has @app/* -> lib/*
        //        child extends base and overrides @app/* -> src/*
        let dir = TempDir::new().unwrap();

        let base_json = r#"{"compilerOptions":{"baseUrl":".","paths":{"@app/*":["lib/*"]}}}"#;
        let base_path = dir.path().join("tsconfig.base.json");
        fs::write(&base_path, base_json).unwrap();

        let child_json = r#"{"extends":"./tsconfig.base.json","compilerOptions":{"baseUrl":".","paths":{"@app/*":["src/*"]}}}"#;
        let child_path = dir.path().join("tsconfig.json");
        fs::write(&child_path, child_json).unwrap();

        // When: parse child
        let child_source = fs::read_to_string(&child_path).unwrap();
        let result = TsconfigPaths::from_str(&child_source, dir.path());

        // Then: @app/* resolves to src/*, not lib/*
        let tc = result.expect("expected Some(TsconfigPaths)");
        let app_alias = tc.aliases.iter().find(|a| a.prefix == "@app/");
        assert!(app_alias.is_some(), "expected @app/ alias");
        let targets = &app_alias.unwrap().targets;
        assert_eq!(
            targets,
            &[("src/".to_string(), "".to_string())],
            "expected child override src/, got {:?}",
            targets
        );
    }

    // TP-07: discover_tsconfig finds tsconfig.json in parent directory
    #[test]
    fn test_discover_tsconfig_in_parent() {
        // Given: parent/tsconfig.json exists, start from parent/sub/
        let dir = TempDir::new().unwrap();
        let parent = dir.path();
        let sub = parent.join("sub");
        fs::create_dir_all(&sub).unwrap();
        let tsconfig = parent.join("tsconfig.json");
        fs::write(&tsconfig, "{}").unwrap();

        // When: discover from sub/
        let result = discover_tsconfig(&sub);

        // Then: finds parent/tsconfig.json
        assert_eq!(
            result,
            Some(tsconfig),
            "expected to find tsconfig.json in parent"
        );
    }

    // TP-08: discover_tsconfig returns None when no tsconfig exists
    #[test]
    fn test_discover_tsconfig_none() {
        // Given: temp dir with no tsconfig.json anywhere
        let dir = TempDir::new().unwrap();

        // When: discover
        let result = discover_tsconfig(dir.path());

        // Then: None
        assert!(
            result.is_none(),
            "expected None when no tsconfig.json exists"
        );
    }

    // TP-09: resolve_alias returns None for non-matching specifier
    #[test]
    fn test_resolve_alias_no_match() {
        // Given: only @app/* alias configured
        let dir = TempDir::new().unwrap();
        let json = r#"{"compilerOptions":{"baseUrl":".","paths":{"@app/*":["src/*"]}}}"#;
        let tc = TsconfigPaths::from_str(json, dir.path()).expect("expected Some");

        // When: resolve a non-matching specifier
        let result = tc.resolve_alias("lodash");

        // Then: None
        assert!(
            result.is_none(),
            "expected None for non-alias specifier 'lodash'"
        );
    }

    // TP-10: resolve_alias with wildcard maps correctly
    #[test]
    fn test_resolve_alias_with_wildcard() {
        // Given: @app/* -> src/*
        let dir = TempDir::new().unwrap();
        let json = r#"{"compilerOptions":{"baseUrl":".","paths":{"@app/*":["src/*"]}}}"#;
        let tc = TsconfigPaths::from_str(json, dir.path()).expect("expected Some");

        // When: resolve @app/services/foo
        let result = tc.resolve_alias("@app/services/foo");

        // Then: base_url/src/services/foo
        let expected = dir.path().join("src/services/foo");
        assert_eq!(
            result,
            Some(expected),
            "expected wildcard resolution to src/services/foo"
        );
    }
}
