// sample and test programs


use std::io;

trait Sounds {
    // add code here
    fn make_sound(&self);
}

#[derive(Debug)]
struct Cat {
    sounds: String,
}

struct Tiger {
    sounds: String,
}

impl Cat {
    fn new(snd: String) -> Cat {
        Cat { sounds: snd }
    }
    fn called(&self) {
        on_scare(self);
    }
}

impl Sounds for Cat {
    // add code here
    fn make_sound(&self) {
        println!("Cat sounds:{:?}", self.sounds);
    }
}

impl Sounds for Tiger {
    fn make_sound(&self) {
        println!("Tiger sounds:{:?}", self.sounds);
    }
}

////////////////////////////////////////
fn on_scare<T: Sounds>(t: &T) {
    t.make_sound();
}
///////////////////////////////////////

fn main() {
    let mut cat = Cat {
        sounds: String::new(),
    };
    let tiger = Tiger {
        sounds: String::from("Roar"),
    };
    // on_scare(&cat);
    on_scare(&tiger);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unable to read the value");
    let val = input.trim().parse().unwrap();
    cat = Cat::new(val);
    cat.called();
}

// testing emotions

trait Emotions {
    // add code here
    fn happy(&self);
    fn angry(&self);
    fn sad(&self);
    fn satisfied(&self);
}
#[derive(Debug)]
struct UserEmotion {
    name: String,
    state: String,
}

impl UserEmotion {
    // add code here
    fn new(name: String, state: String) -> UserEmotion {
        UserEmotion { name, state }
    }
    fn test_emotion(&self) {
        trigger(self, &self.state);
    }
}

impl Emotions for UserEmotion {
    fn happy(&self) {
        println!("I'm happy for the time being..");
    }
    fn angry(&self) {
        println!("I'm angry for the time being..");
    }
    fn satisfied(&self) {
        println!("I'm satisfied for the time being..");
    }
    fn sad(&self) {
        println!("I'm sad for the time being..");
    }
}

//////////////////////
fn trigger<T: Emotions + std::fmt::Debug>(t: &T, x: &String) {
    match x.as_str() {
        "happy" => {
            t.happy();
        }
        "angry" => {
            t.angry();
        }
        "satisfied" => {
            t.satisfied();
        }
        "sad" => {
            t.sad();
        }
        _ => {
            println!("he/she has no emotions");
        }
    }
}
//////////////////////

fn main() {
    // code yet to be written..
    let naive = UserEmotion::new("someone".to_string(), "happy".to_string());
    naive.test_emotion();
    let setteled = UserEmotion::new("Brad".to_string(), String::from("satisfied"));
    setteled.test_emotion();
}
