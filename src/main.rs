use serde::{Deserialize, Serialize};

pub fn main() {
    println!("Nothing to see here, move along");
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<ReferentialConstraint>` tag
///
/// # Child Nodes
/// `1:1 Principal`<br>
/// `1:1 Dependent`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReferentialConstraint {
    pub principal: Principal,
    pub dependent: Dependent,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Dependent>` tag
///
/// # Child Nodes
/// `1:n PropertyRef`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dependent {
    pub role: String,

    #[serde(rename = "PropertyRef", default)]
    pub property_refs: Vec<PropertyRef>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<Principal>` tag
//
// # Child Nodes
// `1:n PropertyRef`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Principal {
    pub role: String,

    #[serde(rename = "PropertyRef", default)]
    pub property_refs: Vec<PropertyRef>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<PropertyRef>` tag
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    pub name: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
