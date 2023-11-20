use serde::{Deserialize, Serialize};

/// A struct containing the tables (inputs) and views for a program.
///
/// Parse from the JSON data-type of the DDL generated by the SQL compiler.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "proptest_derive", derive(proptest_derive::Arbitrary))]
pub struct ProgramSchema {
    #[cfg_attr(
        feature = "proptest_derive",
        proptest(strategy = "vec(any::<Relation>(), 0..2)")
    )]
    pub inputs: Vec<Relation>,
    #[cfg_attr(
        feature = "proptest_derive",
        proptest(strategy = "vec(any::<Relation>(), 0..2)")
    )]
    pub outputs: Vec<Relation>,
}

/// A SQL table or view. It has a name and a list of fields.
///
/// Matches the Calcite JSON format.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "proptest_derive", derive(proptest_derive::Arbitrary))]
pub struct Relation {
    #[cfg_attr(
        feature = "proptest_derive",
        proptest(regex = "relation1|relation2|relation3")
    )]
    pub name: String,
    #[cfg_attr(feature = "proptest_derive", proptest(value = "Vec::new()"))]
    pub fields: Vec<Field>,
}
/// A SQL field.
///
/// Matches the Calcite JSON format.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "proptest_derive", derive(proptest_derive::Arbitrary))]
pub struct Field {
    pub name: String,
    pub columntype: ColumnType,
}

/// A SQL column type description.
///
/// Matches the Calcite JSON format.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "proptest_derive", derive(proptest_derive::Arbitrary))]
pub struct ColumnType {
    #[serde(rename = "type")]
    /// Identifier for the type (e.g., `VARCHAR`, `BIGINT`, `ARRAY` etc.)
    pub typ: String,
    /// Does the type accept NULL values?
    pub nullable: bool,
    /// Precision of the type.
    ///
    /// # Examples
    /// - `VARCHAR` sets precision to `-1`.
    /// - `VARCHAR(255)` sets precision to `255`.
    /// - `BIGINT`, `DATE`, `FLOAT`, `DOUBLE`, `GEOMETRY`, etc. sets precision
    ///   to None
    /// - `TIME`, `TIMESTAMP` set precision to `0`.
    pub precision: Option<i64>,
    /// The scale of the type.
    ///
    /// # Example
    /// - `DECIMAL(1,2)` sets scale to `2`.
    pub scale: Option<i64>,
    /// A component of the type (if available).
    ///
    /// This is in a `Box` because it makes it a recursive types.
    ///
    /// For example, this would specify the `VARCHAR(20)` in the `VARCHAR(20)
    /// ARRAY` type.
    #[cfg_attr(feature = "proptest_derive", proptest(value = "None"))]
    pub component: Option<Box<ColumnType>>,
}
