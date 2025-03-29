use serde::Deserialize;

/// Top-level structure to hold the rules
#[derive(Debug, Deserialize)]
pub struct Config {
    rules: Vec<Rule>,
}

/// Individual rule definition
#[derive(Debug, Deserialize)]
struct Rule {
    name: String,
    #[serde(rename = "type")]
    rule_type: String,
    category: String,
    severity: String,
    description: String,
    query: Vec<Query>,
}

/// Base query structure
#[derive(Debug, Deserialize)]
struct Query {
    field: Option<String>,
    op: String,
    values: Option<Vec<String>>,
    filter: Option<String>,
    operands: Option<Vec<QueryOperand>>,
}

/// Nested query operand structure
#[derive(Debug, Deserialize)]
struct QueryOperand {
    op: String,
    filter: Option<String>,
    values: Option<Vec<String>>,
}

mod tests {
    use super::*;
    use std::fs;
    use toml;

    #[test]
    fn test_rule_set_parsing() {
        let toml_str = r#"
[[rules]]
name = "SSH related file changes"
type = "FileOpened"
category = "credential_access"
severity = "high"
description = "Identifies a Secure Shell (SSH) client or server process creating or writing to a known SSH backdoor log file. Adversaries may modify SSH-related binaries for persistence or credential access via patching sensitive functions to enable unauthorized access or to log SSH credentials for exfiltration."
[[rules.query]]
field = "header.image"
op = "IN"
values = ["/usr/sbin/sshd", "/usr/bin/ssh"]
[[rules.query]]
field = "payload.filename"
op = "AND"
 [[rules.query.operands]]
op = "OR"
filter = "STARTS_WITH"
values = ["~", "." ]
 [[rules.query.operands]]
op = "OR"
filter = "ENDS_WITH"
values = ["~"]
 [[rules.query.operands]]
op = "NOT_IN"
values = [".cache", ".viminfo", ".bash_history", ".google_authenticator", ".jelenv", ".csvignore", ".rtreport"]
 [[rules.query.operands]]
op = "OR"
filter = "STARTS_WITH"
values = ["/private/etc/", "/usr/share/", "/usr/include/", "/usr/local/include/", "/private/tmp/", "/private/var/tmp/", "/usr/tmp/", "/usr/share/man/", "/usr/local/share/"]
 [[rules.query.operands]]
op = "OR"
filter = "ENDS_WITH"
values = [".in", ".out", ".ini", ".h", ".gz", ".so", ".sock", ".sync", ".0", ".1", ".2", ".3", ".4", ".5", ".6", ".7", ".8", ".9"]
 [[rules.query.operands]]
op = "IN"
values = [
"/private/etc/ssh/.sshd_auth",
"/usr/bin/ssd",
"/private/var/opt/power",
"/private/etc/ssh/ssh_known_hosts",
"/private/var/html/lol",
"/private/var/log/utmp",
"/private/var/lib",
"/var/run/sshd/sshd.pid",
"/var/run/nscd/ns.pid",
"/var/run/udev/ud.pid",
"/var/run/udevd.pid"
 ]
"#;

        let decoded: Config = toml::from_str(toml_str).unwrap();
        println!("{:#?}", decoded);
    }
}
