use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[cfg(feature = "testing")]
use proptest::{collection::vec, prelude::any};

/// Returns canonical form of a SQL identifier:
///
/// - If id is _not_ quoted, then it is interpreted as a case-insensitive
///   identifier and is converted to the lowercase representation
/// - If id _is_ quoted, then it is a case-sensitive identifier and is returned
///   as is, without quotes. No other processing is done on the inner string,
///   e.g., un-escaping quotes.
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
#[derive(Serialize, ToSchema, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "testing", derive(proptest_derive::Arbitrary))]
pub struct Field {
    pub name: String,
    #[serde(default)]
    pub case_sensitive: bool,
    pub columntype: ColumnType,
}

/// Thanks to the brain-dead Calcite schema, if we are deserializing a field, the type options
/// end up inside the Field struct.
///
/// This helper struct is used to deserialize the Field struct.
impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
    where
        D: Deserializer<'de>,
    {
        const fn default_is_struct() -> Option<SqlType> {
            Some(SqlType::Struct)
        }

        #[derive(Debug, Clone, Deserialize)]
        struct FieldHelper {
            name: Option<String>,
            #[serde(default)]
            case_sensitive: bool,
            columntype: Option<ColumnType>,
            #[serde(rename = "type")]
            #[serde(default = "default_is_struct")]
            typ: Option<SqlType>,
            nullable: Option<bool>,
            precision: Option<i64>,
            scale: Option<i64>,
            component: Option<Box<ColumnType>>,
            fields: Option<serde_json::Value>,
        }

        fn helper_to_field(helper: FieldHelper) -> Field {
            let columntype = if let Some(ctype) = helper.columntype {
                ctype
            } else if let Some(serde_json::Value::Array(fields)) = helper.fields {
                let fields = fields
                    .into_iter()
                    .map(|field| {
                        let field: FieldHelper = serde_json::from_value(field).unwrap();
                        helper_to_field(field)
                    })
                    .collect::<Vec<Field>>();

                ColumnType {
                    typ: helper.typ.unwrap_or(SqlType::Null),
                    nullable: helper.nullable.unwrap_or(false),
                    precision: helper.precision,
                    scale: helper.scale,
                    component: helper.component,
                    fields: Some(fields),
                }
            } else if let Some(serde_json::Value::Object(obj)) = helper.fields {
                serde_json::from_value(serde_json::Value::Object(obj))
                    .expect("Failed to deserialize object")
            } else {
                ColumnType {
                    typ: helper.typ.unwrap_or(SqlType::Null),
                    nullable: helper.nullable.unwrap_or(false),
                    precision: helper.precision,
                    scale: helper.scale,
                    component: helper.component,
                    fields: None,
                }
            };

            Field {
                name: helper.name.unwrap(),
                case_sensitive: helper.case_sensitive,
                columntype,
            }
        }

        let helper = FieldHelper::deserialize(deserializer)?;
        Ok(helper_to_field(helper))
    }
}

/// The specified units for SQL Interval types.
///
/// `INTERVAL 1 DAY`, `INTERVAL 1 DAY TO HOUR`, `INTERVAL 1 DAY TO MINUTE`,
/// would yield `Day`, `DayToHour`, `DayToMinute`, as the `IntervalUnit` respectively.
#[derive(Serialize, Deserialize, ToSchema, Debug, Eq, PartialEq, Clone, Copy)]
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
#[derive(Serialize, ToSchema, Debug, Eq, PartialEq, Clone, Copy)]
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
    /// SQL `INTERVAL ... X` type where `X` is a unit.
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

