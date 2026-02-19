use rsraw_sys as sys;

#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GpsInfo {
    pub latitude: [f32; 3],
    pub longitude: [f32; 3],
    pub gpstimestamp: [f32; 3],
    pub altitude: f32,
    /// Altitude reference: 0 = above sea level, 1 = below sea level.
    pub altref: i8,
    /// Latitude reference: b'N' or b'S'.
    pub latref: i8,
    /// Longitude reference: b'E' or b'W'.
    pub longref: i8,
    /// GPS status: b'A' = measurement active, b'V' = measurement void.
    pub gpsstatus: i8,
    /// Whether GPS data was parsed (non-zero = valid).
    pub gpsparsed: i8,
}

impl From<sys::libraw_gps_info_t> for GpsInfo {
    fn from(data: sys::libraw_gps_info_t) -> Self {
        Self {
            latitude: data.latitude,
            longitude: data.longitude,
            gpstimestamp: data.gpstimestamp,
            altitude: data.altitude,
            altref: data.altref,
            latref: data.latref,
            longref: data.longref,
            gpsstatus: data.gpsstatus,
            gpsparsed: data.gpsparsed,
        }
    }
}