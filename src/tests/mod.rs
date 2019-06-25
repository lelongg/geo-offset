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
                x: 0.8412535328311812,
                y: 0.5406408174555976,
            },
            Coordinate {
                x: 0.41541501300188644,
                y: 0.9096319953545183,
            },
            Coordinate {
                x: -0.142314838273285,
                y: 0.9898214418809328,
            },
            Coordinate {
                x: -0.654860733945285,
                y: 0.7557495743542583,
            },
            Coordinate {
                x: -0.9594929736144974,
                y: 0.28173255684142967,
            },
            Coordinate {
                x: -0.9594929736144974,
                y: -0.28173255684142984,
            },
            Coordinate {
                x: -0.6548607339452849,
                y: -0.7557495743542585,
            },
            Coordinate {
                x: -0.14231483827328523,
                y: -0.9898214418809327,
            },
            Coordinate {
                x: 0.41541501300188605,
                y: -0.9096319953545186,
            },
            Coordinate {
                x: 0.8412535328311808,
                y: -0.5406408174555982,
            },
            Coordinate {
                x: 1.0,
                y: -0.000000000000001133107779529596,
            },
            Coordinate {
                x: 0.8412535328311812,
                y: 0.5406408174555976,
            },
        ]),
        Vec::new(),
    )]);

    println!("{}", result.to_svg().and(&point.to_svg()).with_margin(10.0));
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
        result.to_svg().and(&segment.to_svg()).with_margin(10.0)
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
        result.to_svg().and(&polygon.to_svg()).with_margin(10.0)
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

    println!(
        "{}",
        result.to_svg().and(&polygon.to_svg()).with_margin(5.0)
    );
}

#[test]
fn test_demo_offset() {
    let result = fixtures::DEMO.offset_with_arc_segments(0.0001, 5).unwrap();

    println!(
        "{}",
        result
            .clone()
            .to_svg()
            .and(&fixtures::DEMO.to_svg())
            .with_margin(0.001)
    );

    // assert_eq!(*fixtures::DEMO_WITH_OFFSET, geo::GeometryCollection::from(fixtures::FeatureCollection::from(result)));
}
