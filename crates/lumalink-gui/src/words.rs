//! The window's words, in the languages Nebula's menus use. The system's language picks them; English is what's left over.
//! A mod's own notes stay in English: they are for whoever made it.

use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq)]
pub enum Word {
    SomethingWrong,
    Reading,
    Ready,
    CantConvert,
    Converting,
    Done,
    DidntWork,
    DropMods,
    ClickToChoose,
    Back,
    Remove,
    ShowFile,
    Convert,
    SaveTo,
    ChangeGame,
    Choose,
    ChooseYourGame,
    OneMod,
    Mods,
    PlayersChoose,
    AlreadyFinished,
    WholeGame,
    NotAKind,
    NotTheGame,
    OpenInNebula,
    FilesLeftOut,
    LeavesFilesOut,
    BuildWithout,
    ARiivolutionMod,
    AMod,
    ATexturePack,
    Files,
    Textures,
    GameplayChanges,
    ModsFilter,
    GameFilter,
    Usa,
    Europe,
    Japan,
    Korea,
}

/// English, French, Spanish, German, Italian, Dutch, Portuguese, Japanese, Korean, Simplified and Traditional Chinese.
const LANGUAGES: [&str; 11] = ["en", "fr", "es", "de", "it", "nl", "pt", "ja", "ko", "zh-hans", "zh-hant"];

