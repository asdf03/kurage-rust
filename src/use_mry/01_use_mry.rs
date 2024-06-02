// use mry::{mry} だけでは関数,型,定数などにアクセスできないので、selfですべて持ってくる
use mry::{self, mry};

#[mry::mry]
struct Cat {
    name: String,
}

#[mry::mry]
impl Cat {
    fn meow(&self, count: usize) -> String {
        format!("{}: {}", self.name, "meow".repeat(count))
    }
}


#[test]
fn meow_returns() {
    let mut cat = mry::new!(Cat { name: "Tama".into() });

    // モック化されたmeowというメソッドが呼び出された際の定義
    // mry::Anyでどのような引数で呼び出されてもモックを適用させている
    cat.mock_meow(mry::Any).returns("Called".to_string());
    
    assert_eq!(cat.meow(2), "Called".to_string());
}