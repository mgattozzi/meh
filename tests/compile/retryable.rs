use meh::Retry;
use meh::Retryable;

fn main() -> Result<(), String> {
    let _ = test()?;
    Ok(())
}

fn test() -> Retryable<(), String, String> {
    // We can use it in a Retryable
    let whee: Retry<(), String> = Retryable::Ok(())?;
    let Retry::Ok(unit) = whee else {
        unreachable!()
    };

    // This fails to compile as it does not implement try
    let unit = whee?;

    Retryable::Ok(unit)
}
