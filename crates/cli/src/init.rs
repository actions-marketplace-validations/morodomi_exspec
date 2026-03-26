/// Detect frameworks by inspecting dependency files.
pub fn detect_frameworks(path: &std::path::Path, languages: &[String]) -> Vec<String> {
    let mut frameworks = std::collections::HashSet::new();

    for lang in languages {
        match lang.as_str() {
            "python" => {
                if path.join("conftest.py").exists() {
                    frameworks.insert("pytest");
                }
                if path.join("manage.py").exists() {
                    frameworks.insert("django");
                }
                // Check pyproject.toml and requirements.txt content
                let content = [
                    std::fs::read_to_string(path.join("pyproject.toml")).unwrap_or_default(),
                    std::fs::read_to_string(path.join("requirements.txt")).unwrap_or_default(),
                ]
                .join("\n")
                .to_lowercase();

                if content.contains("flask") {
                    frameworks.insert("flask");
                }
                if content.contains("fastapi") {
                    frameworks.insert("fastapi");
                }
                if content.contains("pytest") {
                    frameworks.insert("pytest");
                }
            }
            "typescript" => {
                let pkg_path = path.join("package.json");
                if let Ok(content) = std::fs::read_to_string(&pkg_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        let dev_deps = &json["devDependencies"];
                        let deps = &json["dependencies"];
                        if dev_deps.get("jest").is_some() {
                            frameworks.insert("jest");
                        }
                        if dev_deps.get("vitest").is_some() {
                            frameworks.insert("vitest");
                        }
                        if deps.get("@nestjs/core").is_some() {
                            frameworks.insert("nestjs");
                        }
                    }
                }
            }
            "php" => {
                let composer_path = path.join("composer.json");
                if let Ok(content) = std::fs::read_to_string(&composer_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        let require = &json["require"];
                        let require_dev = &json["require-dev"];
                        if require_dev.get("phpunit/phpunit").is_some() {
                            frameworks.insert("phpunit");
                        }
                        if require.get("laravel/framework").is_some() {
                            frameworks.insert("laravel");
                        }
                        if require_dev.get("pestphp/pest").is_some() {
                            frameworks.insert("pest");
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let mut result: Vec<String> = frameworks.into_iter().map(|s| s.to_string()).collect();
    result.sort();
    result
}

/// Get custom assertion patterns for detected frameworks.
pub fn get_custom_patterns(frameworks: &[String]) -> Vec<String> {
    let mut patterns = std::collections::HashSet::new();

    for fw in frameworks {
        match fw.as_str() {
            "laravel" => {
                patterns.insert("expects");
                patterns.insert("shouldBeCalled");
                patterns.insert("shouldReceive");
            }
            "pest" => {
                patterns.insert("expect(");
            }
            "pytest" => {
                patterns.insert("mock.assert_*");
            }
            _ => {}
        }
    }

    let mut result: Vec<String> = patterns.into_iter().map(|s| s.to_string()).collect();
    result.sort();
    result
}

/// Detect languages by checking for marker files in the given directory.
pub fn detect_languages(path: &std::path::Path) -> Vec<String> {
    let mut langs = std::collections::HashSet::new();

    let python_markers = ["pyproject.toml", "requirements.txt", "setup.py"];
    if python_markers.iter().any(|f| path.join(f).exists()) {
        langs.insert("python");
    }
    if path.join("package.json").exists() {
        langs.insert("typescript");
    }
    if path.join("composer.json").exists() {
        langs.insert("php");
    }
    if path.join("Cargo.toml").exists() {
        langs.insert("rust");
    }

    let mut result: Vec<String> = langs.into_iter().map(|s| s.to_string()).collect();
    result.sort();
    result
}

/// Run `exspec init` in the given directory.
/// Detects languages and writes a minimal `.exspec.toml`.
#[allow(dead_code)]
pub fn run_init(path: &std::path::Path, dry_run: bool, force: bool) {
    let out = path.join(".exspec.toml");
    if out.exists() && !force {
        eprintln!(
            "error: {} already exists. Use --force to overwrite.",
            out.display()
        );
        std::process::exit(1);
    }
    let langs = detect_languages(path);
    if langs.is_empty() {
        eprintln!(
            "warning: no supported language markers found in {}",
            path.display()
        );
        return;
    }
    let frameworks = detect_frameworks(path, &langs);
    let toml = generate_toml(&langs, &frameworks);
    if dry_run {
        print!("{toml}");
        return;
    }
    std::fs::write(&out, toml).expect("failed to write .exspec.toml");
    println!("Created {}", out.display());
}

/// Generate minimal .exspec.toml content from detected languages.
pub fn generate_toml(languages: &[String], frameworks: &[String]) -> String {
    // Build lang list
    let lang_entries: Vec<String> = languages.iter().map(|l| format!(r#""{l}""#)).collect();
    let lang_list = lang_entries.join(", ");

    // Build ignore list
    let mut ignores = std::collections::HashSet::new();
    for lang in languages {
        match lang.as_str() {
            "python" => {
                ignores.insert(".venv");
                ignores.insert("__pycache__");
            }
            "typescript" => {
                ignores.insert("node_modules");
                ignores.insert("dist");
                ignores.insert(".next");
            }
            "php" => {
                ignores.insert("vendor");
            }
            "rust" => {
                ignores.insert("target");
            }
            _ => {}
        }
    }
    let mut ignore_list: Vec<&str> = ignores.into_iter().collect();
    ignore_list.sort();
    let ignore_entries: Vec<String> = ignore_list.iter().map(|s| format!(r#""{s}""#)).collect();
    let ignore_str = ignore_entries.join(", ");

    let mut output = format!("# Generated by `exspec init`\n\n[general]\nlang = [{lang_list}]\n");

    // Add [assertions] section if custom_patterns exist
    let patterns = get_custom_patterns(frameworks);
    if !patterns.is_empty() {
        let pattern_entries: Vec<String> = patterns.iter().map(|p| format!(r#""{p}""#)).collect();
        let pattern_str = pattern_entries.join(", ");
        output.push_str(&format!(
            "\n[assertions]\ncustom_patterns = [{pattern_str}]\n"
        ));
    }

    output.push_str(&format!("\n[paths]\nignore = [{ignore_str}]\n"));

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // TC-01: pyproject.toml → ["python"]
    #[test]
    fn init_tc01_detect_python_via_pyproject_toml() {
        // Given: tempdir に pyproject.toml を作成
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("pyproject.toml"), "").unwrap();

        // When
        let result = detect_languages(dir.path());

        // Then
        assert_eq!(result, vec!["python".to_string()]);
    }

    // TC-02: package.json → ["typescript"]
    #[test]
    fn init_tc02_detect_typescript_via_package_json() {
        // Given: tempdir に package.json を作成
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();

        // When
        let result = detect_languages(dir.path());

        // Then
        assert_eq!(result, vec!["typescript".to_string()]);
    }

    // TC-03: composer.json → ["php"]
    #[test]
    fn init_tc03_detect_php_via_composer_json() {
        // Given: tempdir に composer.json を作成
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("composer.json"), "{}").unwrap();

        // When
        let result = detect_languages(dir.path());

        // Then
        assert_eq!(result, vec!["php".to_string()]);
    }

    // TC-04: Cargo.toml → ["rust"]
    #[test]
    fn init_tc04_detect_rust_via_cargo_toml() {
        // Given: tempdir に Cargo.toml を作成
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "").unwrap();

        // When
        let result = detect_languages(dir.path());

        // Then
        assert_eq!(result, vec!["rust".to_string()]);
    }

    // TC-05: package.json + pyproject.toml → ["python", "typescript"] (アルファベット順)
    #[test]
    fn init_tc05_detect_multiple_languages_sorted() {
        // Given: tempdir に package.json と pyproject.toml を作成
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        fs::write(dir.path().join("pyproject.toml"), "").unwrap();

        // When
        let result = detect_languages(dir.path());

        // Then: アルファベット順
        assert_eq!(result, vec!["python".to_string(), "typescript".to_string()]);
    }

    // TC-06: generate_toml(["python"]) → lang = ["python"] を含む
    #[test]
    fn init_tc06_generate_toml_single_language() {
        // Given
        let langs = vec!["python".to_string()];

        // When
        let result = generate_toml(&langs, &[]);

        // Then
        assert!(
            result.contains(r#"lang = ["python"]"#),
            "Expected result to contain `lang = [\"python\"]`, got: {result}"
        );
    }

    // TC-07: 空ディレクトリ → 空 Vec
    #[test]
    fn init_tc07_empty_directory_returns_empty_vec() {
        // Given: 空の tempdir
        let dir = tempdir().unwrap();

        // When
        let result = detect_languages(dir.path());

        // Then
        assert!(result.is_empty(), "Expected empty Vec, got: {result:?}");
    }

    // TC-08: generate_toml(["python", "typescript"]) → lang = ["python", "typescript"] かつ .venv と node_modules を含む
    #[test]
    fn init_tc08_generate_toml_multiple_languages_with_ignores() {
        // Given
        let langs = vec!["python".to_string(), "typescript".to_string()];

        // When
        let result = generate_toml(&langs, &[]);

        // Then
        assert!(
            result.contains(r#"lang = ["python", "typescript"]"#),
            "Expected result to contain `lang = [\"python\", \"typescript\"]`, got: {result}"
        );
        assert!(
            result.contains(".venv"),
            "Expected result to contain `.venv`, got: {result}"
        );
        assert!(
            result.contains("node_modules"),
            "Expected result to contain `node_modules`, got: {result}"
        );
    }

    // TC-09: conftest.py + pyproject.toml → detect_frameworks に "pytest" を含む
    #[test]
    fn init_tc09_detect_pytest_via_conftest_py() {
        // Given: tempdir に conftest.py と pyproject.toml を作成
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("conftest.py"), "").unwrap();
        fs::write(dir.path().join("pyproject.toml"), "").unwrap();

        // When
        let langs = vec!["python".to_string()];
        let result = detect_frameworks(dir.path(), &langs);

        // Then
        assert!(
            result.contains(&"pytest".to_string()),
            "Expected result to contain \"pytest\", got: {result:?}"
        );
    }

    // TC-10: manage.py + pyproject.toml → detect_frameworks に "django" を含む
    #[test]
    fn init_tc10_detect_django_via_manage_py() {
        // Given: tempdir に manage.py と pyproject.toml を作成
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("manage.py"), "").unwrap();
        fs::write(dir.path().join("pyproject.toml"), "").unwrap();

        // When
        let langs = vec!["python".to_string()];
        let result = detect_frameworks(dir.path(), &langs);

        // Then
        assert!(
            result.contains(&"django".to_string()),
            "Expected result to contain \"django\", got: {result:?}"
        );
    }

    // TC-11: package.json (devDependencies に jest) → detect_frameworks に "jest" を含む
    #[test]
    fn init_tc11_detect_jest_via_package_json() {
        // Given: tempdir に package.json を作成 (jest が devDependencies にある)
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("package.json"),
            r#"{"devDependencies":{"jest":"^29.0.0"}}"#,
        )
        .unwrap();

        // When
        let langs = vec!["typescript".to_string()];
        let result = detect_frameworks(dir.path(), &langs);

        // Then
        assert!(
            result.contains(&"jest".to_string()),
            "Expected result to contain \"jest\", got: {result:?}"
        );
    }

    // TC-12: composer.json (require に laravel/framework) → detect_frameworks に "laravel" を含む
    #[test]
    fn init_tc12_detect_laravel_via_composer_json() {
        // Given: tempdir に composer.json を作成 (laravel/framework が require にある)
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("composer.json"),
            r#"{"require":{"laravel/framework":"^10.0"}}"#,
        )
        .unwrap();

        // When
        let langs = vec!["php".to_string()];
        let result = detect_frameworks(dir.path(), &langs);

        // Then
        assert!(
            result.contains(&"laravel".to_string()),
            "Expected result to contain \"laravel\", got: {result:?}"
        );
    }

    // TC-13: frameworks = ["laravel"] → get_custom_patterns に ["expects", "shouldBeCalled", "shouldReceive"] を含む (ソート順)
    #[test]
    fn init_tc13_get_custom_patterns_for_laravel() {
        // Given
        let frameworks = vec!["laravel".to_string()];

        // When
        let mut result = get_custom_patterns(&frameworks);
        result.sort();

        // Then
        assert_eq!(
            result,
            vec![
                "expects".to_string(),
                "shouldBeCalled".to_string(),
                "shouldReceive".to_string()
            ],
            "Expected sorted patterns for laravel, got: {result:?}"
        );
    }

    // TC-14: frameworks = ["pytest"] → get_custom_patterns に ["mock.assert_*"] を含む
    #[test]
    fn init_tc14_get_custom_patterns_for_pytest() {
        // Given
        let frameworks = vec!["pytest".to_string()];

        // When
        let result = get_custom_patterns(&frameworks);

        // Then
        assert!(
            result.contains(&"mock.assert_*".to_string()),
            "Expected result to contain \"mock.assert_*\", got: {result:?}"
        );
    }

    // TC-15: langs = ["python"], frameworks = ["pytest"] → generate_toml に custom_patterns = ["mock.assert_*"] を含む
    #[test]
    fn init_tc15_generate_toml_with_custom_patterns_for_pytest() {
        // Given
        let langs = vec!["python".to_string()];
        let frameworks = vec!["pytest".to_string()];

        // When
        let result = generate_toml(&langs, &frameworks);

        // Then
        assert!(
            result.contains(r#"custom_patterns = ["mock.assert_*"]"#),
            "Expected result to contain `custom_patterns = [\"mock.assert_*\"]`, got: {result}"
        );
    }
}
