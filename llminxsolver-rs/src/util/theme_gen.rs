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
    fn test_scheme_type_default() {
        let default: SchemeType = Default::default();
        assert_eq!(default, SchemeType::TonalSpot);
    }

    #[test]
    fn test_scheme_type_variants() {
        let variants = [
            SchemeType::TonalSpot,
            SchemeType::Content,
            SchemeType::Expressive,
            SchemeType::Fidelity,
            SchemeType::FruitSalad,
            SchemeType::Monochrome,
            SchemeType::Neutral,
            SchemeType::Rainbow,
            SchemeType::Vibrant,
        ];

        for variant in &variants {
            let cloned = *variant;
            assert_eq!(*variant, cloned);
        }
    }

    #[test]
    fn test_scheme_type_clone() {
        let scheme = SchemeType::Vibrant;
        let cloned = scheme;
        assert_eq!(scheme, cloned);
    }

    #[test]
    fn test_scheme_type_eq() {
        assert_eq!(SchemeType::TonalSpot, SchemeType::TonalSpot);
        assert_ne!(SchemeType::TonalSpot, SchemeType::Vibrant);
    }

    #[test]
    fn test_argb_to_hex() {
        let argb = Argb::new(255, 255, 0, 0);
        let hex = argb_to_hex(argb);
        assert_eq!(hex, "#FF0000");

        let argb = Argb::new(255, 0, 255, 0);
        let hex = argb_to_hex(argb);
        assert_eq!(hex, "#00FF00");

        let argb = Argb::new(255, 0, 0, 255);
        let hex = argb_to_hex(argb);
        assert_eq!(hex, "#0000FF");

        let argb = Argb::new(255, 0, 0, 0);
        let hex = argb_to_hex(argb);
        assert_eq!(hex, "#000000");

        let argb = Argb::new(255, 255, 255, 255);
        let hex = argb_to_hex(argb);
        assert_eq!(hex, "#FFFFFF");
    }

    #[test]
    fn test_theme_colors_serialization() {
        let colors = ThemeColors {
            primary: "#FF0000".to_string(),
            on_primary: "#FFFFFF".to_string(),
            primary_container: "#FFCCCC".to_string(),
            on_primary_container: "#000000".to_string(),
            secondary: "#00FF00".to_string(),
            on_secondary: "#000000".to_string(),
            secondary_container: "#CCFFCC".to_string(),
            on_secondary_container: "#000000".to_string(),
            tertiary: "#0000FF".to_string(),
            on_tertiary: "#FFFFFF".to_string(),
            tertiary_container: "#CCCCFF".to_string(),
            on_tertiary_container: "#000000".to_string(),
            error: "#FF0000".to_string(),
            on_error: "#FFFFFF".to_string(),
            error_container: "#FFCCCC".to_string(),
            on_error_container: "#000000".to_string(),
            background: "#FFFFFF".to_string(),
            on_background: "#000000".to_string(),
            surface: "#FFFFFF".to_string(),
            on_surface: "#000000".to_string(),
            surface_variant: "#EEEEEE".to_string(),
            on_surface_variant: "#000000".to_string(),
            outline: "#CCCCCC".to_string(),
            outline_variant: "#DDDDDD".to_string(),
            inverse_surface: "#000000".to_string(),
            inverse_on_surface: "#FFFFFF".to_string(),
            inverse_primary: "#0000FF".to_string(),
            surface_tint: "#FF0000".to_string(),
            surface_dim: "#DDDDDD".to_string(),
            surface_bright: "#FFFFFF".to_string(),
            surface_container_lowest: "#FFFFFF".to_string(),
            surface_container_low: "#F5F5F5".to_string(),
            surface_container: "#EEEEEE".to_string(),
            surface_container_high: "#E5E5E5".to_string(),
            surface_container_highest: "#DDDDDD".to_string(),
        };

        let json = serde_json::to_string(&colors).unwrap();
        assert!(!json.is_empty());

        let deserialized: ThemeColors = serde_json::from_str(&json).unwrap();
        assert_eq!(colors.primary, deserialized.primary);
        assert_eq!(colors.secondary, deserialized.secondary);
        assert_eq!(colors.background, deserialized.background);
    }

    #[test]
    fn test_generate_theme_from_image_nonexistent() {
        let result =
            generate_theme_from_image("/nonexistent/path.jpg", true, SchemeType::TonalSpot);
        assert!(result.is_none());
    }

    #[test]
    fn test_generate_theme_from_image_invalid() {
        let result = generate_theme_from_image("", true, SchemeType::TonalSpot);
        assert!(result.is_none());
    }

    #[test]
    fn test_generate_theme() {
        if let Some(theme) = generate_theme_from_wallpaper(true, SchemeType::TonalSpot) {
            assert!(!theme.primary.is_empty());
            assert!(!theme.on_primary.is_empty());
            assert!(!theme.background.is_empty());
            assert!(!theme.surface.is_empty());

            assert!(theme.primary.starts_with('#'));
            assert!(theme.background.starts_with('#'));
        }
    }

    #[test]
    fn test_scheme_type_serialize() {
        let scheme = SchemeType::TonalSpot;
        let json = serde_json::to_string(&scheme).unwrap();
        let deserialized: SchemeType = serde_json::from_str(&json).unwrap();
        assert_eq!(scheme, deserialized);

        let scheme = SchemeType::Vibrant;
        let json = serde_json::to_string(&scheme).unwrap();
        let deserialized: SchemeType = serde_json::from_str(&json).unwrap();
        assert_eq!(scheme, deserialized);
    }
}
