extern crate phf;

use phf::phf_map;
use publicsuffix::List;

// Stack allocate these at compile time
pub static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

lazy_static! {
    pub static ref DOMAIN_LIST: List =
        List::from_path("src/database/effective_tld_names.dat").unwrap();
}

static QWERTY_KEYBOARD_LAYOUT: phf::Map<char, &'static str> = phf_map! {
    '1' => "2q",
    '2' => "3wq1",
    '3' => "4ew2",
    '4' => "5re3",
    '5' => "6tr4",
    '6' => "7yt5",
    '7' => "8uy6",
    '8' => "9iu7",
    '9' => "0oi8",
    '0' => "po9",
    'q' => "12wa",
    'w' => "3esaq2",
    'e' => "4rdsw3",
    'r' => "5tfde4",
    't' => "6ygfr5",
    'y' => "7uhgt6",
    'u' => "8ijhy7",
    'i' => "9okju8",
    'o' => "0plki9",
    'p' => "lo0",
    'a' => "qwsz",
    's' => "edxzaw",
    'd' => "rfcxse",
    'f' => "tgvcdr",
    'g' => "yhbvft",
    'h' => "ujnbgy",
    'j' => "ikmnhu",
    'k' => "olmji",
    'l' => "kop",
    'z' => "asx",
    'x' => "zsdc",
    'c' => "xdfv",
    'v' => "cfgb",
    'b' => "vghn",
    'n' => "bhjm",
    'm' => "njk"
};
static QWERTZ_KEYBOARD_LAYOUT: phf::Map<char, &'static str> = phf_map! {
    '1'=> "2q",
    '2'=> "3wq1",
    '3'=> "4ew2",
    '4'=> "5re3",
    '5'=> "6tr4",
    '6'=> "7zt5",
    '7'=> "8uz6",
    '8'=> "9iu7",
    '9'=> "0oi8",
    '0'=> "po9",
    'q'=> "12wa",
    'w'=> "3esaq2",
    'e'=> "4rdsw3",
    'r'=> "5tfde4",
    't'=> "6zgfr5",
    'z'=> "7uhgt6",
    'u'=> "8ijhz7",
    'i'=> "9okju8",
    'o'=> "0plki9",
    'p'=> "lo0",
    'a'=> "qwsy",
    's'=> "edxyaw",
    'd'=> "rfcxse",
    'f'=> "tgvcdr",
    'g'=> "zhbvft",
    'h'=> "ujnbgz",
    'j'=> "ikmnhu",
    'k'=> "olmji",
    'l'=> "kop",
    'y'=> "asx",
    'x'=> "ysdc",
    'c'=> "xdfv",
    'v'=> "cfgb",
    'b'=> "vghn",
    'n'=> "bhjm",
    'm'=> "njk"
};
static AZERTY_KEYBOARD_LAYOUT: phf::Map<char, &'static str> = phf_map! {
    '1'=> "2a",
    '2'=> "3za1",
    '3'=> "4ez2",
    '4'=> "5re3",
    '5'=> "6tr4",
    '6'=> "7yt5",
    '7'=> "8uy6",
    '8'=> "9iu7",
    '9'=> "0oi8",
    '0'=> "po9",
    'a'=> "2zq1",
    'z'=> "3esqa2",
    'e'=> "4rdsz3",
    'r'=> "5tfde4",
    't'=> "6ygfr5",
    'y'=> "7uhgt6",
    'u'=> "8ijhy7",
    'i'=> "9okju8",
    'o'=> "0plki9",
    'p'=> "lo0m",
    'q'=> "zswa",
    's'=> "edxwqz",
    'd'=> "rfcxse",
    'f'=> "tgvcdr",
    'g'=> "yhbvft",
    'h'=> "ujnbgy",
    'j'=> "iknhu",
    'k'=> "olji",
    'l'=> "kopm",
    'm'=> "lp",
    'w'=> "sxq",
    'x'=> "wsdc",
    'c'=> "xdfv",
    'v'=> "cfgb",
    'b'=> "vghn",
    'n'=> "bhj"
};

lazy_static! {
    pub static ref KEYBOARD_LAYOUTS: Vec<&'static phf::Map<char, &'static str>> = vec![
        &QWERTY_KEYBOARD_LAYOUT,
        &QWERTZ_KEYBOARD_LAYOUT,
        &AZERTY_KEYBOARD_LAYOUT
    ];
}