use super::*;
use geo::{Coordinate, LineString, Polygon};
use geo_svg::*;

mod fixtures;

#[test]
fn test_edge() {
    let edge = Edge::new(&(0.0, 0.0).into(), &(0.0, 5.0).into());
    assert_eq!(geo::Coordinate::from((0.0, 0.0)), edge.current);
    assert_eq!(geo::Coordinate::from((0.0, 5.0)), edge.next);
    assert_eq!(
        geo::Coordinate::from((-1.0, 0.0)),
        edge.inwards_normal().unwrap()
    );
    assert_eq!(
        geo::Coordinate::from((1.0, 0.0)),
        edge.outwards_normal().unwrap()
    );
}

#[test]
fn test_point_offset() {
    let point = geo::Point::new(0.0, 0.0);
    let result = point.offset_with_arc_segments(1.0, 5).unwrap();
    let expected = geo::MultiPolygon(vec![Polygon::new(
        LineString(vec![
            Coordinate {
                x: 0.841_253_532_831_181_2,
                y: 0.540_640_817_455_597_6,
            },
            Coordinate {
                x: 0.415_415_013_001_886_44,
                y: 0.909_631_995_354_518_3,
            },
            Coordinate {
                x: -0.142_314_838_273_285,
                y: 0.989_821_441_880_932_8,
            },
            Coordinate {
                x: -0.654_860_733_945_285,
                y: 0.755_749_574_354_258_3,
            },
            Coordinate {
                x: -0.959_492_973_614_497_4,
                y: 0.281_732_556_841_429_67,
            },
            Coordinate {
                x: -0.959_492_973_614_497_4,
                y: -0.281_732_556_841_429_84,
            },
            Coordinate {
                x: -0.654_860_733_945_284_9,
                y: -0.755_749_574_354_258_5,
            },
            Coordinate {
                x: -0.142_314_838_273_285_23,
                y: -0.989_821_441_880_932_7,
            },
            Coordinate {
                x: 0.415_415_013_001_886_05,
                y: -0.909_631_995_354_518_6,
            },
            Coordinate {
                x: 0.841_253_532_831_180_8,
                y: -0.540_640_817_455_598_2,
            },
            Coordinate {
                x: 1.0,
                y: -0.000_000_000_000_001_133_107_779_529_596,
            },
            Coordinate {
                x: 0.841_253_532_831_181_2,
                y: 0.540_640_817_455_597_6,
            },
        ]),
        Vec::new(),
    )]);

    println!("{}", result.to_svg().and(point.to_svg()).with_margin(10.0));
    assert_eq!(expected, result);
}

#[test]
fn test_segment_offset() {
    let p1 = geo::Coordinate::from((0.0, 0.0));
    let p2 = geo::Coordinate::from((0.0, 8.0));
    let segment = geo::Line::new(p1, p2);
    let result = segment.offset(5.0).unwrap();

    println!(
        "{}",
        result.to_svg().and(segment.to_svg()).with_margin(10.0)
    );
}

#[test]
fn test_polygon_offset() {
    use geo::polygon;
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
    use geo::polygon;
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
fn test_demo_offset() {
    let result = fixtures::DEMO.offset_with_arc_segments(0.0001, 5).unwrap();

    println!(
        "{}",
        result
            .clone()
            .to_svg()
            .and(fixtures::DEMO.to_svg())
            .with_margin(0.001)
    );

    // assert_eq!(*fixtures::DEMO_WITH_OFFSET, geo::GeometryCollection::from(fixtures::FeatureCollection::from(result)));
}
