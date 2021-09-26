
pub fn asyncronous() {
    let b = async {
        println!("hello");
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();
}

async fn await_test() -> i32 {
    let block = async {
        0
    };

    block.await
}