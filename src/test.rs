use rust_seed::Person;

#[test]
fn person_test() {
    let p = Person::new();
    assert_eq!("Hello!", p.greet());
}
