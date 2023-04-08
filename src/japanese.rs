pub mod japanese {
    use std::collections::hash_map::HashMap;

    pub struct Hiragana {
        map: HashMap<String, String>
    }

    impl Hiragana {
        pub fn new() -> Hiragana {
            let mut hiragana = Hiragana
            {
                map: HashMap::new()
            };
    
            hiragana.map.insert("a".to_string(), "あ".to_string());
            hiragana.map.insert("i".to_string(), "い".to_string());
            hiragana.map.insert("u".to_string(), "う".to_string());
            hiragana.map.insert("e".to_string(), "え".to_string());
            hiragana.map.insert("o".to_string(), "お".to_string());
        
            hiragana.map.insert("ka".to_string(), "か".to_string());
            hiragana.map.insert("ki".to_string(), "き".to_string());
            hiragana.map.insert("ku".to_string(), "く".to_string());
            hiragana.map.insert("ke".to_string(), "け".to_string());
            hiragana.map.insert("ko".to_string(), "こ".to_string());
        
            hiragana.map.insert("sa".to_string(), "さ".to_string());
            hiragana.map.insert("shi".to_string(), "し".to_string());
            hiragana.map.insert("su".to_string(), "す".to_string());
            hiragana.map.insert("se".to_string(), "せ".to_string());
            hiragana.map.insert("so".to_string(), "そ".to_string());
        
            hiragana.map.insert("ta".to_string(), "た".to_string());
            hiragana.map.insert("chi".to_string(), "ち".to_string());
            hiragana.map.insert("tsu".to_string(), "つ".to_string());
            hiragana.map.insert("te".to_string(), "て".to_string());
            hiragana.map.insert("to".to_string(), "と".to_string());
        
            hiragana.map.insert("na".to_string(), "な".to_string());
            hiragana.map.insert("ni".to_string(), "に".to_string());
            hiragana.map.insert("nu".to_string(), "ぬ".to_string());
            hiragana.map.insert("ne".to_string(), "ね".to_string());
            hiragana.map.insert("no".to_string(), "の".to_string());
        
            hiragana.map.insert("ha".to_string(), "は".to_string());
            hiragana.map.insert("hi".to_string(), "ひ".to_string());
            hiragana.map.insert("fu".to_string(), "ふ".to_string());
            hiragana.map.insert("he".to_string(), "へ".to_string());
            hiragana.map.insert("ho".to_string(), "ほ".to_string());
            hiragana.map.insert("ma".to_string(), "ま".to_string());
            hiragana.map.insert("mi".to_string(), "み".to_string());
            hiragana.map.insert("mu".to_string(), "む".to_string());
            hiragana.map.insert("me".to_string(), "め".to_string());
            hiragana.map.insert("mo".to_string(), "も".to_string());
            hiragana.map.insert("ya".to_string(), "や".to_string());
            hiragana.map.insert("yu".to_string(), "ゆ".to_string());
            hiragana.map.insert("yo".to_string(), "よ".to_string());
            hiragana.map.insert("ra".to_string(), "ら".to_string());
            hiragana.map.insert("ri".to_string(), "り".to_string());
            hiragana.map.insert("ru".to_string(), "る".to_string());
            hiragana.map.insert("re".to_string(), "れ".to_string());
            hiragana.map.insert("ro".to_string(), "ろ".to_string());
            hiragana.map.insert("wa".to_string(), "わ".to_string());
            hiragana.map.insert("wo".to_string(), "を".to_string());
            hiragana.map.insert("n".to_string(), "ん".to_string());
            hiragana.map.insert("ga".to_string(), "が".to_string());
            hiragana.map.insert("gi".to_string(), "ぎ".to_string());
            hiragana.map.insert("gu".to_string(), "ぐ".to_string());
            hiragana.map.insert("ge".to_string(), "げ".to_string());
            hiragana.map.insert("go".to_string(), "ご".to_string());
            hiragana.map.insert("za".to_string(), "ざ".to_string());
            hiragana.map.insert("ji".to_string(), "じ".to_string());
            hiragana.map.insert("zu".to_string(), "ず".to_string());
            hiragana.map.insert("ze".to_string(), "ぜ".to_string());
            hiragana.map.insert("zo".to_string(), "ぞ".to_string());
            hiragana.map.insert("da".to_string(), "だ".to_string());
            hiragana.map.insert("ji".to_string(), "ぢ".to_string());
            hiragana.map.insert("zu".to_string(), "づ".to_string());
            hiragana.map.insert("de".to_string(), "で".to_string());
            hiragana.map.insert("do".to_string(), "ど".to_string());
            hiragana.map.insert("ba".to_string(), "ば".to_string());
            hiragana.map.insert("bi".to_string(), "び".to_string());
            hiragana.map.insert("bu".to_string(), "ぶ".to_string());
            hiragana.map.insert("be".to_string(), "べ".to_string());
            hiragana.map.insert("bo".to_string(), "ぼ".to_string());
            hiragana.map.insert("pa".to_string(), "ぱ".to_string());
            hiragana.map.insert("pi".to_string(), "ぴ".to_string());
            hiragana.map.insert("pu".to_string(), "ぷ".to_string());
            hiragana.map.insert("pe".to_string(), "ぺ".to_string());
            hiragana.map.insert("po".to_string(), "ぽ".to_string());
        
            return hiragana
        }

        pub fn translate(&self, word: &str) -> String {
            let mut hiragana_translation = String::new();
            let word_len = word.chars().count();
            
            let mut syllable_jump = 0;

            for (i, char) in word.char_indices() {

                if syllable_jump > 0 {
                    syllable_jump -= 1;
                    continue
                }

                let mut found_translation = false;
                
                for j in 1..=3 {
                    if i + j > word_len {
                        continue;
                    }

                    let slice = &word[i..i+j];

                    if let Some(hiragana) = self.map.get(slice) {
                        syllable_jump = j - 1;
                        found_translation = true;
                        hiragana_translation.push_str(hiragana);
                        break
                    }
                }

                if !found_translation {
                    hiragana_translation.push(char)
                }
            }

            hiragana_translation.chars().collect()
        }
    }
}