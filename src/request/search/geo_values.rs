use serde::Serialize;

/// Represents Latitude and Longitude Coordinates
///
/// Some operations from Elasticsearch require this
/// compound data-type, and do so via `IntoGeoPoint`.
/// Implement this trait in your codebase for your
/// foreign types if you plan to use them directly
/// for operations that require a `GeoPoint`.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Default)]
pub struct GeoPoint {
    /// The latitude is specified by degrees, starting from 0
    /// and ending up with 90° to both sides of the equator,
    /// making latitude Northern and Southern
    pub lat: f64,

    /// The longitude is defined as an angle pointing west or east
    /// from the Greenwich Meridian, which is taken as the Prime Meridian.
    /// The longitude can be defined maximum as 180° east from the
    /// Prime Meridian and 180° west from the Prime Meridian
    pub lon: f64,
}

impl GeoPoint {
    /// Create a new Geopoint by lat/lon
    pub fn new<T, V>(lat: T, lon: V) -> Self
    where
        T: Into<f64>,
        V: Into<f64>,
    {
        Self {
            lat: lat.into(),
            lon: lon.into(),
        }
    }
}

/// Implement this for any foreign type you have
/// which you want to convert over to a GeoPoint.
pub trait IntoGeoPoint {
    /// consumes self and generates a `GeoPoint`
    fn into_geo_point(self) -> GeoPoint;
}

impl IntoGeoPoint for GeoPoint {
    fn into_geo_point(self) -> GeoPoint {
        self
    }
}

/// When expressing a distance, it can be in either
/// miles or kilomenters; whichever you pick is fine
/// but Elasticsearch needs to know.
#[derive(Debug, Copy, Clone)]
pub enum Distance {
    /// distance measurement by the royal standard; god save the king
    Miles(usize),

    /// Modern standard unit of distance measurement
    Kilometers(usize),
}
