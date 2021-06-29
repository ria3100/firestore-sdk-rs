mod query;

use async_trait::async_trait;
use std::collections::HashMap;

pub use firestore_grpc;

pub mod store_field;

use firestore_grpc::v1::{
    structured_query::{
        filter::FilterType, FieldFilter, FieldReference, Filter as QueryFilter, Order,
    },
    value::ValueType,
    Cursor, Document, Value as FsValue,
};

#[derive(Debug, Default)]
pub struct State {
    token: Option<String>,
    project_id: String,
    collection: String,
    document: String,
    where_field: Vec<QueryFilter>,
    order_by: Vec<firestore_grpc::v1::structured_query::Order>,
    start_at: Option<Cursor>,
    end_at: Option<Cursor>,
    // offset: i32,
    limit: Option<i32>,
}

pub struct Direction;
impl Direction {
    pub const ASCENDING: i32 = 1;
    pub const DESCENDING: i32 = 2;
}

pub struct Operator;
impl Operator {
    pub const LESS_THAN: i32 = 1;
    pub const LESS_THAN_OR_EQUAL: i32 = 2;
    pub const GREATER_THAN: i32 = 3;
    pub const GREATER_THAN_OR_EQUAL: i32 = 4;
    pub const EQUAL: i32 = 5;
    pub const NOT_EQUAL: i32 = 6;
    pub const ARRAY_CONTAINS: i32 = 7;
    pub const IN: i32 = 8;
    pub const ARRAY_CONTAINS_ANY: i32 = 9;
    pub const NOT_IN: i32 = 10;
}

pub type BoxError = Box<dyn std::error::Error + Sync + Send + 'static>;

#[async_trait]
pub trait FireStore {
    fn init(&mut self, project_id: &str, token: Option<&str>) -> &mut Self;
    fn collection(&mut self, collection: &str) -> &mut Self;
    fn document(&mut self, document: &str) -> &mut Self;
    fn where_field(&mut self, field: &str, operator: i32, value: ValueType) -> &mut Self;
    fn order_by(&mut self, field: &str, direction: i32) -> &mut Self;
    fn start_at(&mut self, document: Document) -> &mut Self;
    fn start_after(&mut self, document: Document) -> &mut Self;
    fn end_before(&mut self, document: Document) -> &mut Self;
    fn end_at(&mut self, document: Document) -> &mut Self;
    // fn offset(&mut self, offset: i32) -> &mut Self;
    fn limit(&mut self, limit: i32) -> &mut Self;
    async fn get_document(&mut self) -> Result<Document, BoxError>;
    async fn get_documents(&mut self) -> Result<Vec<Document>, BoxError>;
    async fn set_document(
        &mut self,
        document_id: &str,
        fields: HashMap<String, FsValue>,
    ) -> Result<Document, BoxError>;
    async fn add_document(
        &mut self,
        fields: HashMap<String, FsValue>,
    ) -> Result<Document, BoxError>;
    async fn delete(&mut self) -> Result<String, BoxError>;
}

#[async_trait]
impl FireStore for State {
    fn init(&mut self, project_id: &str, token: Option<&str>) -> &mut Self {
        self.project_id = project_id.to_string();
        self.token = match token {
            Some(token) => Some(token.to_string()),
            None => None,
        };

        self
    }

    fn collection(&mut self, collection: &str) -> &mut Self {
        self.collection = collection.to_string();
        self
    }

    fn document(&mut self, document: &str) -> &mut Self {
        self.document = document.to_string();
        self
    }

    fn where_field(&mut self, field: &str, operator: i32, value: ValueType) -> &mut Self {
        let field_filter = FieldFilter {
            field: Some(FieldReference {
                field_path: field.to_string(),
            }),
            op: operator,
            value: Some(FsValue {
                value_type: Some(value),
            }),
        };

        self.where_field.push(QueryFilter {
            filter_type: Some(FilterType::FieldFilter(field_filter)),
        });

        self
    }

    fn order_by(&mut self, field: &str, direction: i32) -> &mut Self {
        self.order_by.push(Order {
            field: Some(FieldReference {
                field_path: field.to_string(),
            }),
            direction,
        });
        self
    }

