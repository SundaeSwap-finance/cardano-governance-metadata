mod cip100;

use anyhow::*;
use iref::IriBuf;
use json_ld::{
    syntax::{Parse, Value},
    JsonLdProcessor, Node, RemoteDocument,
};
use url::Url;

pub use cip100::*;

/// A client for fetching governance metadata from the web
pub struct MetadataClient {}

impl MetadataClient {
    pub fn new() -> MetadataClient {
        MetadataClient {}
    }

    /// Load a document of type T from the given JSON-LD document
    pub async fn load<T: for<'a> TryFrom<&'a Node>>(&self, url: Url) -> Result<Document> {
        // use Reqwest to load the content, since the json_ld reqwest loader is picky about content types for now
        let content = reqwest::get(url.clone()).await?.text().await?;
        let iri = IriBuf::new(url.clone().to_string()).context("invalid url")?;
        let value = Value::parse_str(&content).expect("unable to parse file").0;

        let document = RemoteDocument::new(Some(iri), None, value);

        let expanded = document.expand(&mut json_ld::NoLoader::default()).await?;

        let first_object = expanded
            .objects()
            .iter()
            .next()
            .context("no objects in document")?;
        let node = first_object
            .as_node()
            .context("object in document isn't a node")?;
        let r = node.try_into()?;
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use url::Url;

    #[tokio::test]
    async fn test_metadata_client() -> Result<()> {
        let client = MetadataClient::new();
        let url = Url::parse("https://raw.githubusercontent.com/cardano-foundation/CIPs/master/CIP-0100/example.json").unwrap();
        let cip100 = client
            .load::<Document>(url)
            .await
            .context("unable to load document")?;
        let expected = Document {
            hash_algorithm: "blake2b-256".to_string(),
            authors: vec![
                Author {
                    name: "Pi Lanningham".to_string(),
                    witness: Witness {
                        algorithm: "ed25519".to_string(),
                        public_key: "7ea09a34aebb13c9841c71397b1cabfec5ddf950405293dee496cac2f437480a".to_string(),
                        signature: "340c2ef8d6abda96769844ab9dca2634ae21ef97ddbfad1f8843bea1058e40d656455a2962143adc603d063bbbe27b54b88d002d23d1dff1cd0e05017cd4f506".to_string(),
                    },
                },
            ],
            body: Body {
                references: vec![
                    Reference {
                        reference_type: ReferenceType::Other,
                        label: "CIP-100".to_string(),
                        uri: IriBuf::new("https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md".to_string()).unwrap(),
                    },
                ],
                comment: "This is a test vector for CIP-100".to_string(),
                external_updates: vec![
                    Update {
                        title: "Blog".to_string(),
                        uri: IriBuf::new("https://314pool.com".to_string()).unwrap(),
                    },
                ],
            },
        };
        assert_eq!(cip100, expected);
        Ok(())
    }
}
