use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub fn point_at(center: &Point, distance: f64, angle: f64) -> Point {
    Point {
        x: center.x + distance * angle.cos(),
        y: center.y + distance * angle.sin(),
    }
}

pub fn lerp(p1: &Point, p2: &Point, fraction: f64) -> Point {
    Point {
        x: p1.x * (1.0 - fraction) + p2.x * fraction,
        y: p1.y * (1.0 - fraction) + p2.y * fraction,
    }
}

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}

#[allow(clippy::too_many_arguments)]
pub fn line_intersection(
    x1: f64, y1: f64, x2: f64, y2: f64,
    x3: f64, y3: f64, x4: f64, y4: f64,
) -> Point {
    let det = |a: f64, b: f64, c: f64, d: f64| a * d - b * c;
    let denom = det(x1 - x2, y1 - y2, x3 - x4, y3 - y4);
    let det12 = det(x1, y1, x2, y2);
    let det34 = det(x3, y3, x4, y4);
    Point {
        x: det(det12, x1 - x2, det34, x3 - x4) / denom,
        y: det(det12, y1 - y2, det34, y3 - y4) / denom,
    }
}

#[derive(Serialize, Deserialize)]
pub struct EdgeSticker {
    pub top: Vec<Point>,
    pub bottom: Vec<Point>,
}

#[derive(Serialize, Deserialize)]
pub struct CornerSticker {
    pub top: Vec<Point>,
    pub left: Vec<Point>,
    pub right: Vec<Point>,
}

#[derive(Serialize, Deserialize)]
pub struct MegaminxGeometry {
    pub center_points: Vec<Point>,
    pub inner_corners: Vec<Point>,
    pub middle_corners: Vec<Point>,
    pub outer_corners: Vec<Point>,
    pub middle_edges_left: Vec<Point>,
    pub middle_edges_right: Vec<Point>,
    pub edge_stickers: Vec<EdgeSticker>,
    pub corner_stickers: Vec<CornerSticker>,
}

#[wasm_bindgen]
pub fn calculate_megaminx_geometry(width: f64, height: f64, padding: f64) -> JsValue {
    let geometry = calculate_geometry_internal(width, height, padding);
    serde_wasm_bindgen::to_value(&geometry).unwrap()
}

pub fn calculate_geometry_internal(width: f64, height: f64, padding: f64) -> MegaminxGeometry {
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    let outer_radius = (half_height.min(half_width)) - padding;
    let middle_radius = (3.0 * outer_radius) / 4.0;
    let inner_radius = outer_radius / 3.0;
    let center = Point::new(half_width, half_height);

    let mut inner_corners: Vec<Point> = Vec::with_capacity(5);
    let mut middle_corners: Vec<Point> = Vec::with_capacity(5);
    let mut outer_corners: Vec<Point> = Vec::with_capacity(5);
    let mut center_points: Vec<Point> = Vec::with_capacity(5);

    let pi = std::f64::consts::PI;
    for i in 0..5 {
        let angle = -pi / 2.0 + (i as f64 / 5.0) * pi * 2.0;
        inner_corners.push(point_at(&center, inner_radius, angle));
        middle_corners.push(point_at(&center, middle_radius, angle));
        outer_corners.push(point_at(&center, outer_radius, angle));
        center_points.push(point_at(&center, inner_radius, angle));
    }

    let mut middle_edges_left: Vec<Point> = vec![Point::new(0.0, 0.0); 5];
    let mut middle_edges_right: Vec<Point> = vec![Point::new(0.0, 0.0); 5];

    for i in 0..5 {
        let prev_corner = (i + 4) % 5;
        let next_corner = (i + 1) % 5;

        let intersection_right = line_intersection(
            inner_corners[prev_corner].x, inner_corners[prev_corner].y,
            inner_corners[i].x, inner_corners[i].y,
            middle_corners[i].x, middle_corners[i].y,
            middle_corners[next_corner].x, middle_corners[next_corner].y,
        );
        middle_edges_right[i] = intersection_right;

        let intersection_left = line_intersection(
            inner_corners[i].x, inner_corners[i].y,
            inner_corners[next_corner].x, inner_corners[next_corner].y,
            middle_corners[prev_corner].x, middle_corners[prev_corner].y,
            middle_corners[i].x, middle_corners[i].y,
        );
        middle_edges_left[prev_corner] = intersection_left;
    }

    let mut edge_stickers: Vec<EdgeSticker> = (0..5).map(|_| EdgeSticker { top: vec![], bottom: vec![] }).collect();
    let mut corner_stickers: Vec<CornerSticker> = Vec::with_capacity(5);

    for i in 0..5 {
        let prev_corner = (i + 4) % 5;
        let next_corner = (i + 1) % 5;

        let fraction = distance(&middle_edges_left[prev_corner], &inner_corners[i])
            / distance(&middle_edges_left[prev_corner], &middle_edges_right[next_corner]);

        let left_outer_corner = lerp(&outer_corners[i], &outer_corners[next_corner], fraction);
        let right_outer_corner = lerp(&outer_corners[i], &outer_corners[prev_corner], fraction);
        let left_outer_edge = lerp(&outer_corners[next_corner], &outer_corners[i], fraction);

        let edge_index = (i + 3) % 5;
        edge_stickers[edge_index] = EdgeSticker {
            top: vec![
                inner_corners[i],
                inner_corners[next_corner],
                middle_edges_left[i],
                middle_edges_right[i],
            ],
            bottom: vec![
                left_outer_corner,
                middle_edges_right[i],
                middle_edges_left[i],
                left_outer_edge,
            ],
        };

        corner_stickers.push(CornerSticker {
            top: vec![
                inner_corners[i],
                middle_edges_left[prev_corner],
                middle_corners[i],
                middle_edges_right[i],
            ],
            left: vec![
                middle_corners[i],
                middle_edges_right[i],
                left_outer_corner,
                outer_corners[i],
            ],
            right: vec![
                middle_corners[i],
                middle_edges_left[prev_corner],
                right_outer_corner,
                outer_corners[i],
            ],
        });
    }

    MegaminxGeometry {
        center_points,
        inner_corners,
        middle_corners,
        outer_corners,
        middle_edges_left,
        middle_edges_right,
        edge_stickers,
        corner_stickers,
    }
}

#[wasm_bindgen]
pub fn points_to_path(points: JsValue) -> String {
    let pts: Vec<Point> = serde_wasm_bindgen::from_value(points).unwrap_or_default();
    points_to_path_internal(&pts)
}

pub fn points_to_path_internal(points: &[Point]) -> String {
    if points.is_empty() {
        return String::new();
    }
    
    let mut path = String::new();
    for (i, p) in points.iter().enumerate() {
        let cmd = if i == 0 { "M" } else { "L" };
        path.push_str(&format!("{} {:.2} {:.2} ", cmd, p.x, p.y));
    }
    path.push('Z');
    path
}

#[wasm_bindgen]
pub fn get_center_of_points(points: JsValue) -> JsValue {
    let pts: Vec<Point> = serde_wasm_bindgen::from_value(points).unwrap_or_default();
    let center = get_center_internal(&pts);
    serde_wasm_bindgen::to_value(&center).unwrap()
}

pub fn get_center_internal(points: &[Point]) -> Point {
    if points.is_empty() {
        return Point::new(0.0, 0.0);
    }
    
    let sum = points.iter().fold(Point::new(0.0, 0.0), |acc, p| {
        Point::new(acc.x + p.x, acc.y + p.y)
    });
    
    Point::new(sum.x / points.len() as f64, sum.y / points.len() as f64)
}
