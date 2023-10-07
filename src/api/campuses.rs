use super::super::{AppState, Campus, GetCampusById, MAX_SIZE};
use actix_web::{error, web, HttpResponse, Responder};
use futures::StreamExt;
use log::{error, info};

pub async fn add_campus(state: web::Data<AppState>, mut payload: web::Payload) -> impl Responder {
    let mut campuses = state.campuses.lock().unwrap();
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<Campus>(&body)?;

    campuses.insert(
        obj.id,
        Campus {
            id: obj.id,
            created_at: obj.created_at.to_string(),
            updated_at: obj.updated_at.to_string(),
            name: obj.name.to_string(),
            image_url: obj.image_url.to_string(),
            address: obj.address.to_string(),
            description: obj.description.to_string(),
        },
    );

    if campuses.contains_key(&obj.id) {
        info!("Campus created with ID: {}", obj.id);
        Ok(HttpResponse::Ok().body("Campus created!"))
    } else {
        error!("Failed to create campus with ID: {}", obj.id);
        Ok(HttpResponse::Ok().body("Failed to create campus."))
    }
}

pub async fn get_all_campuses(state: web::Data<AppState>) -> impl Responder {
    let campuses = state.campuses.lock().unwrap();

    for (key, _value) in campuses.iter() {
        println!("key: {}", key);
    }

    let all_campuses: Vec<&Campus> = campuses.values().collect();

    HttpResponse::Ok().json(all_campuses)
}

pub async fn get_campus_by_id(
    state: web::Data<AppState>,
    path: web::Path<GetCampusById>,
) -> impl Responder {
    let campuses = state.campuses.lock().unwrap();

    let campus = campuses.get(&path.id);

    HttpResponse::Ok().json(campus)
}

pub async fn delete_campus_by_id(
    state: web::Data<AppState>,
    path: web::Path<GetCampusById>,
) -> impl Responder {
    let mut campuses = state.campuses.lock().unwrap();
    campuses.remove(&path.id);

    if campuses.contains_key(&path.id) {
        info!("Deleted campus with ID: {}", path.id);
        HttpResponse::Ok().body("Failed to delete campus!")
    } else {
        error!("Failed to delete campus with ID: {}", path.id);
        HttpResponse::Ok().body("Campus deleted!")
    }
}