#[rustfmt::skip]
fn table(word: Word) -> [&'static str; 11] {
    use Word::*;
    match word {
        Reading => ["Reading...", "Lecture...", "Leyendo...", "Wird gelesen...", "Lettura...", "Lezen...", "Lendo...", "読み込み中...", "읽는 중...", "读取中...", "讀取中..."],
        Ready => ["Ready", "Prêt", "Listo", "Bereit", "Pronto", "Klaar", "Pronto", "準備完了", "준비됨", "就绪", "就緒"],
        CantConvert => ["Can't convert", "Conversion impossible", "No se puede convertir", "Nicht umwandelbar", "Non convertibile", "Kan niet omzetten", "Não dá para converter", "変換できません", "변환할 수 없음", "无法转换", "無法轉換"],
        Converting => ["Converting...", "Conversion...", "Convirtiendo...", "Wird umgewandelt...", "Conversione...", "Omzetten...", "Convertendo...", "変換中...", "변환 중...", "转换中...", "轉換中..."],
        Done => ["Done", "Terminé", "Hecho", "Fertig", "Fatto", "Klaar", "Pronto", "完了", "완료", "完成", "完成"],
        DidntWork => ["Didn't work", "Échec", "No funcionó", "Fehlgeschlagen", "Non riuscito", "Mislukt", "Não funcionou", "失敗しました", "실패함", "失败", "失敗"],
        DropMods => ["Drop mods here", "Déposez des mods ici", "Suelta mods aquí", "Mods hier ablegen", "Trascina qui le mod", "Sleep mods hierheen", "Solte mods aqui", "ここにMODをドロップ", "여기에 모드를 놓으세요", "将模组拖到这里", "將模組拖到這裡"],
        ClickToChoose => ["or click to choose", "ou cliquez pour choisir", "o haz clic para elegir", "oder zum Auswählen klicken", "o fai clic per scegliere", "of klik om te kiezen", "ou clique para escolher", "またはクリックして選択", "또는 클릭해서 선택", "或点击选择", "或點擊選擇"],
        Back => ["Back", "Retour", "Atrás", "Zurück", "Indietro", "Terug", "Voltar", "戻る", "뒤로", "返回", "返回"],
        Remove => ["Remove", "Retirer", "Quitar", "Entfernen", "Rimuovi", "Verwijderen", "Remover", "取り除く", "제거", "移除", "移除"],
        ShowFile => ["Show file", "Voir le fichier", "Ver archivo", "Datei zeigen", "Mostra file", "Bestand tonen", "Mostrar arquivo", "ファイルを表示", "파일 보기", "显示文件", "顯示檔案"],
        Convert => ["Convert", "Convertir", "Convertir", "Umwandeln", "Converti", "Omzetten", "Converter", "変換", "변환", "转换", "轉換"],
        SaveTo => ["Save to", "Enregistrer", "Guardar en", "Speichern", "Salva in", "Opslaan", "Salvar em", "保存先", "저장 위치", "保存到", "儲存到"],
        ChangeGame => ["Change game", "Changer de jeu", "Cambiar juego", "Spiel ändern", "Cambia gioco", "Spel wijzigen", "Trocar jogo", "ゲームを変更", "게임 변경", "更改游戏", "變更遊戲"],
        Choose => ["Choose", "Choisir", "Elegir", "Auswählen", "Scegli", "Kiezen", "Escolher", "選択", "선택", "选择", "選擇"],
        ChooseYourGame => ["Choose your game", "Choisissez votre jeu", "Elige tu juego", "Wähle dein Spiel", "Scegli il tuo gioco", "Kies je spel", "Escolha seu jogo", "ゲームを選択", "게임을 선택하세요", "选择你的游戏", "選擇你的遊戲"],
        OneMod => ["1 mod", "1 mod", "1 mod", "1 Mod", "1 mod", "1 mod", "1 mod", "1個のMOD", "모드 1개", "1 个模组", "1 個模組"],
        Mods => ["{n} mods", "{n} mods", "{n} mods", "{n} Mods", "{n} mod", "{n} mods", "{n} mods", "{n}個のMOD", "모드 {n}개", "{n} 个模组", "{n} 個模組"],
        PlayersChoose => ["Options in the game:", "Options dans le jeu :", "Opciones en el juego:", "Optionen im Spiel:", "Opzioni nel gioco:", "Opties in het spel:", "Opções no jogo:", "ゲーム内のオプション:", "게임 내 옵션:", "游戏内选项:", "遊戲內選項:"],
        AlreadyFinished => ["Already converted. Import it in Nebula: Settings > Mods > Import file.", "Déjà converti. Importez-le dans Nebula : Réglages > Mods > Importer un fichier.", "Ya convertido. Impórtalo en Nebula: Ajustes > Mods > Importar archivo.", "Schon umgewandelt. In Nebula importieren: Einstellungen > Mods > Datei importieren.", "Già convertita. Importala in Nebula: Impostazioni > Mod > Importa file.", "Al omgezet. Importeer in Nebula: Instellingen > Mods > Bestand importeren.", "Já convertido. Importe no Nebula: Configurações > Mods > Importar arquivo.", "変換済みです。Nebulaで取り込む: 設定 > MOD > ファイルを取り込む", "이미 변환됨. Nebula에서 가져오기: 설정 > 모드 > 파일 가져오기", "已转换。在 Nebula 中导入:设置 > 模组 > 导入文件", "已轉換。在 Nebula 中匯入:設定 > 模組 > 匯入檔案"],
        WholeGame => ["This is the game itself, not a mod.", "C'est le jeu lui-même, pas un mod.", "Es el juego, no un mod.", "Das ist das Spiel selbst, kein Mod.", "È il gioco, non una mod.", "Dit is het spel zelf, geen mod.", "Este é o jogo, não um mod.", "これはゲーム本体で、MODではありません。", "모드가 아니라 게임 자체입니다.", "这是游戏本体,不是模组。", "這是遊戲本體,不是模組。"],
        NotAKind => ["Not a mod. Mods are .zip, .7z, .rar or a folder.", "Pas un mod. Les mods sont en .zip, .7z, .rar ou un dossier.", "No es un mod. Los mods son .zip, .7z, .rar o una carpeta.", "Kein Mod. Mods sind .zip, .7z, .rar oder ein Ordner.", "Non è una mod. Le mod sono .zip, .7z, .rar o una cartella.", "Geen mod. Mods zijn .zip, .7z, .rar of een map.", "Não é um mod. Mods são .zip, .7z, .rar ou uma pasta.", "MODではありません。MODは.zip、.7z、.rar、またはフォルダーです。", "모드가 아닙니다. 모드는 .zip, .7z, .rar 또는 폴더입니다.", "不是模组。模组是 .zip、.7z、.rar 或文件夹。", "不是模組。模組是 .zip、.7z、.rar 或資料夾。"],
        NotTheGame => ["Not Nebula's game.pak. Pick the one in Nebula's content folder.", "Pas le game.pak de Nebula. Prenez celui du dossier content de Nebula.", "No es el game.pak de Nebula. Elige el de la carpeta content de Nebula.", "Nicht Nebulas game.pak. Nimm die aus Nebulas content-Ordner.", "Non è il game.pak di Nebula. Scegli quello nella cartella content di Nebula.", "Niet de game.pak van Nebula. Kies die in de content-map van Nebula.", "Não é o game.pak do Nebula. Escolha o da pasta content do Nebula.", "Nebulaのgame.pakではありません。Nebulaのcontentフォルダーのものを選んでください。", "Nebula의 game.pak이 아닙니다. Nebula의 content 폴더에 있는 것을 고르세요.", "不是 Nebula 的 game.pak。请选择 Nebula content 文件夹中的那个。", "不是 Nebula 的 game.pak。請選擇 Nebula content 資料夾中的那個。"],
        OpenInNebula => ["Import it in Nebula: Settings > Mods > Import file.", "Importez-le dans Nebula : Réglages > Mods > Importer un fichier.", "Impórtalo en Nebula: Ajustes > Mods > Importar archivo.", "In Nebula importieren: Einstellungen > Mods > Datei importieren.", "Importala in Nebula: Impostazioni > Mod > Importa file.", "Importeer in Nebula: Instellingen > Mods > Bestand importeren.", "Importe no Nebula: Configurações > Mods > Importar arquivo.", "Nebulaで取り込む: 設定 > MOD > ファイルを取り込む", "Nebula에서 가져오기: 설정 > 모드 > 파일 가져오기", "在 Nebula 中导入:设置 > 模组 > 导入文件", "在 Nebula 中匯入:設定 > 模組 > 匯入檔案"],
        LeavesFilesOut => ["Leaves files out", "Omet des fichiers", "Omite archivos", "Lässt Dateien aus", "Esclude dei file", "Laat bestanden weg", "Deixa arquivos de fora", "除かれるファイルがあります", "빠지는 파일이 있음", "会略去一些文件", "會略去一些檔案"],
        BuildWithout => ["The game can't use these files. Convert again to leave them out.", "Le jeu ne peut pas utiliser ces fichiers. Convertissez à nouveau pour les omettre.", "El juego no puede usar estos archivos. Convierte de nuevo para omitirlos.", "Das Spiel kann diese Dateien nicht nutzen. Erneut umwandeln, um sie wegzulassen.", "Il gioco non può usare questi file. Converti di nuovo per escluderli.", "Het spel kan deze bestanden niet gebruiken. Zet opnieuw om om ze weg te laten.", "O jogo não usa estes arquivos. Converta de novo para deixá-los de fora.", "これらのファイルはゲームで使えません。もう一度変換すると除外します。", "게임에서 쓸 수 없는 파일입니다. 다시 변환하면 빼고 만듭니다.", "游戏无法使用这些文件。再次转换即可略去。", "遊戲無法使用這些檔案。再次轉換即可略去。"],
        FilesLeftOut => ["{n} unusable files left out. Import it in Nebula: Settings > Mods > Import file.", "{n} fichiers inutilisables omis. Importez-le dans Nebula : Réglages > Mods > Importer un fichier.", "{n} archivos inservibles omitidos. Impórtalo en Nebula: Ajustes > Mods > Importar archivo.", "{n} unbrauchbare Dateien weggelassen. In Nebula importieren: Einstellungen > Mods > Datei importieren.", "{n} file inutilizzabili esclusi. Importala in Nebula: Impostazioni > Mod > Importa file.", "{n} onbruikbare bestanden weggelaten. Importeer in Nebula: Instellingen > Mods > Bestand importeren.", "{n} arquivos inúteis ficaram de fora. Importe no Nebula: Configurações > Mods > Importar arquivo.", "使えないファイル{n}個を除外しました。Nebulaで取り込む: 設定 > MOD > ファイルを取り込む", "쓸 수 없는 파일 {n}개를 뺐습니다. Nebula에서 가져오기: 설정 > 모드 > 파일 가져오기", "已略去 {n} 个无法使用的文件。在 Nebula 中导入:设置 > 模组 > 导入文件", "已略去 {n} 個無法使用的檔案。在 Nebula 中匯入:設定 > 模組 > 匯入檔案"],
        ARiivolutionMod => ["A Riivolution mod", "Un mod Riivolution", "Un mod de Riivolution", "Ein Riivolution-Mod", "Una mod Riivolution", "Een Riivolution-mod", "Um mod Riivolution", "RiivolutionのMOD", "Riivolution 모드", "Riivolution 模组", "Riivolution 模組"],
        AMod => ["A mod", "Un mod", "Un mod", "Ein Mod", "Una mod", "Een mod", "Um mod", "MOD", "모드", "模组", "模組"],
        ATexturePack => ["A texture pack", "Un pack de textures", "Un paquete de texturas", "Ein Texturpaket", "Un pacchetto di texture", "Een texturepakket", "Um pacote de texturas", "テクスチャパック", "텍스처 팩", "纹理包", "材質包"],
        Files => ["{n} files", "{n} fichiers", "{n} archivos", "{n} Dateien", "{n} file", "{n} bestanden", "{n} arquivos", "{n}個のファイル", "파일 {n}개", "{n} 个文件", "{n} 個檔案"],
        Textures => ["{n} textures", "{n} textures", "{n} texturas", "{n} Texturen", "{n} texture", "{n} textures", "{n} texturas", "{n}個のテクスチャ", "텍스처 {n}개", "{n} 个纹理", "{n} 個材質"],
        GameplayChanges => ["{n} gameplay changes", "{n} changements de jeu", "{n} cambios de juego", "{n} Spieländerungen", "{n} modifiche al gioco", "{n} spelwijzigingen", "{n} mudanças de jogo", "{n}個のゲーム変更", "게임 변경 {n}개", "{n} 处玩法改动", "{n} 處玩法變更"],
        ModsFilter => ["Mods", "Mods", "Mods", "Mods", "Mod", "Mods", "Mods", "MOD", "모드", "模组", "模組"],
        GameFilter => ["Nebula's game", "Le jeu de Nebula", "El juego de Nebula", "Nebulas Spiel", "Il gioco di Nebula", "Het spel van Nebula", "O jogo do Nebula", "Nebulaのゲーム", "Nebula의 게임", "Nebula 的游戏", "Nebula 的遊戲"],
        Usa => ["USA", "États-Unis", "EE. UU.", "USA", "USA", "VS", "EUA", "アメリカ", "미국", "美国", "美國"],
        Europe => ["Europe", "Europe", "Europa", "Europa", "Europa", "Europa", "Europa", "ヨーロッパ", "유럽", "欧洲", "歐洲"],
        Japan => ["Japan", "Japon", "Japón", "Japan", "Giappone", "Japan", "Japão", "日本", "일본", "日本", "日本"],
        SomethingWrong => ["Couldn't read this mod.", "Impossible de lire ce mod.", "No se pudo leer este mod.", "Mod konnte nicht gelesen werden.", "Impossibile leggere questa mod.", "Kan deze mod niet lezen.", "Não foi possível ler este mod.", "このMODを読み込めませんでした。", "이 모드를 읽을 수 없습니다.", "无法读取此模组。", "無法讀取此模組。"],
        Korea => ["Korea", "Corée", "Corea", "Korea", "Corea", "Korea", "Coreia", "韓国", "한국", "韩国", "韓國"],
    }
}

