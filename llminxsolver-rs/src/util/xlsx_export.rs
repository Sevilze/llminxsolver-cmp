use rust_xlsxwriter::{Format, FormatAlign, Image, Workbook, Worksheet, XlsxError};

#[derive(Debug, Clone)]
pub struct ScoredSolutionExport {
    pub mcc: f64,
    pub move_count: u32,
    pub algorithm: String,
}

fn create_header_format() -> Format {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

fn create_cell_format() -> Format {
    Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

fn setup_worksheet_with_image(
    worksheet: &mut Worksheet,
    image_png_bytes: Option<&[u8]>,
    image_size: u32,
) -> Result<(), XlsxError> {
    if let Some(img_bytes) = image_png_bytes {
        let base_image = Image::new_from_buffer(img_bytes)?;
        let img_w = base_image.width();
        let img_h = base_image.height();

        let target_h_px: u32 = image_size.max(1);
        let aspect = if img_h > 0.0 { img_w / img_h } else { 1.0 };
        let row_height_px: u32 = 20;

        let image_row_span: u32 = target_h_px.div_ceil(row_height_px);
        let total_h_px: u32 = image_row_span * row_height_px;
        let final_w_px: u32 = ((total_h_px as f64) * aspect).ceil().max(1.0) as u32;

        worksheet.set_column_width_pixels(0, final_w_px)?;
        for row in 0..image_row_span {
            worksheet.set_row_height_pixels(row, row_height_px)?;
        }

        worksheet.merge_range(
            0,
            0,
            image_row_span.saturating_sub(1),
            0,
            "",
            &Format::new(),
        )?;

        let image = base_image.set_scale_to_size(final_w_px, total_h_px, true);
        worksheet.insert_image(0, 0, &image)?;
    }

    Ok(())
}

pub fn export_scored_xlsx(
    output_path: &str,
    solutions: &[ScoredSolutionExport],
    image_png_bytes: Option<&[u8]>,
    image_size: u32,
) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Solutions").map_err(|e| e.to_string())?;

    let has_image = image_png_bytes.is_some();
    let col_offset: u16 = if has_image { 1 } else { 0 };

    setup_worksheet_with_image(worksheet, image_png_bytes, image_size)
        .map_err(|e| e.to_string())?;

    let header_format = create_header_format();
    let cell_format = create_cell_format();

    let headers = ["MCC", "Movecount", "Algorithm"];
    for (idx, header) in headers.iter().enumerate() {
        worksheet
            .write_string_with_format(0, col_offset + idx as u16, *header, &header_format)
            .map_err(|e| e.to_string())?;
    }

    worksheet
        .set_column_width(col_offset, 10.0)
        .map_err(|e| e.to_string())?;
    worksheet
        .set_column_width(col_offset + 1, 10.0)
        .map_err(|e| e.to_string())?;

    let max_algo_len = solutions
        .iter()
        .map(|s| s.algorithm.len())
        .max()
        .unwrap_or(30);
    let algo_width = (max_algo_len as f64 * 1.0 + 2.0).max(15.0);
    worksheet
        .set_column_width(col_offset + 2, algo_width)
        .map_err(|e| e.to_string())?;

    for (row_idx, solution) in solutions.iter().enumerate() {
        let row = (row_idx + 1) as u32;

        let mcc_rounded = (solution.mcc * 10.0).round() / 10.0;
        worksheet
            .write_number_with_format(row, col_offset, mcc_rounded, &cell_format)
            .map_err(|e| e.to_string())?;
        worksheet
            .write_number_with_format(
                row,
                col_offset + 1,
                solution.move_count as f64,
                &cell_format,
            )
            .map_err(|e| e.to_string())?;
        worksheet
            .write_string_with_format(row, col_offset + 2, &solution.algorithm, &cell_format)
            .map_err(|e| e.to_string())?;
    }

    workbook.save(output_path).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn export_raw_xlsx(
    output_path: &str,
    algorithms: &[String],
    image_png_bytes: Option<&[u8]>,
    image_size: u32,
) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Solutions").map_err(|e| e.to_string())?;

    let has_image = image_png_bytes.is_some();
    let col_offset: u16 = if has_image { 1 } else { 0 };

    setup_worksheet_with_image(worksheet, image_png_bytes, image_size)
        .map_err(|e| e.to_string())?;

    let header_format = create_header_format();
    let cell_format = create_cell_format();

    worksheet
        .write_string_with_format(0, col_offset, "Algorithm", &header_format)
        .map_err(|e| e.to_string())?;

    let max_algo_len = algorithms.iter().map(|s| s.len()).max().unwrap_or(30);
    let algo_width = (max_algo_len as f64 * 1.0 + 2.0).max(15.0);
    worksheet
        .set_column_width(col_offset, algo_width)
        .map_err(|e| e.to_string())?;

    for (row_idx, algorithm) in algorithms.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        worksheet
            .write_string_with_format(row, col_offset, algorithm, &cell_format)
            .map_err(|e| e.to_string())?;
    }

    workbook.save(output_path).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn export_raw_xlsx_from_file(
    output_path: &str,
    solutions_file_path: &str,
    image_png_bytes: Option<&[u8]>,
    image_size: u32,
) -> Result<(), String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(solutions_file_path)
        .map_err(|e| format!("Failed to open solutions file: {}", e))?;
    let reader = BufReader::new(file);
    let algorithms: Vec<String> = reader.lines().map_while(Result::ok).collect();

    export_raw_xlsx(output_path, &algorithms, image_png_bytes, image_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scored_solution_export_struct() {
        let solution = ScoredSolutionExport {
            mcc: 2.5,
            move_count: 10,
            algorithm: "R U R'".to_string(),
        };
        assert_eq!(solution.mcc, 2.5);
        assert_eq!(solution.move_count, 10);
        assert_eq!(solution.algorithm, "R U R'");
    }

    #[test]
    fn test_scored_solution_export_clone() {
        let solution = ScoredSolutionExport {
            mcc: 3.0,
            move_count: 5,
            algorithm: "U R".to_string(),
        };
        let cloned = solution.clone();
        assert_eq!(cloned.mcc, solution.mcc);
        assert_eq!(cloned.move_count, solution.move_count);
        assert_eq!(cloned.algorithm, solution.algorithm);
    }

    #[test]
    fn test_export_scored_xlsx_empty() {
        let temp_path = "/tmp/test_scored_empty.xlsx";
        let result = export_scored_xlsx(temp_path, &[], None, 100);
        assert!(result.is_ok());
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_export_scored_xlsx_with_solutions() {
        let temp_path = "/tmp/test_scored_with_solutions.xlsx";
        let solutions = vec![
            ScoredSolutionExport {
                mcc: 1.5,
                move_count: 5,
                algorithm: "R U R'".to_string(),
            },
            ScoredSolutionExport {
                mcc: 2.0,
                move_count: 8,
                algorithm: "R U R' U'".to_string(),
            },
        ];
        let result = export_scored_xlsx(temp_path, &solutions, None, 100);
        assert!(result.is_ok());
        assert!(std::path::Path::new(temp_path).exists());
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_export_raw_xlsx_empty() {
        let temp_path = "/tmp/test_raw_empty.xlsx";
        let result = export_raw_xlsx(temp_path, &[], None, 100);
        assert!(result.is_ok());
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_export_raw_xlsx_with_algorithms() {
        let temp_path = "/tmp/test_raw_with_algs.xlsx";
        let algorithms = vec![
            "R U R'".to_string(),
            "R U R' U'".to_string(),
            "R U2 R'".to_string(),
        ];
        let result = export_raw_xlsx(temp_path, &algorithms, None, 100);
        assert!(result.is_ok());
        assert!(std::path::Path::new(temp_path).exists());
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_export_raw_xlsx_from_file_nonexistent() {
        let result =
            export_raw_xlsx_from_file("/tmp/output.xlsx", "/nonexistent/file.txt", None, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_export_raw_xlsx_from_file_valid() {
        use std::io::Write;

        let solutions_path = "/tmp/test_solutions.txt";
        let mut file = fs::File::create(solutions_path).unwrap();
        writeln!(file, "R U R'").unwrap();
        writeln!(file, "R U2 R'").unwrap();
        drop(file);

        let output_path = "/tmp/test_raw_from_file.xlsx";
        let result = export_raw_xlsx_from_file(output_path, solutions_path, None, 100);
        assert!(result.is_ok());

        let _ = fs::remove_file(solutions_path);
        let _ = fs::remove_file(output_path);
    }
}
