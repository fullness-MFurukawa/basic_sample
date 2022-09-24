use std::ops::Deref;
use actix_web::{error, get , post , HttpResponse, Responder, web};
use log::info;
use tera::Tera;
use crate::form::CalcForm;

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
// 計算処理要求
#[post("/calc")]
pub async fn answer(form: web::Form<CalcForm> ,
                         tera: web::Data<Tera>) -> impl Responder {
    info!("入力値 {:?}",form);
    // 計算処理と結果の取得
    let result = match calc(form.deref()){
        Ok(result) => result ,
        Err(error)  => error.to_string()
    };
    info!("実行結果 = {}", result);
    // Contextに計算結果を格納
    let mut context = tera::Context::new();
    context.insert("result" , &result);
    // 計算結果をTeraに渡してHTMLを取得
    let resp_body = tera.render(
        "pages/result.html", &context)
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))
        .unwrap();
    // レスポンスの送信
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(resp_body)
}
// 計算関数
fn calc(form: &CalcForm) -> anyhow::Result<String> {
    let result = match form.opt{
        1 => form.value1 + form.value1 ,
        2 => form.value1 - form.value2 ,
        3 => form.value1 * form.value2 ,
        4 => form.value1 / form.value2 ,
        5 => form.value1 % form.value2 ,
        _ => return Err(anyhow::Error::msg("Parameter Error."))
    };
    Ok(result.to_string())
}
