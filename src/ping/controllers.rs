use crate::AppState;
use crate::ping::entities::{DBResponse};
use actix_web::{HttpResponse, Responder, get, web};


#[get("/ping")]
async fn ping(data: web::Data<AppState>) -> impl Responder {
    let connection_exists = sqlx::query_as!(
        DBResponse,
        "SELECT 1 AS connection_exists",
    )
        .fetch_one(&data.db)
        .await;

    match connection_exists {
        Ok(response) => {
            let http_response = serde_json::json!(
                {
                    "status": "success",
                    "data": serde_json::json!(
                        {
                            "all_ok": response
                        }
                    )
                }
            );

            return HttpResponse::Ok().json(http_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(
                    serde_json::json!(
                        {
                            "status": "error",
                            "message": format!("{:?}", e)
                        }
                    )
                );
        }
    }
}
