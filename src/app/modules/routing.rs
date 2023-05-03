use crate::app::modules::slides::controller::routes as slides_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/api/v1/slide", slides_routes() )
    })
}
