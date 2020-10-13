#![allow(clippy::wildcard_imports)]

use heck::TitleCase;
use itertools::Itertools;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use seed::{prelude::*, *};
use synthlang::SynthLang;

#[macro_use]
extern crate version;

fn init(mut url: Url, _: &mut impl Orders<Msg>) -> Model {
    let seed = url
        .next_hash_path_part()
        .and_then(|seed_str| u64::from_str_radix(seed_str, 16).ok())
        .unwrap_or_else(|| {
            let mut seed_rng = thread_rng();
            seed_rng.gen()
        });

    let lang_data = generate_lang(seed);

    Model { lang_data }
}

fn generate_lang(seed: u64) -> LangData {
    let mut our_rng = Pcg64::seed_from_u64(seed);

    let mut lang = SynthLang::new(seed);

    let sample_english = vec![
        "child",
        "horse",
        "soldier",
        "pig",
        "table",
        "cow",
        "pants",
        "torch",
        "scroll",
        "priest",
        "hat",
        "throne",
        "dream",
        "cheese",
        "sword",
        "son",
        "shopkeeper",
        "tunic",
        "daughter",
        "queen",
    ];

    let mut samples = vec![];

    for english in sample_english {
        samples.push((lang.word().to_string(), english.to_string()));
    }

    let mut consonants = vec![];

    for consonant in &lang.consonants {
        consonants.push(consonant.to_string());
    }

    let mut vowels = vec![];

    for vowel in &lang.vowels {
        vowels.push(vowel.to_string());
    }

    let adjectives = vec![
        ("red", lang.word()),
        ("orange", lang.word()),
        ("yellow", lang.word()),
        ("green", lang.word()),
        ("blue", lang.word()),
        ("purple", lang.word()),
        ("black", lang.word()),
        ("white", lang.word()),
        ("brown", lang.word()),
        ("grey", lang.word()),
        ("great", lang.word()),
        ("old", lang.word()),
        ("serene", lang.word()),
        ("frigid", lang.word()),
        ("scorching", lang.word()),
    ];
    let nouns = vec![
        ("river", lang.word()),
        ("island", lang.word()),
        ("harbor", lang.word()),
        ("mountain", lang.word()),
        ("plains", lang.word()),
        ("hills", lang.word()),
        ("woods", lang.word()),
        ("cove", lang.word()),
        ("swamp", lang.word()),
        ("marsh", lang.word()),
        ("lake", lang.word()),
    ];

    let mut places = vec![];

    for _ in 0..13 {
        let adjective = adjectives.choose(&mut our_rng).unwrap();
        let noun = nouns.choose(&mut our_rng).unwrap();

        let compound = lang.compound(&adjective.1, &noun.1).to_string();

        places.push((
            compound,
            adjective.1.to_string(),
            noun.1.to_string(),
            adjective.0.to_string(),
            noun.0.to_string(),
        ));
    }

    places.dedup();

    LangData {
        name: lang.word().to_string().to_title_case(),
        samples,
        consonants,
        vowels,
        places,
        seed,
        lang,
    }
}

struct Model {
    lang_data: LangData,
}

struct LangData {
    name: String,
    samples: Vec<(String, String)>,
    consonants: Vec<String>,
    vowels: Vec<String>,
    places: Vec<(String, String, String, String, String)>,
    seed: u64,
    lang: SynthLang,
}

// update

#[derive(Copy, Clone)]
enum Msg {
    Generate,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Generate => {
            let mut seed_rng = thread_rng();

            (*model).lang_data = generate_lang(seed_rng.gen());
        }
    }
}

// Views

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![
            C!["nav-container"],
            div![
                C!["container grid-lg"],
                header![
                    C!["navbar"],
                    section![
                        C!["navbar-section"],
                        a![C!["navbar-brand mr-2"], "SynthLang ", version!()]
                    ],
                    section![
                        C!["navbar-section"],
                        a![
                            C!["btn btn-primary"],
                            attrs! {At::Href => "https://github.com/rparrett/synthlang-web"},
                            "Source"
                        ]
                    ]
                ],
            ]
        ],
        div![
            C!["container grid-lg"],
            div![
                C!["banner text-center"],
                div![C!["text-gray"], "You have created"],
                h1![
                    "The distinguished language of ",
                    span![model.lang_data.name.as_str(), C!["text-primary"]],
                ],
                div![button![
                    C!["btn btn-primary"],
                    "Generate another!",
                    ev(Ev::Click, |_| Msg::Generate),
                ],]
            ],
            div![
                C!["columns"],
                div![
                    C!["column col-8 col-md-12"],
                    samples_view(model),
                    places_view(model)
                ],
                div![
                    C!["column col-4 col-md-12"],
                    parts_view(model),
                    other_view(model)
                ]
            ],
        ],
    ]
}

