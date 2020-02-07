use crate::ioutils::{Answer, prompt, prompt_yn};

pub trait Guesser {
    fn ask(self: &Self, str: &str) -> Answer;
    fn guess(self: &Self, str: &str) -> Answer;
    fn specialize(self: &Self) -> (String, String, Answer);
}

pub struct IOGuesser;

impl Guesser for IOGuesser {
    fn ask(self: &Self, str: &str) -> Answer {
        prompt_yn(str)
    }

    fn guess(self: &Self, str: &str) -> Answer {
        prompt_yn(format!("Is the answer {}?", str).as_str())
    }

    fn specialize(self: &Self) -> (String, String, Answer) {
        let object = prompt("What object are you thinking of? ");
        let q = prompt(
            "What question could I ask to disambiguate between these?\n"
        );
        let a = prompt_yn("Is the answer 'yes' for this new object?");
        (object, q, a)
    }
}
