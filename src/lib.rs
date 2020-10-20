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

fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    orders.subscribe(Msg::UrlRequested);

    let seed = url_to_seed(&mut url);
    let lang_data = generate_lang(seed);

    Model {
        lang_data,
        more_words_modal: false,
    }
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

    places.sort();
    places.dedup();
    places.shuffle(&mut our_rng);

    LangData {
        name: lang.word().to_string().to_title_case(),
        samples,
        consonants,
        vowels,
        places,
        more_words: vec![],
        seed,
        lang,
    }
}

struct Model {
    lang_data: LangData,
    more_words_modal: bool,
}

struct LangData {
    name: String,
    samples: Vec<(String, String)>,
    consonants: Vec<String>,
    vowels: Vec<String>,
    places: Vec<(String, String, String, String, String)>,
    more_words: Vec<String>,
    seed: u64,
    lang: SynthLang,
}

// update

#[derive(Clone)]
enum Msg {
    UrlChanged(subs::UrlChanged),
    UrlRequested(subs::UrlRequested),
    CloseMoreWordsModal,
    OpenMoreWordsModal,
}

fn url_to_seed(url: &mut Url) -> u64 {
    let seed: Option<u64> = match url.next_path_part() {
        Some("seed") => {
            let _version = url.next_path_part(); // TODO
            let seed = url
                .next_path_part()
                .and_then(|seed_str| u64::from_str_radix(seed_str, 16).ok());
            seed
        }
        _ => None,
    };

    match seed {
        Some(seed) => seed,
        None => {
            let mut seed_rng = thread_rng();
            seed_rng.gen()
        }
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            let seed = url_to_seed(&mut url);
            (*model).lang_data = generate_lang(seed);
        }
        Msg::UrlRequested(subs::UrlRequested(mut url, url_request)) => {
            // Right now the only possible urls are
            //
            // /seed/{version}/{seed} and
            // /
            //
            // The goal here is to only add to the browser history and
            // scroll to the top of the page when we navigate to a new
            // permalink.

            let permalink = Url::current().path().len() != 0;
            let to_permalink = url.path().len() != 0;

            match (permalink, to_permalink) {
                (false, true) => {
                    // We are showing a random language and navigating
                    // to its permalink

                    url_request.handled();
                    url.go_and_replace();
                }
                (false, false) => {
                    // We are generating a new random language

                    url_request.handled_and_prevent_refresh();
                    url.go_and_replace();
                    let seed = url_to_seed(&mut url);
                    (*model).lang_data = generate_lang(seed);
                }
                (true, true) => {
                    // We are clicking the permalink link again for the
                    // same language.

                    url_request.handled_and_prevent_refresh();
                    url.go_and_replace();
                }
                (true, false) => {
                    // We are showing a permalink and generating a new
                    // random language.
                }
            }
        }
        Msg::OpenMoreWordsModal => {
            let more_words = (0..40)
                .map(|_| model.lang_data.lang.word().to_string())
                .collect();
            model.lang_data.more_words = more_words;

            model.more_words_modal = true;
        }
        Msg::CloseMoreWordsModal => {
            model.more_words_modal = false;
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
                div![a![
                    C!["btn btn-primary"],
                    attrs! {At::Href => "/"},
                    "Generate another!",
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
        more_words_modal_view(model),
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
            C!["card-body"],
            div![
                C!["columns"],
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
                                    tr![
                                        td![sample.0.as_str()],
                                        td![C!["text-gray"], sample.1.as_str()]
                                    ]
                                })
                            ]
                        ]
                    })
            ],
            div![button![
                C!["btn btn-link"],
                ev(Ev::Click, |_| Msg::OpenMoreWordsModal),
                "More Words"
            ]]
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
                        td![
                            C!["text-gray"],
                            format!(
                                "\"{} {}\"",
                                place.3.to_title_case(),
                                place.4.to_title_case()
                            )
                        ],
                        td![
                            C!["text-gray"],
                            format!(
                                "from {} (\"{}\") and {} (\"{}\")",
                                place.1, place.3, place.2, place.4
                            )
                        ],
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
                        attrs! {At::Href => format!("/seed/{}/{:x}", version!(), model.lang_data.seed)},
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

fn more_words_modal_view(model: &Model) -> Node<Msg> {
    let c = if model.more_words_modal {
        "modal active"
    } else {
        "modal"
    };

    // chunks doesn't work when len = 0?
    let content: Vec<Node<Msg>> = if model.lang_data.more_words.is_empty() {
        model
            .lang_data
            .more_words
            .iter()
            .chunks(model.lang_data.more_words.len() / 4)
            .into_iter()
            .map(|chunk| {
                div![
                    C!["column col-3"],
                    chunk.into_iter().map(|word| { div![word.as_str()] })
                ]
            })
            .collect()
    } else {
        vec![]
    };

    div![
        C![c],
        div![
            C!["modal-overlay"],
            ev(Ev::Click, |_| Msg::CloseMoreWordsModal)
        ],
        div![
            C!["modal-container"],
            div![
                C!["modal-header"],
                div![
                    C!["btn btn-clear float-right"],
                    ev(Ev::Click, |_| Msg::CloseMoreWordsModal)
                ],
                div![
                    C!["modal-title h5"],
                    "More Words in ",
                    span![C!["text-primary"], model.lang_data.name.as_str(),],
                ]
            ],
            div![C!["modal-body"], div![C!["content columns"], content]]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
