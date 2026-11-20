//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "EffectCraft";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://icasys.mx";
pub const WEBSITE: &str = "https://icasys.mx";
pub const APP_PAGE: &str = "https://icasys.mx";
pub const GITHUB: &str = "https://github.com/arodrigosote/effectcraft";
pub const ISSUES: &str = "https://github.com/arodrigosote/effectcraft/issues";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("ICASYS Foto", "photocraft"),
    ("ICASYS Vectores", "vectorcraft"),
    ("ICASYS Video", "filmcraft"),
    ("ICASYS Revelado", "lightcraft"),
    ("ICASYS PDF", "pdfcraft"),
    ("ICASYS Maquetación", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/arodrigosote/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://icasys.mx/?app={slug}")
}
