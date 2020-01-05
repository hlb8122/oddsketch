use oddsketch::Oddsketch;

#[test]
fn is_empty() {
    let mut oddsketch = Oddsketch::default();
    assert!(oddsketch.is_empty());

    oddsketch.insert(0);

    assert!(!oddsketch.is_empty());
}

#[test]
fn involution() {
    let mut oddsketch = Oddsketch::default();
    oddsketch.insert(0);
    oddsketch.insert(0);
    assert!(oddsketch.is_empty());
}

#[test]
fn size() {
    let mut oddsketch = Oddsketch::default();
    oddsketch.insert(0);
    assert_eq!(oddsketch.size(), 1);
}

#[test]
fn fold_size() {
    for i in 0..256 {
        let oddsketch = Oddsketch::default();
        let folded = oddsketch.fold(32);
        assert_eq!(folded.len(), 32);
    }
}
