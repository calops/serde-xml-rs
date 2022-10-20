use serde::{Deserialize, Serialize};
use serde_xml_rs::{self, from_str, to_string, EventReader, ParserConfig};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Items {
    #[serde(rename = "$value")]
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Node {
    Boolean(bool),
    Identifier { value: String, index: u32 },
    EOF,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Nodes {
    #[serde(rename = "$value")]
    items: Vec<Node>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Quiet(bool);
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ObjectVersionId(String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Delete {
    pub objects: ObjectIdentifierList,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet: Option<Quiet>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ObjectIdentifier {
    pub key: ObjectKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ObjectKey(String);

type ObjectIdentifierList = Vec<ObjectIdentifier>;

#[test]
fn basic_struct() {
    let src = r#"<?xml version="1.0" encoding="UTF-8"?><Item><name>Banana</name><source>Store</source></Item>"#;
    let should_be = Item {
        name: "Banana".to_string(),
        source: "Store".to_string(),
    };

    let item: Item = from_str(src).unwrap();
    assert_eq!(item, should_be);

    let reserialized_item = to_string(&item).unwrap();
    assert_eq!(src, reserialized_item);
}

#[test]
fn round_trip_list_of_enums() {
    // Construct some inputs
    let nodes = Nodes {
        items: vec![
            Node::Boolean(true),
            Node::Identifier {
                value: "foo".to_string(),
                index: 5,
            },
            Node::EOF,
        ],
    };

    let should_be = r#"<?xml version="1.0" encoding="UTF-8"?><Nodes><Boolean>true</Boolean><Identifier><value>foo</value><index>5</index></Identifier><EOF /></Nodes>"#;

    let serialized_nodes = to_string(&nodes).unwrap();
    assert_eq!(serialized_nodes, should_be);

    // Then turn it back into a `Nodes` struct and make sure it's the same
    // as the original
    let deserialized_nodes: Nodes = from_str(serialized_nodes.as_str()).unwrap();
    assert_eq!(deserialized_nodes, nodes);
}

#[test]
fn whitespace_preserving_config() {
    // Test a configuration which does not clip whitespace from tags

    let src = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <Item>
        <name>  space banana  </name>
        <source>   fantasy costco   </source>
    </Item>"#;

    let item_should_be = Item {
        name: "  space banana  ".to_string(),
        source: "   fantasy costco   ".to_string(),
    };
    let config = ParserConfig::new()
        .trim_whitespace(false)
        .whitespace_to_characters(false);
    let mut deserializer =
        serde_xml_rs::Deserializer::new(EventReader::new_with_config(src.as_bytes(), config));

    let item = Item::deserialize(&mut deserializer).unwrap();
    assert_eq!(item, item_should_be);

    // Space outside values is not preserved.
    let serialized_should_be =
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Item><name>  space banana  </name><source>   fantasy costco   </source></Item>";
    let reserialized_item = to_string(&item).unwrap();
    assert_eq!(reserialized_item, serialized_should_be);
}

#[test]
fn round_trip_list_of_structs() {
    let src = r#"<?xml version="1.0" encoding="UTF-8"?><Items><Item><name>Apple</name><source>Store</source></Item><Item><name>Orange</name><source>Store</source></Item></Items>"#;
    let should_be = Items {
        items: vec![
            Item {
                name: "Apple".to_string(),
                source: "Store".to_string(),
            },
            Item {
                name: "Orange".to_string(),
                source: "Store".to_string(),
            },
        ],
    };

    let items: Items = from_str(src).unwrap();
    assert_eq!(items, should_be);

    let reserialized_items = to_string(&items).unwrap();
    assert_eq!(src, reserialized_items);
}

#[test]
#[ignore]
// Serialization is working, but not deserialization
fn round_trip_complex() {
    let src = r#"<?xml version="1.0" encoding="UTF-8"?><Delete><objects><ObjectIdentifier><key>toto</key><version_id>version</version_id></ObjectIdentifier><ObjectIdentifier><key>toto</key></ObjectIdentifier><ObjectIdentifier><key>toto</key><version_id>version</version_id></ObjectIdentifier></objects><quiet>true</quiet></Delete>"#;
    let should_be = Delete {
        objects: vec![
            ObjectIdentifier {
                key: ObjectKey("foo".to_owned()),
                version_id: Some(ObjectVersionId("version1".to_owned())),
            },
            ObjectIdentifier {
                key: ObjectKey("bar".to_owned()),
                version_id: None,
            },
            ObjectIdentifier {
                key: ObjectKey("baz".to_owned()),
                version_id: Some(ObjectVersionId("version2".to_owned())),
            },
        ],
        quiet: Some(Quiet(true)),
    };

    let delete: Delete = from_str(src).unwrap();
    assert_eq!(delete, should_be);

    let reserialized_items = to_string(&delete).unwrap();
    assert_eq!(src, reserialized_items);
}
