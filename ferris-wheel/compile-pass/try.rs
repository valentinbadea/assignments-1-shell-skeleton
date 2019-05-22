// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.
#[derive(Debug)]
struct ErrorA;
#[derive(Debug)]
struct ErrorB;

#[derive(Debug)]
enum Error {
    A(ErrorA),
    B(ErrorB)
}

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

impl From<ErrorA> for Error {
    fn from(err: ErrorA) -> Error {
        Error::A(err)
    }
}

impl From<ErrorB> for Error {
    fn from(err: ErrorB) -> Error {
        Error::B(err)
    }
}

fn do_both() -> Result<(u16, u32), Error> {
    let a = do_a()?;
    let b = do_b()?;
    Ok((a, b))
}

fn main() { }
