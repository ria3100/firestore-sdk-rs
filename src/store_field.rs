use chrono::NaiveDateTime;
use firestore_grpc::v1::{value::ValueType, ArrayValue, MapValue, Value as FsValue};
use prost_types::Timestamp;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct FromValues {
    fields: HashMap<String, FsValue>,
}
impl FromValues {
    pub fn get_string(self, key: &str) -> String {
        let value = self.fields.get(&key.to_string());

        match value {
            Some(value) => {
                let value_type = value.value_type.clone().unwrap();

                match value_type {
                    ValueType::StringValue(value) => value,
                    _ => "".to_string(),
                }
            }
            None => "".to_string(),
        }
    }
}
pub fn from_values(fields: HashMap<String, FsValue>) -> FromValues {
    FromValues { fields }
}

#[derive(Default)]
pub struct ToValues {
    fields: HashMap<String, FsValue>,
}
impl ToValues {
    pub fn add(mut self, key: &str, value: FsValue) -> Self {
        self.fields.insert(key.to_string(), value);
        self
    }

    pub fn get_fields(self) -> HashMap<String, FsValue> {
        self.fields
    }
}

pub fn to_values() -> ToValues {
    ToValues {
        fields: HashMap::new(),
    }
}

pub struct Value {}
impl Value {
    pub fn null() -> FsValue {
        FsValue {
            value_type: Some(ValueType::NullValue(0)),
        }
    }

    pub fn boolean(value: bool) -> FsValue {
        FsValue {
            value_type: Some(ValueType::BooleanValue(value)),
        }
    }

    pub fn integer(value: i64) -> FsValue {
        FsValue {
            value_type: Some(ValueType::IntegerValue(value)),
        }
    }

    pub fn double(value: f64) -> FsValue {
        FsValue {
            value_type: Some(ValueType::DoubleValue(value)),
        }
    }

    pub fn timestamp(value: NaiveDateTime) -> FsValue {
        FsValue {
            value_type: Some(ValueType::TimestampValue(Timestamp {
                seconds: value.timestamp(),
                nanos: value.timestamp_subsec_nanos() as i32,
            })),
        }
    }

    pub fn string(value: &str) -> FsValue {
        FsValue {
            value_type: Some(ValueType::StringValue(value.to_string())),
        }
    }

    // pub fn bytes(value: Vec) -> FsValue {
    //     FsValue {
    //         value_type: Some(ValueType::BytesValue(value)),
    //     }
    // }

    pub fn reference(value: String) -> FsValue {
        FsValue {
            value_type: Some(ValueType::ReferenceValue(value)),
        }
    }

    // pub fn geo_point(value: LatLng) -> FsValue {
    //     FsValue {
    //         value_type: Some(ValueType::GeoPointValue(value)),
    //     }
    // }

    pub fn array(value: ArrayValue) -> FsValue {
        FsValue {
            value_type: Some(ValueType::ArrayValue(value)),
        }
    }

    pub fn map(value: MapValue) -> FsValue {
        FsValue {
            value_type: Some(ValueType::MapValue(value)),
        }
    }
}
