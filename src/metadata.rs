use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RuleSet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    pub type_: String,
    pub category: String,
    pub severity: String,
    pub description: String,
    pub query: Vec<ConditionNode>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "UPPERCASE")]
pub enum ConditionNode {
    #[serde(rename = "IN")]
    InCondition {
        field: String,
        values: Vec<String>,
    },

    #[serde(rename = "NOT_IN")]
    NotInCondition {
        values: Vec<String>,
    },

    #[serde(rename = "OR")]
    Or {
        #[serde(flatten)]
        filter: Option<FilterCondition>, // This is optional
        operands: Option<Vec<ConditionNode>>, // OR may contain operands
    },

    #[serde(rename = "AND")]
    And {
        operands: Vec<ConditionNode>, // AND must have operands
    },
}

#[derive(Debug, Deserialize)]
pub struct FilterCondition {
    pub field: Option<String>,  // 🔥 Optional because OR doesn't always have a field
    pub filter: String,
    pub values: Vec<String>,
}

