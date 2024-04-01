use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[cfg(feature = "testing")]
use proptest::{collection::vec, prelude::any};
use serde_arrow::_impl::arrow::datatypes::TimeUnit;

/// Returns canonical form of a SQL identifier:
/// - If id is _not_ quoted, then it is interpreted as a case-insensitive
///   identifier and is converted to the lowercase representation
/// - If id _is_ quoted, then it is a case-sensitive identifier and is returned
///   as is, without quotes. No other processing is done on the inner string,
///   e.g., unescaping quotes.
pub fn canonical_identifier(id: &str) -> String {
    if id.starts_with('"') && id.ends_with('"') && id.len() >= 2 {
        id[1..id.len() - 1].to_string()
    } else {
        id.to_lowercase()
    }
}

/// A struct containing the tables (inputs) and views for a program.
///
/// Parse from the JSON data-type of the DDL generated by the SQL compiler.
#[derive(Serialize, Deserialize, ToSchema, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
pub struct ProgramSchema {
    #[cfg_attr(
        feature = "testing",
        proptest(strategy = "vec(any::<Relation>(), 0..2)")
    )]
    pub inputs: Vec<Relation>,
    #[cfg_attr(
        feature = "testing",
        proptest(strategy = "vec(any::<Relation>(), 0..2)")
    )]
    pub outputs: Vec<Relation>,
}

/// A SQL table or view. It has a name and a list of fields.
///
/// Matches the Calcite JSON format.
#[derive(Serialize, Deserialize, ToSchema, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
pub struct Relation {
    // This field should only be accessed via the `name()` method.
    #[cfg_attr(feature = "testing", proptest(regex = "relation1|relation2|relation3"))]
    name: String,
    #[serde(default)]
    pub case_sensitive: bool,
    #[cfg_attr(feature = "testing", proptest(value = "Vec::new()"))]
    pub fields: Vec<Field>,
}

impl Relation {
    pub fn new(name: &str, case_sensitive: bool, fields: Vec<Field>) -> Self {
        Self {
            name: name.to_string(),
            case_sensitive,
            fields,
        }
    }

    /// Returns canonical name of the relation: case-insensitive names are
    /// converted to lowercase; case-sensitive names returned as is.
    pub fn name(&self) -> String {
        if self.case_sensitive {
            self.name.clone()
        } else {
            self.name.to_lowercase()
        }
    }
}

/// A SQL field.
///
/// Matches the SQL compiler JSON format.
#[derive(Serialize, Deserialize, ToSchema, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
pub struct Field {
    pub name: String,
    #[serde(default)]
    pub case_sensitive: bool,
    pub columntype: ColumnType,
}

/// The specified units for SQL Interval types.
///
/// `INTERVAL 1 DAY`, `INTERVAL 1 DAY TO HOUR`, `INTERVAL 1 DAY TO MINUTE`,
/// would yield `Day`, `DayToHour`, `DayToMinute`, as the `IntervalUnit` respectively.
#[derive(Serialize, Deserialize, ToSchema, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
#[serde(rename_all = "UPPERCASE")]
pub enum IntervalUnit {
    /// Unit for `INTERVAL ... DAY`.
    Day,
    /// Unit for `INTERVAL ... DAY TO HOUR`.
    DayToHour,
    /// Unit for `INTERVAL ... DAY TO MINUTE`.
    DayToMinute,
    /// Unit for `INTERVAL ... DAY TO SECOND`.
    DayToSecond,
    /// Unit for `INTERVAL ... HOUR`.
    Hour,
    /// Unit for `INTERVAL ... HOUR TO MINUTE`.
    HourToMinute,
    /// Unit for `INTERVAL ... HOUR TO SECOND`.
    HourToSecond,
    /// Unit for `INTERVAL ... MINUTE`.
    Minute,
    /// Unit for `INTERVAL ... MINUTE TO SECOND`.
    MinuteToSecond,
    /// Unit for `INTERVAL ... MONTH`.
    Month,
    /// Unit for `INTERVAL ... SECOND`.
    Second,
    /// Unit for `INTERVAL ... YEAR`.
    Year,
    /// Unit for `INTERVAL ... YEAR TO MONTH`.
    YearToMonth,
}

