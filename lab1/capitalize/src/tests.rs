#[cfg(test)]
mod tests
{
    use crate::capitalize;

    #[test]
    fn more_than_one_word()
    {
        let s = "more than one word";
        assert_eq!("More Than One Word", capitalize(s));
    }

    #[test]
    fn single_word_no_spaces()
    {
        let s = "nospaceword";
        assert_eq!("Nospaceword", capitalize(s));
    }

    #[test]
    fn begin_with_accent()
    {
        let s = "è una stringa con è";
        assert_eq!("È una stringa con È", capitalize(s));
    }

    #[test]
    fn empty_string()
    {
        let s = "";
        assert_eq!("", capitalize(s));
    }

    #[test]
    fn multiple_spaces()
    {
        let s = "i  have multiple     spaces";
        assert_eq!("I  Have Multiple     Spaces", capitalize(s));
    }
}