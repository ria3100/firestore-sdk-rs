# firestore-sdk


## Examples

### create

```rust
use firestore_sdk::{db, FireStore, State, store_field, store_field::Value};

let project_id = "YOUR_PROJECT_ID";
let token = "YOUR_TOKEN";

let conn = db().init(project_id, Some(token));

let fields = store_field::to_values()
    .add("name", Value::string("Alice")
    .add("age", Value::integer(20))
    .get_fields();

let res = conn
    .collection("users")
    .add_document(fields)
    .await?;
```

### create or update

```rust
use firestore_sdk::{db, FireStore, State, store_field, store_field::Value};

let project_id = "YOUR_PROJECT_ID";
let token = "YOUR_TOKEN";

let conn = db().init(project_id, Some(token));

let document_id = "example_id";

let fields = store_field::to_values()
    .add("name", Value::string("Bob")
    .add("age", Value::integer(23))
    .get_fields();

let res = conn
    .collection("users")
    .document(document_id)
    .set_document(fields)
    .await?;
```

### delete

```rust
use firestore_sdk::{db, FireStore, State};

let project_id = "YOUR_PROJECT_ID";
let token = "YOUR_TOKEN";

let conn = db().init(project_id, Some(token));

let document_id = "example_id"

let res = conn
    .collection("users")
    .document(document_id)
    .delete()
    .await?;
```

### get one

```rust
use firestore_sdk::{db, FireStore, State};

let project_id = "YOUR_PROJECT_ID";
let token = "YOUR_TOKEN";

let conn = db().init(project_id, Some(token));

let document_id = "example_id"

let res = conn
    .collection("users")
    .document(document_id)
    .await?;
```

### find

```rust
use firestore_sdk::{
    db, FireStore, State, Direction, Operator,
    firestore_grpc::v1::value::ValueType
};

let project_id = "YOUR_PROJECT_ID";
let token = "YOUR_TOKEN";

let conn = db().init(project_id, Some(token));

let res = conn
    .collection("users")
    .order_by("age", Direction::ASCENDING)
    .where_field(
        "age",
        Operator::LESS_THAN,
        ValueType::IntegerValue(20),
    )
    .get_documents()
    .await?;
```

### find (start couser)

```rust
use firestore_sdk::{
    db, FireStore, State, Direction, Operator,
    firestore_grpc::v1::value::ValueType
};

let project_id = "YOUR_PROJECT_ID";
let token = "YOUR_TOKEN";

let conn = db().init(project_id, Some(token));

let document_id = "example_id"

let before_last = db()
    .collection("users")
    .document(document_id)
    .get_document()
    .await?;

let res = db()
    .collection("foo")
    .start_after(before_last)
    .limit(10)
    .get_documents()
    .await?;
```

### get document to struct

```rust
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    age: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

fn document_to_user(document: &Document) -> Result<User, BoxError> {
    let v: Vec<&str> = document.name.rsplit('/').collect();
    let uid = (*v.first().unwrap()).to_string();

    let created_time = &document.create_time.as_ref().unwrap();
    let created_at =
        NaiveDateTime::from_timestamp(created_time.seconds, created_time.nanos.try_into().unwrap());

    let updated_time = &document.update_time.as_ref().unwrap();
    let updated_at =
        NaiveDateTime::from_timestamp(updated_time.seconds, updated_time.nanos.try_into().unwrap());

    let values = store_field::from_values(document.fields.clone());

    let user = User {
        uid,
        name: values.clone().get_string("name"),
        age: values.clone().get_string("age"),
        created_at,
        updated_at,
    };

    Ok(user)
}
```
