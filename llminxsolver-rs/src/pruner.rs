use crate::coordinate::{CKN, CoordinateUtil, FAC, POWERS_OF_THREE, POWERS_OF_TWO};
use crate::data_directory::get_data_directory;
use crate::minx::{LLMinx, NUM_CORNERS, NUM_EDGES};
use crate::search_mode::Metric;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

pub const MIN_PRUNING_DEPTH: u8 = 8;
pub const MAX_PRUNING_DEPTH: u8 = 18;
pub const DEFAULT_PRUNING_DEPTH: u8 = 12;

const COMPRESSED_EXTENSION: &str = ".prn.lz4";

pub trait Pruner: Send + Sync {
    fn name(&self) -> &str;
    fn table_path(&self) -> &str;
    fn table_size(&self) -> usize;
    fn get_coordinate(&self, minx: &LLMinx) -> usize;
    fn set_minx(&self, coordinate: usize, minx: &mut LLMinx);
    fn uses_corner_permutation(&self) -> bool;
    fn uses_edge_permutation(&self) -> bool;
    fn uses_corner_orientation(&self) -> bool;
    fn uses_edge_orientation(&self) -> bool;

    fn is_precomputed(&self, metric: Metric) -> bool {
        self.get_table_file(metric).exists()
    }

    fn get_table_file(&self, metric: Metric) -> PathBuf {
        let metric_suffix = match metric {
            Metric::Fifth => "FIFTH",
            Metric::Face => "FACE",
        };
        let filename = format!(
            "{}{}{}",
            self.table_path(),
            metric_suffix,
            COMPRESSED_EXTENSION
        );

        if let Some(data_dir) = get_data_directory() {
            data_dir.join(&filename)
        } else {
            PathBuf::from(filename)
        }
    }

    fn get_table_file_with_depth(&self, metric: Metric, depth: u8) -> PathBuf {
        let metric_suffix = match metric {
            Metric::Fifth => "FIFTH",
            Metric::Face => "FACE",
        };
        let filename = format!(
            "d{}_{}{}{}",
            depth,
            self.table_path(),
            metric_suffix,
            COMPRESSED_EXTENSION
        );

        if let Some(data_dir) = get_data_directory() {
            data_dir.join(&filename)
        } else {
            PathBuf::from(filename)
        }
    }

    fn is_precomputed_with_depth(&self, metric: Metric, depth: u8) -> bool {
        self.get_table_file_with_depth(metric, depth).exists()
    }

    fn find_best_existing_table(&self, metric: Metric, max_depth: u8) -> Option<(PathBuf, u8)> {
        for depth in (MIN_PRUNING_DEPTH..=max_depth).rev() {
            let path = self.get_table_file_with_depth(metric, depth);
            if path.exists() {
                return Some((path, depth));
            }
        }
        None
    }

    fn load_table(&self, metric: Metric) -> Option<Vec<u8>> {
        let path = self.get_table_file(metric);
        if path.exists() {
            return self.load_compressed_table(&path);
        }
        None
    }

    fn load_table_with_depth(&self, metric: Metric, depth: u8) -> Option<Vec<u8>> {
        let path = self.get_table_file_with_depth(metric, depth);
        if path.exists() {
            return self.load_compressed_table(&path);
        }
        None
    }

    fn load_compressed_table(&self, path: &PathBuf) -> Option<Vec<u8>> {
        let file = File::open(path).ok()?;
        let mut reader = BufReader::with_capacity(1 << 20, file);
        let mut compressed = Vec::new();
        reader.read_to_end(&mut compressed).ok()?;
        decompress_size_prepended(&compressed).ok()
    }

    fn save_table(&self, table: &[u8], metric: Metric) {
        let path = self.get_table_file(metric);
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(file) = File::create(&path) {
            let compressed = compress_prepend_size(table);
            let mut writer = BufWriter::with_capacity(1 << 22, file);
            let _ = writer.write_all(&compressed);
            let _ = writer.flush();
        }
    }

    fn save_table_with_depth(&self, table: &[u8], metric: Metric, depth: u8) {
        let path = self.get_table_file_with_depth(metric, depth);
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(file) = File::create(&path) {
            let compressed = compress_prepend_size(table);
            let mut writer = BufWriter::with_capacity(1 << 22, file);
            let _ = writer.write_all(&compressed);
            let _ = writer.flush();
        }
    }
}

pub struct CornerOrientationPruner {
    name: String,
    table_path: String,
    corners: Vec<u8>,
}

