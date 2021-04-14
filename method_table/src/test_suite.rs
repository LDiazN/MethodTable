#[allow(unused)]
use crate::method_table::*;


#[allow(unused)]
fn s(string : &str) -> String {
    string.to_string()
}

#[test]
fn test_valid_add() {
    let mut m = ClassManager::new();

    assert!(m.add( s("A"), None, vec![]).is_ok());
    assert!(m.add( s("B"), None, vec![s("f"), s("g")]).is_ok());
    assert!(m.add( s("C"), Some( s("B") ), vec![ s("h"), s("g") ] ).is_ok() );
}

#[test]
fn test_invalid_add() {
    let mut m = ClassManager::new();

    assert_eq!(m.add(s("A"), Some(s("X")), vec![]), Err(ClassError::NotDefined(s("X"))));
    assert_eq!(m.add(s("A"), None, vec![]), Ok(()));
    assert_eq!(m.add(s("A"), None, vec![]), Err(ClassError::ClassRedefinition(s("A"))));
}