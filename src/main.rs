use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    // array to store the names of the 12 notes in the chromatic scale
    let mut chromatic: [&str; 12] = ["A","A#","B","C","C#","D","D#","E","F","F#","G","G#"];

    //create random number generator used for shuffling array of notes
    let mut rng = thread_rng();

    //shuffle chromatic array in place
    chromatic.shuffle(&mut rng);

    //print shuffled array to command line
    println!("random note sequence: {:?}", chromatic);
  
}

//TODO: add tests