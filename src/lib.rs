mod utils;

use std::str::FromStr;

use once_cell::sync::Lazy;
use tokenizers::Tokenizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

static TOKENIZER_JSON: &'static str = include_str!("./assets/tokenizer.min.json");
static TOKENIZER: Lazy<Tokenizer> = Lazy::new(|| Tokenizer::from_str(TOKENIZER_JSON).unwrap());

#[wasm_bindgen]
pub fn tokenize(text: &str) -> Token {
    let encoding = TOKENIZER.encode(text, true).unwrap();

    Token {
        tokens: encoding.get_tokens().into_iter().cloned().collect(),
        input_ids: encoding.get_ids().into_iter().cloned().collect(),
        attention_mask: encoding.get_attention_mask().into_iter().cloned().collect(),
    }
}

#[wasm_bindgen]
pub struct Token {
    tokens: Vec<String>,
    input_ids: Vec<u32>,
    attention_mask: Vec<u32>,
}

#[wasm_bindgen]
impl Token {
    #[wasm_bindgen(getter = tokens)]
    pub fn get_tokens(&self) -> Vec<String> {
        self.tokens.clone()
    }

    #[wasm_bindgen(getter = inputIds)]
    pub fn get_ids(&self) -> Vec<u32> {
        self.input_ids.clone()
    }

    #[wasm_bindgen(getter = attentionMask)]
    pub fn get_attention_mask(&self) -> Vec<u32> {
        self.attention_mask.clone()
    }
}
