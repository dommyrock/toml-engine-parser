use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RuleSet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub category: String,
    pub severity: String,
    pub description: String,
    pub query: Vec<ConditionNode>, // Updated to match "query" instead of "matches"
}

#[derive(Debug, Deserialize)]
#[serde(untagged)] // Allows handling both simple and compound conditions
pub enum ConditionNode {
    Simple {
        field: Option<String>,
        op: String,
        values: Option<Vec<String>>,
        filter: Option<String>,
        value: Option<String>,
    },
    Compound {
        op: String,
        operands: Vec<ConditionNode>,
    },
}

