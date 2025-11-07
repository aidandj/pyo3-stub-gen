use pyo3_stub_gen::pyproject::PyProject;

#[test]
fn test_pyo3_stub_gen_config() {
    let toml_content = r#"
[project]
name = "test-package"

[tool.pyo3-stub-gen]
module = "my_module"
output-dir = "stubs"
"#;

    let pyproject: PyProject = toml::from_str(toml_content).unwrap();

    assert_eq!(pyproject.project.name, "test-package");

    let tool = pyproject.tool.as_ref().unwrap();
    let pyo3_stub_gen = tool.pyo3_stub_gen.as_ref().unwrap();

    assert_eq!(pyo3_stub_gen.module.as_deref(), Some("my_module"));
    assert_eq!(pyo3_stub_gen.output_dir.as_deref(), Some("stubs"));
}

#[test]
fn test_pyo3_stub_gen_config_optional() {
    let toml_content = r#"
[project]
name = "test-package"
"#;

    let pyproject: PyProject = toml::from_str(toml_content).unwrap();

    assert_eq!(pyproject.project.name, "test-package");
    assert!(pyproject.tool.is_none() || pyproject.tool.as_ref().unwrap().pyo3_stub_gen.is_none());
}

#[test]
fn test_pyo3_stub_gen_with_maturin() {
    let toml_content = r#"
[project]
name = "test-package"

[tool.maturin]
python-source = "python"
module-name = "custom_module"

[tool.pyo3-stub-gen]
module = "my_module"
output-dir = "stubs"
"#;

    let pyproject: PyProject = toml::from_str(toml_content).unwrap();

    assert_eq!(pyproject.project.name, "test-package");

    let tool = pyproject.tool.as_ref().unwrap();

    // Check maturin config
    let maturin = tool.maturin.as_ref().unwrap();
    assert_eq!(maturin.python_source.as_deref(), Some("python"));
    assert_eq!(maturin.module_name.as_deref(), Some("custom_module"));

    // Check pyo3-stub-gen config
    let pyo3_stub_gen = tool.pyo3_stub_gen.as_ref().unwrap();
    assert_eq!(pyo3_stub_gen.module.as_deref(), Some("my_module"));
    assert_eq!(pyo3_stub_gen.output_dir.as_deref(), Some("stubs"));
}

#[test]
fn test_pyo3_stub_gen_partial_config() {
    // Test with only module specified
    let toml_content = r#"
[project]
name = "test-package"

[tool.pyo3-stub-gen]
module = "my_module"
"#;

    let pyproject: PyProject = toml::from_str(toml_content).unwrap();

    let tool = pyproject.tool.as_ref().unwrap();
    let pyo3_stub_gen = tool.pyo3_stub_gen.as_ref().unwrap();

    assert_eq!(pyo3_stub_gen.module.as_deref(), Some("my_module"));
    assert!(pyo3_stub_gen.output_dir.is_none());
}

#[test]
fn test_pyo3_stub_gen_output_dir_only() {
    // Test with only output-dir specified
    let toml_content = r#"
[project]
name = "test-package"

[tool.pyo3-stub-gen]
output-dir = "stubs"
"#;

    let pyproject: PyProject = toml::from_str(toml_content).unwrap();

    let tool = pyproject.tool.as_ref().unwrap();
    let pyo3_stub_gen = tool.pyo3_stub_gen.as_ref().unwrap();

    assert!(pyo3_stub_gen.module.is_none());
    assert_eq!(pyo3_stub_gen.output_dir.as_deref(), Some("stubs"));
}

#[test]
fn test_output_dir_method_prioritizes_pyo3_stub_gen() {
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    // Create a temporary directory with a pyproject.toml
    let temp_dir = TempDir::new().unwrap();
    let toml_path = temp_dir.path().join("pyproject.toml");

    let toml_content = r#"
[project]
name = "test-package"

[tool.maturin]
python-source = "python"

[tool.pyo3-stub-gen]
output-dir = "stubs"
"#;

    let mut file = fs::File::create(&toml_path).unwrap();
    file.write_all(toml_content.as_bytes()).unwrap();
    drop(file);

    let pyproject = PyProject::parse_toml(&toml_path).unwrap();

    // output_dir() should return the pyo3-stub-gen output-dir
    let output = pyproject.output_dir().unwrap();
    assert_eq!(output, temp_dir.path().join("stubs"));
}

#[test]
fn test_output_dir_method_falls_back_to_python_source() {
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    // Create a temporary directory with a pyproject.toml
    let temp_dir = TempDir::new().unwrap();
    let toml_path = temp_dir.path().join("pyproject.toml");

    let toml_content = r#"
[project]
name = "test-package"

[tool.maturin]
python-source = "python"
"#;

    let mut file = fs::File::create(&toml_path).unwrap();
    file.write_all(toml_content.as_bytes()).unwrap();
    drop(file);

    let pyproject = PyProject::parse_toml(&toml_path).unwrap();

    // output_dir() should fall back to python_source
    let output = pyproject.output_dir().unwrap();
    assert_eq!(output, temp_dir.path().join("python"));
}

#[test]
fn test_output_dir_method_returns_none_when_no_config() {
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    // Create a temporary directory with a minimal pyproject.toml
    let temp_dir = TempDir::new().unwrap();
    let toml_path = temp_dir.path().join("pyproject.toml");

    let toml_content = r#"
[project]
name = "test-package"
"#;

    let mut file = fs::File::create(&toml_path).unwrap();
    file.write_all(toml_content.as_bytes()).unwrap();
    drop(file);

    let pyproject = PyProject::parse_toml(&toml_path).unwrap();

    // output_dir() should return None
    assert!(pyproject.output_dir().is_none());
}
