use crate::dedicated_solver::MegaminxState;
use llminxsolver_rs::MemoryConfig;

#[derive(Debug, Clone)]
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

impl From<llminxsolver_rs::ThemeColors> for ThemeColors {
    fn from(colors: llminxsolver_rs::ThemeColors) -> Self {
        Self {
            primary: colors.primary,
            on_primary: colors.on_primary,
            primary_container: colors.primary_container,
            on_primary_container: colors.on_primary_container,
            secondary: colors.secondary,
            on_secondary: colors.on_secondary,
            secondary_container: colors.secondary_container,
            on_secondary_container: colors.on_secondary_container,
            tertiary: colors.tertiary,
            on_tertiary: colors.on_tertiary,
            tertiary_container: colors.tertiary_container,
            on_tertiary_container: colors.on_tertiary_container,
            error: colors.error,
            on_error: colors.on_error,
            error_container: colors.error_container,
            on_error_container: colors.on_error_container,
            background: colors.background,
            on_background: colors.on_background,
            surface: colors.surface,
            on_surface: colors.on_surface,
            surface_variant: colors.surface_variant,
            on_surface_variant: colors.on_surface_variant,
            outline: colors.outline,
            outline_variant: colors.outline_variant,
            inverse_surface: colors.inverse_surface,
            inverse_on_surface: colors.inverse_on_surface,
            inverse_primary: colors.inverse_primary,
            surface_tint: colors.surface_tint,
            surface_dim: colors.surface_dim,
            surface_bright: colors.surface_bright,
            surface_container_lowest: colors.surface_container_lowest,
            surface_container_low: colors.surface_container_low,
            surface_container: colors.surface_container,
            surface_container_high: colors.surface_container_high,
            surface_container_highest: colors.surface_container_highest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

impl From<SchemeType> for llminxsolver_rs::SchemeType {
    fn from(scheme_type: SchemeType) -> Self {
        match scheme_type {
            SchemeType::TonalSpot => llminxsolver_rs::SchemeType::TonalSpot,
            SchemeType::Content => llminxsolver_rs::SchemeType::Content,
            SchemeType::Expressive => llminxsolver_rs::SchemeType::Expressive,
            SchemeType::Fidelity => llminxsolver_rs::SchemeType::Fidelity,
            SchemeType::FruitSalad => llminxsolver_rs::SchemeType::FruitSalad,
            SchemeType::Monochrome => llminxsolver_rs::SchemeType::Monochrome,
            SchemeType::Neutral => llminxsolver_rs::SchemeType::Neutral,
            SchemeType::Rainbow => llminxsolver_rs::SchemeType::Rainbow,
            SchemeType::Vibrant => llminxsolver_rs::SchemeType::Vibrant,
        }
    }
}

pub fn generate_theme_from_image(
    image_path: String,
    dark_theme: bool,
    scheme_type: SchemeType,
) -> Option<ThemeColors> {
    llminxsolver_rs::generate_theme_from_image(&image_path, dark_theme, scheme_type.into())
        .map(Into::into)
}

pub fn generate_theme_from_wallpaper(
    dark_theme: bool,
    scheme_type: SchemeType,
) -> Option<ThemeColors> {
    llminxsolver_rs::generate_theme_from_wallpaper(dark_theme, scheme_type.into()).map(Into::into)
}

pub fn detect_wallpaper_path() -> Option<String> {
    llminxsolver_rs::detect_wallpaper_path()
}

#[derive(Debug, Clone)]
pub struct ScoredSolutionExport {
    pub mcc: f64,
    pub move_count: u32,
    pub algorithm: String,
}

impl From<ScoredSolutionExport> for llminxsolver_rs::ScoredSolutionExport {
    fn from(export: ScoredSolutionExport) -> Self {
        Self {
            mcc: export.mcc,
            move_count: export.move_count,
            algorithm: export.algorithm,
        }
    }
}

pub fn export_scored_xlsx(
    output_path: String,
    solutions: Vec<ScoredSolutionExport>,
    image_png_bytes: Option<Vec<u8>>,
    image_size: u32,
) -> Option<String> {
    let rs_solutions: Vec<llminxsolver_rs::ScoredSolutionExport> =
        solutions.into_iter().map(Into::into).collect();

    llminxsolver_rs::export_scored_xlsx(
        &output_path,
        &rs_solutions,
        image_png_bytes.as_deref(),
        image_size,
    )
    .err()
}

pub fn export_raw_xlsx(
    output_path: String,
    algorithms: Vec<String>,
    image_png_bytes: Option<Vec<u8>>,
    image_size: u32,
) -> Option<String> {
    llminxsolver_rs::export_raw_xlsx(
        &output_path,
        &algorithms,
        image_png_bytes.as_deref(),
        image_size,
    )
    .err()
}

pub fn export_raw_xlsx_from_file(
    output_path: String,
    solutions_file_path: String,
    image_png_bytes: Option<Vec<u8>>,
    image_size: u32,
) -> Option<String> {
    llminxsolver_rs::export_raw_xlsx_from_file(
        &output_path,
        &solutions_file_path,
        image_png_bytes.as_deref(),
        image_size,
    )
    .err()
}

pub struct TempFile {
    inner: std::sync::Mutex<llminxsolver_rs::TempFile>,
}

impl Default for TempFile {
    fn default() -> Self {
        Self::new()
    }
}

impl TempFile {
    pub fn new() -> Self {
        let file =
            llminxsolver_rs::TempFile::new().expect("Failed to create raw solutions temp file");
        Self {
            inner: std::sync::Mutex::new(file),
        }
    }

    pub fn append(&self, solution: String) -> Option<String> {
        let mut file = match self.inner.lock() {
            Ok(f) => f,
            Err(e) => return Some(e.to_string()),
        };
        file.append(&solution).err()
    }

    pub fn get_path(&self) -> String {
        let file = self.inner.lock().unwrap();
        file.get_path().to_string_lossy().to_string()
    }

    pub fn count(&self) -> u64 {
        let file = self.inner.lock().unwrap();
        file.count() as u64
    }

    pub fn flush_file(&self) {
        let file = self.inner.lock().unwrap();
        file.flush_file();
    }

    pub fn delete_file(&self) {
        let mut file = self.inner.lock().unwrap();
        file.delete_file();
    }

    pub fn read_page(&self, offset: u64, limit: u64) -> Vec<String> {
        self.flush_file();
        let file = self.inner.lock().unwrap();
        file.read_page(offset as usize, limit as usize)
            .unwrap_or_default()
    }
}

pub fn cleanup_stale_temp_files() {
    llminxsolver_rs::cleanup_stale_temp_files();
}

pub struct BatchTempFile {
    inner: std::sync::Mutex<llminxsolver_rs::BatchTempFile>,
}

impl Default for BatchTempFile {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchTempFile {
    pub fn new() -> Self {
        let file = llminxsolver_rs::BatchTempFile::new()
            .expect("Failed to create batch temp file directory");
        Self {
            inner: std::sync::Mutex::new(file),
        }
    }

    pub fn append(&self, case_number: u32, solution: String) -> Option<String> {
        let mut file = match self.inner.lock() {
            Ok(f) => f,
            Err(e) => return Some(e.to_string()),
        };
        file.append(case_number as usize, &solution).err()
    }

    pub fn get_path(&self) -> String {
        let file = self.inner.lock().unwrap();
        file.get_path().to_string_lossy().to_string()
    }

    pub fn count(&self) -> u64 {
        let file = self.inner.lock().unwrap();
        file.count() as u64
    }

    pub fn case_count(&self, case_number: u32) -> u64 {
        let file = self.inner.lock().unwrap();
        file.case_count(case_number as usize) as u64
    }

    pub fn flush(&self) {
        let file = self.inner.lock().unwrap();
        file.flush();
    }

    pub fn delete_file(&self) {
        let mut file = self.inner.lock().unwrap();
        file.delete();
    }

    pub fn read_case_page(&self, case_number: u32, offset: u64, limit: u64) -> Vec<String> {
        let file = self.inner.lock().unwrap();
        file.read_case_page(case_number as usize, offset as usize, limit as usize)
            .unwrap_or_default()
    }
}

pub fn cleanup_stale_batch_temp_files() {
    llminxsolver_rs::cleanup_stale_batch_temp_files();
}

pub fn get_move_count(algorithm: String, metric: String) -> u32 {
    llminxsolver_rs::get_move_count(&algorithm, &metric)
}

pub fn calculate_mcc(sequence: String) -> f64 {
    llminxsolver_rs::calculate_mcc(&sequence)
}

pub fn set_data_directory(path: String) {
    llminxsolver_rs::set_data_directory(&path);
}

pub fn get_available_cpus() -> u32 {
    MemoryConfig::available_cpus() as u32
}

pub fn get_available_memory_mb() -> u32 {
    MemoryConfig::available_memory_mb() as u32
}

pub fn get_default_pruning_depth() -> u8 {
    llminxsolver_rs::DEFAULT_PRUNING_DEPTH
}

pub fn get_min_pruning_depth() -> u8 {
    llminxsolver_rs::MIN_PRUNING_DEPTH
}

pub fn get_max_pruning_depth() -> u8 {
    llminxsolver_rs::MAX_PRUNING_DEPTH
}

pub fn validate_megaminx_state(state: MegaminxState) -> Option<String> {
    let rs_state = llminxsolver_rs::MegaminxState {
        corner_positions: state.corner_positions,
        corner_orientations: state.corner_orientations,
        edge_positions: state.edge_positions,
        edge_orientations: state.edge_orientations,
    };

    match llminxsolver_rs::validate_last_layer_state(&rs_state) {
        Ok(()) => None,
        Err(e) => Some(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    fn lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn test_scheme_type_conversion_all_variants() {
        let _guard = lock().lock().unwrap();
        let values = [
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

        for value in values {
            let rs: llminxsolver_rs::SchemeType = value.into();
            assert!(matches!(
                rs,
                llminxsolver_rs::SchemeType::TonalSpot
                    | llminxsolver_rs::SchemeType::Content
                    | llminxsolver_rs::SchemeType::Expressive
                    | llminxsolver_rs::SchemeType::Fidelity
                    | llminxsolver_rs::SchemeType::FruitSalad
                    | llminxsolver_rs::SchemeType::Monochrome
                    | llminxsolver_rs::SchemeType::Neutral
                    | llminxsolver_rs::SchemeType::Rainbow
                    | llminxsolver_rs::SchemeType::Vibrant
            ));
        }
    }

    #[test]
    fn test_temp_file_wrapper_basic_flow() {
        let _guard = lock().lock().unwrap();
        let file = TempFile::new();
        assert!(file.append("R U R' U'".to_string()).is_none());
        assert_eq!(file.count(), 1);

        let page = file.read_page(0, 10);
        assert_eq!(page, vec!["R U R' U'".to_string()]);

        let path = file.get_path();
        assert!(path.contains("llminx_solutions"));
        file.delete_file();
    }

    #[test]
    fn test_batch_temp_file_wrapper_basic_flow() {
        let _guard = lock().lock().unwrap();
        let file = BatchTempFile::new();
        assert!(file.append(2, "alg1".to_string()).is_none());
        assert!(file.append(2, "alg2".to_string()).is_none());
        assert!(file.append(3, "alg3".to_string()).is_none());

        assert_eq!(file.count(), 3);
        assert_eq!(file.case_count(2), 2);
        assert_eq!(file.case_count(3), 1);

        let page = file.read_case_page(2, 0, 10);
        assert_eq!(page, vec!["alg1".to_string(), "alg2".to_string()]);

        let path = file.get_path();
        assert!(path.contains("llminx_batch_solutions"));
        file.delete_file();
    }

    #[test]
    fn test_export_wrappers_return_error_for_invalid_paths() {
        let _guard = lock().lock().unwrap();
        let bad = "/definitely/not/writable/out.xlsx".to_string();
        let err = export_raw_xlsx(bad.clone(), vec!["R U".to_string()], None, 64);
        assert!(err.is_some());

        let err2 = export_scored_xlsx(
            bad.clone(),
            vec![ScoredSolutionExport {
                mcc: 1.0,
                move_count: 4,
                algorithm: "R U R' U'".to_string(),
            }],
            None,
            64,
        );
        assert!(err2.is_some());

        let err3 = export_raw_xlsx_from_file(bad, "/no/such/source.txt".to_string(), None, 64);
        assert!(err3.is_some());
    }

    #[test]
    fn test_move_count_mcc_and_pruning_limits() {
        let _guard = lock().lock().unwrap();
        assert!(get_move_count("R U R' U'".to_string(), "ftm".to_string()) > 0);
        assert!(calculate_mcc("R U".to_string()) > 0.0);
        assert!(get_available_cpus() >= 1);
        assert!(get_available_memory_mb() > 0);
        assert!(get_default_pruning_depth() >= get_min_pruning_depth());
        assert!(get_max_pruning_depth() >= get_default_pruning_depth());
    }

    #[test]
    fn test_theme_colors_conversion_and_theme_wrappers() {
        let _guard = lock().lock().unwrap();
        let rs_colors = llminxsolver_rs::ThemeColors {
            primary: "#000001".to_string(),
            on_primary: "#000002".to_string(),
            primary_container: "#000003".to_string(),
            on_primary_container: "#000004".to_string(),
            secondary: "#000005".to_string(),
            on_secondary: "#000006".to_string(),
            secondary_container: "#000007".to_string(),
            on_secondary_container: "#000008".to_string(),
            tertiary: "#000009".to_string(),
            on_tertiary: "#00000A".to_string(),
            tertiary_container: "#00000B".to_string(),
            on_tertiary_container: "#00000C".to_string(),
            error: "#00000D".to_string(),
            on_error: "#00000E".to_string(),
            error_container: "#00000F".to_string(),
            on_error_container: "#000010".to_string(),
            background: "#000011".to_string(),
            on_background: "#000012".to_string(),
            surface: "#000013".to_string(),
            on_surface: "#000014".to_string(),
            surface_variant: "#000015".to_string(),
            on_surface_variant: "#000016".to_string(),
            outline: "#000017".to_string(),
            outline_variant: "#000018".to_string(),
            inverse_surface: "#000019".to_string(),
            inverse_on_surface: "#00001A".to_string(),
            inverse_primary: "#00001B".to_string(),
            surface_tint: "#00001C".to_string(),
            surface_dim: "#00001D".to_string(),
            surface_bright: "#00001E".to_string(),
            surface_container_lowest: "#00001F".to_string(),
            surface_container_low: "#000020".to_string(),
            surface_container: "#000021".to_string(),
            surface_container_high: "#000022".to_string(),
            surface_container_highest: "#000023".to_string(),
        };
        let mapped: ThemeColors = rs_colors.into();
        assert_eq!(mapped.primary, "#000001");
        assert_eq!(mapped.surface_container_highest, "#000023");

        let none_theme = generate_theme_from_image(
            "/definitely/missing/image.png".to_string(),
            true,
            SchemeType::TonalSpot,
        );
        assert!(none_theme.is_none());
        let _ = generate_theme_from_wallpaper(false, SchemeType::Neutral);
        let _ = detect_wallpaper_path();
    }

    #[test]
    fn test_cleanup_temp_wrappers() {
        let _guard = lock().lock().unwrap();
        cleanup_stale_temp_files();
        cleanup_stale_batch_temp_files();
    }

    #[test]
    fn test_set_data_dir_and_validate_state_wrapper() {
        let _guard = lock().lock().unwrap();
        let temp = std::env::temp_dir().join("llminx_uniffi_data_dir_test");
        let _ = fs::create_dir_all(&temp);
        set_data_directory(temp.to_string_lossy().to_string());

        let valid = MegaminxState {
            corner_positions: vec![0, 1, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        assert!(validate_megaminx_state(valid).is_none());

        let invalid = MegaminxState {
            corner_positions: vec![0, 0, 2, 3, 4],
            corner_orientations: vec![0, 0, 0, 0, 0],
            edge_positions: vec![0, 1, 2, 3, 4],
            edge_orientations: vec![0, 0, 0, 0, 0],
        };
        assert!(validate_megaminx_state(invalid).is_some());
    }

    #[test]
    fn test_scored_solution_export_into_rs() {
        let input = ScoredSolutionExport {
            mcc: 2.5,
            move_count: 11,
            algorithm: "R U R' U'".to_string(),
        };
        let out: llminxsolver_rs::ScoredSolutionExport = input.into();
        assert_eq!(out.mcc, 2.5);
        assert_eq!(out.move_count, 11);
        assert_eq!(out.algorithm, "R U R' U'");
    }

    #[test]
    fn test_temp_file_append_lock_poison_returns_error() {
        let _guard = lock().lock().unwrap();
        let file = TempFile::new();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _locked = file.inner.lock().unwrap();
            panic!("poison");
        }));

        let err = file.append("R U".to_string());
        assert!(err.is_some());
    }

    #[test]
    fn test_batch_temp_file_append_lock_poison_returns_error() {
        let _guard = lock().lock().unwrap();
        let file = BatchTempFile::new();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _locked = file.inner.lock().unwrap();
            panic!("poison");
        }));

        let err = file.append(1, "R U".to_string());
        assert!(err.is_some());
    }
}
