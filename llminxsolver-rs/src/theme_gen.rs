use material_colors::{
    color::Argb,
    hct::Hct,
    image::{FilterType, ImageReader},
    scheme::{
        Scheme,
        variant::{
            SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
            SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
        },
    },
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SchemeType {
    #[default]
    TonalSpot,
    Content,
    Expressive,
    Fidelity,
    FruitSalad,
    Monochrome,
    Neutral,
    Rainbow,
    Vibrant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,
    pub secondary: String,
    pub on_secondary: String,
    pub secondary_container: String,
    pub on_secondary_container: String,
    pub tertiary: String,
    pub on_tertiary: String,
    pub tertiary_container: String,
    pub on_tertiary_container: String,
    pub error: String,
    pub on_error: String,
    pub error_container: String,
    pub on_error_container: String,
    pub background: String,
    pub on_background: String,
    pub surface: String,
    pub on_surface: String,
    pub surface_variant: String,
    pub on_surface_variant: String,
    pub outline: String,
    pub outline_variant: String,
    pub inverse_surface: String,
    pub inverse_on_surface: String,
    pub inverse_primary: String,
    pub surface_tint: String,
    pub surface_dim: String,
    pub surface_bright: String,
    pub surface_container_lowest: String,
    pub surface_container_low: String,
    pub surface_container: String,
    pub surface_container_high: String,
    pub surface_container_highest: String,
}

fn argb_to_hex(argb: Argb) -> String {
    format!("#{:02X}{:02X}{:02X}", argb.red, argb.green, argb.blue)
}

fn scheme_to_theme_colors(scheme: &Scheme) -> ThemeColors {
    ThemeColors {
        primary: argb_to_hex(scheme.primary),
        on_primary: argb_to_hex(scheme.on_primary),
        primary_container: argb_to_hex(scheme.primary_container),
        on_primary_container: argb_to_hex(scheme.on_primary_container),
        secondary: argb_to_hex(scheme.secondary),
        on_secondary: argb_to_hex(scheme.on_secondary),
        secondary_container: argb_to_hex(scheme.secondary_container),
        on_secondary_container: argb_to_hex(scheme.on_secondary_container),
        tertiary: argb_to_hex(scheme.tertiary),
        on_tertiary: argb_to_hex(scheme.on_tertiary),
        tertiary_container: argb_to_hex(scheme.tertiary_container),
        on_tertiary_container: argb_to_hex(scheme.on_tertiary_container),
        error: argb_to_hex(scheme.error),
        on_error: argb_to_hex(scheme.on_error),
        error_container: argb_to_hex(scheme.error_container),
        on_error_container: argb_to_hex(scheme.on_error_container),
        background: argb_to_hex(scheme.background),
        on_background: argb_to_hex(scheme.on_background),
        surface: argb_to_hex(scheme.surface),
        on_surface: argb_to_hex(scheme.on_surface),
        surface_variant: argb_to_hex(scheme.surface_variant),
        on_surface_variant: argb_to_hex(scheme.on_surface_variant),
        outline: argb_to_hex(scheme.outline),
        outline_variant: argb_to_hex(scheme.outline_variant),
        inverse_surface: argb_to_hex(scheme.inverse_surface),
        inverse_on_surface: argb_to_hex(scheme.inverse_on_surface),
        inverse_primary: argb_to_hex(scheme.inverse_primary),
        surface_tint: argb_to_hex(scheme.primary),
        surface_dim: argb_to_hex(scheme.surface_dim),
        surface_bright: argb_to_hex(scheme.surface_bright),
        surface_container_lowest: argb_to_hex(scheme.surface_container_lowest),
        surface_container_low: argb_to_hex(scheme.surface_container_low),
        surface_container: argb_to_hex(scheme.surface_container),
        surface_container_high: argb_to_hex(scheme.surface_container_high),
        surface_container_highest: argb_to_hex(scheme.surface_container_highest),
    }
}

fn generate_scheme(source: Hct, dark: bool, scheme_type: SchemeType) -> Scheme {
    let dynamic_scheme = match scheme_type {
        SchemeType::TonalSpot => SchemeTonalSpot::new(source, dark, None).scheme,
        SchemeType::Content => SchemeContent::new(source, dark, None).scheme,
        SchemeType::Expressive => SchemeExpressive::new(source, dark, None).scheme,
        SchemeType::Fidelity => SchemeFidelity::new(source, dark, None).scheme,
        SchemeType::FruitSalad => SchemeFruitSalad::new(source, dark, None).scheme,
        SchemeType::Monochrome => SchemeMonochrome::new(source, dark, None).scheme,
        SchemeType::Neutral => SchemeNeutral::new(source, dark, None).scheme,
        SchemeType::Rainbow => SchemeRainbow::new(source, dark, None).scheme,
        SchemeType::Vibrant => SchemeVibrant::new(source, dark, None).scheme,
    };
    dynamic_scheme.into()
}

pub fn generate_theme_from_image(
    image_path: &str,
    dark_theme: bool,
    scheme_type: SchemeType,
) -> Option<ThemeColors> {
    let path = Path::new(image_path);
    if !path.exists() {
        return None;
    }

    let image_data = fs::read(path).ok()?;
    let mut reader = ImageReader::read(image_data).ok()?;
    reader.resize(128, 128, FilterType::Lanczos3);

    let source_color = ImageReader::extract_color(&reader);
    let source_hct = Hct::new(source_color);
    let scheme = generate_scheme(source_hct, dark_theme, scheme_type);

    Some(scheme_to_theme_colors(&scheme))
}

pub fn generate_theme_from_wallpaper(
    dark_theme: bool,
    scheme_type: SchemeType,
) -> Option<ThemeColors> {
    let wallpaper_path = crate::wallpaper::detect_wallpaper_path()?;
    generate_theme_from_image(&wallpaper_path, dark_theme, scheme_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_theme() {
        if let Some(theme) = generate_theme_from_wallpaper(true, SchemeType::TonalSpot) {
            println!("Generated dark theme primary: {}", theme.primary);
        }
    }
}