impl CornerOrientationPruner {
    pub fn new(name: &str, table_path: &str, corners: &[u8]) -> Self {
        Self {
            name: name.to_string(),
            table_path: table_path.to_string(),
            corners: corners.to_vec(),
        }
    }
}

impl Pruner for CornerOrientationPruner {
    fn name(&self) -> &str {
        &self.name
    }

    fn table_path(&self) -> &str {
        &self.table_path
    }

    fn table_size(&self) -> usize {
        POWERS_OF_THREE[self.corners.len() - 1] as usize
    }

    fn get_coordinate(&self, minx: &LLMinx) -> usize {
        CoordinateUtil::get_corner_orientation_coordinate(minx.corner_orientations(), &self.corners)
            as usize
    }

    fn set_minx(&self, coordinate: usize, minx: &mut LLMinx) {
        minx.set_corner_orientations(CoordinateUtil::get_corner_orientation(
            coordinate as u32,
            &self.corners,
        ));
    }

    fn uses_corner_permutation(&self) -> bool {
        false
    }

    fn uses_edge_permutation(&self) -> bool {
        false
    }

    fn uses_corner_orientation(&self) -> bool {
        true
    }

    fn uses_edge_orientation(&self) -> bool {
        false
    }
}

pub struct CornerPermutationPruner {
    name: String,
    table_path: String,
    corners: Vec<u8>,
}

impl CornerPermutationPruner {
    pub fn new(name: &str, table_path: &str, corners: &[u8]) -> Self {
        Self {
            name: name.to_string(),
            table_path: table_path.to_string(),
            corners: corners.to_vec(),
        }
    }
}

impl Pruner for CornerPermutationPruner {
    fn name(&self) -> &str {
        &self.name
    }

    fn table_path(&self) -> &str {
        &self.table_path
    }

    fn table_size(&self) -> usize {
        (FAC[self.corners.len()] / 2) as usize
    }

    fn get_coordinate(&self, minx: &LLMinx) -> usize {
        CoordinateUtil::get_permutation_coordinate(minx.corner_positions(), &self.corners) as usize
    }

    fn set_minx(&self, coordinate: usize, minx: &mut LLMinx) {
        CoordinateUtil::get_permutation(
            coordinate as u32,
            minx.corner_positions_mut(),
            &self.corners,
        );
    }

    fn uses_corner_permutation(&self) -> bool {
        true
    }

    fn uses_edge_permutation(&self) -> bool {
        false
    }

    fn uses_corner_orientation(&self) -> bool {
        false
    }

    fn uses_edge_orientation(&self) -> bool {
        false
    }
}

pub struct EdgeOrientationPruner {
    name: String,
    table_path: String,
    edges: Vec<u8>,
}

impl EdgeOrientationPruner {
    pub fn new(name: &str, table_path: &str, edges: &[u8]) -> Self {
        Self {
            name: name.to_string(),
            table_path: table_path.to_string(),
            edges: edges.to_vec(),
        }
    }
}

impl Pruner for EdgeOrientationPruner {
    fn name(&self) -> &str {
        &self.name
    }

    fn table_path(&self) -> &str {
        &self.table_path
    }

    fn table_size(&self) -> usize {
        POWERS_OF_TWO[self.edges.len() - 1] as usize
    }

    fn get_coordinate(&self, minx: &LLMinx) -> usize {
        CoordinateUtil::get_edge_orientation_coordinate(minx.edge_orientations(), self.edges.len())
            as usize
    }

    fn set_minx(&self, coordinate: usize, minx: &mut LLMinx) {
        minx.set_edge_orientations(CoordinateUtil::get_edge_orientation(
            coordinate as u32,
            self.edges.len(),
        ));
    }

    fn uses_corner_permutation(&self) -> bool {
        false
    }

    fn uses_edge_permutation(&self) -> bool {
        false
    }

    fn uses_corner_orientation(&self) -> bool {
        false
    }

    fn uses_edge_orientation(&self) -> bool {
        true
    }
}

pub struct EdgePermutationPruner {
    name: String,
    table_path: String,
    edges: Vec<u8>,
}

impl EdgePermutationPruner {
    pub fn new(name: &str, table_path: &str, edges: &[u8]) -> Self {
        Self {
            name: name.to_string(),
            table_path: table_path.to_string(),
            edges: edges.to_vec(),
        }
    }
}

impl Pruner for EdgePermutationPruner {
    fn name(&self) -> &str {
        &self.name
    }

    fn table_path(&self) -> &str {
        &self.table_path
    }

    fn table_size(&self) -> usize {
        (FAC[self.edges.len()] / 2) as usize
    }

