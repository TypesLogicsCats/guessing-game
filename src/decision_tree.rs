use crate::guesser::Guesser;
use crate::ioutils::Answer;

pub enum Tree {
    Branch(Question),
    Leaf(String),
}

pub struct Question {
    text: String,
    yes: Box<Tree>,
    no: Box<Tree>,
}

pub fn play<G: Guesser>(guesser: &G, tree: &mut Tree) {
    // The most natural way to express this code would be using recursion, but
    // Rust doesn't guarantee tail-call optimization.
    //
    // Instead, use a loop: If there is another iteration to do, `opt` is
    // `Some(tree)` where `tree` is the child node to visit. If the base case
    // has been reached, `opt` is `None`.
    let mut opt = Some(tree);
    while let Some(tree) = opt {
        opt = match tree {
            Tree::Branch(Question { text, ref mut yes, ref mut no }) => {
                // Run the next iteration on the appropriate child node
                match guesser.ask(text) {
                    Answer::Yes => Some(&mut **yes),
                    Answer::No => Some(&mut **no)
                }
            }
            Tree::Leaf(answer) => {
                match guesser.guess(answer) {
                    Answer::Yes => {},
                    Answer::No => {
                        let (new_item, question, y_or_n) = guesser.specialize();
                        let (yes, no) = match y_or_n {
                            Answer::Yes => (new_item, answer.clone()),
                            Answer::No => (answer.clone(), new_item)
                        };
                        let yes = Box::new(Tree::Leaf(yes));
                        let no = Box::new(Tree::Leaf(no));
                        let text = question.to_string();
                        *tree = Tree::Branch(Question { text, yes, no })
                    }
                }
                None
            }
        }
    }
}
