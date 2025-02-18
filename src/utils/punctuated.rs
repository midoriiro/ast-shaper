use syn::punctuated::Punctuated;

pub trait PunctuatedExt<T, P> {
    fn single(value: T) -> Self;
}

impl <T, P> PunctuatedExt<T, P> for Punctuated<T, P>
where
    P: Default
{
    fn single(value: T) -> Self {
        let mut result = Punctuated::new();
        result.push(value);
        result
    }
}