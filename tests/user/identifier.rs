use owapi::BNetUser;


#[test]
fn str() {
    assert_eq!("наб-2170".identifier(), "наб-2170");
}

#[test]
fn string() {
    assert_eq!("наб-2170".to_string().identifier(), "наб-2170");
}

#[test]
fn ref_string() {
    assert_eq!((&"наб-2170".to_string()).identifier(), "наб-2170");
}

#[test]
fn str_u32() {
    assert_eq!(("наб", 2170).identifier(), "наб-2170");
}

#[test]
fn string_u32() {
    assert_eq!(("наб".to_string(), 2170).identifier(), "наб-2170");
}

#[test]
fn ref_string_u32() {
    assert_eq!((&"наб".to_string(), 2170).identifier(), "наб-2170");
}
