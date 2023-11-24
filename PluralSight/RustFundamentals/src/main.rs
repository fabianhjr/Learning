const EARTH_RADIUS: f64 = 6371.0;

struct Location {
	latitude: f64,
	longitud: f64,
}

impl Location {
	/// Using the Haversine Formula from the course
	fn distance(&self, other: &Location) -> f64 {
		let a_latitude_rad = self.latitude.to_radians();
		let b_latitude_rad = other.latitude.to_radians();
		let d_latitude_rad = (self.latitude - other.latitude).to_radians();
		let d_longitud_rad = (self.longitud - other.longitud).to_radians();

		let inner_angle = (d_latitude_rad / 2.0).sin().powi(2) +
			a_latitude_rad.cos() * b_latitude_rad.cos() *
			(d_longitud_rad / 2.0).sin().powi(2);

		2.0 * EARTH_RADIUS * inner_angle.sqrt().asin()
	}
}

struct Waypoint<'l> {
	code: &'l str,
	location: &'l Location
}

impl Waypoint<'_> {
	fn distance(&self, other: &Waypoint) -> f64 {
		self.location.distance(other.location)
	}
}

#[test]
fn distance_test1() {
	let loc1 = Waypoint {
		code: "MEX",
		location: &Location {
			latitude: 41.4075,
			longitud: -81.851111,
		}
	};
	let loc2 = Waypoint {
		code: "MEX",
		location: &Location {
			latitude: 41.51030,
			longitud: -83.88080,
		}
	};
	assert_eq!(format!("{:.2}", &loc1.distance(&loc2)), "169.52");
}


fn main() {
	println!("Hello, world!");
}
