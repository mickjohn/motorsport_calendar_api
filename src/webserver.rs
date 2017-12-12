use rocket;
use rocket::response::content;
use diesel::prelude::*;
use super::database;

use super::model::Event as MEvent;
use super::model::Session as MSession;
use schema::events;
use schema::sessions;

use super::model;
use itertools::Itertools;
use serde_json;

#[get("/")]
fn all_events() -> content::Json<String> {
    let connection = database::establish_connection();
    let my_events: Vec<(MEvent, Option<MSession>)> = events::table.left_join(sessions::table).load(&connection).expect("Error loading events");

    let mut cevents = Vec::new();
    for (_ ,group) in &my_events.iter().group_by(|t| t.0.id) {
        let mut mevent = None;
        let mut msessions = Vec::new();
        for &(ref ev, ref session) in group {

            if mevent.is_none() {
                mevent = Some(ev.clone());
            }

            if session.is_some() {
                let s = session.as_ref().unwrap().clone();
                msessions.push(s);
            }
        }
        if let Some(ev) = mevent {
            let e = model::from_model(ev.clone(), msessions);
            cevents.push(e);
        }
        // println!("EVENT = {:?}", cevents);
    }
    let json = serde_json::to_string(&cevents).unwrap();
    content::Json(json)
}


#[get("/<event_id>")]
fn event(event_id: i32) -> content::Json<String> {
    let conn = database::establish_connection();
    let model_event = events::table
        .filter(events::id.eq(Some(event_id)))
        .first::<MEvent>(&conn)
        .expect("Can't load event");

    let model_sessions: Vec<MSession> = sessions::table
        .filter(sessions::event_id.eq(&event_id))
        .load(&conn)
        .expect("Can't load sessions");

    let e = model::from_model(model_event, model_sessions);
    let json = serde_json::to_string(&e).unwrap();
    content::Json(json)
}

pub fn start() {
    rocket::ignite()
        .mount("/events", routes![
               all_events,
               event,
               ])
        .launch();
}

#[cfg(test)]
mod test {
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method, ContentType};
    use rocket::{Response, Rocket};
    use serde_json;
    use diesel::prelude::*;
    use chrono::{NaiveDate, NaiveDateTime};
    // use motorsport_calendar_common::event::Event;
    use super::*;
    use super::super::model::Event as MEvent;
    use super::super::model::Sessions as MSession;
    use super::super::event_loader;
    use super::super::data::json_data;
    use std::io::prelude::*;
    use std::fs::File;

    // const TEST_DATA_DIR: &'static str = "test/data";

    fn init_test_data() -> Vec<Event> {
        let events = event_loader::load_events_from_yml_file(&format!("{}/test_data.yml", TEST_DATA_DIR)).unwrap();
        json_data::init(&events);
        events
    }

    // fn init_db(events: Vec<Event>) -> SqliteConnection {
    // }
    

    fn get_test_data() -> (Vec<MEvent>, Vec<MSession>) {
        (load_test_events(), load_test_sessions())
    }

    fn load_test_events() -> Vec<MEvent> {
        let mut f = File::open("test/data/test_events.json").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        serde_json::from_str(&s).unwrap()
    }

    fn load_test_sessions() -> Vec<MSession> {
        let mut f = File::open("test/data/test_sessions.json").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        serde_json::from_str(&s).unwrap()
    }

    fn init_rocket() -> Rocket {
        rocket::ignite()
            .mount("/", routes![
                   events,
                   event,
                   ])
    }

    fn assert_status_and_string(mut response: Response,status: Status, s: String) {
        assert_eq!(response.status(), status);
        let body_string = response.body().and_then(|b| b.into_string()).unwrap();
        assert_eq!(body_string, s);
    }

    #[test]
    fn test_get_events() {
        let test_events = init_test_data();
        let rocket = init_rocket();
        let mut req = MockRequest::new(Method::Get, "/events").header(ContentType::Json);
        let response = req.dispatch_with(&rocket);
        let expected = serde_json::to_string(&test_events).unwrap();
        assert_status_and_string(response, Status::Ok, expected);
    }

    #[test]
    fn test_get_events_by_type() {
        init_test_data();
        let rocket = init_rocket();
        for sport_type in vec!["DTM", "GP2", "Formula 1", "doesnt_exist"] {
            let get = format!("/events/{}", sport_type);
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
            let response = req.dispatch_with(&rocket);
            let expected = match json_data::DATA.read().unwrap().events_type.get(sport_type) {
                None => "{}".to_string(),
                Some(x) => x.to_string()
            };
            assert_status_and_string(response, Status::Ok, expected);
        }
    }
    
    
    #[test]
    fn test_get_events_by_type_and_round() {
        init_test_data();
        let rocket = init_rocket();
        let keys = vec![
            ("DTM".to_string(), 0),
            ("DTM".to_string(), 1),
            ("DTM".to_string(), 2),
            ("DTM".to_string(), 3),
            ("GP2".to_string(), 999),
            ("GP2".to_string(), 1),
            ("GP2".to_string(), 2),
            ("Formula 1".to_string(), 0),
            ("Formula 1".to_string(), 1),
            ];

        for key in keys {
            let get = format!("/events/{}/{}", key.0, key.1);
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
            let response = req.dispatch_with(&rocket);
            let expected = match json_data::DATA.read().unwrap().events_type_round.get(&key) {
                None => "{}".to_string(),
                Some(x) => x.to_string()
            };
            assert_status_and_string(response, Status::Ok, expected);
        }
    }

    #[test]
    fn test_get_events_by_type_and_round_and_session() {
        init_test_data();
        let rocket = init_rocket();
        let keys = vec![
            ("DTM".to_string(), 0, 1),
            ("DTM".to_string(), 0, 2),
            ("DTM".to_string(), 1, 0),
            ("DTM".to_string(), 1, 1),
            ("DTM".to_string(), 1, 2),
            ("DTM".to_string(), 1, 3),
            ("DTM".to_string(), 2, 1),
            ("DTM".to_string(), 2, 2),
            ("DTM".to_string(), 2, 30),
            ("Formula 1".to_string(), 0, 1),
            ("Formula 1".to_string(), 1, 2),
            ("Formula 1".to_string(), 1, 3),
            ("Formula 1".to_string(), 1, 444),
            ];

        for key in keys {
            let get = format!("/events/{}/{}/{}", key.0, key.1, key.2);
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
            let response = req.dispatch_with(&rocket);
            let expected = match json_data::DATA.read().unwrap().events_type_round_num.get(&key) {
                None => "{}".to_string(),
                Some(x) => x.to_string()
            };
            assert_status_and_string(response, Status::Ok, expected);
        }
    }
}
