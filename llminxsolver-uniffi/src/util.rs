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
