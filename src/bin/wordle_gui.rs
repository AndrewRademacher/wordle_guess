use slint::{ModelRc, VecModel};

slint::slint! {
    WordOption := HorizontalLayout {
        property<string> word;

        width: 400px;
        padding-top: 6px;
        padding-bottom: 6px;
        padding-left: 12px;
        Text {
            text: word;
            color: black;
            font-size: 24pt;
        }
    }

    WordleGuess := Window {
        property<bool> red_bg;
        property<[string]> words;

        title: "Wordle Guess";

        VerticalLayout {
            HorizontalLayout {
                alignment: center;
                Text {
                    text: "WAL 9000";
                    color: black;
                    font-size: 24pt;
                }
            }
            HorizontalLayout {
                Rectangle {
                    color: red_bg ? red : green;
                    height: 400px;
                    width: 400px;

                    animate color { duration: 250ms; }

                    TouchArea {
                        clicked => {
                            root.red_bg = !root.red_bg;
                        }
                    }
                }
                VerticalLayout {
                    alignment: start;
                    for word in words: WordOption {
                        word: word;
                    }
                }
            }
        }
    }
}

fn main() {
    let guess = WordleGuess::new();
    let model = ModelRc::new(VecModel::from(vec![
        "slaps".into(),
        "slips".into(),
        "dipss".into(),
        "crips".into(),
    ]));
    guess.set_words(model);
    guess.run()
}
