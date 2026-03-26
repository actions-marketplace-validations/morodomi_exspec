use std::fs;
use tempfile::tempdir;

// TC-16: tempdir に pyproject.toml → exspec init <dir> で .exspec.toml が生成される (E2E)
#[test]
fn init_tc16_e2e_init_creates_exspec_toml() {
    // Given: tempdir に pyproject.toml を作成
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("pyproject.toml"), "").unwrap();

    // When: exspec init <dir> を実行
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_exspec"))
        .args(["init", dir.path().to_str().unwrap()])
        .output()
        .expect("failed to run exspec");

    // Then: .exspec.toml が生成される
    assert!(
        output.status.success(),
        "Expected exit 0, got: {}. stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        dir.path().join(".exspec.toml").exists(),
        "Expected .exspec.toml to exist"
    );
}

// TC-17: 既存 .exspec.toml あり → exspec init <dir> が exit code != 0
#[test]
fn init_tc17_e2e_init_fails_if_toml_exists() {
    // Given: tempdir に既存の .exspec.toml を作成
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("pyproject.toml"), "").unwrap();
    fs::write(dir.path().join(".exspec.toml"), "# existing").unwrap();

    // When: exspec init <dir> を実行
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_exspec"))
        .args(["init", dir.path().to_str().unwrap()])
        .output()
        .expect("failed to run exspec");

    // Then: exit code != 0
    assert!(
        !output.status.success(),
        "Expected non-zero exit code when .exspec.toml already exists, got: {}",
        output.status
    );
}

// TC-18: 既存 .exspec.toml あり + --force → .exspec.toml が上書きされる (E2E)
#[test]
fn init_tc18_e2e_init_force_overwrites_existing_toml() {
    // Given: tempdir に既存の .exspec.toml を作成
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("pyproject.toml"), "").unwrap();
    fs::write(dir.path().join(".exspec.toml"), "# existing").unwrap();

    // When: exspec init --force <dir> を実行
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_exspec"))
        .args(["init", "--force", dir.path().to_str().unwrap()])
        .output()
        .expect("failed to run exspec");

    // Then: 成功し、.exspec.toml の内容が新しいものになっている
    assert!(
        output.status.success(),
        "Expected exit 0 with --force, got: {}. stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    let content = fs::read_to_string(dir.path().join(".exspec.toml")).unwrap();
    assert!(
        content != "# existing",
        "Expected .exspec.toml to be overwritten, but content is still: {content}"
    );
}
