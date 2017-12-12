use rocket;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};

mod create;
mod read;
mod update;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

pub fn launch_admin_pages() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
               static_file,
               read::get_sport_types,
               read::get_sport_type_events,
               read::get_event,
               read::get_session,
               read::create_session_form,
               read::create_event_form,
               read::create_event_form_for_sport_type,
               update::update_event,
               update::update_session,
               create::create_session,
               create::create_event,
               ])
        .launch();
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/static").join(file)).ok()
}
