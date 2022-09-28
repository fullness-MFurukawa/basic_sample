use actix_web::{error, HttpResponse, Responder, web};
use log::info;
//use actix_web::{ get , post }; use log::info;
use tera::Tera;
use crate::form::CalcForm;

// 入力画面要求
//#[get("/calc")]
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
//#[post("/calc")]
pub async fn answer(form: web::Form<CalcForm> ,
                         tera: web::Data<Tera>) -> impl Responder {
    info!("入力値 {:?}",form);
    // 計算処理と結果の取得
    let result = match calc(&form){
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

#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::{App, web, test};
    use actix_http::Request;
    use actix_web::dev::ServiceResponse;
    use actix_web::http::StatusCode;
    use actix_web::web::resource;

    // テスト用Serviceの準備
    async fn init_test_service() -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
        let test_service = test::init_service(
            App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(
                web::scope("/basic_sample")
                .service(
                    resource("/calc")
                        .route(web::get().to(enter))
                    .route(web::post().to(answer)))
                )
        ).await;
        test_service
    }
    // GETリクエストのテスト enter()関数
    #[actix_web::test]
    async fn test_enter() -> () {
        // テスト用Serviceの取得する
        let test_service = init_test_service().await;

        // GETリクエストを生成する
        let enter_request = test::TestRequest::get().uri("/basic_sample/calc").to_request();
        // リクエストハンドラenter()を実行する
        let response = test::call_service(&test_service, enter_request).await;
        // ヘッダーを出力する
        println!("{:?}" , response.headers());
        // ボディを出力する
        println!("{:?}" , response.response().body());
        // ステータスコードを評価する
        assert_eq!(response.status() , StatusCode::OK);
    }

    // POSTリクエストをテスト answer関数
    #[actix_web::test]
    async fn test_answer() -> () {
        // テスト用Serviceの取得する
        let test_service = init_test_service().await;
        // 入力データを準備する
        let calc_form = CalcForm{value1:100,value2:200,opt:1};
        // CalcFormを格納したPostリクエストを生成する
        let answer_request = test::TestRequest::post().uri("/basic_sample/calc").set_form(&calc_form).to_request();
        // リクエストハンドラanswer()を実行する
        let response = test::call_service(&test_service , answer_request).await;
        // ヘッダーを出力する
        println!("{:?}" , response.headers());
        // ボディを出力する
        println!("{:?}" , response.response().body());
        // ステータスコードを評価する
        assert_eq!(response.status() , StatusCode::OK);
    }


}