impl<'de> Deserialize<'de> for SqlType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match value.to_lowercase().as_str() {
            "interval_day" => Ok(SqlType::Interval(IntervalUnit::Day)),
            "interval_day_hour" => Ok(SqlType::Interval(IntervalUnit::DayToHour)),
            "interval_day_minute" => Ok(SqlType::Interval(IntervalUnit::DayToMinute)),
            "interval_day_second" => Ok(SqlType::Interval(IntervalUnit::DayToSecond)),
            "interval_hour" => Ok(SqlType::Interval(IntervalUnit::Hour)),
            "interval_hour_minute" => Ok(SqlType::Interval(IntervalUnit::HourToMinute)),
            "interval_hour_second" => Ok(SqlType::Interval(IntervalUnit::HourToSecond)),
            "interval_minute" => Ok(SqlType::Interval(IntervalUnit::Minute)),
            "interval_minute_second" => Ok(SqlType::Interval(IntervalUnit::MinuteToSecond)),
            "interval_month" => Ok(SqlType::Interval(IntervalUnit::Month)),
            "interval_second" => Ok(SqlType::Interval(IntervalUnit::Second)),
            "interval_year" => Ok(SqlType::Interval(IntervalUnit::Year)),
            "interval_year_month" => Ok(SqlType::Interval(IntervalUnit::YearToMonth)),
            "boolean" => Ok(SqlType::Boolean),
            "tinyint" => Ok(SqlType::TinyInt),
            "smallint" => Ok(SqlType::SmallInt),
            "integer" => Ok(SqlType::Int),
            "bigint" => Ok(SqlType::BigInt),
            "real" => Ok(SqlType::Real),
            "double" => Ok(SqlType::Double),
            "decimal" => Ok(SqlType::Decimal),
            "char" => Ok(SqlType::Char),
            "varchar" => Ok(SqlType::Varchar),
            "binary" => Ok(SqlType::Binary),
            "varbinary" => Ok(SqlType::Varbinary),
            "time" => Ok(SqlType::Time),
            "date" => Ok(SqlType::Date),
            "timestamp" => Ok(SqlType::Timestamp),
            "array" => Ok(SqlType::Array),
            "struct" => Ok(SqlType::Struct),
            "null" => Ok(SqlType::Null),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown SQL type: {}",
                value
            ))),
        }
    }
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
            SqlType::Interval(_) => "INTERVAL",
            SqlType::Array => "ARRAY",
            SqlType::Struct => "STRUCT",
            SqlType::Null => "NULL",
        }
    }
}

/// It so happens that when the type field is missing in the Calcite schema, it's a struct,
/// so we use it as the default.
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

#[cfg(test)]
mod tests {
    use crate::program_schema::SqlType;

