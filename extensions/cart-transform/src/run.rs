query RunInput {
  cart {
    lines {
      id
      quantity
      merchandise {
        ... on ProductVariant {
          id
          sku
        }
      }
      attribute(key: "_bundle_id") {
        value
      }
    }
  }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Output {
    pub operations: Vec<Operation>,
}

pub fn run(input: RunInput) -> Result<Output, Box<dyn std::error::Error>> {
    let mut operations = Vec::new();

    // 1. Group lines by their "_bundle_id" attribute
    let bundle_groups = group_lines_by_bundle_id(input.cart.lines);

    for (bundle_id, lines) in bundle_groups {
        if lines.len() > 1 {
            // 2. Create the Merge Operation
            operations.push(Operation::Merge(MergeOperation {
                parent_variant_id: "gid://shopify/ProductVariant/YOUR_PARENT_ID".to_string(),
                child_lines: lines.iter().map(|l| l.id.clone()).collect(),
                title: Some(format!("Custom Bundle: {}", bundle_id)),
                price: Some(calculate_bundle_price(&lines)), // Custom pricing logic here
            }));
        }
    }

    Ok(Output { operations })
}