    fn get_coordinate(&self, minx: &LLMinx) -> usize {
        CoordinateUtil::get_permutation_coordinate(minx.edge_positions(), &self.edges) as usize
    }

    fn set_minx(&self, coordinate: usize, minx: &mut LLMinx) {
        CoordinateUtil::get_permutation(coordinate as u32, minx.edge_positions_mut(), &self.edges);
    }

    fn uses_corner_permutation(&self) -> bool {
        false
    }

    fn uses_edge_permutation(&self) -> bool {
        true
    }

    fn uses_corner_orientation(&self) -> bool {
        false
    }

    fn uses_edge_orientation(&self) -> bool {
        false
    }
}

pub struct SeparationPruner {
    name: String,
    table_path: String,
    corners: Vec<u8>,
    edges: Vec<u8>,
}

impl SeparationPruner {
    pub fn new(name: &str, table_path: &str, corners: &[u8], edges: &[u8]) -> Self {
        Self {
            name: name.to_string(),
            table_path: table_path.to_string(),
            corners: corners.to_vec(),
            edges: edges.to_vec(),
        }
    }
}

impl Pruner for SeparationPruner {
    fn name(&self) -> &str {
        &self.name
    }

    fn table_path(&self) -> &str {
        &self.table_path
    }

    fn table_size(&self) -> usize {
        (CKN[NUM_CORNERS][self.corners.len()] * CKN[NUM_EDGES][self.edges.len()]) as usize
    }

    fn get_coordinate(&self, minx: &LLMinx) -> usize {
        let corner_coord =
            CoordinateUtil::get_separation_coordinate(minx.corner_positions(), &self.corners);
        let edge_coord =
            CoordinateUtil::get_separation_coordinate(minx.edge_positions(), &self.edges);
        (corner_coord * CKN[NUM_EDGES][self.edges.len()] + edge_coord) as usize
    }

    fn set_minx(&self, coordinate: usize, minx: &mut LLMinx) {
        let edge_table_size = CKN[NUM_EDGES][self.edges.len()] as usize;
        CoordinateUtil::get_separation(
            (coordinate % edge_table_size) as u32,
            minx.edge_positions_mut(),
            &self.edges,
        );
        CoordinateUtil::get_separation(
            (coordinate / edge_table_size) as u32,
            minx.corner_positions_mut(),
            &self.corners,
        );
    }

    fn uses_corner_permutation(&self) -> bool {
        self.corners.len() > 1
    }

    fn uses_edge_permutation(&self) -> bool {
        self.edges.len() > 1
    }

    fn uses_corner_orientation(&self) -> bool {
        false
    }

    fn uses_edge_orientation(&self) -> bool {
        false
    }
}

pub struct CompositePruner {
    name: String,
    table_path: String,
    pruner_a: Box<dyn Pruner>,
    pruner_b: Box<dyn Pruner>,
}

impl CompositePruner {
    pub fn new(
        name: &str,
        table_path: &str,
        pruner_a: Box<dyn Pruner>,
        pruner_b: Box<dyn Pruner>,
    ) -> Self {
        Self {
            name: name.to_string(),
            table_path: table_path.to_string(),
            pruner_a,
            pruner_b,
        }
    }
}

impl Pruner for CompositePruner {
    fn name(&self) -> &str {
        &self.name
    }

    fn table_path(&self) -> &str {
        &self.table_path
    }

    fn table_size(&self) -> usize {
        self.pruner_a.table_size() * self.pruner_b.table_size()
    }

    fn get_coordinate(&self, minx: &LLMinx) -> usize {
        self.pruner_a.get_coordinate(minx) * self.pruner_b.table_size()
            + self.pruner_b.get_coordinate(minx)
    }

    fn set_minx(&self, coordinate: usize, minx: &mut LLMinx) {
        let size_b = self.pruner_b.table_size();
        self.pruner_b.set_minx(coordinate % size_b, minx);
        self.pruner_a.set_minx(coordinate / size_b, minx);
    }

    fn uses_corner_permutation(&self) -> bool {
        self.pruner_a.uses_corner_permutation() || self.pruner_b.uses_corner_permutation()
    }

    fn uses_edge_permutation(&self) -> bool {
        self.pruner_a.uses_edge_permutation() || self.pruner_b.uses_edge_permutation()
    }

    fn uses_corner_orientation(&self) -> bool {
        self.pruner_a.uses_corner_orientation() || self.pruner_b.uses_corner_orientation()
    }

    fn uses_edge_orientation(&self) -> bool {
        self.pruner_a.uses_edge_orientation() || self.pruner_b.uses_edge_orientation()
    }
}
