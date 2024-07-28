pub trait Pipe<A, B, C> {
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

pub fn pipe<F, A, B>(f: F) -> impl FnOnce(A) -> B
where
    F: FnOnce(A) -> B,
{
    f
}

#[macro_export]
macro_rules! pipe {
    ( $ident:ident: $ty:ty; $first:expr => $( $rest:expr )=>* ) => {
        pipe(|$ident: $ty| $first)
        $(
            .pipe(|$ident| $rest)
        )+
    };
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
