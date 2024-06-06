# Cardano Governance Metadata

This rust library aids in fetching and resolving cardano governance metadata that conforms to [CIP-100](https://github.com/cardano-foundation/CIPs/tree/master/CIP-0100).

CIP-100 specifies that governance metadata documents can be "JSON-LD" documents, which allows them to be very explicit about the meaning of each field in the document.
This allows a diverse and collaborative ecosystem of different document formats, and for tools to unequivocally know the "intent" behind a specific field.

For example, across many different document authors, "name" might actually be intended to mean "first name", "full name", "username", among others. By annotating the document with a "context", that explicitly defines the meaning of each field, someone building a tool can opt-in to the fields they support, and know that they are interpreting the document as the original author intended (up to human error and misuse of the format).

This library is designed to be extensible, so that future CIPs can define their own document and field types, reusing types as is appropriate.

To load a metadata document:

```rs
let client = MetadataClient::new();
let url = Url::parse("https://raw.githubusercontent.com/cardano-foundation/CIPs/master/CIP-0100/example.json").unwrap();
let cip100 = client
    .load::<Document>(url)
    .await
    .context("unable to load document")?;
```

To define your own extension types for some other CIP, just implement:
```rs
impl TryFrom<&Node> for MyType {
    type Error = anyhow::Error;

    fn try_from(value: &Node) -> std::prelude::v1::Result<Self, Self::Error> {
        ...
    }
}
```

# Contributing

This is a first draft of a rather simple library. Feedback and pull requests welcome!
I will endeavor to keep this up to date with most commonly used CIPs over time.