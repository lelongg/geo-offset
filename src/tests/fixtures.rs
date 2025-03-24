use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub static POLYGON_POLYLINE: Lazy<geo_types::GeometryCollection<f64>> = Lazy::new(|| {
    let data = include_str!("fixtures/polygon_polyline.json");
    let feature_collection: FeatureCollection = serde_json::from_str(&data).unwrap();
    feature_collection.into()
});

#[allow(dead_code)]
pub static DEMO: Lazy<geo_types::GeometryCollection<f64>> = Lazy::new(|| {
    let data = include_str!("fixtures/demo.json");
    let feature_collection: FeatureCollection = serde_json::from_str(&data).unwrap();
    feature_collection.into()
});

#[allow(dead_code)]
pub static DEMO_WITH_OFFSET: Lazy<geo_types::GeometryCollection<f64>> = Lazy::new(|| {
    let data = include_str!("fixtures/demo_with_offset.json");
    let feature_collection: FeatureCollection = serde_json::from_str(&data).unwrap();
    feature_collection.into()
});

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Feature {
    pub geometry: Geometry,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Geometry {
    Polygon(Polygon),
    LineString(LineString),
    Point(Point),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Polygon {
    pub coordinates: Vec<Vec<(f64, f64)>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LineString {
    pub coordinates: Vec<(f64, f64)>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Point {
    pub coordinates: (f64, f64),
}

impl From<FeatureCollection> for geo_types::GeometryCollection<f64> {
    fn from(feature_collection: FeatureCollection) -> Self {
        Self(
            feature_collection
                .features
                .iter()
                .map(|feature| geo_types::Geometry::from(feature.clone()))
                .collect(),
        )
    }
}

impl From<Feature> for geo_types::Geometry<f64> {
    fn from(feature: Feature) -> Self {
        feature.geometry.into()
    }
}

impl From<Geometry> for geo_types::Geometry<f64> {
    fn from(geometry: Geometry) -> Self {
        match geometry {
            Geometry::Polygon(polygon) => {
                let polygon: geo_types::Polygon<f64> = polygon.into();
                polygon.into()
            }
            Geometry::LineString(line_string) => {
                let line_string: geo_types::LineString<f64> = line_string.into();
                line_string.into()
            }
            Geometry::Point(point) => {
                let point: geo_types::Point<f64> = point.into();
                point.into()
            }
        }
    }
}

impl From<Polygon> for geo_types::Polygon<f64> {
    fn from(polygon: Polygon) -> Self {
        Self::new(
            polygon
                .coordinates
                .get(0)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(geo_types::Coord::from)
                .collect::<Vec<_>>()
                .into(),
            polygon
                .coordinates
                .iter()
                .skip(1)
                .map(|interior| {
                    interior
                        .iter()
                        .cloned()
                        .map(geo_types::Coord::from)
                        .collect::<Vec<_>>()
                        .into()
                })
                .collect(),
        )
    }
}

impl From<LineString> for geo_types::LineString<f64> {
    fn from(line_string: LineString) -> Self {
        Self(
            line_string
                .coordinates
                .iter()
                .map(|coordinate| geo_types::Coord::from(*coordinate))
                .collect(),
        )
    }
}

impl From<Point> for geo_types::Point<f64> {
    fn from(point: Point) -> Self {
        Self::from(point.coordinates)
    }
}

impl From<geo_types::GeometryCollection<f64>> for FeatureCollection {
    fn from(geometry_collection: geo_types::GeometryCollection<f64>) -> Self {
        Self {
            features: geometry_collection
                .into_iter()
                .flat_map(|geometry| FeatureCollection::from(geometry).features)
                .collect(),
        }
    }
}

impl From<geo_types::Geometry<f64>> for FeatureCollection {
    fn from(geometry: geo_types::Geometry<f64>) -> Self {
        match geometry {
            geo_types::Geometry::Polygon(polygon) => {
                let polygon: Polygon = polygon.into();
                Self {
                    features: vec![Feature {
                        geometry: Geometry::Polygon(polygon),
                    }],
                }
            }
            geo_types::Geometry::LineString(line_string) => {
                let line_string: LineString = line_string.into();
                Self {
                    features: vec![Feature {
                        geometry: Geometry::LineString(line_string),
                    }],
                }
            }
            geo_types::Geometry::Point(point) => {
                let point: Point = point.into();
                Self {
                    features: vec![Feature {
                        geometry: Geometry::Point(point),
                    }],
                }
            }
            geo_types::Geometry::MultiPolygon(multi_polygon) => multi_polygon.into(),
            _ => unimplemented!(),
        }
    }
}

impl From<geo_types::MultiPolygon<f64>> for FeatureCollection {
    fn from(multi_polygon: geo_types::MultiPolygon<f64>) -> Self {
        Self {
            features: multi_polygon
                .into_iter()
                .map(|polygon| Feature {
                    geometry: Geometry::Polygon(polygon.into()),
                })
                .collect(),
        }
    }
}

impl From<geo_types::Polygon<f64>> for Polygon {
    fn from(polygon: geo_types::Polygon<f64>) -> Self {
        Self {
            coordinates: vec![polygon
                .exterior()
                .clone()
                .into_iter()
                .map(|coords| (coords.x, coords.y))
                .collect()],
        }
    }
}

impl From<geo_types::LineString<f64>> for LineString {
    fn from(line_string: geo_types::LineString<f64>) -> Self {
        Self {
            coordinates: line_string
                .points()
                .map(|coords| (coords.x(), coords.y()))
                .collect(),
        }
    }
}

impl From<geo_types::Point<f64>> for Point {
    fn from(point: geo_types::Point<f64>) -> Self {
        Self {
            coordinates: (point.x(), point.y()),
        }
    }
}
