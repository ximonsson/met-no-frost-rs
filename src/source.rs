pub struct Point {
    // The source type of the Source. ,
    r#type: Option<String>, // optional):

    // Coordinates of the geometry object
    coordinates: Vec<f32>,
}

pub struct Source {
    // The source type of the Source. ,
    r#type: Option<String>, // optional):

    // The Frost API id of the source. ,
    id: Option<String>, // optional):

    // The name of the source. ,
    name: String, // optional):

    // The short name of the source. ,
    short_name: String, // optional):

    // The country affiliation of the source. ,
    country: String, // optional):

    // The ISO 3166-1 alpha-2 code of the country. ,
    country_code: String, // optional):

    // The assigned WMO number for a SensorSystem, if one exists. ,
    wmo_id: String, // optional):

    // Spatial location data for the source. ,
    geometry: Point, // optional):

    // The distance (in kilometers) from a reference point. ,
    distance: String, // optional):

    // The elevation of the source in meters above sea level. ,
    masl: String, // optional):

    // The datetime from which the source is valid. ,
    valid_from: String, // optional):

    // The datetime to which the source was valid (if no longer valid). ,
    valid_to: String, // optional):

    // County name. ,
    county: String, // optional):

    // County id. ,
    county_id: String, // optional):

    // Municipality name. ,
    municipality: String, // optional):

    // Municipality id. ,
    municipality_id: String, // optional):

    // Ontology id ,
    ontology_id: String, // optional):

    // Station holders. ,
    station_holders: Vec<String>, // optional):

    // External ids. ,
    external_ids: Vec<String>, // optional):

    // ICAO codes. ,
    icao_codes: Vec<String>, // optional):

    // Ship codes. ,
    ship_codes: Vec<String>, // optional):

    // WIGOS id.
    wigos_id: String, // optional):
}

pub struct SourceResponse {
    // The Json-LD context. ,
    context: String, // optional):

    // The object type. ,
    r#type: String, // optional):

    // The version of the API that generated this response. ,
    api_version: String, // optional):

    // The license that applies to this content. ,
    license: String, // optional):

    // The time at which this document was created (RFC 3339). ,
    created_at: String, // optional):

    // The time, in seconds, that this document took to generate. ,
    query_time: String, // optional):

    // The current number of items in this result set. ,
    current_item_count: i32, // optional):

    // The maximum number of items in a result set. ,
    items_per_page: i32, // optional):

    // The offset of the first item in the result set. The Frost API uses a zero-base index. ,
    offset: i32, // optional):

    // The total number of items in this specific result set. ,
    total_item_count: i32, // optional):

    // The next link indicates how more data can be retrieved. It points to the URI to load the next set of data. ,
    next_link: String, // optional):

    // The previous link indicates how more data can be retrieved. It points to the URI to load the previous set of data. ,
    previous_link: String, // optional):

    // The current link indicates the URI that was used to generate the current API response ,
    current_link: String, // optional):

    // Container for all the data from the response.
    data: Vec<Source>, // optional):
}
