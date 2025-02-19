use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use turbo_tasks::{
    primitives::{StringVc, StringsVc},
    trace::TraceRawVcs,
};

pub(crate) struct DefaultFallbackFont {
    pub name: String,
    pub az_avg_width: f64,
    pub units_per_em: u32,
}

// From https://github.com/vercel/next.js/blob/a3893bf69c83fb08e88c87bf8a21d987a0448c8e/packages/font/src/utils.ts#L4
pub(crate) static DEFAULT_SANS_SERIF_FONT: Lazy<DefaultFallbackFont> =
    Lazy::new(|| DefaultFallbackFont {
        name: "Arial".to_owned(),
        az_avg_width: 934.5116279069767,
        units_per_em: 2048,
    });

pub(crate) static DEFAULT_SERIF_FONT: Lazy<DefaultFallbackFont> =
    Lazy::new(|| DefaultFallbackFont {
        name: "Times New Roman".to_owned(),
        az_avg_width: 854.3953488372093,
        units_per_em: 2048,
    });

/// An automatically generated fallback font generated by next/font.
#[turbo_tasks::value(shared)]
pub(crate) struct AutomaticFontFallback {
    /// e.g. `__Roboto_Fallback_c123b8`
    pub scoped_font_family: StringVc,
    /// The name of font locally, used in `src: local("{}")`
    pub local_font_family: StringVc,
    pub adjustment: Option<FontAdjustment>,
}

#[derive(Debug)]
#[turbo_tasks::value(shared)]
pub(crate) enum FontFallback {
    /// An automatically generated fallback font generated by next/font. May
    /// include an optional [[FontAdjustment]].
    Automatic(AutomaticFontFallbackVc),
    /// There was an issue preparing the font fallback. Since resolving the
    /// font css cannot fail, proper Errors cannot be returned. Emit an issue,
    /// return this and omit fallback information instead.
    Error,
    /// A list of manually provided font names to use a fallback, as-is.
    Manual(StringsVc),
}

#[turbo_tasks::value(transparent)]
pub(crate) struct FontFallbacks(Vec<FontFallbackVc>);

/// An adjustment to be made to a fallback font to approximate the geometry of
/// the main webfont. Rendered as e.g. `ascent-override: 56.8%;` in the
/// stylesheet
#[derive(Debug, PartialEq, Serialize, Deserialize, TraceRawVcs)]
pub(crate) struct FontAdjustment {
    pub ascent: f64,
    pub descent: f64,
    pub line_gap: f64,
    pub size_adjust: f64,
}

// Necessary since floating points in this struct don't implement Eq, but it's
// required for turbo tasks values.
impl Eq for FontAdjustment {}
