use anyhow::*;
use iref::{Iri, IriBuf};
use json_ld::Node;

// The context fields used in the context of CIP-100 documents
// Mostly just here for convenience, to have namespaced constants
pub struct CIP100Fields {
    pub hash_algorithm: &'static str,
    pub authors: &'static str,
    pub body: &'static str,
    pub body_references: &'static str,
    pub body_comment: &'static str,
    pub body_external_updates: &'static str,
    pub update_title: &'static str,
    pub update_uri: &'static str,
    pub reference_type: &'static str,
    pub reference_type_governance_metadata: &'static str,
    pub reference_type_other: &'static str,
    pub reference_label: &'static str,
    pub reference_uri: &'static str,
    pub author_name: &'static str,
    pub author_witness: &'static str,
    pub witness_algorithm: &'static str,
    pub witness_public_key: &'static str,
    pub witness_signature: &'static str,
}

pub const CIP100_FIELDS: CIP100Fields = CIP100Fields {
    hash_algorithm: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#hashAlgorithm",
    authors: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#authors",
    body: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#body",
    body_references: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#references",
    body_comment: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#comment",
    body_external_updates: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#externalUpdates",
    update_title: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#update-title",
    update_uri: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#update-uri",
    reference_type: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#referenceType",
    reference_type_governance_metadata: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#GovernanceMetadataReference",
    reference_type_other: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#OtherReference",
    reference_label: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#reference-label",
    reference_uri: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#reference-uri",
    author_name: "http://xmlns.com/foaf/0.1/name",
    author_witness: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#witness",
    witness_algorithm: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#witnessAlgorithm",
    witness_public_key: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#publicKey",
    witness_signature: "https://github.com/cardano-foundation/CIPs/blob/master/CIP-0100/README.md#signature",
};

/// A witness from an author who has signed the document
#[derive(Debug, PartialEq, Eq)]
pub struct Witness {
    /// The algorithm used to sign the document
    pub algorithm: String,
    /// The public key used to sign the document
    pub public_key: String,
    /// The signature of the document
    pub signature: String,
}
/// An author who has signed the metadata document
#[derive(Debug, PartialEq, Eq)]
pub struct Author {
    /// The authors display name; self-reported, so may be inaccurate, if not strongly associated with a public key via some other means
    pub name: String,
    /// The witness attesting to this authors approval of the metadata
    pub witness: Witness,
}

/// The type of document being referenced
#[derive(Debug, PartialEq, Eq)]
pub enum ReferenceType {
    /// The referenced document should be parsed as if it were another governance metadata document, i.e. with reference to CIP-100
    GovernanceMetadata,
    /// The referenced document is some other type of document, and should not be assumed to be CIP-100 compatible
    Other,
}

/// A reference to some other document for additional context to help understand this governance document
#[derive(Debug, PartialEq, Eq)]
pub struct Reference {
    /// The type of document being referenced
    pub reference_type: ReferenceType,
    /// The label to display for the reference
    pub label: String,
    /// The Internationalized resource identifier for where to find the document
    pub uri: IriBuf,
}

/// The place to find updated information pertaining to this document
#[derive(Debug, PartialEq, Eq)]
pub struct Update {
    /// The title of the update source
    pub title: String,
    /// The Internationalized resource identifier for the update source
    pub uri: IriBuf,
}

/// The body of the governance metadata document
#[derive(Debug, PartialEq, Eq)]
pub struct Body {
    /// Any references included in the document
    pub references: Vec<Reference>,
    /// A free-form textual and generic comment associated with this governance metadata document
    pub comment: String,
    /// A series of locations where updates may be found, such as a twitter feed, blog, etc.
    /// Note that the updates themselves should be treated as unauthenticated materials
    pub external_updates: Vec<Update>,
}

/// The governance metadata document itself
#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    /// The hash algorithm used to hash the document when signing
    pub hash_algorithm: String,
    /// The authors who cosign / attest to this document
    pub authors: Vec<Author>,
    /// The body of the document
    pub body: Body,
}

impl TryFrom<&Node> for Document {
    type Error = anyhow::Error;

    fn try_from(object: &Node) -> Result<Self, Self::Error> {
        let hash_algorithm = object
            .get_any(&Iri::new(CIP100_FIELDS.hash_algorithm)?)
            .context("no hash_algorithm field")?
            .as_str()
            .context("hash_algorithm is not a string")?
            .to_string();

        let authors = object
            .get(&Iri::new(CIP100_FIELDS.authors)?)
            .map(|author| author.inner().as_node().unwrap().try_into())
            .collect::<Result<Vec<Author>>>()?;

        let body = object
            .get_any(&Iri::new(CIP100_FIELDS.body)?)
            .context("no body field")?
            .as_node()
            .context("body field isn't an object")?
            .try_into()?;

        Ok(Self {
            hash_algorithm,
            authors,
            body,
        })
    }
}