fn samples_view(model: &Model) -> Node<Msg> {
    div![
        C!["card"],
        div![
            C!["card-header"],
            div![
                C!["card-title h5"],
                "Sample vocabulary in ",
                span![C!["text-primary"], model.lang_data.name.as_str(),]
            ],
            div![
                C!["card-subtitle text-gray"],
                format!(
                    "These are some sample {} vocabulary words, with their English translations.",
                    model.lang_data.name
                ),
            ]
        ],
        div![
            C!["card-body columns"],
            model
                .lang_data
                .samples
                .iter()
                .chunks(model.lang_data.samples.len() / 2)
                .into_iter()
                .map(|chunk| {
                    div![
                        C!["column col-6"],
                        table![
                            C!["table"],
                            tr![th!["Word"], th!["Meaning"]],
                            chunk.into_iter().map(|sample| {
                                tr![td![sample.0.as_str()], td![C!["text-gray"],sample.1.as_str()]]
                            })
                        ]
                    ]
                })
        ]
    ]
}

fn parts_view(model: &Model) -> Node<Msg> {
    div![
        C!["card"],
        div![
            C!["card-header"],
            div![
                C!["card-title h5"],
                "Syllable parts in ",
                span![C!["text-primary"], model.lang_data.name.as_str(),],
            ],
            div![
                C!["card-subtitle text-gray"],
                format!(
                    "These the building blocks of words in {}.",
                    model.lang_data.name
                ),
            ]
        ],
        div![
            C!["card-body columns"],
            div![
                C!["column col-6"],
                table![
                    C!["table"],
                    tr![th!["Consonants",],],
                    model
                        .lang_data
                        .consonants
                        .iter()
                        .map(|consonant| { tr![td![consonant.as_str()]] })
                ],
            ],
            div![
                C!["column col-6"],
                table![
                    C!["table"],
                    tr![th!["Vowels",],],
                    model
                        .lang_data
                        .vowels
                        .iter()
                        .map(|consonant| { tr![td![consonant.as_str()]] })
                ]
            ],
        ]
    ]
}

fn places_view(model: &Model) -> Node<Msg> {
    div![
        C!["card"],
        div![
            C!["card-header"],
            div![
                C!["card-title h5"],
                "Famous places in ",
                span![C!["text-primary"], model.lang_data.name.as_str(),],
            ],
            div![
                C!["card-subtitle text-gray"],
                format!(
                    "These are some fictional places, and what people speaking {} might call them.",
                    model.lang_data.name
                ),
            ]
        ],
        div![
            C!["card-body"],
            table![
                C!["table"],
                tr![th!["Name"], th!["Meaning"], th![""]],
                model.lang_data.places.iter().map(|place| {
                    tr![
                        td![place.0.to_title_case()],
                        td![C!["text-gray"], format!(
                            "\"{} {}\"",
                            place.3.to_title_case(),
                            place.4.to_title_case()
                        )],
                        td![C!["text-gray"], format!(
                            "from {} (\"{}\") and {} (\"{}\")",
                            place.1, place.3, place.2, place.4
                        )],
                    ]
                })
            ]
        ]
    ]
}

fn other_view(model: &Model) -> Node<Msg> {
    div![
        C!["card"],
        div![
            C!["card-header"],
            div![
                C!["card-title h5"],
                "Other parameters for ",
                span![C!["text-primary"], model.lang_data.name.as_str(),],
            ],
            div![
                C!["card-subtitle text-gray"],
                format!(
                    "These values affect the overall shape of words in {} ",
                    model.lang_data.name
                ),
            ]
        ],
        div![
            C!["card-body"],
            table![
                C!["table"],
                tr![th!["Name"], th!["Value"],],
                tr![
                    td!["Seed"],
                    td![a![
                        attrs! {At::Href => format!("#/{:x}", model.lang_data.seed)},
                        format!("{:x}", model.lang_data.seed),
                    ]]
                ],
                tr![td!["VC Weight"], td![model.lang_data.lang.vc_weight],],
                tr![td!["CV Weight"], td![model.lang_data.lang.cv_weight],],
                tr![td!["CVC Weight"], td![model.lang_data.lang.cvc_weight],]
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
