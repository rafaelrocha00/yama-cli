pub mod japanese {
    use std::collections::hash_map::HashMap;

    pub struct Kana {
        map: HashMap<String, String>
    }

    impl Kana {
        pub fn new(use_katakana: bool) -> Kana {
            let mut kana = Kana
            {
                map: HashMap::new()
            };
    
            kana.map.insert("a".to_string(), "あ".to_string());
            kana.map.insert("i".to_string(), "い".to_string());
            kana.map.insert("u".to_string(), "う".to_string());
            kana.map.insert("e".to_string(), "え".to_string());
            kana.map.insert("o".to_string(), "お".to_string());
        
            kana.map.insert("ka".to_string(), "か".to_string());
            kana.map.insert("ki".to_string(), "き".to_string());
            kana.map.insert("ku".to_string(), "く".to_string());
            kana.map.insert("ke".to_string(), "け".to_string());
            kana.map.insert("ko".to_string(), "こ".to_string());
        
            kana.map.insert("sa".to_string(), "さ".to_string());
            kana.map.insert("shi".to_string(), "し".to_string());
            kana.map.insert("su".to_string(), "す".to_string());
            kana.map.insert("se".to_string(), "せ".to_string());
            kana.map.insert("so".to_string(), "そ".to_string());
        
            kana.map.insert("ta".to_string(), "た".to_string());
            kana.map.insert("chi".to_string(), "ち".to_string());
            kana.map.insert("tsu".to_string(), "つ".to_string());
            kana.map.insert("te".to_string(), "て".to_string());
            kana.map.insert("to".to_string(), "と".to_string());
        
            kana.map.insert("na".to_string(), "な".to_string());
            kana.map.insert("ni".to_string(), "に".to_string());
            kana.map.insert("nu".to_string(), "ぬ".to_string());
            kana.map.insert("ne".to_string(), "ね".to_string());
            kana.map.insert("no".to_string(), "の".to_string());
        
            kana.map.insert("ha".to_string(), "は".to_string());
            kana.map.insert("hi".to_string(), "ひ".to_string());
            kana.map.insert("fu".to_string(), "ふ".to_string());
            kana.map.insert("he".to_string(), "へ".to_string());
            kana.map.insert("ho".to_string(), "ほ".to_string());
            kana.map.insert("ma".to_string(), "ま".to_string());
            kana.map.insert("mi".to_string(), "み".to_string());
            kana.map.insert("mu".to_string(), "む".to_string());
            kana.map.insert("me".to_string(), "め".to_string());
            kana.map.insert("mo".to_string(), "も".to_string());
            kana.map.insert("ya".to_string(), "や".to_string());
            kana.map.insert("yu".to_string(), "ゆ".to_string());
            kana.map.insert("yo".to_string(), "よ".to_string());
            kana.map.insert("ra".to_string(), "ら".to_string());
            kana.map.insert("ri".to_string(), "り".to_string());
            kana.map.insert("ru".to_string(), "る".to_string());
            kana.map.insert("re".to_string(), "れ".to_string());
            kana.map.insert("ro".to_string(), "ろ".to_string());
            kana.map.insert("wa".to_string(), "わ".to_string());
            kana.map.insert("wo".to_string(), "を".to_string());
            kana.map.insert("nn".to_string(), "ん".to_string());
            kana.map.insert("ga".to_string(), "が".to_string());
            kana.map.insert("gi".to_string(), "ぎ".to_string());
            kana.map.insert("gu".to_string(), "ぐ".to_string());
            kana.map.insert("ge".to_string(), "げ".to_string());
            kana.map.insert("go".to_string(), "ご".to_string());
            kana.map.insert("za".to_string(), "ざ".to_string());
            kana.map.insert("ji".to_string(), "じ".to_string());
            kana.map.insert("zu".to_string(), "ず".to_string());
            kana.map.insert("ze".to_string(), "ぜ".to_string());
            kana.map.insert("zo".to_string(), "ぞ".to_string());
            kana.map.insert("da".to_string(), "だ".to_string());
            kana.map.insert("ji".to_string(), "ぢ".to_string());
            kana.map.insert("zu".to_string(), "づ".to_string());
            kana.map.insert("de".to_string(), "で".to_string());
            kana.map.insert("do".to_string(), "ど".to_string());
            kana.map.insert("ba".to_string(), "ば".to_string());
            kana.map.insert("bi".to_string(), "び".to_string());
            kana.map.insert("bu".to_string(), "ぶ".to_string());
            kana.map.insert("be".to_string(), "べ".to_string());
            kana.map.insert("bo".to_string(), "ぼ".to_string());
            kana.map.insert("pa".to_string(), "ぱ".to_string());
            kana.map.insert("pi".to_string(), "ぴ".to_string());
            kana.map.insert("pu".to_string(), "ぷ".to_string());
            kana.map.insert("pe".to_string(), "ぺ".to_string());
            kana.map.insert("po".to_string(), "ぽ".to_string());

            if use_katakana {
                kana.map.insert("A".to_string(), "ア".to_string());
                kana.map.insert("I".to_string(), "イ".to_string());
                kana.map.insert("U".to_string(), "ウ".to_string());
                kana.map.insert("E".to_string(), "エ".to_string());
                kana.map.insert("O".to_string(), "オ".to_string());
                kana.map.insert("KA".to_string(), "カ".to_string());
                kana.map.insert("KI".to_string(), "キ".to_string());
                kana.map.insert("KU".to_string(), "ク".to_string());
                kana.map.insert("KE".to_string(), "ケ".to_string());
                kana.map.insert("KO".to_string(), "コ".to_string());
                kana.map.insert("SA".to_string(), "サ".to_string());
                kana.map.insert("SHI".to_string(), "シ".to_string());
                kana.map.insert("CHI".to_string(), "チ".to_string());
                kana.map.insert("TSU".to_string(), "ツ".to_string());
                kana.map.insert("SU".to_string(), "ス".to_string());
                kana.map.insert("SE".to_string(), "セ".to_string());
                kana.map.insert("SO".to_string(), "ソ".to_string());
                kana.map.insert("TA".to_string(), "タ".to_string());
                kana.map.insert("TE".to_string(), "テ".to_string());
                kana.map.insert("TO".to_string(), "ト".to_string());
                kana.map.insert("NA".to_string(), "ナ".to_string());
                kana.map.insert("NI".to_string(), "ニ".to_string());
                kana.map.insert("NU".to_string(), "ヌ".to_string());
                kana.map.insert("NE".to_string(), "ネ".to_string());
                kana.map.insert("NO".to_string(), "ノ".to_string());
                kana.map.insert("HA".to_string(), "ハ".to_string());
                kana.map.insert("HI".to_string(), "ヒ".to_string());
                kana.map.insert("FU".to_string(), "フ".to_string());
                kana.map.insert("HE".to_string(), "ヘ".to_string());
                kana.map.insert("HO".to_string(), "ホ".to_string());
                kana.map.insert("MA".to_string(), "マ".to_string());
                kana.map.insert("MI".to_string(), "ミ".to_string());
                kana.map.insert("MU".to_string(), "ム".to_string());
                kana.map.insert("ME".to_string(), "メ".to_string());
                kana.map.insert("MO".to_string(), "モ".to_string());
                kana.map.insert("YA".to_string(), "ヤ".to_string());
                kana.map.insert("YU".to_string(), "ユ".to_string());
                kana.map.insert("YO".to_string(), "ヨ".to_string());
                kana.map.insert("RA".to_string(), "ラ".to_string());
                kana.map.insert("RI".to_string(), "リ".to_string());
                kana.map.insert("RU".to_string(), "ル".to_string());
                kana.map.insert("RE".to_string(), "レ".to_string());
                kana.map.insert("RO".to_string(), "ロ".to_string());
                kana.map.insert("WA".to_string(), "ワ".to_string());
                kana.map.insert("WO".to_string(), "ヲ".to_string());
                kana.map.insert("NN".to_string(), "ン".to_string());
                kana.map.insert("GA".to_string(), "ガ".to_string());
                kana.map.insert("GI".to_string(), "ギ".to_string());
                kana.map.insert("GU".to_string(), "グ".to_string());
                kana.map.insert("GE".to_string(), "ゲ".to_string());
                kana.map.insert("GO".to_string(), "ゴ".to_string());
                kana.map.insert("ZA".to_string(), "ザ".to_string());
                kana.map.insert("JI".to_string(), "ジ".to_string());
                kana.map.insert("ZU".to_string(), "ズ".to_string());
                kana.map.insert("ZE".to_string(), "ゼ".to_string());
                kana.map.insert("ZO".to_string(), "ゾ".to_string());
                kana.map.insert("DA".to_string(), "ダ".to_string());
                kana.map.insert("DI".to_string(), "ヂ".to_string());
                kana.map.insert("DU".to_string(), "ヅ".to_string());
                kana.map.insert("DE".to_string(), "デ".to_string());
                kana.map.insert("DO".to_string(), "ド".to_string());
                kana.map.insert("BA".to_string(), "バ".to_string());
                kana.map.insert("BI".to_string(), "ビ".to_string());
                kana.map.insert("BU".to_string(), "ブ".to_string());
                kana.map.insert("BE".to_string(), "ベ".to_string());
                kana.map.insert("BO".to_string(), "ボ".to_string());
                kana.map.insert("PA".to_string(), "パ".to_string());
                kana.map.insert("PI".to_string(), "ピ".to_string());
                kana.map.insert("PU".to_string(), "プ".to_string());
                kana.map.insert("PE".to_string(), "ペ".to_string());
                kana.map.insert("PO".to_string(), "ポ".to_string());
            }
        
            return kana
        }

        pub fn translate(&self, word: &str) -> String {
            let mut kana_translation = String::new();
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

                    if let Some(Kana) = self.map.get(slice) {
                        syllable_jump = j - 1;
                        found_translation = true;
                        kana_translation.push_str(Kana);
                        break
                    }
                }

                if !found_translation {
                    kana_translation.push(char)
                }
            }

            kana_translation.chars().collect()
        }
    }
}