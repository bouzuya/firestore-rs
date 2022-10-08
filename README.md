[![Cargo](https://img.shields.io/crates/v/firestore.svg)](https://crates.io/crates/firestore)
![tests and formatting](https://github.com/abdolence/firestore-rs/workflows/tests%20&amp;%20formatting/badge.svg)
![security audit](https://github.com/abdolence/firestore-rs/workflows/security%20audit/badge.svg)

# Firestore for Rust

Library provides a simple API for Google Firestore based on the official gRPC API:
- Create or update documents using Rust structures and Serde; 
- Support for:
  - Querying/streaming docs/objects;
  - Listing documents/objects (and auto pages scrolling support);
  - Listening changes from Firestore;
  - Transactions;
- Full async based on Tokio runtime;
- Macro that helps you use JSON paths as references to your structure fields;
- Implements own Serde serializer to Firestore protobuf values;
- Supports for Firestore timestamp with `#[serde(with)]`
- Google client based on [gcloud-sdk library](https://github.com/abdolence/gcloud-sdk-rs) 
  that automatically detects GKE environment or application default accounts for local development;

## Quick start

Cargo.toml:
```toml
[dependencies]
firestore = "0.11"
```

Example code:
```rust

    // Create an instance
    let db = FirestoreDb::new(&config_env_var("PROJECT_ID")?).await?;

    const TEST_COLLECTION_NAME: &'static str = "test";

    let my_struct = MyTestStructure {
        some_id: "test-1".to_string(),
        some_string: "Test".to_string(),
        one_more_string: "Test2".to_string(),
        some_num: 42,
    };

    // Remove if it already exist
    db.delete_by_id(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
    ).await?;

    // Let's insert some data
    db.create_obj(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
        &my_struct,
    ).await?;

    // Update some field in it
    let updated_obj = db.update_obj(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
        &MyTestStructure {
            some_num: my_struct.some_num + 1,
            some_string: "updated-value".to_string(),
            ..my_struct.clone()
        },
        Some(
            paths!(MyTestStructure::{
                some_num,
                some_string
            })
        ),
    ).await?;

    println!("Updated object: {:?}", updated_obj);

    // Get object by id
    let find_it_again: MyTestStructure = db.get_obj(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
    ).await?;

    println!("Should be the same: {:?}", find_it_again);

    // Query our data
    let objects: Vec<MyTestStructure> = db.query_obj(
        FirestoreQueryParams::new(
            TEST_COLLECTION_NAME.into()
        ).with_filter(
            FirestoreQueryFilter::Compare(Some(
                FirestoreQueryFilterCompare::Equal(
                    path!(MyTestStructure::some_num),
                    find_it_again.some_num.into(),
                ),
            ))
        )
    ).await?;

    println!("Now in the list: {:?}", objects);
```

All examples available at [examples](examples) directory.

To run example use it with environment variables:
```
# PROJECT_ID=<your-google-project-id> cargo run --example simple-crud
```

## Timestamps support
By default, the types such as DateTime<Utc> serializes as a string
to Firestore (while deserialization works from Timestamps and Strings).
To change it to support Timestamp natively use `#[serde(with)]`:

```
#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
    #[serde(with = "firestore::serialize_as_timestamp")]
    created_at: DateTime<Utc>,
}
```
This will change it only for firestore serialization and it still serializes as string
to JSON (so you can reuse the same model for JSON and Firestore).

In queries you need to use a special wrapping class `firestore::FirestoreTimestamp`, for example:
```
FirestoreQueryFilter::Compare(Some(FirestoreQueryFilterCompare::LessThanOrEqual(
    path!(MyTestStructure::created_at),
    // Using the wrapping type to indicate serialization without attribute
    firestore::FirestoreTimestamp(Utc::now()).into(),
)))
```

## Nested collections
You can work with nested collection using functions like `db.create_object_at` and specifying path/location to documents:

```rust

// Creating a parent doc
db.create_obj(
    TEST_PARENT_COLLECTION_NAME,
    &parent_struct.some_id,
    &parent_struct,
)
.await?;

// The doc path where we store our childs
let parent_path = format!(
  "{}/{}/{}",
  db.get_documents_path(),
  TEST_PARENT_COLLECTION_NAME,
  parent_struct.some_id
);

// Create a child doc
db.create_obj_at(
    parent_path.as_str(),
    TEST_CHILD_COLLECTION_NAME,
    &child_struct.some_id,
    &child_struct,
)
.await?;

// Querying children
let mut objs_stream: BoxStream<MyChildStructure> = db
.stream_list_obj(
    FirestoreListDocParams::new(TEST_CHILD_COLLECTION_NAME.into())
        .with_parent(parent_path),
)
.await?;
```

## Google authentication

Looks for credentials in the following places, preferring the first location found:
- A JSON file whose path is specified by the GOOGLE_APPLICATION_CREDENTIALS environment variable.
- A JSON file in a location known to the gcloud command-line tool using `gcloud auth application-default login`.
- On Google Compute Engine, it fetches credentials from the metadata server.

### Local development
Don't confuse `gcloud auth login` with `gcloud auth application-default login` for local development,
since the first authorize only `gcloud` tool to access the Cloud Platform.

The latter obtains user access credentials via a web flow and puts them in the well-known location for Application Default Credentials (ADC).
This command is useful when you are developing code that would normally use a service account but need to run the code in a local development environment where it's easier to provide user credentials.
So to work for local development you need to use `gcloud auth application-default login`.


## Licence
Apache Software License (ASL)

## Author
Abdulla Abdurakhmanov
