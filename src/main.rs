mod metadata;

use metadata::{ConditionNode, RuleSet};
use std::fs;
use toml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml_content = fs::read_to_string("rules_v2.toml")?;
    let ruleset: RuleSet = toml::from_str(&toml_content)?;

    println!("{:#?}", ruleset);

    for rule in &ruleset.rules {
        println!("Processing rule: {}", rule.name);
        for query in &rule.query {
            process_condition_node(query, 0);
        }
    }

    Ok(())
}

fn process_condition_node(condition: &ConditionNode, indent: usize) {
    let padding = " ".repeat(indent * 4);

    match condition {
        ConditionNode::InCondition { field, values } => {
            println!("{}IN condition: field={}, values={:?}", padding, field, values);
        }
        ConditionNode::NotInCondition { values } => {
            println!("{}NOT_IN condition: values={:?}", padding, values);
        }
        ConditionNode::Or { filter, operands } => {
            if let Some(filter) = filter {
                println!("{}OR condition: filter={}, values={:?}", padding, filter.filter, filter.values);
            }
            if let Some(ops) = operands {
                println!("{}OR group:", padding);
                for op in ops {
                    process_condition_node(op, indent + 1);
                }
            }
        }
        ConditionNode::And { operands } => {
            println!("{}AND group:", padding);
            for op in operands {
                process_condition_node(op, indent + 1);
            }
        }
    }
}
