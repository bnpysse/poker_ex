/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {hands:?}, which hand wins?")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Suite {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rank{
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Suite {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suite::Spade => write!(fmt, "♤"),
            Suite::Heart => write!(fmt, "♡"),
            Suite::Club => write!(fmt, "♧"),
            Suite::Diamond => write!(fmt, "♢"),
        }
    }
}

#[test]
fn test_display() {
    println!("{}", Suite::Spade);
    println!("{}", Suite::Club);
}