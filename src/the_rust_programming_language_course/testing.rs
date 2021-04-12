pub fn run() {
    //
}

fn greetings() -> String {
    "hello".to_string()
}

#[test]
fn test_greetings() {
    assert_eq!("hello!", greetings()); // assert_eq! >> expect an equality between the tow input parameters
                                       // assert_ne! >> expect none equality between the tow input parameters
                                       // assert! >> expect a true return from the input parameter
}
