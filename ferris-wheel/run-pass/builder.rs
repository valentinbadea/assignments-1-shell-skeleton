// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

use std::fmt::Display;
impl Builder {
    fn to_string(&self)->String{
        match (&self.string, &self.number){
            (Some(x),Some(y))=>format!("{} {}",x,y),
            (None,Some(x))=>format!("{}",x),
            (Some(x),None)=>format!("{}",x),
            (None,None)=>format!(""),
        }
    }

    fn string<S: Display>(&mut self, string:S)->&mut Self{
        self.string = Some(string.to_string());
        self
    }

    fn number(&mut self, number:usize)->&mut Self{
        self.number = Some(number);
        self
    }

    fn default() -> Self { 
        Builder {
            number: None,
            string: None
        }
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
