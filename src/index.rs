use actix_web::{HttpRequest, get, HttpResponseBuilder, http, HttpResponse, web::Data};
use std::sync::Mutex;
use tera::{Tera, Context};
use crate::TaskManager;

const INDEX_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset='utf-8'>
    <meta http-equiv='X-UA-Compatible' content='IE=edge'>
    <title>Page Title</title>
    <meta name='viewport' content='width=device-width, initial-scale=1'>
    <link rel='stylesheet' type='text/css' media='screen' href='main.css'>
    <script>
        function builder(data) {
            console.log(data.tasks)
            let body = document.body
            body.innerHTML = ""
            let main_div = document.createElement("div")
            body.appendChild(main_div)
            for (item of data.tasks) {
                let div = document.createElement("div")
                div.appendChild(document.createTextNode(item.name))
                div.appendChild(document.createTextNode(item.all_time_pretty))
                main_div.appendChild(div)
            }
        }
        setInterval(() => {
            fetch('api/times')
                .then(response => response.json())
                .then(data => builder(data));
        }, 2000);
    </script>
</head>
<body>
</body>
</html>
"#;

#[get("/")]
async fn index_view(req: HttpRequest) -> HttpResponse {
    // let data = req.app_data::<Data<Mutex<TaskManager>>>().unwrap();
    // let mut tm = data.lock().unwrap();
    // let mut tera = Tera::default();
    // let template_name = "something-cool.html";
    // tera.add_raw_template(template_name, INDEX_TEMPLATE).unwrap();
    // let mut ctx = Context::new();
    // ctx.insert("message", "Coming from Rust! Check the console!");
    // ctx.insert("start_time", &tm.times());
    // let render = tera.render(template_name, &ctx).unwrap();
    HttpResponse::Ok()
        // .body(render)
        .body(INDEX_TEMPLATE)
}
