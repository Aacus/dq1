fn main() {
    println!("//// [FC]ドラゴンクエスト攻略 (https://www.aacus.art/2022/02/8-5.html)\n////  by エイカサート (https://aacus.art)\n");
    help();
    loop {
        let mut com = String::new();
        std::io::stdin().read_line(&mut com).unwrap();
        let comm: Vec<&str> = com.splitn(2, ' ').collect();
        comm.push("");
        match comm[0].trim() {
            "" => help(),
            "ex" => lv(comm[1].trim().parse::<i32>().unwrap_or(0), 0),
            "mo" => mon(),
            "bu" => buki(),
            "bo" => bougu(),
            "do" => dougu(),
            "ju" => jumon(),
            "ko" => tekikou(),
            "lv" => lv(0, comm[1].trim().parse::<i32>().unwrap_or(1)),
            "na" => name(comm[1]),
            _ => (),
        }
    }
}
fn help() {
    println!("Enter(機能一覧) bu(武器) bo(防具) do(道具) ju(呪文) mo(モンスター) ko(敵攻撃)\nna なまえ(ステータスボーナス表示)\nex 経験値数(次のレベルまで)\nlv LV数(ステータス)")
}
fn name(na: &str) {
    // 名前からステータス判定
    let mut po = 60;
    let st;
    for x in na.trim().chars() {
        po -= 15; // 4文字に足りない分は空白扱い
        match x {
            'き' | 'ぬ' | 'ら' => (),
            'く' | 'ね' | 'り' => po += 1,
            'け' | 'の' | 'る' => po += 2,
            'こ' | 'は' | 'れ' => po += 3,
            'さ' | 'ひ' | 'ろ' => po += 4,
            'し' | 'ふ' | 'わ' => po += 5,
            'す' | 'へ' | 'を' => po += 6,
            'せ' | 'ほ' | 'ん' => po += 7,
            'そ' | 'ま' | 'っ' => po += 8,
            'た' | 'み' | 'ゃ' => po += 9,
            'あ' | 'ち' | 'む' | 'ゅ' => po += 10,
            'い' | 'つ' | 'め' | 'ょ' => po += 11,
            'う' | 'て' | 'も' | '"' => po += 12,
            'え' | 'と' | 'や' | '。' => po += 13,
            'お' | 'な' | 'ゆ' | '-' => po += 14,
            'か' | 'に' | 'よ' | ' ' | '　' => po += 15,
            _ => po -= 100,
        }
    }
    match po % 4 {
        0 => st = "HP:+ / MP:+ / 力:- / 早:-",
        1 => st = "HP:+ / MP:- / 力:+ / 早:-",
        2 => st = "HP:- / MP:+ / 力:- / 早:+",
        3 => st = "HP:- / MP:- / 力:+ / 早:+",
        _ => st = "",
    }
    if po < 0 {
        println!("使用できない文字が含まれています。｢あ-ん \"。-(空白)｣を使用できます。")
    } else {
        println!(
            "{} -の初期値ボーナス{} (余り:{}) [{}]",
            st,
            po % 16 / 4,
            po % 16,
            if po % 16 == 13 {
                "オススメ"
            } else {
                "余り13がオススメです"
            }
        )
    }
}
fn lv(ex: i32, lv: i32) {
    // 経験値からLVとステータスと残りのEXを表示
    let mut lv = lv;
    if lv == 0 {
        // EX指定なら次レベルを判定
        match ex {
            0..=6 => lv = 2,
            7..=22 => lv = 3,
            23..=46 => lv = 4,
            47..=109 => lv = 5,
            110..=219 => lv = 6,
            220..=449 => lv = 7,
            450..=799 => lv = 8,
            800..=1299 => lv = 9,
            1300..=1999 => lv = 10,
            2000..=2899 => lv = 11,
            2900..=3999 => lv = 12,
            4000..=5499 => lv = 13,
            5500..=7499 => lv = 14,
            7500..=9999 => lv = 15,
            10000..=12999 => lv = 16,
            13000..=16999 => lv = 17,
            17000..=20999 => lv = 18,
            21000..=24999 => lv = 19,
            25000..=28999 => lv = 20,
            29000..=32999 => lv = 21,
            33000..=36999 => lv = 22,
            37000..=40999 => lv = 23,
            41000..=44999 => lv = 24,
            45000..=48999 => lv = 25,
            49000..=52999 => lv = 26,
            53000..=56999 => lv = 27,
            57000..=60999 => lv = 28,
            61000..=64999 => lv = 29,
            65000..=65534 => lv = 30,
            _ => lv = 30,
        }
    }
    match lv {
        // ステータス表示
        1 => st(1, 4, 4, 15, 0, 0, "", ex),
        2 => st(2, 5, 4, 22, 0, 7, "", ex),
        3 => st(3, 7, 6, 24, 5, 23, "ホイミ", ex),
        4 => st(4, 7, 8, 31, 16, 47, "ギラ", ex),
        5 => st(5, 12, 10, 35, 20, 110, "", ex),
        6 => st(6, 16, 10, 38, 24, 220, "", ex),
        7 => st(7, 18, 17, 40, 26, 450, "ラリホー", ex),
        8 => st(8, 22, 20, 46, 29, 800, "", ex),
        9 => st(9, 30, 22, 50, 36, 1300, "レミーラ", ex),
        10 => st(10, 35, 31, 54, 40, 2000, "マホトーン", ex),
        11 => st(11, 40, 35, 62, 50, 2900, "", ex),
        12 => st(12, 48, 40, 63, 58, 4000, "リレミト", ex),
        13 => st(13, 52, 48, 70, 64, 5500, "ルーラ", ex),
        14 => st(14, 60, 55, 78, 70, 7500, "", ex),
        15 => st(15, 68, 64, 86, 72, 10000, "トヘロス", ex),
        16 => st(16, 72, 70, 92, 95, 13000, "", ex),
        17 => st(17, 72, 78, 100, 100, 17000, "ベホイミ", ex),
        18 => st(18, 85, 84, 115, 108, 21000, "", ex),
        19 => st(19, 87, 86, 130, 115, 25000, "ベギラマ", ex),
        20 => st(20, 92, 88, 138, 128, 29000, "", ex),
        21 => st(21, 95, 90, 149, 135, 33000, "", ex),
        22 => st(22, 97, 90, 158, 146, 37000, "", ex),
        23 => st(23, 99, 94, 165, 153, 41000, "", ex),
        24 => st(24, 103, 98, 170, 161, 45000, "", ex),
        25 => st(25, 113, 100, 174, 161, 49000, "", ex),
        26 => st(26, 117, 105, 180, 168, 53000, "", ex),
        27 => st(27, 125, 107, 189, 175, 57000, "", ex),
        28 => st(28, 130, 115, 195, 180, 61000, "", ex),
        29 => st(29, 135, 120, 200, 190, 65000, "", ex),
        30 => st(30, 140, 130, 210, 200, 65535, "", ex),
        _ => st(30, 140, 130, 210, 200, 65535, "", ex),
    }
    fn st(l: i32, t: i32, s: i32, h: i32, m: i32, e: i32, j: &str, ex: i32) {
        // 表示処理
        println!(
            "[Lv{}] 力:{} 早:{} HP:{} MP:{} EX:{}(あと{}) {}",
            l,
            t,
            s,
            h,
            m,
            e,
            e - ex,
            j
        )
    }
}
fn mon() {
    println!(
        "<モンスター名>| EX|  G| HP| 攻| 守|回|呪|眠|黙|特殊攻撃 *回避(順に)打撃/ギラ/ラリ/マホ
スライム　　　|　1|　1|　3|　5|　3| 1| 0|15| 0|
スライムベス　|　1|　2|　4|　7|　3| 1| 0|15| 0|
ドラキー　　　|　2|　3|　6|　9|　6| 1| 0|15| 0|
ゴースト　　　|　3|　5|　7| 11|　8| 4| 0|15| 0|
まほうつかい　|　4| 12| 13| 11| 12| 1| 0| 0| 0| 1/2ギラ
メイジドラキー|　5| 12| 15| 14| 14| 1| 0| 0| 0| 1/2ギラ
おおさそり　　|　6| 16| 20| 18| 16| 1| 0|15| 0|
がいこつ　　　| 11| 30| 30| 28| 22| 4| 0|15| 0|
メーダ　　　　|　7| 16| 22| 20| 18| 2| 0|15| 0|
メトロゴースト|　8| 16| 25| 18| 20| 6| 0| 0| 0| 3/4ギラ

<モンスター名>| EX|  G| HP| 攻| 守|回|呪|眠|黙|特殊攻撃
ドロル　　　　| 10| 25| 25| 24| 24| 2| 0|14| 0|
ドラキーマ　　| 11| 20| 21| 22| 26| 6| 0| 0| 2| 1/2ギラ 1/4ホイミ
まどうし　　　| 13| 35| 30| 28| 22| 2| 0| 1| 3| 1/2ギラ 1/4ラリホー
てつのさそり　| 14| 40| 23| 36| 42| 2| 0|15| 0|
リカント　　　| 16| 50| 34| 40| 30| 2| 0|15| 1|
しりょう　　　| 17| 60| 36| 44| 34| 4| 0| 0| 7| 1/4ホイミ
リカントマムル| 20| 80| 38| 50| 36| 2| 0| 7| 4|
キメラ　　　　| 24|100| 42| 56| 48| 2| 0|15| 4|
ゴールドマン　|  6|200| 50| 48| 40| 1| 0|15|13|
ヘルゴースト　| 18| 70| 36| 40| 38| 4| 0| 1| 3| 3/4ギラ 1/4ラリホー

<モンスター名>| EX|  G| HP| 攻| 守|回|呪|眠|黙|特殊攻撃
メーダロード　| 20| 85| 35| 47| 40| 4| 0| 0|15| 1/4ギラ 3/4ホイミ
ドロルメイジ　| 20| 90| 38| 52| 50| 1| 0| 2| 2| 1/2マホトーン
しりょうのきし| 28|120| 46| 68| 56| 4| 3| 0| 5| 3/4ホイミ
しのさそり　　| 26|110| 36| 60| 90| 2| 0|15| 7|
よろいのきし　| 33|130| 55| 76| 78| 1| 0| 7| 6| 1/2マホトーン
かげのきし　　| 37|150| 50| 79| 64|15|15|15|15|
メタルスライム|115|  6|  4| 10|255| 1|15|15|15| 3/4ギラ
メイジキメラ　| 34|140| 58| 78| 68| 2| 0| 0| 2| 1/2ラリホー
キラーリカント| 40|155| 60| 86| 70| 7| 0|15| 7|
ドラゴン　　　| 45|160| 65| 88| 74| 2| 2|15| 7| 1/4炎

<モンスター名>| EX|  G| HP| 攻| 守|回|呪|眠|黙|特殊攻撃
スターキメラ　| 43|160| 65| 86| 80| 2| 1| 0| 8| 1/4炎 3/4ベホイミ
だいまどう　　| 50|165| 65| 80| 70| 2|15| 7|15| 1/2ベギラマ
ゴーレム　　　|　5| 10| 70|120| 60| 0|15|15|15|
あくまのきし　| 54|165| 70| 94| 82| 1| 1| 3|15| 1/4ラリホー
キースドラゴン| 60|150| 70| 98| 84| 2| 7|15|15| 1/4炎
ストーンマン　| 65|140|160|100| 40| 1| 7|15| 2|
しにがみのきし| 70|140| 90|105| 86| 2| 1| 7|15| 1/4ベギラマ 3/4ベホイミ
ダースドラゴン|100|140|100|120| 90| 2|15| 7|15| 1/4炎 1/4ラリホー
りゅうおう　　|  0|  0|100| 90| 75| 0|15|15|15| 3/4ベギラマ 1/4マホトーン
りゅうおう(竜)|  0|  0|130|140|200| 0|15|15|15| 1/2炎"
    );
}
fn buki() {
    println!(
        "<武器名>　　　|価格|売値|攻撃
たけざお　　　|　10|　 5|　 2
こんぼう　　　|　60|　30|　 4
どうのつるぎ　| 180|　90|　10
てつのおの　　| 560| 280|　15
はがねのつるぎ|1500| 750|　20
ほのおのつるぎ|9800|4900|　28
ロトのつるぎ　|　 2|　 1|　40(最強)"
    );
}
fn bougu() {
    println!(
        "<鎧名>　　　　|価格 |売値|防御|効果
ぬののふく　　|　 20|　10|　 2|
かわのふく　　|　 70|　35|　 4|
くさりかたびら|　300| 150|　10|
てつのよろい　| 1000| 500|　16|
はがねのよろい| 3000|1500|　24|
まほうのよろい| 7700|3850|　24|呪文ダメ1/3軽減 4歩でHP1回復
ロトのよろい　|　　2|　 1|　28|(最強)呪文と炎ダメ1/3軽減 マホトーンとダメージ床無効 1歩でHP1回復
<盾名>　　　　|価格 |売値|防御|
かわのたて　　|　 90|　45|　 4|
てつのたて　　|　800| 400|　10|
みかがみのたて|14800|7400|　20|(最強)"
    );
}
fn dougu() {
    println!(
        "<道具名>　　　|価格|売値|種類|効果
やくそう　　　|　24|　12|道具|HP23-30回復
たいまつ　　　|　 8|　 4|道具|ダンジョンを出るまで周囲1マス照らす
せいすい　　　|　38|　19|道具|トヘロスの効果
キメラのつばさ|　70|　35|道具|ラダトームの城へ瞬間移動する
かぎ　　　　　|　53|　26|道具|扉を開ける
りゅうのうろこ|　20|　10|装飾|道具で使うと守備+2 売っても効果は切れない
せんしのゆびわ|　30|　15|装飾|身につけられるが何の効果もない
のろいのベルト| 360| 180|装飾|身につけると呪われる
しのくびかざり|2400|1200|装飾|身につけると呪われる
ぎんのたてごと| -- | -- |イベ|使うとモンスターが出現
ようせいのふえ| -- | -- |イベ|ゴーレムを眠らせる
おうじょのあい| -- | -- |イベ|次のレベルまでの経験値とラダトームの城までの距離を教えてくれる
あまぐものつえ| -- | -- |イベ|にじのしずくの入手に必要
たいようのいし| -- | -- |イベ|にじのしずくの入手に必要
ロトのしるし　| -- | -- |イベ|にじのしずくの入手に必要
にじのしずく　| -- | -- |イベ|竜王の城の島へ渡るために必要"
    );
}
fn jumon() {
    println!(
        "<呪文名>　|LV|MP|時|効果
ホイミ　　| 3| 4|常|HP10-17回復
ギラ　　　| 4| 2|戦|ダメ5-12
ラリホー　| 7| 2|戦|眠らせる
レミーラ　| 9| 3|移|3マス照らす(効果低下:80/60/60歩)
マホトーン|10| 2|戦|呪文を封じる
リレミト　|12| 6|移|ダンジョンから脱出
ルーラ　　|13| 8|移|ラダトームの城へ瞬間移動する
トヘロス　|15| 2|移|フィールドで127歩の間､自分の守備力より低い攻撃力の敵が出なくなる
ベホイミ　|17|10|常|HP85-100回復
ベギラマ　|19| 5|戦|ダメ58-65"
    );
}
fn tekikou() {
    println!(
        "<敵の攻撃>|効果
ホイミ　　|HP20-27回復
ベホイミ　|HP85-100回復
ギラ　　　|ダメ3-10
ベギラマ　|ダメ30-45
ラリホー　|100%眠らせる
マホトーン|50%呪文を封じる
炎　　　　|ダメ16-23 / ダメ65-72(竜王)"
    )
}
