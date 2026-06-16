use novadraw_geometry::{
    AffineTransform, ApproxEq, Dimension, Insets, Point, PointList, PrecisionDimension,
    PrecisionPoint, PrecisionRectangle, Rectangle, Transform, Vector,
};

fn accepts_precision_point(_: PrecisionPoint) {}
fn accepts_precision_rectangle(_: PrecisionRectangle) {}
fn accepts_precision_dimension(_: PrecisionDimension) {}
fn accepts_vector(_: Vector) {}
fn accepts_affine_transform(_: AffineTransform) {}

#[test]
fn m1_geometry_product_type_names_are_importable() {
    let point = Point::new(1.0, 2.0);
    let rect = Rectangle::new(0.0, 0.0, 10.0, 20.0);
    let dimension = Dimension::new(10.0, 20.0);
    let vector = Vector::new(3.0, 4.0);
    let transform = Transform::from_translation(5.0, 6.0);

    accepts_precision_point(point);
    accepts_precision_rectangle(rect);
    accepts_precision_dimension(dimension);
    accepts_vector(vector);
    accepts_affine_transform(transform);

    let insets = Insets::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(insets.left, 2.0);

    let points = PointList::from_points(vec![point, Point::new(4.0, 6.0)]);
    assert_eq!(points.bounds(), Some(Rectangle::new(1.0, 2.0, 3.0, 4.0)));
}

#[test]
fn m1_precision_aliases_keep_approx_eq_contract() {
    let precise_point: PrecisionPoint = Point::new(1.0, 1.0 + 1e-11);
    let expected_point = Point::new(1.0, 1.0);
    assert!(precise_point.approx_eq_default(expected_point));

    let precise_rect: PrecisionRectangle = Rectangle::new(0.0, 0.0, 10.0, 10.0 + 1e-11);
    let expected_rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    assert!(precise_rect.approx_eq_default(expected_rect));

    let affine: AffineTransform = Transform::from_translation(3.0, 4.0);
    let expected = Transform::new(1.0, 0.0, 0.0, 1.0, 3.0, 4.0);
    assert!(affine.approx_eq_default(expected));
}
