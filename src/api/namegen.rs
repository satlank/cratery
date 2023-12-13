//! Module to generate unique logins

use rand::Rng;

/// Generate a new name
pub fn generate_name() -> String {
    let adjective = ADJECTIVES[rand::thread_rng().gen_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rand::thread_rng().gen_range(0..NOUNS.len())];
    format!("{adjective}_{noun}")
}

/// List of adjectives
const ADJECTIVES: &[&'static str] = &[
    "adorable",
    "adventurous",
    "aggressive",
    "agreeable",
    "alert",
    "alive",
    "amused",
    "angry",
    "annoyed",
    "annoying",
    "anxious",
    "arrogant",
    "ashamed",
    "attractive",
    "average",
    "awful",
    "bad",
    "beautiful",
    "better",
    "bewildered",
    "black",
    "bloody",
    "blue",
    "blue-eyed",
    "blushing",
    "bored",
    "brainy",
    "brave",
    "breakable",
    "bright",
    "busy",
    "calm",
    "careful",
    "cautious",
    "charming",
    "cheerful",
    "clean",
    "clear",
    "clever",
    "cloudy",
    "clumsy",
    "colorful",
    "combative",
    "comfortable",
    "concerned",
    "condemned",
    "confused",
    "cooperative",
    "courageous",
    "crazy",
    "creepy",
    "crowded",
    "cruel",
    "curious",
    "cute",
    "dangerous",
    "dark",
    "dead",
    "defeated",
    "defiant",
    "delightful",
    "depressed",
    "determined",
    "different",
    "difficult",
    "disgusted",
    "distinct",
    "disturbed",
    "dizzy",
    "doubtful",
    "drab",
    "dull",
    "eager",
    "easy",
    "elated",
    "elegant",
    "embarrassed",
    "enchanting",
    "encouraging",
    "energetic",
    "enthusiastic",
    "envious",
    "evil",
    "excited",
    "expensive",
    "exuberant",
    "fair",
    "faithful",
    "famous",
    "fancy",
    "fantastic",
    "fierce",
    "filthy",
    "fine",
    "foolish",
    "fragile",
    "frail",
    "frantic",
    "friendly",
    "frightened",
    "funny",
    "gentle",
    "gifted",
    "glamorous",
    "gleaming",
    "glorious",
    "good",
    "gorgeous",
    "graceful",
    "grieving",
    "grotesque",
    "grumpy",
    "handsome",
    "happy",
    "healthy",
    "helpful",
    "helpless",
    "hilarious",
    "homeless",
    "homely",
    "horrible",
    "hungry",
    "hurt",
    "ill",
    "important",
    "impossible",
    "inexpensive",
    "innocent",
    "inquisitive",
    "itchy",
    "jealous",
    "jittery",
    "jolly",
    "joyous",
    "kind",
    "lazy",
    "light",
    "lively",
    "lonely",
    "long",
    "lovely",
    "lucky",
    "magnificent",
    "misty",
    "modern",
    "motionless",
    "muddy",
    "mushy",
    "mysterious",
    "nasty",
    "naughty",
    "nervous",
    "nice",
    "nutty",
    "obedient",
    "obnoxious",
    "odd",
    "old-fashioned",
    "open",
    "outrageous",
    "outstanding",
    "panicky",
    "perfect",
    "plain",
    "pleasant",
    "poised",
    "poor",
    "powerful",
    "precious",
    "prickly",
    "proud",
    "putrid",
    "puzzled",
    "quaint",
    "real",
    "relieved",
    "repulsive",
    "rich",
    "scary",
    "selfish",
    "shiny",
    "shy",
    "silly",
    "sleepy",
    "smiling",
    "smoggy",
    "sore",
    "sparkling",
    "splendid",
    "spotless",
    "stormy",
    "strange",
    "stupid",
    "successful",
    "super",
    "talented",
    "tame",
    "tasty",
    "tender",
    "tense",
    "terrible",
    "thankful",
    "thoughtful",
    "thoughtless",
    "tired",
    "tough",
    "troubled",
    "ugliest",
    "ugly",
    "uninterested",
    "unsightly",
    "unusual",
    "upset",
    "uptight",
    "vast",
    "victorious",
    "vivacious",
    "wandering",
    "weary",
    "wicked",
    "wide-eyed",
    "wild",
    "witty",
    "worried",
    "worrisome",
    "wrong",
    "zany",
    "zealous",
];

/// List of nouns
const NOUNS: &[&'static str] = &[
    "aardvark",
    "alligator",
    "alpaca",
    "anaconda",
    "ant",
    "anteater",
    "antelope",
    "aphid",
    "armadillo",
    "asp",
    "ass",
    "baboon",
    "badger",
    "bald-eagle",
    "barracuda",
    "bass",
    "basset-hound",
    "bat",
    "bearded-dragon",
    "beaver",
    "bedbug",
    "bee",
    "bee-eater",
    "bird",
    "bison",
    "black-panther",
    "black-widow-spider",
    "blue-jay",
    "blue-whale",
    "bobcat",
    "buffalo",
    "butterfly",
    "buzzard",
    "camel",
    "canada-lynx",
    "carp",
    "cat",
    "caterpillar",
    "catfish",
    "cheetah",
    "chicken",
    "chimpanzee",
    "chipmunk",
    "cobra",
    "cod",
    "condor",
    "cougar",
    "cow",
    "coyote",
    "crab",
    "crane-fly",
    "cricket",
    "crocodile",
    "crow",
    "cuckoo",
    "deer",
    "dinosaur",
    "dog",
    "dolphin",
    "donkey",
    "dove",
    "dragonfly",
    "duck",
    "eagle",
    "eel",
    "elephant",
    "emu",
    "falcon",
    "ferret",
    "finch",
    "fish",
    "flamingo",
    "flea",
    "fly",
    "fox",
    "frog",
    "goat",
    "goose",
    "gopher",
    "gorilla",
    "guinea-pig",
    "hamster",
    "hare",
    "hawk",
    "hippopotamus",
    "horse",
    "hummingbird",
    "humpback-whale",
    "husky",
    "iguana",
    "impala",
    "kangaroo",
    "lemur",
    "leopard",
    "lion",
    "lizard",
    "llama",
    "lobster",
    "margay",
    "monitor-lizard",
    "monkey",
    "moose",
    "mosquito",
    "moth",
    "mountain-zebra",
    "mouse",
    "mule",
    "octopus",
    "orca",
    "ostrich",
    "otter",
    "owl",
    "ox",
    "oyster",
    "panda",
    "parrot",
    "peacock",
    "pelican",
    "penguin",
    "perch",
    "pheasant",
    "pig",
    "pigeon",
    "polar-bear",
    "porcupine",
    "quagga",
    "rabbit",
    "raccoon",
    "rat",
    "rattlesnake",
    "red-wolf",
    "rooster",
    "seal",
    "sheep",
    "skunk",
    "sloth",
    "snail",
    "snake",
    "spider",
    "tiger",
    "whale",
    "wolf",
    "wombat",
    "zebra",
];
