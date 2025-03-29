mod metadata;

use metadata::{ConditionNode, RuleSet};
use toml;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml_content = fs::read_to_string("rules.toml")?;
    let ruleset: RuleSet = toml::from_str(&toml_content)?;
    
    println!("{:#?}", ruleset);
    
    // Process the parsed query
    for rule in &ruleset.rules {
        println!("Processing rule: {}", rule.name);
        for condition in &rule.query {
            process_condition_node(condition, 0);
        }
    }
    
    Ok(())
}

fn process_condition_node(operand: &ConditionNode, indent: usize) {
    let padding = " ".repeat(indent * 2);
    match operand {
        ConditionNode::Simple { field, op, values, filter, value } => {
            println!(
                "{}Simple condition: field={:?}, op={}, values={:?}, filter={:?}, value={:?}",
                padding, field, op, values, filter, value
            );
        }
        ConditionNode::Compound { op, operands } => {
            println!("{}Compound condition: op={}", padding, op);
            for operand in operands {
                process_condition_node(operand, indent + 1);
            }
        }
    }
}