    fn start_at(&mut self, document: Document) -> &mut Self {
        let mut cursor_values = Vec::new();

        for order_by in self.order_by.clone() {
            let field_path = order_by.field.unwrap().field_path;
            cursor_values.push(document.fields.get(&field_path).unwrap().clone());
        }

        self.order_by.push(Order {
            field: Some(FieldReference {
                field_path: "__name__".to_string(),
            }),
            direction: Direction::ASCENDING,
        });

        cursor_values.push(store_field::Value::reference(document.name));

        self.start_at = Some(Cursor {
            values: cursor_values,
            before: true,
        });

        self
    }

    fn start_after(&mut self, document: Document) -> &mut Self {
        let mut cursor_values = Vec::new();

        for order_by in self.order_by.clone() {
            let field_path = order_by.field.unwrap().field_path;
            cursor_values.push(document.fields.get(&field_path).unwrap().clone());
        }

        self.order_by.push(Order {
            field: Some(FieldReference {
                field_path: "__name__".to_string(),
            }),
            direction: Direction::ASCENDING,
        });

        cursor_values.push(store_field::Value::reference(document.name));

        self.start_at = Some(Cursor {
            values: cursor_values,
            before: false,
        });

        self
    }

    fn end_before(&mut self, document: Document) -> &mut Self {
        let mut cursor_values = Vec::new();

        for order_by in self.order_by.clone() {
            let field_path = order_by.field.unwrap().field_path;
            cursor_values.push(document.fields.get(&field_path).unwrap().clone());
        }

        self.order_by.push(Order {
            field: Some(FieldReference {
                field_path: "__name__".to_string(),
            }),
            direction: Direction::ASCENDING,
        });

        cursor_values.push(store_field::Value::reference(document.name));

        self.end_at = Some(Cursor {
            values: cursor_values,
            before: true,
        });
        self
    }

    fn end_at(&mut self, document: Document) -> &mut Self {
        let mut cursor_values = Vec::new();

        for order_by in self.order_by.clone() {
            let field_path = order_by.field.unwrap().field_path;
            cursor_values.push(document.fields.get(&field_path).unwrap().clone());
        }

        self.order_by.push(Order {
            field: Some(FieldReference {
                field_path: "__name__".to_string(),
            }),
            direction: Direction::ASCENDING,
        });

        cursor_values.push(store_field::Value::reference(document.name));

        self.end_at = Some(Cursor {
            values: cursor_values,
            before: false,
        });
        self
    }

    // RPC not working
    // fn offset(&mut self, offset: i32) -> &mut Self {
    //     self.offset = offset;
    //     self
    // }

    fn limit(&mut self, limit: i32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    async fn get_document(&mut self) -> Result<Document, BoxError> {
        let res = query::get_document(self).await;
        Ok(res.unwrap().into_inner())
    }

    async fn get_documents(&mut self) -> Result<Vec<Document>, BoxError> {
        let res = query::run_query(self).await;
        let mut stream = res.unwrap().into_inner();

        let mut vec = Vec::new();

        loop {
            let item = stream.message().await?;
            match item {
                Some(_) => {
                    let document = item.unwrap().document;
                    match document {
                        Some(document) => vec.push(document),
                        None => break,
                    }
                }
                None => break,
            }
        }

        Ok(vec)
    }

    async fn set_document(
        &mut self,
        document_id: &str,
        fields: HashMap<String, FsValue>,
    ) -> Result<Document, BoxError> {
        let exists = query::get_document(self).await;

        let res;

        match exists {
            Ok(_) => res = query::update_document(self, document_id, fields).await,
            Err(_) => res = query::create_document(self, document_id, fields).await,
        };

        Ok(res.unwrap().into_inner())
    }

    async fn add_document(
        &mut self,
        fields: HashMap<String, FsValue>,
    ) -> Result<Document, BoxError> {
        let res = query::create_document(self, "".into(), fields).await;

        Ok(res.unwrap().into_inner())
    }

    async fn delete(&mut self) -> Result<String, BoxError> {
        query::delete_document(self).await.unwrap();

        Ok(self.document.clone())
    }
}

pub fn db() -> State {
    State::default()
}
