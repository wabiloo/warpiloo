use super::*;

#[test]
fn parses_section_names_only() {
    let contents = "\
[default]
api_key = secret

[acme-prod]
username = bob
password = hunter2

  [indented-is-a-continuation]
[ spaced ]
[]
; [commented]
";
    assert_eq!(
        parse_tenant_labels(contents),
        vec!["default", "acme-prod", "spaced"]
    );
}

#[test]
fn empty_file_has_no_tenants() {
    assert!(parse_tenant_labels("").is_empty());
}

#[test]
fn sorts_labels_case_insensitively() {
    let mut labels = vec![
        "zeta".to_string(),
        "Beta".to_string(),
        "alpha".to_string(),
        "default".to_string(),
    ];
    sort_tenant_labels(&mut labels);
    assert_eq!(labels, vec!["alpha", "Beta", "default", "zeta"]);
}
