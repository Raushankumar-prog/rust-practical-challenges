use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ContextError {
    context: String,
    source: Box<dyn Error + Send + Sync + 'static>,
}

impl ContextError {
    pub fn new<S: Into<String>>(context: S, source: Box<dyn Error + Send + Sync + 'static>) -> Self {
        Self {
            context: context.into(),
            source,
        }
    }
}


impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.source)
    }
}

impl Error for ContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source)
    }
}

pub trait ResultContext<T> {
    fn context<S>(self, ctx: S) -> Result<T, ContextError>
    where
        S: Into<String>;

    fn context_lazy<F, S>(self, f: F) -> Result<T, ContextError>
    where
        F: FnOnce() -> S,
        S: Into<String>;
}

impl<T, E> ResultContext<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    fn context<S>(self, ctx: S) -> Result<T, ContextError>
    where
        S: Into<String>,
    {
        self.map_err(|e| ContextError::new(ctx, Box::new(e)))
    }

    fn context_lazy<F, S>(self, f: F) -> Result<T, ContextError>
    where
        F: FnOnce() -> S,
        S: Into<String>,
    {
        self.map_err(|e| ContextError::new(f().into(), Box::new(e)))
    }
}

fn might_fail(ok: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    if ok {
        Ok(())
    } else {
        Err("operation failed".into())
    }
}

fn main() {
   
    let result1 = might_fail(false)
        .context("while trying to open file");

    if let Err(e) = result1 {
        println!("Error: {}", e);
    }

  
    let result2 = might_fail(false)
        .context_lazy(|| format!("lazy context: failed at {}", "runtime operation"));

    if let Err(e) = result2 {
        println!("Error: {}", e);
    }
}
