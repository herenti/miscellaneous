use urlencoding::encode;

pub struct Tran;

impl Tran {

    pub fn tran(input: &str, output: &str, search: &str) -> String {

        let lang_list = vec![("abkhaz", "abk"), ("acehnese", "ace"), ("acholi", "ach"), ("afar", "aar"), ("afrikaans", "af"), ("albanian", "sq"), ("alur", "alz"), ("amharic", "am"), ("arabic", "ar"), ("armenian", "hy"), ("assamese", "as"), ("avar", "ava"), ("awadhi", "awa"), ("aymara", "ay"), ("azerbaijani", "az"), ("balinese", "ban"), ("baluchi", "bal"), ("bambara", "bm"), ("baoulé", "bci"), ("bashkir", "bak"), ("basque", "eu"), ("batak", "bbc"), ("belarusian", "be"), ("bemba", "bem"), ("bengali", "bn"), ("betawi", "bew"), ("bhojpuri", "bho"), ("bikol", "bik"), ("bosnian", "bs"), ("breton", "bre"), ("bulgarian", "bg"), ("buryat", "bua"), ("cantonese", "yue"), ("catalan", "ca"), ("cebuano", "ceb"), ("chamorro", "cha"), ("chechen", "che"), ("chinese", "zh-tw"), ("chuukese", "chk"), ("chuvash", "chv"), ("corsican", "co"), ("crimean", "crh"), ("croatian", "hr"), ("czech", "cs"), ("danish", "da"), ("dari", "fa-af"), ("dhivehi", "dv"), ("dinka", "din"), ("dogri", "doi"), ("dombe", "dom"), ("dutch", "nl"), ("dyula", "dyu"), ("dzongkha", "dzo"), ("english", "en"), ("esperanto", "eo"), ("estonian", "et"), ("faroese", "fao"), ("fijian", "fij"), ("filipino", "fil"), ("finnish", "fi"), ("fon", "fon"), ("french", "fr"), ("frisian", "fy"), ("friulian", "fur"), ("fulani", "ful"), ("ga", "gaa"), ("galician", "gl"), ("georgian", "ka"), ("german", "de"), ("greek", "el"), ("guarani", "gn"), ("gujarati", "gu"), ("haitian", "ht"), ("hakha", "cnh"), ("hausa", "ha"), ("hawaiian", "haw"), ("hebrew", "iw"), ("hiligaynon", "hil"), ("hindi", "hi"), ("hmong", "hmn"), ("hungarian", "hu"), ("hunsrik", "hrx"), ("iban", "iba"), ("icelandic", "is"), ("igbo", "ig"), ("ilocano", "ilo"), ("indonesian", "id"), ("irish", "ga"), ("italian", "it"), ("jamaican", "jam"), ("japanese", "ja"), ("javanese", "jw"), ("jingpo", "kac"), ("kalaallisut", "kal"), ("kannada", "kn"), ("kanuri", "kau"), ("kapampangan", "pam"), ("kazakh", "kk"), ("khasi", "kha"), ("khmer", "km"), ("kiga", "cgg"), ("kikongo", "kik"), ("kinyarwanda", "rw"), ("kituba", "ktu"), ("kokborok", "trp"), ("komi", "kom"), ("konkani", "gom"), ("korean", "ko"), ("krio", "kri"), ("kurdish", "ckb"), ("kyrgyz", "ky"), ("lao", "lo"), ("latgalian", "ltg"), ("latin", "la"), ("latvian", "lv"), ("ligurian", "lij"), ("limburgish", "lim"), ("lingala", "ln"), ("lithuanian", "lt"), ("lombard", "lmo"), ("luganda", "lg"), ("luo", "luo"), ("luxembourgish", "lb"), ("macedonian", "mk"), ("madurese", "mad"), ("maithili", "mai"), ("makassar", "mak"), ("malagasy", "mg"), ("malay", "ms-arab"), ("malayalam", "ml"), ("maltese", "mt"), ("mam", "mam"), ("manx", "glv"), ("maori", "mi"), ("marathi", "mr"), ("marshallese", "mah"), ("marwadi", "mwr"), ("mauritian", "mfe"), ("meadow", "mhr"), ("meiteilon", "mni-mtei"), ("minang", "min"), ("mizo", "lus"), ("mongolian", "mn"), ("myanmar", "my"), ("nahuatl", "nhe"), ("ndau", "ndc-zw"), ("ndebele", "nde"), ("nepalbhasa", "new"), ("nepali", "ne"), ("norwegian", "no"), ("nuer", "nus"), ("nyanja", "ny"), ("occitan", "oci"), ("odia", "or"), ("oromo", "om"), ("ossetian", "oss"), ("pangasinan", "pag"), ("papiamento", "pap"), ("pashto", "ps"), ("persian", "fa"), ("polish", "pl"), ("portuguese", "pt"), ("punjabi", "pa-arab"), ("q'eqchi'", "kek"), ("quechua", "qu"), ("romani", "rom"), ("romanian", "ro"), ("rundi", "run"), ("russian", "ru"), ("sami", "sme"), ("samoan", "sm"), ("sango", "sag"), ("sanskrit", "sa"), ("santali", "sat"), ("scots", "gd"), ("sepedi", "nso"), ("serbian", "sr"), ("sesotho", "st"), ("seychellois", "crs"), ("shan", "shn"), ("shona", "sn"), ("sicilian", "scn"), ("silesian", "szl"), ("sindhi", "sd"), ("sinhala", "si"), ("slovak", "sk"), ("slovenian", "sl"), ("somali", "so"), ("spanish", "es"), ("sundanese", "su"), ("susu", "sus"), ("swahili", "sw"), ("swati", "ssw"), ("swedish", "sv"), ("tagalog", "tl"), ("tahitian", "tah"), ("tajik", "tg"), ("tamazight", "ber"), ("tamil", "ta"), ("tatar", "tt"), ("telugu", "te"), ("tetum", "tet"), ("thai", "th"), ("tibetan", "bod"), ("tigrinya", "ti"), ("tiv", "tiv"), ("tok", "tpi"), ("tongan", "ton"), ("tsonga", "ts"), ("tswana", "tsn"), ("tulu", "tcy"), ("tumbuka", "tum"), ("turkish", "tr"), ("turkmen", "tk"), ("tuvan", "tuk"), ("twi", "ak"), ("udmurt", "udm"), ("ukrainian", "uk"), ("urdu", "ur"), ("uyghur", "ug"), ("uzbek", "uz"), ("venda", "ven"), ("venetian", "vec"), ("vietnamese", "vi"), ("waray", "war"), ("welsh", "cy"), ("wolof", "wol"), ("xhosa", "xh"), ("yakut", "sah"), ("yiddish", "yi"), ("yoruba", "yo"), ("yucatec", "yua"), ("zapotec", "zap"), ("zulu", "zu")];

        let mut output = output;
        for (x, y) in &lang_list{
            if &output == x{
                output = y;
            }
        }

        let mut cont = false;
        for (x, y) in &lang_list{
            if y == &output{
                cont = true;
            }
        }
        if cont{
            let url = format!("https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl={}&tl={}&q={}", input, output, encode(search));
            let res = reqwest::blocking::get(url).expect("REASON").text().unwrap();
            let data: serde_json::Value = serde_json::from_str(&res).unwrap();
            let result = if data[0][0].as_str().is_some(){
                data[0][0].as_str().unwrap().to_string()
            } else {
                "No Translation found.".to_string()
            };
            result
        } else {
            "That is not a valid language.".to_string()
        }
    }
}
