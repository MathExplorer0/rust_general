fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self, x: u32) {
            println!("{x}");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call(69);
    let x: Option<u32> = Option::None;

    assert_eq!(x.is_none(), true);
}
