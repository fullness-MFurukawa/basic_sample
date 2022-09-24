use actix_web::{error, get, HttpResponse, Responder, web};
use tera::Tera;

// 入力画面要求
#[get("/calc")]
pub async fn enter(tera: web::Data<Tera>) -> impl Responder {
    // HTMLの取得
    let resp_body = tera.render(
        "pages/enter.html", &tera::Context::new())
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))
        .unwrap();
    // レスポンスの送信
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(resp_body)
}