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

fn do_both() -> Result<(u16, u32), Error> {
    let a = do_a().map_err(Error::A)?;
    let b = do_b().map_err(Error::B)?;
    Ok((a, b))
}

fn main() { }
