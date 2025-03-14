use super::*;
use geo_svg::*;
use geo_types::{Coord, LineString, Polygon};

mod fixtures;

#[test]
fn test_edge() {
    let edge = Edge::new(&(0.0, 0.0).into(), &(0.0, 5.0).into());
    assert_eq!(geo_types::Coord::from((0.0, 0.0)), edge.current);
    assert_eq!(geo_types::Coord::from((0.0, 5.0)), edge.next);
    assert_eq!(
        geo_types::Coord::from((-1.0, 0.0)),
        edge.inwards_normal().unwrap()
    );
    assert_eq!(
        geo_types::Coord::from((1.0, 0.0)),
        edge.outwards_normal().unwrap()
    );
}

const POINT_OFFSET_COORDS: [Coord; 12] = [
    Coord {
        x: 0.841_253_532_831_181_2,
        y: 0.540_640_817_455_597_6,
    },
    Coord {
        x: 0.415_415_013_001_886_44,
        y: 0.909_631_995_354_518_3,
    },
    Coord {
        x: -0.142_314_838_273_285,
        y: 0.989_821_441_880_932_8,
    },
    Coord {
        x: -0.654_860_733_945_285,
        y: 0.755_749_574_354_258_3,
    },
    Coord {
        x: -0.959_492_973_614_497_4,
        y: 0.281_732_556_841_429_67,
    },
    Coord {
        x: -0.959_492_973_614_497_4,
        y: -0.281_732_556_841_429_84,
    },
    Coord {
        x: -0.654_860_733_945_284_9,
        y: -0.755_749_574_354_258_5,
    },
    Coord {
        x: -0.142_314_838_273_285_23,
        y: -0.989_821_441_880_932_7,
    },
    Coord {
        x: 0.415_415_013_001_886_05,
        y: -0.909_631_995_354_518_6,
    },
    Coord {
        x: 0.841_253_532_831_180_8,
        y: -0.540_640_817_455_598_2,
    },
    Coord {
        x: 1.0,
        y: -0.000_000_000_000_001_133_107_779_529_596,
    },
    Coord {
        x: 0.841_253_532_831_181_2,
        y: 0.540_640_817_455_597_6,
    },
];

#[test]
fn test_point_offset() {
    let point = geo_types::Point::new(0.0, 0.0);
    let resolution = ArcResolution::SegmentCount(POINT_OFFSET_COORDS.len() - 1); // -1 because the test LineString is closed
    let result = point.offset_with_arc_resolution(1.0, resolution).unwrap();
    let expected = geo_types::MultiPolygon(vec![Polygon::new(
        LineString(Vec::from(POINT_OFFSET_COORDS)),
        Vec::new(),
    )]);

    println!("{}", result.to_svg().and(point.to_svg()).with_margin(10.0));
    assert_eq!(expected, result);
}

#[test]
fn test_point_offset_targeting_segment_length() {
    let point = geo_types::Point::new(0.0, 0.0);

    let radius = 1.0;
    let segments_expected = POINT_OFFSET_COORDS.len() - 1; // -1 because the test LineString is closed
    let segment_length = radius * std::f64::consts::TAU / segments_expected as f64;
    let resolution = ArcResolution::SegmentLength(segment_length);

    let result = point.offset_with_arc_resolution(radius, resolution).unwrap();
    let expected = geo_types::MultiPolygon(vec![Polygon::new(
        LineString(Vec::from(POINT_OFFSET_COORDS)),
        Vec::new(),
    )]);

    println!("{}", result.to_svg().and(point.to_svg()).with_margin(10.0));
    assert_eq!(expected, result);
}

#[test]
fn test_segment_offset() {
    let p1 = geo_types::Coord::from((0.0, 0.0));
    let p2 = geo_types::Coord::from((0.0, 8.0));
    let segment = geo_types::Line::new(p1, p2);
    let result = segment.offset(5.0).unwrap();

    println!(
        "{}",
        result.to_svg().and(segment.to_svg()).with_margin(10.0)
    );
}

#[test]
fn test_polygon_offset() {
    use geo_types::polygon;
    let polygon = polygon![
        (x: -10., y: 10.),
        (x: 10., y: 10.),
        (x: 10., y: -10.),
        (x: -10., y: -10.),
    ];
    let result = polygon.offset(5.0).unwrap();

    println!(
        "{}",
        result.to_svg().and(polygon.to_svg()).with_margin(10.0)
    );
}

#[test]
fn test_polygon_with_hole_offset() {
    use geo_types::polygon;
    let polygon = polygon![
        exterior: [
        (x: -15., y: 15.),
        (x: 15., y: 15.),
        (x: 15., y: -15.),
        (x: -15., y: -15.),],
        interiors: [[
            (x: -10., y: 10.),
            (x: 10., y: 10.),
            (x: 10., y: -10.),
            (x: -10., y: -10.),
    ]]];

    let result = polygon.offset(1.0).unwrap();

    println!("{}", result.to_svg().and(polygon.to_svg()).with_margin(5.0));
}

#[test]
fn test_polygon_with_hole_offset_high_resolution() {
    use geo_types::polygon;
    let polygon = polygon![
        exterior: [
        (x: -15., y: 15.),
        (x: 15., y: 15.),
        (x: 15., y: -15.),
        (x: -15., y: -15.),],
        interiors: [[
            (x: -10., y: 10.),
            (x: 10., y: 10.),
            (x: 10., y: -10.),
            (x: -10., y: -10.),
    ]]];

    let resolution = ArcResolution::SegmentLength(0.1);
    let result = polygon.offset_with_arc_resolution(1.0, resolution).unwrap();

    println!("{}", result.to_svg().and(polygon.to_svg()).with_margin(5.0));
}

#[test]
fn test_demo_offset() {
    let result = fixtures::DEMO.offset_with_arc_resolution(0.0001, ArcResolution::SegmentCount(5)).unwrap();

    println!(
        "{}",
        result
            .clone()
            .to_svg()
            .and(fixtures::DEMO.to_svg())
            .with_margin(0.001)
    );

    // assert_eq!(*fixtures::DEMO_WITH_OFFSET, geo_types::GeometryCollection::from(fixtures::FeatureCollection::from(result)));
}
