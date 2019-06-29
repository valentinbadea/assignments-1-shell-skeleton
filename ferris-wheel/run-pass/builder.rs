// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn default() -> Builder {
        Builder {
            string: None,
            number: None,
        }
    }

    fn to_string(&self) -> String {
        let mut v_str: Vec<String> = Vec::new();
        match self.string {
            Some(ref x) => v_str.push(x.clone()),
            None => (),
        };
        match self.number {
            Some(ref x) => v_str.push(x.to_string()),
            None => (),
        }
        v_str.join(" ")
    }

    fn string<S: Into<String>>(mut self, s: S) -> Self {
        self.string = Some(s.into());
        self
    }

    fn number(mut self, number: usize) -> Self {
        self.number = Some(number);
        self
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
