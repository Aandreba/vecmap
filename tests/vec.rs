use vector_mapp::vec::VecMap;

#[test]
fn alpha () {
    let mut v = VecMap::new();
    v.insert("hello", "world");
    v.insert("alex", "andreba");
    v.insert("rust 🦀", "is awesome");

    assert_eq!(v.get("hello"), Some(&"world"));
    assert_eq!(v.get("alex"), Some(&"andreba"));
    assert_eq!(v.get("rust 🦀"), Some(&"is awesome"));
    assert_eq!(v.get("python 🐍"), None);
}