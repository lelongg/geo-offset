use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geo_offset::{ArcResolution, Offset};

fn offset_point(offset: f32) {
    use geo_types::point;
    let point = point!(x: 100., y: 100.);

    let resolution = ArcResolution::SegmentLength(0.01);
    let _ = point.offset_with_arc_resolution(offset, resolution).unwrap();
}

fn offset_line_string(offset: f32) {
    use geo_types::line_string;
    let line_string = line_string![
        (x: -15., y: 15.),
        (x: 15., y: 15.),
        (x: 15., y: -15.),
        (x: -15., y: -15.),
    ];

    let resolution = ArcResolution::SegmentLength(0.01);
    let _ = line_string.offset_with_arc_resolution(offset, resolution).unwrap();
}

fn offset_polygon(offset: f32) {
    use geo_types::polygon;
    let polygon = polygon![
        (x: -15., y: 15.),
        (x: 15., y: 15.),
        (x: 15., y: -15.),
        (x: -15., y: -15.),
    ];

    let resolution = ArcResolution::SegmentLength(0.01);
    let _ = polygon.offset_with_arc_resolution(offset, resolution).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("point", |b| b.iter(|| black_box(offset_point(5.0))));
    c.bench_function("line_string", |b| b.iter(|| black_box(offset_line_string(1.0))));
    c.bench_function("polygon grow", |b| b.iter(|| black_box(offset_polygon(1.0))));
    c.bench_function("polygon shrink", |b| b.iter(|| black_box(offset_polygon(-1.0))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
