use crate::State;

use std::collections::HashMap;

pub use firestore_grpc;

use firestore_grpc::{
    tonic::{
        metadata::MetadataValue,
        transport::{Channel, ClientTlsConfig},
        Request, Response, Streaming,
    },
    v1::{
        firestore_client::FirestoreClient,
        run_query_request::QueryType,
        structured_query::{
            filter::FilterType, CollectionSelector, CompositeFilter, Filter as QueryFilter,
        },
        value::ValueType,
        CreateDocumentRequest, Cursor, DeleteDocumentRequest, Document, GetDocumentRequest,
        RunQueryRequest, RunQueryResponse, StructuredQuery, UpdateDocumentRequest,
        Value as FsValue,
    },
};

const URL: &'static str = "https://firestore.googleapis.com";
const DOMAIN: &'static str = "firestore.googleapis.com";

pub type BoxError = Box<dyn std::error::Error + Sync + Send + 'static>;

async fn get_client(token: Option<String>) -> Result<FirestoreClient<Channel>, BoxError> {
    let endpoint = Channel::from_static(URL).tls_config(ClientTlsConfig::new().domain_name(DOMAIN));
    let channel = endpoint.connect().await.unwrap();

    match token {
        Some(token) => {
            let service = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("authorization", MetadataValue::from_str(&token).unwrap());

                Ok(req)
            });

            Ok(service)
        }
        None => {
            let service =
                FirestoreClient::with_interceptor(channel, move |req: Request<()>| Ok(req));
            Ok(service)
        }
    }
}

pub async fn get_document(state: &State) -> Result<Response<Document>, BoxError> {
    let res = get_client(state.token.clone())
        .await?
        .get_document(GetDocumentRequest {
            name: format!(
                "projects/{}/databases/(default)/documents/{}/{}",
                state.project_id, state.collection, state.document
            ),
            mask: None,
            consistency_selector: None,
        })
        .await?;

    Ok(res)
}

pub async fn run_query(state: &State) -> Result<Response<Streaming<RunQueryResponse>>, BoxError> {
    let parent = format!(
        "projects/{}/databases/(default)/documents",
        state.project_id
    );

    let composite_filter = CompositeFilter {
        op: 1,
        filters: state.where_field.clone(),
    };

    let mut structured_query = StructuredQuery::default();
    structured_query.from = vec![CollectionSelector {
        collection_id: state.collection.clone().to_string(),
        all_descendants: false,
    }];
    structured_query.r#where = Some(QueryFilter {
        filter_type: Some(FilterType::CompositeFilter(composite_filter)),
    });
    structured_query.order_by = state.order_by.clone();

    structured_query.start_at = state.start_at.clone();
    structured_query.end_at = state.end_at.clone();

    // structured_query.offset = state.offset;
    structured_query.limit = state.limit;

    let mut run_query_request = RunQueryRequest::default();
    run_query_request.parent = parent;
    run_query_request.query_type = Some(QueryType::StructuredQuery(structured_query));

    let res = get_client(state.token.clone())
        .await?
        .run_query(run_query_request)
        .await?;

    Ok(res)
}

pub async fn update_document(
    state: &State,
    document_id: &str,
    fields: HashMap<String, FsValue>,
) -> Result<Response<Document>, BoxError> {
    let parent = format!(
        "projects/{}/databases/(default)/documents",
        state.project_id
    );
    let document = Some(Document {
        name: format!(
            "{}/{}/{}",
            parent,
            state.collection,
            document_id.to_string()
        ),
        fields,
        create_time: None,
        update_time: None,
    });

    let res = get_client(state.token.clone())
        .await?
        .update_document(UpdateDocumentRequest {
            document,
            update_mask: None,
            mask: None,
            current_document: None,
        })
        .await?;

    Ok(res)
}

pub async fn delete_document(state: &State) -> Result<Response<()>, BoxError> {
    let res = get_client(state.token.clone())
        .await?
        .delete_document(DeleteDocumentRequest {
            name: format!(
                "projects/{}/databases/(default)/documents/{}/{}",
                state.project_id,
                state.collection.to_string(),
                state.document.to_string(),
            ),
            current_document: None,
        })
        .await?;

    Ok(res)
}

pub async fn create_document(
    state: &State,
    document_id: &str,
    fields: HashMap<String, FsValue>,
) -> Result<Response<Document>, BoxError> {
    let parent = format!(
        "projects/{}/databases/(default)/documents",
        state.project_id
    );

    let document = Some(Document {
        name: "".to_string(),
        fields,
        create_time: None,
        update_time: None,
    });

    let res = get_client(state.token.clone())
        .await?
        .create_document(CreateDocumentRequest {
            parent,
            collection_id: state.collection.clone().to_string(),
            document_id: document_id.to_string(),
            document,
            mask: None,
        })
        .await?;

    Ok(res)
}
