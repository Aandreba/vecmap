use vector_mapp::binary::BinaryMap;

#[test]
fn alpha () {
    let mut v = BinaryMap::new();
    v.insert("hello", "world");
    v.insert("alex", "andreba");
    v.insert("rust 🦀", "is awesome");

    assert_eq!(v.get("hello"), Some(&"world"));
    assert_eq!(v.get("alex"), Some(&"andreba"));
    assert_eq!(v.get("rust 🦀"), Some(&"is awesome"));
    assert_eq!(v.get("python 🐍"), None);
}