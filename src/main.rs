use warp::Filter;
use std::fs;
use warp;

#[tokio::main]
async fn main() {
    // Чтение содержимого файла JSON в виде строки
    let file_content = fs::read_to_string("C:\\ПРОГИ\\json_pars\\src\\data.json")
        .expect("Не удалось прочитать файл");

    // Роут для отправки содержимого JSON-файла на клиент
    let send_json_file = warp::path("get_json_file")
        .map(move || {
            // Отправка содержимого JSON как текстового ответа
            warp::reply::with_status(file_content.clone(), warp::http::StatusCode::OK)
        });

    // Запуск сервера на порту 3030
    warp::serve(send_json_file)
        .run(([127, 0, 0, 1], 3030))
        .await;
}