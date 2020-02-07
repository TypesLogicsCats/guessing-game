mod decision_tree;
mod guesser;
mod ioutils;
use decision_tree::{Tree, play};
use ioutils::Answer;

fn main() {
    let initial = ioutils::prompt("What is your favorite object?\n");
    let mut tree = Tree::Leaf(initial);
    play(&guesser::IOGuesser, &mut tree);
    while let Answer::Yes = ioutils::prompt_yn("Play again?") {
        play(&guesser::IOGuesser, &mut tree);
    }
}