/// The available SQL types as specified in `CREATE` statements.
#[derive(Serialize, Deserialize, ToSchema, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
pub enum SqlType {
    /// SQL `BOOLEAN` type.
    #[serde(rename = "BOOLEAN")]
    Boolean,
    /// SQL `TINYINT` type.
    #[serde(rename = "TINYINT")]
    TinyInt,
    /// SQL `SMALLINT` or `INT2` type.
    #[serde(rename = "SMALLINT")]
    SmallInt,
    /// SQL `INTEGER`, `INT`, `SIGNED`, `INT4` type.
    #[serde(rename = "INTEGER")]
    Int,
    /// SQL `BIGINT` or `INT64` type.
    #[serde(rename = "BIGINT")]
    BigInt,
    /// SQL `REAL` or `FLOAT4` or `FLOAT32` type.
    #[serde(rename = "REAL")]
    Real,
    /// SQL `DOUBLE` or `FLOAT8` or `FLOAT64` type.
    #[serde(rename = "DOUBLE")]
    Double,
    /// SQL `DECIMAL` or `DEC` or `NUMERIC` type.
    #[serde(rename = "DECIMAL")]
    Decimal,
    /// SQL `CHAR(n)` or `CHARACTER(n)` type.
    #[serde(rename = "CHAR")]
    Char,
    /// SQL `VARCHAR`, `CHARACTER VARYING`, `TEXT`, or `STRING` type.
    #[serde(rename = "VARCHAR")]
    Varchar,
    /// SQL `BINARY(n)` type.
    #[serde(rename = "BINARY")]
    Binary,
    /// SQL `VARBINARY` or `BYTEA` type.
    #[serde(rename = "VARBINARY")]
    Varbinary,
    /// SQL `TIME` type.
    #[serde(rename = "TIME")]
    Time,
    /// SQL `DATE` type.
    #[serde(rename = "DATE")]
    Date,
    /// SQL `TIMESTAMP` type.
    #[serde(rename = "TIMESTAMP")]
    Timestamp,
    /// SQL `INTERVAL ... X` type.
    Interval(IntervalUnit),
    /// SQL `ARRAY` type.
    #[serde(rename = "ARRAY")]
    Array,
    /// A complex SQL struct type (`CREATE TYPE x ...`).
    #[serde(rename = "STRUCT")]
    Struct,
    /// SQL `NULL` type.
    #[serde(rename = "NULL")]
    Null,
}

impl From<SqlType> for &'static str {
    fn from(value: SqlType) -> &'static str {
        match value {
            SqlType::Boolean => "BOOLEAN",
            SqlType::TinyInt => "TINYINT",
            SqlType::SmallInt => "SMALLINT",
            SqlType::Int => "INTEGER",
            SqlType::BigInt => "BIGINT",
            SqlType::Real => "REAL",
            SqlType::Double => "DOUBLE",
            SqlType::Decimal => "DECIMAL",
            SqlType::Char => "CHAR",
            SqlType::Varchar => "VARCHAR",
            SqlType::Binary => "BINARY",
            SqlType::Varbinary => "VARBINARY",
            SqlType::Time => "TIME",
            SqlType::Date => "DATE",
            SqlType::Timestamp => "TIMESTAMP",
            SqlType::Interval() => "INTERVAL",
            SqlType::Array => "ARRAY",
            SqlType::Struct => "STRUCT",
            SqlType::Null => "NULL",
        }
    }
}

impl<S: AsRef<str>> From<S> for SqlType {
    fn from(s: S) -> Self {
        match s.as_ref().to_lowercase().as_str() {
            "boolean" => SqlType::Boolean,
            "tinyint" => SqlType::TinyInt,
            "smallint" => SqlType::SmallInt,
            "integer" => SqlType::Int,
            "bigint" => SqlType::BigInt,
            "real" => SqlType::Real,
            "double" => SqlType::Double,
            "decimal" => SqlType::Decimal,
            "char" => SqlType::Char,
            "varchar" => SqlType::Varchar,
            "binary" => SqlType::Binary,
            "varbinary" => SqlType::Varbinary,
            "time" => SqlType::Time,
            "date" => SqlType::Date,
            "timestamp" => SqlType::Timestamp,
            "interval" => SqlType::Interval,
            "array" => SqlType::Array,
            "type" => SqlType::Struct,
            "null" => SqlType::Null,
            _ => panic!("Found unknown SQL type: {}", s.as_ref()),
        }
    }
}

const fn default_is_struct() -> SqlType {
    SqlType::Struct
}

/// A SQL column type description.
///
/// Matches the Calcite JSON format.
#[derive(Serialize, Deserialize, ToSchema, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
pub struct ColumnType {
    /// Identifier for the type (e.g., `VARCHAR`, `BIGINT`, `ARRAY` etc.)
    #[serde(rename = "type")]
    #[serde(default = "default_is_struct")]
    pub typ: SqlType,
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
    #[cfg_attr(feature = "testing", proptest(value = "None"))]
    pub component: Option<Box<ColumnType>>,
    /// The fields of the type (if available).
    ///
    /// For example this would specify the fields of a `CREATE TYPE` construct.
    ///
    /// ```sql
    /// CREATE TYPE person_typ AS (
    ///   firstname       VARCHAR(30),
    ///   lastname        VARCHAR(30),
    ///   address         ADDRESS_TYP
    /// );
    /// ```
    ///
    /// Would lead to the following `fields` value:
    ///
    /// ```sql
    /// [
    ///  ColumnType { name: "firstname, ... },
    ///  ColumnType { name: "lastname", ... },
    ///  ColumnType { name: "address", fields: [ ... ] }
    /// ]
    /// ```
    #[cfg_attr(feature = "testing", proptest(value = "Some(Vec::new())"))]
    pub fields: Option<Vec<Field>>,
}
