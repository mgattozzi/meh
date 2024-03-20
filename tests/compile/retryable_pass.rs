use meh::Retry;
use meh::Retryable;

fn main() -> Result<(), String> {
    // We can use it in a Result
    let retry = test()?;
    match retry {
        Retry::Ok(()) => Ok(()),
        Retry::Retry(_e) => {
            // retry logic here
            todo!()
        }
    }
}

pub fn test() -> Retryable<(), String, String> {
    // We can use it in a Retryable
    let whee: Retry<(), String> = Retryable::Ok(())?;
    let Retry::Ok(unit) = whee else {
        unreachable!()
    };

    Retryable::Ok(unit)
}
