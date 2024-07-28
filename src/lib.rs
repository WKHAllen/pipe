/// The `Pipe` trait creates a functional pipe by wrapping operations within one
/// another. Each call to the `pipe` method creates another wrapper and returns
/// the resulting function.
///
/// When in scope, this trait is implemented for all types implementing
/// `FnOnce(A) -> B`, for any types `A` and `B`.
pub trait Pipe<A, B, C> {
    /// Wraps the provided function or closure inside the currently constructed
    /// pipeline. See the documentation for the [`pipe`] function for examples.
    fn pipe<F>(self, f: F) -> impl FnOnce(A) -> C
    where
        F: FnOnce(B) -> C;
}

impl<F1, A, B, C> Pipe<A, B, C> for F1
where
    F1: FnOnce(A) -> B,
{
    fn pipe<F2>(self, f: F2) -> impl FnOnce(A) -> C
    where
        F2: FnOnce(B) -> C,
    {
        |a| f(self(a))
    }
}

/// This is a convenience function to start a pipeline.
///
/// The compiler often has difficulty inferring pipe input types, so it is
/// usually a good idea to explicitly provide the input type when using this
/// function.
///
/// ```
/// # use pipe::*;
/// let remove_long_words = pipe(|s: &str| s.split(' '))
///     .pipe(|split| split.filter(|s| s.len() <= 4))
///     .pipe(|filtered| filtered.collect::<Vec<_>>())
///     .pipe(|words| words.join(" "));
/// let short_words = remove_long_words("foo bar hello world baz");
/// assert_eq!(short_words, "foo bar baz");
/// ```
pub fn pipe<F, A, B>(f: F) -> impl FnOnce(A) -> B
where
    F: FnOnce(A) -> B,
{
    f
}

/// The `pipe` macro provides an alternative syntax for constructing pipelines.
///
/// The macro syntax is as follows:
/// `<input identifier>: <first input type>; <pipe expression> => <pipe expression> => ... => <pipe expression>`
///
/// ```
/// # use pipe::*;
/// let remove_long_words = pipe! { this: &str;
///        this.split(' ')
///     => this.filter(|s| s.len() <= 4)
///     => this.collect::<Vec<_>>()
///     => this.join(" ")
/// };
/// let short_words = remove_long_words("foo bar hello world baz");
/// assert_eq!(short_words, "foo bar baz");
/// ```
#[macro_export]
macro_rules! pipe {
    ( $ident:ident: $ty:ty; $first:expr => $( $rest:expr )=>* ) => {{
        use $crate::Pipe;
        $crate::pipe(|$ident: $ty| $first)
        $(
            .pipe(|$ident| $rest)
        )+
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_function() {
        let parse_words = pipe(|s: &str| s.split(' '))
            .pipe(|split| split.map(|s| s.trim()))
            .pipe(|split| split.filter(|s| s.len() > 3))
            .pipe(|split| split.collect::<Vec<_>>())
            .pipe(|v| v.join(" - "));
        assert_eq!(
            parse_words("hello world foo lorem bar ipsum baz"),
            "hello - world - lorem - ipsum"
        );
    }

    #[test]
    fn test_pipe_macro() {
        let parse_words = pipe! { this: &str;
               this.split(' ')
            => this.map(|s| s.trim())
            => this.filter(|s| s.len() > 3)
            => this.collect::<Vec<_>>()
            => this.join(" - ")
        };
        assert_eq!(
            parse_words("hello world foo lorem bar ipsum baz"),
            "hello - world - lorem - ipsum"
        );
    }
}
