use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

/// JSON parser configuration.
///
/// Describes the shape of an input JSON stream.
///
/// # Examples
///
/// A configuration with `update_format="raw"` and `array=false`
/// is used to parse a stream of JSON objects without any envelope
/// that get inserted in the input table.
///
/// ```json
/// {"b": false, "i": 100, "s": "foo"}
/// {"b": true, "i": 5, "s": "bar"}
/// ```
///
/// A configuration with `update_format="insert_delete"` and
/// `array=false` is used to parse a stream of JSON data change events
/// in the insert/delete format:
///
/// ```json
/// {"delete": {"b": false, "i": 15, "s": ""}}
/// {"insert": {"b": false, "i": 100, "s": "foo"}}
/// ```
///
/// A configuration with `update_format="insert_delete"` and
/// `array=true` is used to parse a stream of JSON arrays
/// where each array contains multiple data change events in
/// the insert/delete format.
///
/// ```json
/// [{"insert": {"b": true, "i": 0}}, {"delete": {"b": false, "i": 100, "s": "foo"}}]
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema)]
#[serde(default)]
pub struct JsonParserConfig {
    /// JSON update format.
    pub update_format: JsonUpdateFormat,

    /// Specifies JSON encoding used for individual table records.
    pub json_flavor: JsonFlavor,

    /// Set to `true` if updates in this stream are packaged into JSON arrays.
    ///
    /// # Example
    ///
    /// ```json
    /// [{"b": true, "i": 0},{"b": false, "i": 100, "s": "foo"}]
    /// ```
    pub array: bool,
}

/// Supported JSON data change event formats.
///
/// Each element in a JSON-formatted input stream specifies
/// an update to one or more records in an input table.  We support
/// several different ways to represent such updates.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq, Eq, ToSchema)]
pub enum JsonUpdateFormat {
    /// Insert/delete format.
    ///
    /// Each element in the input stream consists of an "insert" or "delete"
    /// command and a record to be inserted to or deleted from the input table.
    ///
    /// # Example
    ///
    /// ```json
    /// {"insert": {"column1": "hello, world!", "column2": 100}}
    /// ```
    #[default]
    #[serde(rename = "insert_delete")]
    InsertDelete,

    #[serde(rename = "weighted")]
    Weighted,

    /// Simplified Debezium CDC format.
    ///
    /// We support a simplified version of the Debezium CDC format.  All fields
    /// except `payload` are ignored.
    ///
    /// # Example
    ///
    /// ```json
    /// {"payload": {"op": "u", "before": {"b": true, "i": 123}, "after": {"b": true, "i": 0}}}
    /// ```
    #[serde(rename = "debezium")]
    Debezium,

    /// Format used to output JSON data to Snowflake.
    ///
    /// Uses flat structure so that fields can get parsed directly into SQL
    /// columns.  Defines three metadata fields:
    ///
    /// * `__action` - "insert" or "delete"
    /// * `__stream_id` - unique 64-bit ID of the output stream (records within
    ///   a stream are totally ordered)
    /// * `__seq_number` - monotonically increasing sequence number relative to
    ///   the start of the stream.
    ///
    /// ```json
    /// {"PART":1,"VENDOR":2,"EFFECTIVE_SINCE":"2019-05-21","PRICE":"10000","__action":"insert","__stream_id":4523666124030717756,"__seq_number":1}
    /// ```
    #[serde(rename = "snowflake")]
    Snowflake,

    /// Raw input format.
    ///
    /// This format is suitable for insert-only streams (no deletions).
    /// Each element in the input stream contains a record without any
    /// additional envelope that gets inserted in the input table.
    #[serde(rename = "raw")]
    Raw,
}

impl Display for JsonUpdateFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonUpdateFormat::InsertDelete => write!(f, "insert_delete"),
            JsonUpdateFormat::Weighted => write!(f, "weighted"),
            JsonUpdateFormat::Debezium => write!(f, "debezium"),
            JsonUpdateFormat::Snowflake => write!(f, "snowflake"),
            JsonUpdateFormat::Raw => write!(f, "raw"),
        }
    }
}

/// Specifies JSON encoding used of table records.
#[derive(Clone, Default, Deserialize, Serialize, Debug, PartialEq, Eq, ToSchema)]
pub enum JsonFlavor {
    /// Default encoding used by Feldera, documented
    /// [here](https://docs.feldera.com/formats/json#types).
    #[default]
    #[serde(rename = "default")]
    Default,
    /// Debezium MySQL JSON produced by the default configuration of the
    /// Debezium [Kafka Connect connector](https://debezium.io/documentation/reference/stable/connectors/mysql.html#mysql-data-types)
    /// with `decimal.handling.mode` set to "string".
    #[serde(rename = "debezium_mysql")]
    DebeziumMySql,
    /// Debezium Postgres JSON produced by the default configuration of the
    /// Debezium [Kafka Connect connector](https://debezium.io/documentation/reference/stable/connectors/postgresql.html#postgresql-data-types)
    /// with `decimal.handling.mode` set to "string".
    #[serde(rename = "debezium_postgres")]
    DebeziumPostgres,
    /// JSON format accepted by Snowflake using default settings.
    #[serde(rename = "snowflake")]
    Snowflake,
    /// JSON format accepted by the Kafka Connect `JsonConverter` class.
    #[serde(rename = "kafka_connect_json_converter")]
    KafkaConnectJsonConverter,
    #[serde(rename = "pandas")]
    Pandas,
    /// Parquet to-json format.
    /// (For internal use only)
    #[serde(skip)]
    ParquetConverter,
    /// Datagen format.
    /// (For internal use only)
    #[serde(rename = "datagen")]
    Datagen,
}

// TODO: support multiple update formats, e.g., `WeightedUpdate`
// supports arbitrary weights beyond `MAX_DUPLICATES`.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(default)]
pub struct JsonEncoderConfig {
    pub update_format: JsonUpdateFormat,
    pub json_flavor: Option<JsonFlavor>,
    pub buffer_size_records: usize,
    pub array: bool,

    /// When this option is set, only the listed fields appear in the Debezium message key.
    ///
    /// This option is useful when writing to a table with primary keys.
    /// For such tables, Debezium expects the message key to contain only
    /// the primary key columns.
    ///
    /// This option is only valid with the `debezium` update format.
    pub key_fields: Option<Vec<String>>,
}

impl Default for JsonEncoderConfig {
    fn default() -> Self {
        Self {
            update_format: JsonUpdateFormat::default(),
            json_flavor: None,
            buffer_size_records: 10_000,
            array: false,
            key_fields: None,
        }
    }
}
