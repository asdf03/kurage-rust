use mry::{self, mry};

#[mry::mry]
fn great(count: usize) -> String {
    "hello".repeat(count)
}

#[test]
#[mry::lock(greet)] // グローバルな状態を使うためロックが必要
fn mock_function() {
    mock_greet(mry::Any).returns("hi".to_string());

    assert_eq!(great(3), "hi".to_string());
}
