use super::{Edge, EdgeError};
use geo_clipper::Clipper;
use geo_types::CoordFloat;

/// If offset computing fails this error is returned.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OffsetError {
    /// This error can be produced when manipulating edges.
    EdgeError(EdgeError),
}

/// Resolution of arcs generated around corners for positive offsets.
///
/// ```
/// # use geo_offset::ArcResolution;
/// // The default resolution is 5 segments.
/// let resolution: ArcResolution<f32> = Default::default();
/// assert_eq!(resolution, ArcResolution::SegmentCount(5));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArcResolution<F: CoordFloat> {
    /// Sets the exact number of arc segments to be generated.
    SegmentCount(usize),
    /// Sets the desired segment length, so that the number of segments is chosen based on the length of the arc.
    SegmentLength(F),
}

impl<F: CoordFloat> Default for ArcResolution<F> {
    fn default() -> Self {
        Self::SegmentCount(5)
    }
}

pub trait Offset<F: CoordFloat> {
    fn offset(&self, distance: F) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        self.offset_with_arc_resolution(distance, Default::default())
    }

    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError>;
}

impl<F: CoordFloat> Offset<F> for geo_types::GeometryCollection<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        let mut geometry_collection_with_offset = geo_types::MultiPolygon::<F>(Vec::new());
        for geometry in self.0.iter() {
            let geometry_with_offset = geometry.offset_with_arc_resolution(distance, arc_resolution)?;
            geometry_collection_with_offset = geometry_collection_with_offset
                .union(&geometry_with_offset, F::from(1000.0).unwrap());
        }

        Ok(geometry_collection_with_offset)
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::Geometry<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        match self {
            geo_types::Geometry::Point(point) => {
                point.offset_with_arc_resolution(distance, arc_resolution)
            }
            geo_types::Geometry::Line(line) => {
                line.offset_with_arc_resolution(distance, arc_resolution)
            }
            geo_types::Geometry::LineString(line_tring) => {
                line_tring.offset_with_arc_resolution(distance, arc_resolution)
            }
            geo_types::Geometry::Triangle(triangle) => triangle
                .to_polygon()
                .offset_with_arc_resolution(distance, arc_resolution),
            geo_types::Geometry::Rect(rect) => rect
                .to_polygon()
                .offset_with_arc_resolution(distance, arc_resolution),
            geo_types::Geometry::Polygon(polygon) => {
                polygon.offset_with_arc_resolution(distance, arc_resolution)
            }
            geo_types::Geometry::MultiPoint(multi_point) => {
                multi_point.offset_with_arc_resolution(distance, arc_resolution)
            }
            geo_types::Geometry::MultiLineString(multi_line_string) => {
                multi_line_string.offset_with_arc_resolution(distance, arc_resolution)
            }
            geo_types::Geometry::MultiPolygon(multi_polygon) => {
                multi_polygon.offset_with_arc_resolution(distance, arc_resolution)
            }
            geo_types::Geometry::GeometryCollection(geometry_collection) => {
                geometry_collection.offset_with_arc_resolution(distance, arc_resolution)
            }
        }
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::MultiPolygon<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        let mut polygons = geo_types::MultiPolygon::<F>(Vec::new());
        for polygon in self.0.iter() {
            let polygon_with_offset = polygon.offset_with_arc_resolution(distance, arc_resolution)?;
            polygons = polygons.union(&polygon_with_offset, F::from(1000.0).unwrap());
        }

        Ok(polygons)
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::Polygon<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        let exterior_with_offset = self
            .exterior()
            .offset_with_arc_resolution(distance.abs(), arc_resolution)?;
        let interiors_with_offset = geo_types::MultiLineString::<F>(self.interiors().to_vec())
            .offset_with_arc_resolution(distance.abs(), arc_resolution)?;

        Ok(if distance.is_sign_positive() {
            self.union(&exterior_with_offset, F::from(1000.0).unwrap())
                .union(&interiors_with_offset, F::from(1000.0).unwrap())
        } else {
            self.difference(&exterior_with_offset, F::from(1000.0).unwrap())
                .difference(&interiors_with_offset, F::from(1000.0).unwrap())
        })
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::MultiLineString<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        if distance < F::zero() {
            return Ok(geo_types::MultiPolygon(Vec::new()));
        }

        let mut multi_line_string_with_offset = geo_types::MultiPolygon::<F>(Vec::new());
        for line_string in &self.0 {
            let line_string_with_offset =
                line_string.offset_with_arc_resolution(distance, arc_resolution)?;
            multi_line_string_with_offset = multi_line_string_with_offset
                .union(&line_string_with_offset, F::from(1000.0).unwrap());
        }
        Ok(multi_line_string_with_offset)
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::LineString<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        if distance < F::zero() {
            return Ok(geo_types::MultiPolygon(Vec::new()));
        }

        let mut line_string_with_offset = geo_types::MultiPolygon::<F>(Vec::new());
        for line in self.lines() {
            let line_with_offset = line.offset_with_arc_resolution(distance, arc_resolution)?;
            line_string_with_offset =
                line_string_with_offset.union(&line_with_offset, F::from(1000.0).unwrap());
        }

        let line_string_with_offset = line_string_with_offset.0.iter().skip(1).fold(
            geo_types::MultiPolygon::<F>(
                line_string_with_offset
                    .0
                    .first()
                    .map(|polygon| vec![polygon.clone()])
                    .unwrap_or_default(),
            ),
            |result, hole| result.difference(hole, F::from(1000.0).unwrap()),
        );

        Ok(line_string_with_offset)
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::Line<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        if distance < F::zero() {
            return Ok(geo_types::MultiPolygon(Vec::new()));
        }

        let v1 = &self.start;
        let v2 = &self.end;
        let e1 = Edge::new(v1, v2);

        if let (Ok(in_normal), Ok(out_normal)) = (e1.inwards_normal(), e1.outwards_normal()) {
            let offsets = [
                e1.with_offset(in_normal.x * distance, in_normal.y * distance),
                e1.inverse_with_offset(out_normal.x * distance, out_normal.y * distance),
            ];

            let len = 2;
            let mut vertices = Vec::new();

            for i in 0..len {
                let current_edge = offsets.get(i).unwrap();
                let prev_edge = offsets.get((i + len + 1) % len).unwrap();
                create_arc(
                    &mut vertices,
                    if i == 0 { v1 } else { v2 },
                    distance,
                    &prev_edge.next,
                    &current_edge.current,
                    arc_resolution,
                    true,
                );
            }

            Ok(geo_types::MultiPolygon(vec![geo_types::Polygon::new(
                geo_types::LineString(vertices),
                vec![],
            )]))
        } else {
            geo_types::Point::from(self.start).offset_with_arc_resolution(distance, arc_resolution)
        }
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::MultiPoint<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        if distance < F::zero() {
            return Ok(geo_types::MultiPolygon(Vec::new()));
        }

        let mut multi_point_with_offset = geo_types::MultiPolygon::<F>(Vec::new());
        for point in self.0.iter() {
            let point_with_offset = point.offset_with_arc_resolution(distance, arc_resolution)?;
            multi_point_with_offset =
                multi_point_with_offset.union(&point_with_offset, F::from(1000.0).unwrap());
        }
        Ok(multi_point_with_offset)
    }
}

