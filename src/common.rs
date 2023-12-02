pub trait Day {
    type Input;
    type Output: std::fmt::Display;

    fn parse(buf: &[u8]) -> Self::Input;

    fn calc(input: Self::Input) -> Self::Output;
    fn calc2(_: Self::Input) -> Self::Output {
        todo!("implement part 2")
    }
}
