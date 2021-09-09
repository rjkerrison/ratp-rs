pub mod types {
    #[allow(unused_imports)]
    use savon::internal::chrono::{offset::Utc, DateTime};
    use savon::internal::xmltree;
    use savon::rpser::xml::*;
    #[derive(Clone, Debug, Default)]
    pub struct GeoPoint {
        pub r_type: Option<String>,
        pub name: Option<String>,
        pub id: Option<String>,
        pub y: Double,
        pub name_suffix: Option<String>,
        pub x: Double,
    }
    impl savon::gen::ToElements for GeoPoint {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec ! [vec ! [xmltree :: Element :: node ("type") . with_text (self . type . to_string ())] , vec ! [xmltree :: Element :: node ("name") . with_text (self . name . to_string ())] , vec ! [xmltree :: Element :: node ("id") . with_text (self . id . to_string ())] , vec ! [xmltree :: Element :: node ("y") . with_children (self . y . to_elements ())] , vec ! [xmltree :: Element :: node ("nameSuffix") . with_text (self . name_suffix . to_string ())] , vec ! [xmltree :: Element :: node ("x") . with_children (self . x . to_elements ())]] . drain (..) . flatten () . collect ()
        }
    }
    impl savon::gen::FromElement for GeoPoint {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(GeoPoint {
                r_type: element
                    .get_at_path(&["type"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                y: element
                    .get_at_path(&["y"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Double::from_element(&e).map_err(savon::Error::from))?,
                name_suffix: element
                    .get_at_path(&["nameSuffix"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                x: element
                    .get_at_path(&["x"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Double::from_element(&e).map_err(savon::Error::from))?,
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Direction {
        pub stations_end_line: Option<Vec<Station>>,
        pub sens: Option<String>,
        pub line: Option<Line>,
        pub name: Option<String>,
    }
    impl savon::gen::ToElements for Direction {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                self.stations_end_line
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stationsEndLine")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("sens").with_text(self.sens.to_string())],
                vec![xmltree::Element::node("line").with_children(self.line.to_elements())],
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Direction {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Direction {
                stations_end_line: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Station::from_element(&elem)?);
                    }
                    v
                }),
                sens: element
                    .get_at_path(&["sens"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                line: element
                    .get_at_path(&["line"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct WrDirections {
        pub argument_line: Option<Line>,
        pub directions: Option<Vec<Direction>>,
        pub ambiguity_message: Option<String>,
        pub ambiguous_lines: Option<Vec<Line>>,
    }
    impl savon::gen::ToElements for WrDirections {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("argumentLine")
                    .with_children(self.argument_line.to_elements())],
                self.directions
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("directions").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("ambiguityMessage")
                    .with_text(self.ambiguity_message.to_string())],
                self.ambiguous_lines
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousLines")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for WrDirections {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(WrDirections {
                argument_line: element
                    .get_at_path(&["argumentLine"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                directions: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Direction::from_element(&elem)?);
                    }
                    v
                }),
                ambiguity_message: element
                    .get_at_path(&["ambiguityMessage"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                ambiguous_lines: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Line::from_element(&elem)?);
                    }
                    v
                }),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Mission {
        pub stations_stops: Option<Vec<bool>>,
        pub station_end_line: Option<Station>,
        pub stations_platforms: Option<Vec<String>>,
        pub id: Option<String>,
        pub stations_messages: Option<Vec<String>>,
        pub code: Option<String>,
        pub perturbations: Option<Vec<Perturbation>>,
        pub stations: Option<Vec<Station>>,
        pub line: Option<Line>,
        pub stations_dates: Option<Vec<String>>,
        pub direction: Option<Direction>,
    }
    impl savon::gen::ToElements for Mission {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                self.stations_stops
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stationsStops")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("stationEndLine")
                    .with_children(self.station_end_line.to_elements())],
                self.stations_platforms
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stationsPlatforms")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                self.stations_messages
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stationsMessages")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("code").with_text(self.code.to_string())],
                self.perturbations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("perturbations")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                self.stations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stations").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("line").with_children(self.line.to_elements())],
                self.stations_dates
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stationsDates")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![
                    xmltree::Element::node("direction").with_children(self.direction.to_elements())
                ],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Mission {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Mission {
                stations_stops: element
                    .get_at_path(&["stationsStops"])
                    .and_then(|e| e.as_boolean())
                    .ok(),
                station_end_line: element
                    .get_at_path(&["stationEndLine"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Station::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                stations_platforms: element
                    .get_at_path(&["stationsPlatforms"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                stations_messages: element
                    .get_at_path(&["stationsMessages"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                code: element
                    .get_at_path(&["code"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                perturbations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Perturbation::from_element(&elem)?);
                    }
                    v
                }),
                stations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Station::from_element(&elem)?);
                    }
                    v
                }),
                line: element
                    .get_at_path(&["line"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                stations_dates: element
                    .get_at_path(&["stationsDates"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                direction: element
                    .get_at_path(&["direction"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Direction::from_element(&e).map_err(savon::Error::from))
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct WrPerturbations {
        pub argument_media: Option<String>,
        pub perturbations: Option<Vec<Perturbation>>,
        pub argument_source: Option<String>,
    }
    impl savon::gen::ToElements for WrPerturbations {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("argumentMedia")
                    .with_text(self.argument_media.to_string())],
                self.perturbations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("perturbations")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("argumentSource")
                    .with_text(self.argument_source.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for WrPerturbations {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(WrPerturbations {
                argument_media: element
                    .get_at_path(&["argumentMedia"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                perturbations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Perturbation::from_element(&elem)?);
                    }
                    v
                }),
                argument_source: element
                    .get_at_path(&["argumentSource"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct StationAcces {
        pub address: Option<String>,
        pub index: Option<String>,
        pub time_days_label: Option<String>,
        pub time_days_status: Option<String>,
        pub id: Option<String>,
        pub name: Option<String>,
        pub time_start: Option<String>,
        pub time_end: Option<String>,
        pub x: Double,
        pub y: Double,
    }
    impl savon::gen::ToElements for StationAcces {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("address").with_text(self.address.to_string())],
                vec![xmltree::Element::node("index").with_text(self.index.to_string())],
                vec![xmltree::Element::node("timeDaysLabel")
                    .with_text(self.time_days_label.to_string())],
                vec![xmltree::Element::node("timeDaysStatus")
                    .with_text(self.time_days_status.to_string())],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
                vec![xmltree::Element::node("timeStart").with_text(self.time_start.to_string())],
                vec![xmltree::Element::node("timeEnd").with_text(self.time_end.to_string())],
                vec![xmltree::Element::node("x").with_children(self.x.to_elements())],
                vec![xmltree::Element::node("y").with_children(self.y.to_elements())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for StationAcces {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(StationAcces {
                address: element
                    .get_at_path(&["address"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                index: element
                    .get_at_path(&["index"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                time_days_label: element
                    .get_at_path(&["timeDaysLabel"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                time_days_status: element
                    .get_at_path(&["timeDaysStatus"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                time_start: element
                    .get_at_path(&["timeStart"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                time_end: element
                    .get_at_path(&["timeEnd"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                x: element
                    .get_at_path(&["x"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Double::from_element(&e).map_err(savon::Error::from))?,
                y: element
                    .get_at_path(&["y"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Double::from_element(&e).map_err(savon::Error::from))?,
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct WrItineraries {
        pub ambiguity_message: Option<String>,
        pub ambiguous_geo_points_end: Option<Vec<GeoPoint>>,
        pub ambiguous_geo_points_start: Option<Vec<GeoPoint>>,
        pub itineraries: Option<Vec<Itinerary>>,
        pub argument_date: Option<String>,
    }
    impl savon::gen::ToElements for WrItineraries {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("ambiguityMessage")
                    .with_text(self.ambiguity_message.to_string())],
                self.ambiguous_geo_points_end
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousGeoPointsEnd")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                self.ambiguous_geo_points_start
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousGeoPointsStart")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                self.itineraries
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("itineraries").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("argumentDate")
                    .with_text(self.argument_date.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for WrItineraries {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(WrItineraries {
                ambiguity_message: element
                    .get_at_path(&["ambiguityMessage"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                ambiguous_geo_points_end: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(GeoPoint::from_element(&elem)?);
                    }
                    v
                }),
                ambiguous_geo_points_start: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(GeoPoint::from_element(&elem)?);
                    }
                    v
                }),
                itineraries: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Itinerary::from_element(&elem)?);
                    }
                    v
                }),
                argument_date: element
                    .get_at_path(&["argumentDate"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PerturbationMessage {
        pub text: Option<String>,
        pub r_type: Option<String>,
        pub media_specific: bool,
        pub updated: bool,
    }
    impl savon::gen::ToElements for PerturbationMessage {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("text").with_text(self.text.to_string())],
                vec![xmltree::Element::node("type").with_text(self.r_type.to_string())],
                vec![xmltree::Element::node("mediaSpecific")
                    .with_text(self.media_specific.to_string())],
                vec![xmltree::Element::node("updated").with_text(self.updated.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for PerturbationMessage {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(PerturbationMessage {
                text: element
                    .get_at_path(&["text"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                r_type: element
                    .get_at_path(&["type"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                media_specific: element
                    .get_at_path(&["mediaSpecific"])
                    .and_then(|e| e.as_boolean())?,
                updated: element
                    .get_at_path(&["updated"])
                    .and_then(|e| e.as_boolean())?,
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct StationArea {
        pub name: Option<String>,
        pub stations: Option<Vec<Station>>,
        pub id: Option<String>,
        pub access: Option<Vec<StationAcces>>,
        pub tarifs_to_paris: Option<Vec<Tarif>>,
        pub zone_carte_orange: Option<String>,
    }
    impl savon::gen::ToElements for StationArea {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
                self.stations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stations").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                self.access
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("access").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                self.tarifs_to_paris
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("tarifsToParis")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("zoneCarteOrange")
                    .with_text(self.zone_carte_orange.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for StationArea {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(StationArea {
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                stations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Station::from_element(&elem)?);
                    }
                    v
                }),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                access: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(StationAcces::from_element(&elem)?);
                    }
                    v
                }),
                tarifs_to_paris: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Tarif::from_element(&elem)?);
                    }
                    v
                }),
                zone_carte_orange: element
                    .get_at_path(&["zoneCarteOrange"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct WrMission {
        pub ambiguity_message: Option<String>,
        pub argument_date: Option<String>,
        pub ambiguous_lines: Option<Vec<Line>>,
        pub argument_line: Option<Line>,
        pub mission: Option<Mission>,
    }
    impl savon::gen::ToElements for WrMission {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("ambiguityMessage")
                    .with_text(self.ambiguity_message.to_string())],
                vec![xmltree::Element::node("argumentDate")
                    .with_text(self.argument_date.to_string())],
                self.ambiguous_lines
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousLines")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("argumentLine")
                    .with_children(self.argument_line.to_elements())],
                vec![xmltree::Element::node("mission").with_children(self.mission.to_elements())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for WrMission {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(WrMission {
                ambiguity_message: element
                    .get_at_path(&["ambiguityMessage"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                argument_date: element
                    .get_at_path(&["argumentDate"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                ambiguous_lines: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Line::from_element(&elem)?);
                    }
                    v
                }),
                argument_line: element
                    .get_at_path(&["argumentLine"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                mission: element
                    .get_at_path(&["mission"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Mission::from_element(&e).map_err(savon::Error::from))
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Itinerary {
        pub geo_point_start: Option<GeoPoint>,
        pub date_end: Option<String>,
        pub missions: Option<Vec<Mission>>,
        pub date_start: Option<String>,
        pub tarif: Option<Tarif>,
        pub durations_transit: Option<Vec<i64>>,
        pub geo_point_end: Option<GeoPoint>,
    }
    impl savon::gen::ToElements for Itinerary {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("geoPointStart")
                    .with_children(self.geo_point_start.to_elements())],
                vec![xmltree::Element::node("dateEnd").with_text(self.date_end.to_string())],
                self.missions
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("missions").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("dateStart").with_text(self.date_start.to_string())],
                vec![xmltree::Element::node("tarif").with_children(self.tarif.to_elements())],
                self.durations_transit
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("durationsTransit")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("geoPointEnd")
                    .with_children(self.geo_point_end.to_elements())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Itinerary {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Itinerary {
                geo_point_start: element
                    .get_at_path(&["geoPointStart"])
                    .map_err(savon::Error::from)
                    .and_then(|e| GeoPoint::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                date_end: element
                    .get_at_path(&["dateEnd"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                missions: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Mission::from_element(&elem)?);
                    }
                    v
                }),
                date_start: element
                    .get_at_path(&["dateStart"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                tarif: element
                    .get_at_path(&["tarif"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Tarif::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                durations_transit: element
                    .get_at_path(&["durationsTransit"])
                    .and_then(|e| e.as_long())
                    .ok(),
                geo_point_end: element
                    .get_at_path(&["geoPointEnd"])
                    .map_err(savon::Error::from)
                    .and_then(|e| GeoPoint::from_element(&e).map_err(savon::Error::from))
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PerturbationIncident {
        pub message_global: Option<PerturbationMessage>,
        pub status: Option<String>,
        pub date: Option<String>,
        pub id: Option<String>,
        pub incident_lines: Option<Vec<PerturbationIncidentLine>>,
    }
    impl savon::gen::ToElements for PerturbationIncident {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("messageGlobal")
                    .with_children(self.message_global.to_elements())],
                vec![xmltree::Element::node("status").with_text(self.status.to_string())],
                vec![xmltree::Element::node("date").with_text(self.date.to_string())],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                self.incident_lines
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("incidentLines")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for PerturbationIncident {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(PerturbationIncident {
                message_global: element
                    .get_at_path(&["messageGlobal"])
                    .map_err(savon::Error::from)
                    .and_then(|e| PerturbationMessage::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                status: element
                    .get_at_path(&["status"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                date: element
                    .get_at_path(&["date"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                incident_lines: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(PerturbationIncidentLine::from_element(&elem)?);
                    }
                    v
                }),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Perturbation {
        pub date_end: Option<String>,
        pub consequence: Option<PerturbationConsequence>,
        pub message: Option<PerturbationMessage>,
        pub line: Option<Line>,
        pub time_end: Option<String>,
        pub title: Option<String>,
        pub cause: Option<PerturbationCause>,
        pub id: Option<String>,
        pub station: Option<Station>,
        pub media: Option<String>,
        pub source: Option<String>,
        pub date_start: Option<String>,
        pub incidents: Option<Vec<PerturbationIncident>>,
        pub level: Option<String>,
        pub status: Option<String>,
        pub time_start: Option<String>,
    }
    impl savon::gen::ToElements for Perturbation {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("dateEnd").with_text(self.date_end.to_string())],
                vec![xmltree::Element::node("consequence")
                    .with_children(self.consequence.to_elements())],
                vec![xmltree::Element::node("message").with_children(self.message.to_elements())],
                vec![xmltree::Element::node("line").with_children(self.line.to_elements())],
                vec![xmltree::Element::node("timeEnd").with_text(self.time_end.to_string())],
                vec![xmltree::Element::node("title").with_text(self.title.to_string())],
                vec![xmltree::Element::node("cause").with_children(self.cause.to_elements())],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                vec![xmltree::Element::node("station").with_children(self.station.to_elements())],
                vec![xmltree::Element::node("media").with_text(self.media.to_string())],
                vec![xmltree::Element::node("source").with_text(self.source.to_string())],
                vec![xmltree::Element::node("dateStart").with_text(self.date_start.to_string())],
                self.incidents
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("incidents").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("level").with_text(self.level.to_string())],
                vec![xmltree::Element::node("status").with_text(self.status.to_string())],
                vec![xmltree::Element::node("timeStart").with_text(self.time_start.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Perturbation {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Perturbation {
                date_end: element
                    .get_at_path(&["dateEnd"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                consequence: element
                    .get_at_path(&["consequence"])
                    .map_err(savon::Error::from)
                    .and_then(|e| {
                        PerturbationConsequence::from_element(&e).map_err(savon::Error::from)
                    })
                    .ok(),
                message: element
                    .get_at_path(&["message"])
                    .map_err(savon::Error::from)
                    .and_then(|e| PerturbationMessage::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                line: element
                    .get_at_path(&["line"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                time_end: element
                    .get_at_path(&["timeEnd"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                title: element
                    .get_at_path(&["title"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                cause: element
                    .get_at_path(&["cause"])
                    .map_err(savon::Error::from)
                    .and_then(|e| PerturbationCause::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                station: element
                    .get_at_path(&["station"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Station::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                media: element
                    .get_at_path(&["media"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                source: element
                    .get_at_path(&["source"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                date_start: element
                    .get_at_path(&["dateStart"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                incidents: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(PerturbationIncident::from_element(&elem)?);
                    }
                    v
                }),
                level: element
                    .get_at_path(&["level"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                status: element
                    .get_at_path(&["status"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                time_start: element
                    .get_at_path(&["timeStart"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PerturbationCause {
        pub parent_name: Option<String>,
        pub id: Option<String>,
        pub name: Option<String>,
        pub parent_id: Option<String>,
    }
    impl savon::gen::ToElements for PerturbationCause {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("parentName").with_text(self.parent_name.to_string())],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
                vec![xmltree::Element::node("parentId").with_text(self.parent_id.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for PerturbationCause {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(PerturbationCause {
                parent_name: element
                    .get_at_path(&["parentName"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                parent_id: element
                    .get_at_path(&["parentId"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Station {
        pub name: Option<String>,
        pub geo_point_a: Option<GeoPoint>,
        pub ids_next_a: Option<Vec<String>>,
        pub line: Option<Line>,
        pub ids_next_r: Option<Vec<String>>,
        pub station_area: Option<StationArea>,
        pub direction: Option<Direction>,
        pub id: Option<String>,
        pub geo_point_r: Option<GeoPoint>,
    }
    impl savon::gen::ToElements for Station {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
                vec![xmltree::Element::node("geoPointA")
                    .with_children(self.geo_point_a.to_elements())],
                self.ids_next_a
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("idsNextA").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("line").with_children(self.line.to_elements())],
                self.ids_next_r
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("idsNextR").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("stationArea")
                    .with_children(self.station_area.to_elements())],
                vec![
                    xmltree::Element::node("direction").with_children(self.direction.to_elements())
                ],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                vec![xmltree::Element::node("geoPointR")
                    .with_children(self.geo_point_r.to_elements())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Station {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Station {
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                geo_point_a: element
                    .get_at_path(&["geoPointA"])
                    .map_err(savon::Error::from)
                    .and_then(|e| GeoPoint::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                ids_next_a: element
                    .get_at_path(&["idsNextA"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                line: element
                    .get_at_path(&["line"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                ids_next_r: element
                    .get_at_path(&["idsNextR"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                station_area: element
                    .get_at_path(&["stationArea"])
                    .map_err(savon::Error::from)
                    .and_then(|e| StationArea::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                direction: element
                    .get_at_path(&["direction"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Direction::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                geo_point_r: element
                    .get_at_path(&["geoPointR"])
                    .map_err(savon::Error::from)
                    .and_then(|e| GeoPoint::from_element(&e).map_err(savon::Error::from))
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PerturbationConsequence {
        pub code: Option<String>,
        pub id: Option<String>,
        pub level: Option<String>,
        pub name: Option<String>,
    }
    impl savon::gen::ToElements for PerturbationConsequence {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("code").with_text(self.code.to_string())],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                vec![xmltree::Element::node("level").with_text(self.level.to_string())],
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for PerturbationConsequence {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(PerturbationConsequence {
                code: element
                    .get_at_path(&["code"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                level: element
                    .get_at_path(&["level"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct WrStations {
        pub ambiguous_lines: Option<Vec<Line>>,
        pub stations: Option<Vec<Station>>,
        pub ambiguity_message: Option<String>,
        pub ambiguous_geo_points: Option<Vec<GeoPoint>>,
        pub argument_direction: Option<Direction>,
        pub argument_geo_point: Option<GeoPoint>,
        pub argument_line: Option<Line>,
        pub distances: Option<Vec<i64>>,
    }
    impl savon::gen::ToElements for WrStations {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                self.ambiguous_lines
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousLines")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                self.stations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stations").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("ambiguityMessage")
                    .with_text(self.ambiguity_message.to_string())],
                self.ambiguous_geo_points
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousGeoPoints")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("argumentDirection")
                    .with_children(self.argument_direction.to_elements())],
                vec![xmltree::Element::node("argumentGeoPoint")
                    .with_children(self.argument_geo_point.to_elements())],
                vec![xmltree::Element::node("argumentLine")
                    .with_children(self.argument_line.to_elements())],
                self.distances
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("distances").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for WrStations {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(WrStations {
                ambiguous_lines: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Line::from_element(&elem)?);
                    }
                    v
                }),
                stations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Station::from_element(&elem)?);
                    }
                    v
                }),
                ambiguity_message: element
                    .get_at_path(&["ambiguityMessage"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                ambiguous_geo_points: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(GeoPoint::from_element(&elem)?);
                    }
                    v
                }),
                argument_direction: element
                    .get_at_path(&["argumentDirection"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Direction::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                argument_geo_point: element
                    .get_at_path(&["argumentGeoPoint"])
                    .map_err(savon::Error::from)
                    .and_then(|e| GeoPoint::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                argument_line: element
                    .get_at_path(&["argumentLine"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                distances: element
                    .get_at_path(&["distances"])
                    .and_then(|e| e.as_long())
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Tarif {
        pub demi_tarif: f64,
        pub via_reseau: Option<Reseau>,
        pub plein_tarif: f64,
        pub via_line: Option<Line>,
    }
    impl savon::gen::ToElements for Tarif {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("demiTarif").with_text(self.demi_tarif.to_string())],
                vec![xmltree::Element::node("viaReseau")
                    .with_children(self.via_reseau.to_elements())],
                vec![xmltree::Element::node("pleinTarif").with_text(self.plein_tarif.to_string())],
                vec![xmltree::Element::node("viaLine").with_children(self.via_line.to_elements())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Tarif {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Tarif {
                demi_tarif: element
                    .get_at_path(&["demiTarif"])
                    .map_err(savon::Error::from)
                    .and_then(|e| {
                        e.get_text()
                            .ok_or(savon::rpser::xml::Error::Empty)
                            .map_err(savon::Error::from)
                            .and_then(|s| s.parse().map_err(savon::Error::from))
                    })?,
                via_reseau: element
                    .get_at_path(&["viaReseau"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Reseau::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                plein_tarif: element
                    .get_at_path(&["pleinTarif"])
                    .map_err(savon::Error::from)
                    .and_then(|e| {
                        e.get_text()
                            .ok_or(savon::rpser::xml::Error::Empty)
                            .map_err(savon::Error::from)
                            .and_then(|s| s.parse().map_err(savon::Error::from))
                    })?,
                via_line: element
                    .get_at_path(&["viaLine"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Reseau {
        pub name: Option<String>,
        pub image: Option<String>,
        pub id: Option<String>,
        pub code: Option<String>,
    }
    impl savon::gen::ToElements for Reseau {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
                vec![xmltree::Element::node("image").with_text(self.image.to_string())],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                vec![xmltree::Element::node("code").with_text(self.code.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Reseau {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Reseau {
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                image: element
                    .get_at_path(&["image"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                code: element
                    .get_at_path(&["code"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct WrMissions {
        pub ambiguous_directions: Option<Vec<Direction>>,
        pub argument_direction: Option<Direction>,
        pub argument_line: Option<Line>,
        pub ambiguous_lines: Option<Vec<Line>>,
        pub ambiguity_message: Option<String>,
        pub ambiguous_stations: Option<Vec<Station>>,
        pub argument_date: Option<String>,
        pub missions: Option<Vec<Mission>>,
        pub perturbations: Option<Vec<Perturbation>>,
        pub argument_station: Option<Station>,
    }
    impl savon::gen::ToElements for WrMissions {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                self.ambiguous_directions
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousDirections")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("argumentDirection")
                    .with_children(self.argument_direction.to_elements())],
                vec![xmltree::Element::node("argumentLine")
                    .with_children(self.argument_line.to_elements())],
                self.ambiguous_lines
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousLines")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("ambiguityMessage")
                    .with_text(self.ambiguity_message.to_string())],
                self.ambiguous_stations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("ambiguousStations")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("argumentDate")
                    .with_text(self.argument_date.to_string())],
                self.missions
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("missions").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                self.perturbations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("perturbations")
                                    .with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("argumentStation")
                    .with_children(self.argument_station.to_elements())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for WrMissions {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(WrMissions {
                ambiguous_directions: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Direction::from_element(&elem)?);
                    }
                    v
                }),
                argument_direction: element
                    .get_at_path(&["argumentDirection"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Direction::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                argument_line: element
                    .get_at_path(&["argumentLine"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                ambiguous_lines: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Line::from_element(&elem)?);
                    }
                    v
                }),
                ambiguity_message: element
                    .get_at_path(&["ambiguityMessage"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                ambiguous_stations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Station::from_element(&elem)?);
                    }
                    v
                }),
                argument_date: element
                    .get_at_path(&["argumentDate"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                missions: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Mission::from_element(&elem)?);
                    }
                    v
                }),
                perturbations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Perturbation::from_element(&elem)?);
                    }
                    v
                }),
                argument_station: element
                    .get_at_path(&["argumentStation"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Station::from_element(&e).map_err(savon::Error::from))
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Line {
        pub name: Option<String>,
        pub realm: Option<String>,
        pub id: Option<String>,
        pub reseau: Option<Reseau>,
        pub code: Option<String>,
        pub image: Option<String>,
        pub code_stif: Option<String>,
    }
    impl savon::gen::ToElements for Line {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("name").with_text(self.name.to_string())],
                vec![xmltree::Element::node("realm").with_text(self.realm.to_string())],
                vec![xmltree::Element::node("id").with_text(self.id.to_string())],
                vec![xmltree::Element::node("reseau").with_children(self.reseau.to_elements())],
                vec![xmltree::Element::node("code").with_text(self.code.to_string())],
                vec![xmltree::Element::node("image").with_text(self.image.to_string())],
                vec![xmltree::Element::node("codeStif").with_text(self.code_stif.to_string())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for Line {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(Line {
                name: element
                    .get_at_path(&["name"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                realm: element
                    .get_at_path(&["realm"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                id: element
                    .get_at_path(&["id"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                reseau: element
                    .get_at_path(&["reseau"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Reseau::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                code: element
                    .get_at_path(&["code"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                image: element
                    .get_at_path(&["image"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
                code_stif: element
                    .get_at_path(&["codeStif"])
                    .and_then(|e| {
                        e.get_text()
                            .map(|s| s.to_string())
                            .ok_or(savon::rpser::xml::Error::Empty)
                    })
                    .ok(),
            })
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PerturbationIncidentLine {
        pub consequence: Option<PerturbationConsequence>,
        pub message_medium: Option<PerturbationMessage>,
        pub stations: Option<Vec<Station>>,
        pub line: Option<Line>,
        pub message_short: Option<PerturbationMessage>,
        pub message_large: Option<PerturbationMessage>,
    }
    impl savon::gen::ToElements for PerturbationIncidentLine {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            vec![
                vec![xmltree::Element::node("consequence")
                    .with_children(self.consequence.to_elements())],
                vec![xmltree::Element::node("messageMedium")
                    .with_children(self.message_medium.to_elements())],
                self.stations
                    .as_ref()
                    .map(|v| {
                        v.iter()
                            .map(|i| {
                                xmltree::Element::node("stations").with_children(i.to_elements())
                            })
                            .collect()
                    })
                    .unwrap_or_else(Vec::new),
                vec![xmltree::Element::node("line").with_children(self.line.to_elements())],
                vec![xmltree::Element::node("messageShort")
                    .with_children(self.message_short.to_elements())],
                vec![xmltree::Element::node("messageLarge")
                    .with_children(self.message_large.to_elements())],
            ]
            .drain(..)
            .flatten()
            .collect()
        }
    }
    impl savon::gen::FromElement for PerturbationIncidentLine {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            Ok(PerturbationIncidentLine {
                consequence: element
                    .get_at_path(&["consequence"])
                    .map_err(savon::Error::from)
                    .and_then(|e| {
                        PerturbationConsequence::from_element(&e).map_err(savon::Error::from)
                    })
                    .ok(),
                message_medium: element
                    .get_at_path(&["messageMedium"])
                    .map_err(savon::Error::from)
                    .and_then(|e| PerturbationMessage::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                stations: Some({
                    let mut v = vec![];
                    for elem in element.children.iter().filter_map(|c| c.as_element()) {
                        v.push(Station::from_element(&elem)?);
                    }
                    v
                }),
                line: element
                    .get_at_path(&["line"])
                    .map_err(savon::Error::from)
                    .and_then(|e| Line::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                message_short: element
                    .get_at_path(&["messageShort"])
                    .map_err(savon::Error::from)
                    .and_then(|e| PerturbationMessage::from_element(&e).map_err(savon::Error::from))
                    .ok(),
                message_large: element
                    .get_at_path(&["messageLarge"])
                    .map_err(savon::Error::from)
                    .and_then(|e| PerturbationMessage::from_element(&e).map_err(savon::Error::from))
                    .ok(),
            })
        }
    }
}
pub mod messages {
    use super::types;
    use savon::internal::xmltree;
    #[derive(Clone, Debug, Default)]
    pub struct getLinesResponse(pub types::getLinesResponse);
    impl savon::gen::ToElements for getLinesResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getLinesResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getLinesResponse::from_element(element).map(getLinesResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getLinesRequest(pub types::getLines);
    impl savon::gen::ToElements for getLinesRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getLinesRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getLines::from_element(element).map(getLinesRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getStationsRequest(pub types::getStations);
    impl savon::gen::ToElements for getStationsRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getStationsRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getStations::from_element(element).map(getStationsRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionsNextRequest(pub types::getMissionsNext);
    impl savon::gen::ToElements for getMissionsNextRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionsNextRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMissionsNext::from_element(element).map(getMissionsNextRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getPerturbationsResponse(pub types::getPerturbationsResponse);
    impl savon::gen::ToElements for getPerturbationsResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getPerturbationsResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getPerturbationsResponse::from_element(element).map(getPerturbationsResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionResponse(pub types::getMissionResponse);
    impl savon::gen::ToElements for getMissionResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMissionResponse::from_element(element).map(getMissionResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionsNextResponse(pub types::getMissionsNextResponse);
    impl savon::gen::ToElements for getMissionsNextResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionsNextResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMissionsNextResponse::from_element(element).map(getMissionsNextResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionsFrequencyResponse(pub types::getMissionsFrequencyResponse);
    impl savon::gen::ToElements for getMissionsFrequencyResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionsFrequencyResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMissionsFrequencyResponse::from_element(element)
                .map(getMissionsFrequencyResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getDirectionsRequest(pub types::getDirections);
    impl savon::gen::ToElements for getDirectionsRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getDirectionsRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getDirections::from_element(element).map(getDirectionsRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getDirectionsResponse(pub types::getDirectionsResponse);
    impl savon::gen::ToElements for getDirectionsResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getDirectionsResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getDirectionsResponse::from_element(element).map(getDirectionsResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionsFrequencyRequest(pub types::getMissionsFrequency);
    impl savon::gen::ToElements for getMissionsFrequencyRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionsFrequencyRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMissionsFrequency::from_element(element).map(getMissionsFrequencyRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getVersionResponse(pub types::getVersionResponse);
    impl savon::gen::ToElements for getVersionResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getVersionResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getVersionResponse::from_element(element).map(getVersionResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getGeoPointsResponse(pub types::getGeoPointsResponse);
    impl savon::gen::ToElements for getGeoPointsResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getGeoPointsResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getGeoPointsResponse::from_element(element).map(getGeoPointsResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getPerturbationsRequest(pub types::getPerturbations);
    impl savon::gen::ToElements for getPerturbationsRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getPerturbationsRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getPerturbations::from_element(element).map(getPerturbationsRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionsFirstLastRequest(pub types::getMissionsFirstLast);
    impl savon::gen::ToElements for getMissionsFirstLastRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionsFirstLastRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMissionsFirstLast::from_element(element).map(getMissionsFirstLastRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getGeoPointsRequest(pub types::getGeoPoints);
    impl savon::gen::ToElements for getGeoPointsRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getGeoPointsRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getGeoPoints::from_element(element).map(getGeoPointsRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionRequest(pub types::getMission);
    impl savon::gen::ToElements for getMissionRequest {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionRequest {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMission::from_element(element).map(getMissionRequest)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getMissionsFirstLastResponse(pub types::getMissionsFirstLastResponse);
    impl savon::gen::ToElements for getMissionsFirstLastResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getMissionsFirstLastResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getMissionsFirstLastResponse::from_element(element)
                .map(getMissionsFirstLastResponse)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct getStationsResponse(pub types::getStationsResponse);
    impl savon::gen::ToElements for getStationsResponse {
        fn to_elements(&self) -> Vec<xmltree::Element> {
            self.0.to_elements()
        }
    }
    impl savon::gen::FromElement for getStationsResponse {
        fn from_element(element: &xmltree::Element) -> Result<Self, savon::Error> {
            types::getStationsResponse::from_element(element).map(getStationsResponse)
        }
    }
}
pub struct Wsiv {
    pub base_url: String,
    pub client: savon::internal::reqwest::Client,
}
#[allow(dead_code)]
impl Wsiv {
    pub fn new(base_url: String) -> Self {
        Self::with_client(base_url, savon::internal::reqwest::Client::new())
    }
    pub fn with_client(base_url: String, client: savon::internal::reqwest::Client) -> Self {
        Wsiv { base_url, client }
    }
    pub async fn get_missions_first_last(
        &self,
        get_missions_first_last_request: messages::GetMissionsFirstLastRequest,
    ) -> Result<Result<messages::getMissionsFirstLastResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getMissionsFirstLast",
            &get_missions_first_last_request,
        )
        .await
    }
    pub async fn get_geo_points(
        &self,
        get_geo_points_request: messages::GetGeoPointsRequest,
    ) -> Result<Result<messages::getGeoPointsResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getGeoPoints",
            &get_geo_points_request,
        )
        .await
    }
    pub async fn get_missions_frequency(
        &self,
        get_missions_frequency_request: messages::GetMissionsFrequencyRequest,
    ) -> Result<Result<messages::getMissionsFrequencyResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getMissionsFrequency",
            &get_missions_frequency_request,
        )
        .await
    }
    pub async fn get_perturbations(
        &self,
        get_perturbations_request: messages::GetPerturbationsRequest,
    ) -> Result<Result<messages::getPerturbationsResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getPerturbations",
            &get_perturbations_request,
        )
        .await
    }
    pub async fn get_version(
        &self,
        get_version_request: messages::GetVersionRequest,
    ) -> Result<Result<messages::getVersionResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getVersion",
            &get_version_request,
        )
        .await
    }
    pub async fn get_lines(
        &self,
        get_lines_request: messages::GetLinesRequest,
    ) -> Result<Result<messages::getLinesResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getLines",
            &get_lines_request,
        )
        .await
    }
    pub async fn get_stations(
        &self,
        get_stations_request: messages::GetStationsRequest,
    ) -> Result<Result<messages::getStationsResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getStations",
            &get_stations_request,
        )
        .await
    }
    pub async fn get_missions_next(
        &self,
        get_missions_next_request: messages::GetMissionsNextRequest,
    ) -> Result<Result<messages::getMissionsNextResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getMissionsNext",
            &get_missions_next_request,
        )
        .await
    }
    pub async fn get_mission(
        &self,
        get_mission_request: messages::GetMissionRequest,
    ) -> Result<Result<messages::getMissionResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getMission",
            &get_mission_request,
        )
        .await
    }
    pub async fn get_directions(
        &self,
        get_directions_request: messages::GetDirectionsRequest,
    ) -> Result<Result<messages::getDirectionsResponse, ()>, savon::Error> {
        savon::http::request_response(
            &self.client,
            &self.base_url,
            "http://wsiv.ratp.fr",
            "getDirections",
            &get_directions_request,
        )
        .await
    }
}
