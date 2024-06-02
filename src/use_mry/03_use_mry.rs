use mry::{self, mry};

#[mry::mry]
pub trait Animal {
    fn sound(&self) -> String;
}

#[test]
fn mock_trait() {
    let mut animal = MockAnimal::default();

    animal.mock_sound().returns("roar".into());

    assert_eq!(animal.sound(), "roar".to_string());
}