impl TryFrom<&Node> for Author {
    type Error = anyhow::Error;

    fn try_from(node: &Node) -> std::prelude::v1::Result<Self, Self::Error> {
        let name = node
            .get_any(&Iri::new(CIP100_FIELDS.author_name)?)
            .context("no author name field")?
            .as_str()
            .context("author name is not a string")?
            .to_string();
        let witness = node
            .get_any(&Iri::new(CIP100_FIELDS.author_witness)?)
            .context("no author witness field")?
            .as_node()
            .context("witness field isn't an object")?
            .try_into()?;
        Ok(Self { name, witness })
    }
}

impl TryFrom<&Node> for Witness {
    type Error = anyhow::Error;

    fn try_from(node: &Node) -> std::prelude::v1::Result<Self, Self::Error> {
        let algorithm = node
            .get_any(&Iri::new(CIP100_FIELDS.witness_algorithm)?)
            .context("no witness algorithm field")?
            .as_str()
            .context("witness algorithm is not a string")?
            .to_string();
        let public_key = node
            .get_any(&Iri::new(CIP100_FIELDS.witness_public_key)?)
            .context("no witness public key field")?
            .as_str()
            .context("witness public key is not a string")?
            .to_string();
        let signature = node
            .get_any(&Iri::new(CIP100_FIELDS.witness_signature)?)
            .context("no witness signature field")?
            .as_str()
            .context("witness signature is not a string")?
            .to_string();
        Ok(Self {
            algorithm,
            public_key,
            signature,
        })
    }
}

impl TryFrom<&Node> for Reference {
    type Error = anyhow::Error;

    fn try_from(value: &Node) -> std::prelude::v1::Result<Self, Self::Error> {
        let reference_type = value.types();
        if reference_type.len() != 1 {
            bail!("Reference type must have exactly one type");
        }
        let reference_type = reference_type[0].as_str();
        let reference_type = if reference_type == CIP100_FIELDS.reference_type_governance_metadata {
            ReferenceType::GovernanceMetadata
        } else if reference_type == CIP100_FIELDS.reference_type_other {
            ReferenceType::Other
        } else {
            bail!("Invalid reference type");
        };
        let label = value
            .get_any(&Iri::new(CIP100_FIELDS.reference_label)?)
            .context("no reference label field")?
            .as_str()
            .context("reference label is not a string")?
            .to_string();
        let iri = value
            .get_any(&Iri::new(CIP100_FIELDS.reference_uri)?)
            .context("no reference uri field")?
            .as_str()
            .context("reference uri is not a string")?;
        let iri = IriBuf::new(iri.to_string()).context("reference uri is not a valid IRI")?;
        Ok(Self {
            reference_type,
            label,
            uri: iri,
        })
    }
}

impl TryFrom<&Node> for Update {
    type Error = anyhow::Error;

    fn try_from(value: &Node) -> std::prelude::v1::Result<Self, Self::Error> {
        let title = value
            .get_any(&Iri::new(CIP100_FIELDS.update_title)?)
            .context("no update title field")?
            .as_str()
            .context("update title is not a string")?
            .to_string();
        let iri = value
            .get_any(&Iri::new(CIP100_FIELDS.update_uri)?)
            .context("no update uri field")?
            .as_str()
            .context("update uri is not a string")?;
        let iri = IriBuf::new(iri.to_string()).context("update uri is not a valid IRI")?;
        Ok(Self { title, uri: iri })
    }
}

impl TryFrom<&Node> for Body {
    type Error = anyhow::Error;

    fn try_from(value: &Node) -> std::prelude::v1::Result<Self, Self::Error> {
        let references = value
            .get(&Iri::new(CIP100_FIELDS.body_references)?)
            .map(|reference| reference.inner().as_node().unwrap().try_into())
            .collect::<Result<Vec<Reference>>>()?;
        let comment = value
            .get_any(&Iri::new(CIP100_FIELDS.body_comment)?)
            .context("no body comment field")?
            .as_str()
            .context("body comment is not a string")?
            .to_string();
        let external_updates = value
            .get(&Iri::new(CIP100_FIELDS.body_external_updates)?)
            .map(|update| update.inner().as_node().unwrap().try_into())
            .collect::<Result<Vec<Update>>>()?;
        Ok(Self {
            references,
            comment,
            external_updates,
        })
    }
}
