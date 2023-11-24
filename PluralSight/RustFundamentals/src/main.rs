const EARTH_RADIUS: f64 = 6371.0;

struct Waypoint {
	latitude: f64,
	longitud: f64,
}

/// Using the Haversine Formula from the course
fn distance(a: &Waypoint, b: &Waypoint) -> f64 {
	let a_latitude_rad = a.latitude.to_radians();
	let b_latitude_rad = b.latitude.to_radians();
	let d_latitude_rad = (a.latitude - b.latitude).to_radians();
	let d_longitud_rad = (a.longitud - b.longitud).to_radians();

	let inner_angle = (d_latitude_rad / 2.0).sin().powi(2) +
		a_latitude_rad.cos() * b_latitude_rad.cos() *
		(d_longitud_rad / 2.0).sin().powi(2);

	2.0 * EARTH_RADIUS * inner_angle.sqrt().asin()
}

#[test]
fn distance_test1() {
	let loc1 = Waypoint {
		latitude: 41.4075,
		longitud: -81.851111,
	};
	let loc2 = Waypoint {
		latitude: 41.51030,
		longitud: -83.88080,
	};
	assert_eq!(format!("{:.2}", distance(&loc1, &loc2)), "169.52");
}

fn main() {
	println!("Hello, world!");
}