/// Which column: `LUMALINK_LANGUAGE`, or the system's language.
fn column() -> usize {
    static COLUMN: OnceLock<usize> = OnceLock::new();
    *COLUMN.get_or_init(|| {
        let wanted = std::env::var("LUMALINK_LANGUAGE").ok().or_else(sys_locale::get_locale).unwrap_or_default().to_ascii_lowercase().replace('_', "-");
        pick(&wanted)
    })
}

fn pick(code: &str) -> usize {
    if code.starts_with("zh") {
        let traditional = code.contains("hant") || code.ends_with("-tw") || code.ends_with("-hk") || code.ends_with("-mo");
        return if traditional { 10 } else { 9 };
    }
    let base = code.split('-').next().unwrap_or("");
    LANGUAGES.iter().position(|l| *l == base).unwrap_or(0)
}

pub fn word(word: Word) -> &'static str {
    table(word)[column()]
}

/// A word with its number in it.
pub fn counted(word: Word, n: usize) -> String {
    self::word(word).replace("{n}", &n.to_string())
}

/// Whether the language's text needs the system's font throughout: the rounded font holds Japanese and only some of these.
pub fn system_font_first() -> bool {
    column() >= 8
}

/// The system's fonts for languages the rounded font doesn't hold, this language's own first, each with its face in a
/// collection. A line reads from one font: two fonts' characters don't share a baseline.
pub fn system_fonts(bold: bool) -> Vec<(std::path::PathBuf, u32)> {
    // Japanese, Korean, Simplified Chinese, Traditional Chinese: regular, bold, face.
    const WINDOWS: [(&str, &str, u32); 4] = [
        ("C:/Windows/Fonts/YuGothM.ttc", "C:/Windows/Fonts/YuGothB.ttc", 0),
        ("C:/Windows/Fonts/malgun.ttf", "C:/Windows/Fonts/malgunbd.ttf", 0),
        ("C:/Windows/Fonts/msyh.ttc", "C:/Windows/Fonts/msyhbd.ttc", 0),
        ("C:/Windows/Fonts/msjh.ttc", "C:/Windows/Fonts/msjhbd.ttc", 0),
    ];
    const MAC: [(&str, &str, u32); 4] = [
        ("/System/Library/Fonts/ヒラギノ角ゴシック W4.ttc", "/System/Library/Fonts/ヒラギノ角ゴシック W7.ttc", 0),
        ("/System/Library/Fonts/AppleSDGothicNeo.ttc", "/System/Library/Fonts/AppleSDGothicNeo.ttc", 0),
        ("/System/Library/Fonts/STHeiti Medium.ttc", "/System/Library/Fonts/STHeiti Medium.ttc", 1),
        ("/System/Library/Fonts/STHeiti Medium.ttc", "/System/Library/Fonts/STHeiti Medium.ttc", 0),
    ];
    const NOTO: [&str; 3] = ["/usr/share/fonts/opentype/noto", "/usr/share/fonts/noto-cjk", "/usr/share/fonts/google-noto-cjk"];
    let own = match column() {
        8 => 1,
        9 => 2,
        10 => 3,
        _ => 0,
    };
    let order = [own, (own + 1) % 4, (own + 2) % 4, (own + 3) % 4];
    let mut out: Vec<(std::path::PathBuf, u32)> = Vec::new();
    for which in order {
        if cfg!(windows) || cfg!(target_os = "macos") {
            let (regular, heavy, face) = if cfg!(windows) { WINDOWS[which] } else { MAC[which] };
            out.push((std::path::PathBuf::from(if bold { heavy } else { regular }), face));
        } else {
            // One collection holds all four: Japanese, Korean, Simplified, Traditional, in that order.
            for folder in NOTO {
                let file = if bold { "NotoSansCJK-Bold.ttc" } else { "NotoSansCJK-Regular.ttc" };
                out.push((std::path::Path::new(folder).join(file), which as u32));
            }
        }
    }
    out.retain(|(path, _)| path.is_file());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_language_picks_its_column() {
        assert_eq!(pick("en-US"), 0);
        assert_eq!(pick("fr-ca"), 1);
        assert_eq!(pick("ja"), 7);
        assert_eq!(pick("zh-cn"), 9);
        assert_eq!(pick("zh-hans-cn"), 9);
        assert_eq!(pick("zh-tw"), 10);
        assert_eq!(pick("zh-hant"), 10);
        assert_eq!(pick("sv-se"), 0);
        assert_eq!(pick(""), 0);
    }

    #[test]
    fn every_word_with_a_number_has_it_in_every_language() {
        for word in [Word::Mods, Word::FilesLeftOut, Word::Files, Word::Textures, Word::GameplayChanges] {
            assert!(table(word).iter().all(|w| w.contains("{n}")));
        }
    }
}
