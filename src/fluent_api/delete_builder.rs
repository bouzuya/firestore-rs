use crate::{FirestoreDeleteSupport, FirestoreResult};

#[derive(Clone, Debug)]
pub struct FirestoreDeleteInitialBuilder<'a, D>
where
    D: FirestoreDeleteSupport,
{
    db: &'a D,
}

impl<'a, D> FirestoreDeleteInitialBuilder<'a, D>
where
    D: FirestoreDeleteSupport,
{
    #[inline]
    pub(crate) fn new(db: &'a D) -> Self {
        Self { db }
    }

    #[inline]
    pub fn from(self, collection_id: &str) -> FirestoreDeleteDocIdBuilder<'a, D> {
        FirestoreDeleteDocIdBuilder::new(self.db, collection_id.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct FirestoreDeleteDocIdBuilder<'a, D>
where
    D: FirestoreDeleteSupport,
{
    db: &'a D,
    collection_id: String,
    parent: Option<String>,
}

impl<'a, D> FirestoreDeleteDocIdBuilder<'a, D>
where
    D: FirestoreDeleteSupport,
{
    #[inline]
    pub(crate) fn new(db: &'a D, collection_id: String) -> Self {
        Self {
            db,
            collection_id,
            parent: None,
        }
    }

    #[inline]
    pub fn parent<S>(self, parent: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            parent: Some(parent.as_ref().to_string()),
            ..self
        }
    }

    #[inline]
    pub fn document_id<S>(self, document_id: S) -> FirestoreDeleteExecuteBuilder<'a, D>
    where
        S: AsRef<str> + Send,
    {
        FirestoreDeleteExecuteBuilder::new(
            self.db,
            self.collection_id.to_string(),
            document_id.as_ref().to_string(),
            self.parent,
        )
    }
}

#[derive(Clone, Debug)]
pub struct FirestoreDeleteExecuteBuilder<'a, D>
where
    D: FirestoreDeleteSupport,
{
    db: &'a D,
    collection_id: String,
    document_id: String,
    parent: Option<String>,
}

impl<'a, D> FirestoreDeleteExecuteBuilder<'a, D>
where
    D: FirestoreDeleteSupport,
{
    #[inline]
    pub(crate) fn new(
        db: &'a D,
        collection_id: String,
        document_id: String,
        parent: Option<String>,
    ) -> Self {
        Self {
            db,
            collection_id,
            document_id,
            parent,
        }
    }

    pub async fn execute(self) -> FirestoreResult<()> {
        if let Some(parent) = self.parent {
            self.db
                .delete_by_id_at(
                    parent.as_str(),
                    self.collection_id.as_str(),
                    self.document_id,
                )
                .await
        } else {
            self.db
                .delete_by_id(self.collection_id.as_str(), self.document_id)
                .await
        }
    }
}