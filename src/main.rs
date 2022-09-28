use actix_web::{App, HttpServer, middleware, web};
use actix_web::web::{resource, ServiceConfig};
use tera::Tera;
use basic_sample::handlers;

//    main関数(configure()メソッドの利用)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* 前処理 */
    // ロガーの初期化する
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Template Engine Teraを生成する
    let tera = Tera::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();

    // HttpServerの起動
    HttpServer::new(move || {
        /* 提供する機能を定義する */
        App::new()
            .wrap(middleware::Logger::default()) // ロギングミドルウェアの登録
            .app_data(web::Data::new(tera.clone())) // Template Engineの登録
            .configure(set_configure) //  パスとハンドラのマッピング
    }).bind("127.0.0.1:8080")?.run().await
}
// パスとハンドラのマッピング定義
fn set_configure(cfg: &mut ServiceConfig) -> () {
    cfg.service(
        // スコープを設定する
        web::scope("/basic_sample")
        .service(
            // パスパターン設定する
            resource("/calc")
                // パスパターンに対するハンドラを設定する
                .route(web::get().to(handlers::enter))
                .route(web::post().to(handlers::answer))
        ));
}

//    main関数(resource()関数、rout()メソッドの利用)
/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* 前処理 */
    // ロガーの初期化する
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Template Engine Teraを生成する
    let tera = Tera::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
    // HttpServerの起動
    HttpServer::new(move || {
        /* 提供する機能を定義する */
        App::new()
            .wrap(middleware::Logger::default()) // ロギングミドルウェアの登録
            .app_data(web::Data::new(tera.clone())) // Template Engineの登録
            // Scopeを設定する
            .service(web::scope("/basic_sample")
                .service(
                        // パスパターン設定
                    resource("/calc")
                        // パスパターンに対するハンドラマッピング
                        .route(web::get().to(handlers::enter))
                        .route(web::post().to(handlers::answer))
                )
            )
    }).bind("127.0.0.1:8080")?.run().await
}
*/

//    main関数(rout()メソッドの利用)
/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* 前処理 */
    // ロガーの初期化する
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Template Engine Teraを生成する
    let tera = Tera::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
    // HttpServerの起動
    HttpServer::new(move || {
        /* 提供する機能を定義する */
        App::new()
            .wrap(middleware::Logger::default()) // ロギングミドルウェアの登録
            .app_data(web::Data::new(tera.clone())) // Template Engineの登録
            .service(web::scope("/basic_sample")
                // route()メソッドを利用してリクエストとハンドラをマッピングする
                .route("/calc",web::get().to(handlers::enter))
                .route("/calc",web::post().to(handlers::answer)))
    }).bind("127.0.0.1:8080")?.run().await
}
*/

//    main関数(HTTPメソッドに対応したマクロの利用)
/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* 前処理 */
    // ロガーの初期化する
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Template Engine Teraを生成する
    let tera = Tera::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
    // HttpServerの起動
    HttpServer::new(move || {
        /* 提供する機能を定義する */
        App::new()
            .wrap(middleware::Logger::default()) // ロギングミドルウェアの登録
            .app_data(web::Data::new(tera.clone())) // Template Engineの登録
            // サービス(ルーティング)を設定する
            .service(
                web::scope("/basic_sample")  // 共通なパスを設定
                    // リクエストハンドラを設定
                    .service(handlers::enter)
                    .service(handlers::answer)
            )
    }).bind("127.0.0.1:8080")?.run().await
}
*/