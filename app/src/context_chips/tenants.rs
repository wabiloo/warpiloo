use std::path::PathBuf;

const TENANTS_FILE_NAME: &str = "tenants";

fn tenants_file_path() -> Option<PathBuf> {
    let home = match std::env::var_os("BPKIO_HOME").filter(|home| !home.is_empty()) {
        Some(home) => PathBuf::from(home),
        None => dirs::home_dir()?.join(".bpkio"),
    };
    Some(home.join(TENANTS_FILE_NAME))
}

/// Returns the tenants configured on this machine, sorted alphabetically (case-insensitive).
pub fn list_tenants() -> Vec<String> {
    let mut tenants = tenants_file_path()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .map(|contents| parse_tenant_labels(&contents))
        .unwrap_or_default();
    sort_tenant_labels(&mut tenants);
    tenants
}

fn sort_tenant_labels(labels: &mut [String]) {
    labels.sort_by_cached_key(|label| label.to_lowercase());
}

/// Returns the `[section]` names of an INI file, in order.
///
/// Only section headers are read: the file also holds credentials, which must never be
/// retained or logged.
fn parse_tenant_labels(contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter_map(|line| {
            let header = line.strip_prefix('[')?;
            let label = header[..header.rfind(']')?].trim();
            (!label.is_empty()).then(|| label.to_string())
        })
        .collect()
}

#[cfg(test)]
#[path = "tenants_tests.rs"]
mod tests;
