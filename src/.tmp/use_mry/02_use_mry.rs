use mry::{self, mry};

#[mry::mry]
impl Cat {
    fn meow(&self, count: usize) -> String {
        self.meow_single().repeat(count)
    }

    fn meow_single(&self) -> String {
        "meow".into()
    }
}


#[test]
fn partial_mock() {
    let mut cat = mry::new!(Cat { name: "Tama".into() });

    // `meow_single`メソッドのモックを設定
    cat.mock_meow_gingle().returns("hello".to_string());

    // `meow`メソッドの実際の実装を呼び出す設定
    cat.mock_meow(mry::Any).calls_real_impl();

    // テスト
    assert_eq!(cat.meow(2), "hellohello".to_string());

}