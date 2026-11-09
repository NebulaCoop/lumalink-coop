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
        PlayersChoose => ["Players choose these in the game:", "Les joueurs choisissent ceci dans le jeu :", "Los jugadores eligen esto en el juego:", "Das wählen Spieler im Spiel:", "I giocatori scelgono questo nel gioco:", "Spelers kiezen dit in het spel:", "Os jogadores escolhem isto no jogo:", "プレイヤーがゲーム内で選ぶ項目:", "플레이어가 게임에서 고르는 항목:", "玩家在游戏中选择这些:", "玩家在遊戲中選擇這些:"],
        AlreadyFinished => ["This one's already finished. Open it in Nebula: Settings, Mods, Import.", "Celui-ci est déjà terminé. Ouvrez-le dans Nebula : Paramètres, Mods, Importer.", "Este ya está terminado. Ábrelo en Nebula: Ajustes, Mods, Importar.", "Dieser ist schon fertig. Öffne ihn in Nebula: Einstellungen, Mods, Importieren.", "Questa è già pronta. Aprila in Nebula: Impostazioni, Mod, Importa.", "Deze is al klaar. Open hem in Nebula: Instellingen, Mods, Importeren.", "Este já está pronto. Abra-o no Nebula: Configurações, Mods, Importar.", "これは完成済みです。Nebulaで開いてください: 設定、MOD、インポート。", "이미 완성된 파일입니다. Nebula에서 여세요: 설정, 모드, 가져오기.", "这个已经完成。在 Nebula 中打开:设置、模组、导入。", "這個已經完成。在 Nebula 中開啟:設定、模組、匯入。"],
        WholeGame => ["This is the whole game, not a mod. Nebula sets the game up from it; LumaLink only needs mods.", "C'est le jeu entier, pas un mod. Nebula installe le jeu à partir de ce fichier ; LumaLink n'a besoin que de mods.", "Este es el juego entero, no un mod. Nebula instala el juego con él; LumaLink solo necesita mods.", "Das ist das ganze Spiel, kein Mod. Nebula richtet damit das Spiel ein; LumaLink braucht nur Mods.", "Questo è il gioco intero, non una mod. Nebula installa il gioco da qui; a LumaLink servono solo le mod.", "Dit is het hele spel, geen mod. Nebula installeert het spel hiermee; LumaLink heeft alleen mods nodig.", "Este é o jogo inteiro, não um mod. O Nebula instala o jogo com ele; o LumaLink só precisa de mods.", "これはゲーム本体で、MODではありません。ゲームの準備はNebulaで行います。LumaLinkに必要なのはMODだけです。", "이것은 모드가 아니라 게임 전체입니다. 게임 설치는 Nebula에서 합니다. LumaLink에는 모드만 있으면 됩니다.", "这是整个游戏,不是模组。Nebula 用它来安装游戏;LumaLink 只需要模组。", "這是整個遊戲,不是模組。Nebula 用它來安裝遊戲;LumaLink 只需要模組。"],
        NotAKind => ["LumaLink can't read this kind of file. Mods come as a .zip, .7z or .rar file, or a folder.", "LumaLink ne peut pas lire ce type de fichier. Les mods sont des fichiers .zip, .7z ou .rar, ou un dossier.", "LumaLink no puede leer este tipo de archivo. Los mods vienen como .zip, .7z, .rar o una carpeta.", "LumaLink kann diese Dateiart nicht lesen. Mods sind .zip-, .7z- oder .rar-Dateien oder ein Ordner.", "LumaLink non può leggere questo tipo di file. Le mod sono file .zip, .7z, .rar o una cartella.", "LumaLink kan dit soort bestand niet lezen. Mods zijn .zip-, .7z- of .rar-bestanden, of een map.", "O LumaLink não lê este tipo de arquivo. Mods vêm em .zip, .7z, .rar ou numa pasta.", "この種類のファイルは読めません。MODは.zip、.7z、.rarファイルかフォルダーです。", "이 종류의 파일은 읽을 수 없습니다. 모드는 .zip, .7z, .rar 파일이나 폴더입니다.", "LumaLink 无法读取这种文件。模组是 .zip、.7z、.rar 文件或文件夹。", "LumaLink 無法讀取這種檔案。模組是 .zip、.7z、.rar 檔案或資料夾。"],
        NotTheGame => ["That isn't Nebula's game.pak. Choose the one in Nebula's content folder.", "Ce n'est pas le game.pak de Nebula. Choisissez celui du dossier content de Nebula.", "Ese no es el game.pak de Nebula. Elige el de la carpeta content de Nebula.", "Das ist nicht Nebulas game.pak. Wähle die aus Nebulas content-Ordner.", "Non è il game.pak di Nebula. Scegli quello nella cartella content di Nebula.", "Dat is niet de game.pak van Nebula. Kies die in de content-map van Nebula.", "Esse não é o game.pak do Nebula. Escolha o da pasta content do Nebula.", "それはNebulaのgame.pakではありません。Nebulaのcontentフォルダーのものを選んでください。", "Nebula의 game.pak이 아닙니다. Nebula의 content 폴더에 있는 것을 고르세요.", "这不是 Nebula 的 game.pak。请选择 Nebula 的 content 文件夹中的那个。", "這不是 Nebula 的 game.pak。請選擇 Nebula 的 content 資料夾中的那個。"],
        OpenInNebula => ["Open this file in Nebula: Settings, Mods, Import. It works on every version of the game.", "Ouvrez ce fichier dans Nebula : Paramètres, Mods, Importer. Il fonctionne sur toutes les versions du jeu.", "Abre este archivo en Nebula: Ajustes, Mods, Importar. Funciona en todas las versiones del juego.", "Öffne diese Datei in Nebula: Einstellungen, Mods, Importieren. Sie läuft auf jeder Version des Spiels.", "Apri questo file in Nebula: Impostazioni, Mod, Importa. Funziona su ogni versione del gioco.", "Open dit bestand in Nebula: Instellingen, Mods, Importeren. Het werkt op elke versie van het spel.", "Abra este arquivo no Nebula: Configurações, Mods, Importar. Funciona em todas as versões do jogo.", "このファイルをNebulaで開いてください: 設定、MOD、インポート。ゲームのどのバージョンでも動きます。", "이 파일을 Nebula에서 여세요: 설정, 모드, 가져오기. 게임의 모든 버전에서 작동합니다.", "在 Nebula 中打开这个文件:设置、模组、导入。它适用于游戏的所有版本。", "在 Nebula 中開啟這個檔案:設定、模組、匯入。它適用於遊戲的所有版本。"],
        LeavesFilesOut => ["Leaves files out", "Omet des fichiers", "Omite archivos", "Lässt Dateien aus", "Esclude dei file", "Laat bestanden weg", "Deixa arquivos de fora", "除かれるファイルがあります", "빠지는 파일이 있음", "会略去一些文件", "會略去一些檔案"],
        BuildWithout => ["These files can't be used in the game. Convert again to build the mod without them.", "Ces fichiers ne peuvent pas être utilisés dans le jeu. Convertissez à nouveau pour créer le mod sans eux.", "Estos archivos no se pueden usar en el juego. Convierte de nuevo para crear el mod sin ellos.", "Diese Dateien kann das Spiel nicht nutzen. Wandle erneut um, um den Mod ohne sie zu erstellen.", "Questi file non si possono usare nel gioco. Converti di nuovo per creare la mod senza di essi.", "Deze bestanden kan het spel niet gebruiken. Zet opnieuw om om de mod zonder ze te maken.", "Esses arquivos não podem ser usados no jogo. Converta de novo para criar o mod sem eles.", "これらのファイルはゲームで使えません。もう一度変換すると、これらを除いてMODを作ります。", "이 파일들은 게임에서 쓸 수 없습니다. 다시 변환하면 이 파일들 없이 모드를 만듭니다.", "这些文件无法在游戏中使用。再次转换即可不含这些文件生成模组。", "這些檔案無法在遊戲中使用。再次轉換即可不含這些檔案產生模組。"],
        FilesLeftOut => ["{n} files the game can't use were left out. Open this file in Nebula: Settings, Mods, Import.", "{n} fichiers que le jeu ne peut pas utiliser ont été laissés de côté. Ouvrez ce fichier dans Nebula : Paramètres, Mods, Importer.", "Se omitieron {n} archivos que el juego no puede usar. Abre este archivo en Nebula: Ajustes, Mods, Importar.", "{n} Dateien, die das Spiel nicht nutzen kann, wurden ausgelassen. Öffne diese Datei in Nebula: Einstellungen, Mods, Importieren.", "{n} file che il gioco non può usare sono stati esclusi. Apri questo file in Nebula: Impostazioni, Mod, Importa.", "{n} bestanden die het spel niet kan gebruiken zijn weggelaten. Open dit bestand in Nebula: Instellingen, Mods, Importeren.", "{n} arquivos que o jogo não usa ficaram de fora. Abra este arquivo no Nebula: Configurações, Mods, Importar.", "ゲームで使えない{n}個のファイルは除きました。このファイルをNebulaで開いてください: 設定、MOD、インポート。", "게임에서 쓸 수 없는 파일 {n}개는 뺐습니다. 이 파일을 Nebula에서 여세요: 설정, 모드, 가져오기.", "已略去 {n} 个游戏无法使用的文件。在 Nebula 中打开这个文件:设置、模组、导入。", "已略去 {n} 個遊戲無法使用的檔案。在 Nebula 中開啟這個檔案:設定、模組、匯入。"],
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
        SomethingWrong => ["Something went wrong reading this mod.", "Une erreur s'est produite en lisant ce mod.", "Algo salió mal al leer este mod.", "Beim Lesen dieses Mods ist etwas schiefgelaufen.", "Si è verificato un problema durante la lettura di questa mod.", "Er ging iets mis bij het lezen van deze mod.", "Algo correu mal ao ler este mod.", "このMODの読み込み中に問題が起きました。", "이 모드를 읽는 중 문제가 발생했습니다.", "读取此模组时出错。", "讀取此模組時發生錯誤。"],
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