    #[test]
    fn deserialize_interval_types() {
        use super::IntervalUnit::*;
        use super::SqlType::*;

        let schema = r#"
{
  "inputs" : [ {
    "name" : "sales",
    "case_sensitive" : false,
    "fields" : [ {
      "name" : "sales_id",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTEGER",
        "nullable" : true
      }
    }, {
      "name" : "customer_id",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTEGER",
        "nullable" : true
      }
    }, {
      "name" : "amount",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "DECIMAL",
        "nullable" : true,
        "precision" : 10,
        "scale" : 2
      }
    }, {
      "name" : "sale_date",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "DATE",
        "nullable" : true
      }
    } ],
    "primary_key" : [ "sales_id" ]
  } ],
  "outputs" : [ {
    "name" : "salessummary",
    "case_sensitive" : false,
    "fields" : [ {
      "name" : "customer_id",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTEGER",
        "nullable" : true
      }
    }, {
      "name" : "total_sales",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "DECIMAL",
        "nullable" : true,
        "precision" : 38,
        "scale" : 2
      }
    }, {
      "name" : "interval_day",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_DAY",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_day_to_hour",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_DAY_HOUR",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_day_to_minute",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_DAY_MINUTE",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_day_to_second",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_DAY_SECOND",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_hour",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_HOUR",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_hour_to_minute",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_HOUR_MINUTE",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_hour_to_second",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_HOUR_SECOND",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_minute",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_MINUTE",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_minute_to_second",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_MINUTE_SECOND",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_month",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_MONTH",
        "nullable" : false
      }
    }, {
      "name" : "interval_second",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_SECOND",
        "nullable" : false,
        "precision" : 2,
        "scale" : 6
      }
    }, {
      "name" : "interval_year",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_YEAR",
        "nullable" : false
      }
    }, {
      "name" : "interval_year_to_month",
      "case_sensitive" : false,
      "columntype" : {
        "type" : "INTERVAL_YEAR_MONTH",
        "nullable" : false
      }
    } ]
  } ]
}
"#;

        let schema: super::ProgramSchema = serde_json::from_str(schema).unwrap();
        let types = schema
            .outputs
            .iter()
            .flat_map(|r| r.fields.iter().map(|f| f.columntype.typ));
        let expected_types = [
            Int,
            Decimal,
            Interval(Day),
            Interval(DayToHour),
            Interval(DayToMinute),
            Interval(DayToSecond),
            Interval(Hour),
            Interval(HourToMinute),
            Interval(HourToSecond),
            Interval(Minute),
            Interval(MinuteToSecond),
            Interval(Month),
            Interval(Second),
            Interval(Year),
            Interval(YearToMonth),
        ];

        assert_eq!(types.collect::<Vec<_>>(), &expected_types);
    }

    #[test]
    fn serialize_struct_schemas() {
        let schema = r#"{
  "inputs" : [ {
    "name" : "PERS",
    "case_sensitive" : false,
    "fields" : [ {
      "name" : "P0",
      "case_sensitive" : false,
      "columntype" : {
        "fields" : [ {
          "type" : "VARCHAR",
          "nullable" : true,
          "precision" : 30,
          "name" : "FIRSTNAME"
        }, {
          "type" : "VARCHAR",
          "nullable" : true,
          "precision" : 30,
          "name" : "LASTNAME"
        }, {
          "fields" : {
            "fields" : [ {
              "type" : "VARCHAR",
              "nullable" : true,
              "precision" : 30,
              "name" : "STREET"
            }, {
              "type" : "VARCHAR",
              "nullable" : true,
              "precision" : 30,
              "name" : "CITY"
            }, {
              "type" : "CHAR",
              "nullable" : true,
              "precision" : 2,
              "name" : "STATE"
            }, {
              "type" : "VARCHAR",
              "nullable" : true,
              "precision" : 6,
              "name" : "POSTAL_CODE"
            } ],
            "nullable" : false
          },
          "nullable" : false,
          "name" : "ADDRESS"
        } ],
        "nullable" : false
      }
    }]
  } ],
  "outputs" : [ ]
}
"#;
        let schema: super::ProgramSchema = serde_json::from_str(schema).unwrap();
        eprintln!("{:#?}", schema);
        let pers = schema.inputs.iter().find(|r| r.name == "PERS").unwrap();
        let p0 = pers.fields.iter().find(|f| f.name == "P0").unwrap();
        assert_eq!(p0.columntype.typ, SqlType::Struct);
        let p0_fields = p0.columntype.fields.as_ref().unwrap();
        assert_eq!(p0_fields[0].columntype.typ, SqlType::Varchar);
        assert_eq!(p0_fields[1].columntype.typ, SqlType::Varchar);
        assert_eq!(p0_fields[2].columntype.typ, SqlType::Struct);
        assert_eq!(p0_fields[2].name, "ADDRESS");
        let address = &p0_fields[2].columntype.fields.as_ref().unwrap();
        assert_eq!(address.len(), 4);
        assert_eq!(address[0].name, "STREET");
        assert_eq!(address[0].columntype.typ, SqlType::Varchar);
        assert_eq!(address[1].columntype.typ, SqlType::Varchar);
        assert_eq!(address[2].columntype.typ, SqlType::Char);
        assert_eq!(address[3].columntype.typ, SqlType::Varchar);
    }
}
