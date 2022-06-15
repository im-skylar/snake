use crate::Args;
use bincode::{Decode, Encode};
use std::fs::{self, File};
use std::io::{Error, Write};

#[derive(Encode, Decode, PartialEq)]
struct ScoreEntry {
    settings: Args,
    score: u16,
}

#[derive(Encode, Decode, PartialEq)]
struct Scores(Vec<ScoreEntry>);

pub fn get_highscore(args: Args) -> u16 {
    if let Ok(scores) = read_and_parse() {
        if let Some(idx) = scores.0.iter().position(|x| x.settings == args) {
            scores.0[idx].score
        } else {
            0
        }
    } else {
        create_new_file().unwrap();
        0
    }
}

fn read_and_parse() -> Result<Scores, Box<dyn std::error::Error>> {
    let f = fs::read(".snake-highscores")?;

    let (scores, _): (Scores, usize) =
        bincode::decode_from_slice(&f, bincode::config::standard())?;
    
    Ok(scores)
}

fn create_new_file() -> Result<(), Error> {
    let scores = Scores(vec![]);

    let encoded_data = bincode::encode_to_vec(&scores, bincode::config::standard()).unwrap();

    let mut f = File::create(".snake-highscores")?;
    f.write(&encoded_data[..])?;

    Ok(())
}

pub fn set_highscore(args: Args, score: u16) {
    if let Ok(mut parsed_data) = read_and_parse() {
        let mut scores = parsed_data.0;

        if let Some(idx) = scores.iter().position(|x| x.settings == args) {
            if scores[idx].score < score {
                scores[idx].score = score;
            }
        } else {
            scores.push(ScoreEntry {
                settings: args,
                score
            });
        }

        write_file(Scores( scores )).unwrap();
    } else {
        create_new_file().unwrap();
    }
}

fn write_file(scores: Scores) -> Result<(), Error> {
    let encoded_data = bincode::encode_to_vec(&scores, bincode::config::standard()).unwrap();

    let mut f = File::create(".snake-highscores")?;
    f.write(&encoded_data[..])?;

    Ok(())
}