impl<F: CoordFloat> Offset<F> for geo_types::Point<F> {
    fn offset_with_arc_resolution(
        &self,
        distance: F,
        arc_resolution: ArcResolution<F>,
    ) -> Result<geo_types::MultiPolygon<F>, OffsetError> {
        if distance < F::zero() {
            return Ok(geo_types::MultiPolygon(Vec::new()));
        }

        let tau = F::from(std::f64::consts::TAU).unwrap();
        let mut angle = F::zero();

        let segment_count = match arc_resolution {
            ArcResolution::SegmentCount(segment_count) => segment_count,
            ArcResolution::SegmentLength(segment_length) => {
                let circumference = tau * distance;
                (circumference / segment_length).to_usize().unwrap()
            },
        };
        let segment_count = segment_count.max(3); // A circle should have at least three sides :)

        let contour = (0..segment_count)
            .map(|_| {
                angle =
                    angle + F::from(2.0 * std::f64::consts::PI / segment_count as f64).unwrap(); // counter-clockwise

                geo_types::Coord::from((
                    self.x() + (distance * angle.cos()),
                    self.y() + (distance * angle.sin()),
                ))
            })
            .collect();

        Ok(geo_types::MultiPolygon(vec![geo_types::Polygon::new(
            contour,
            Vec::new(),
        )]))
    }
}

fn create_arc<F: CoordFloat>(
    vertices: &mut Vec<geo_types::Coord<F>>,
    center: &geo_types::Coord<F>,
    radius: F,
    start_vertex: &geo_types::Coord<F>,
    end_vertex: &geo_types::Coord<F>,
    arc_resolution: ArcResolution<F>,
    outwards: bool,
) {
    let tau = F::from(std::f64::consts::TAU).unwrap();

    let start_angle = (start_vertex.y - center.y).atan2(start_vertex.x - center.x);
    let start_angle = if start_angle.is_sign_negative() {
        start_angle + tau
    } else {
        start_angle
    };

    let end_angle = (end_vertex.y - center.y).atan2(end_vertex.x - center.x);
    let end_angle = if end_angle.is_sign_negative() {
        end_angle + tau
    } else {
        end_angle
    };

    let angle = if start_angle > end_angle {
        start_angle - end_angle
    } else {
        start_angle + tau - end_angle
    };

    let segment_count = match arc_resolution {
        ArcResolution::SegmentCount(segment_count) => segment_count,
        ArcResolution::SegmentLength(segment_length) => {
            let arc_length = angle * radius;
            (arc_length / segment_length).to_usize().unwrap()
        },
    };

    let segment_angle =
        if outwards { -angle } else { tau - angle } / F::from(segment_count).unwrap();

    vertices.push(*start_vertex);
    for i in 1..segment_count {
        let angle = start_angle + segment_angle * F::from(i).unwrap();
        vertices.push(geo_types::Coord::from((
            center.x + angle.cos() * radius,
            center.y + angle.sin() * radius,
        )));
    }
    vertices.push(*end_vertex);
}
