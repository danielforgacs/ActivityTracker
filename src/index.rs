use actix_web::{HttpRequest, get, HttpResponseBuilder, http, HttpResponse, web::Data};
use std::sync::Mutex;
use tera::{Tera, Context};
use crate::TaskManager;

const INDEX_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TimeTrack</title>
    <style>
        [active] {
            background-color: green;
        }
        [stopped] {
            background-color: grey;
        }
    </style>
</head>
<body>
    <div>
        <div>
            <table>
                <tr>
                    <td>time</td>
                    <td>2022-07-02 00:14:45.053681369 +01:00</td>
                </tr>
                <tr>
                    <td>elapsed day</td>
                    <td>21h:37m</td>
                </tr>
                <tr>
                    <td>total acivity time</td>
                    <td>21h:36m</td>
                </tr>
            </table>
        </div>
        <div id="activities">
        </div>
    </div>
</body>
<script>
    function builder(data) {
        console.log(data.tasks)
        let activities_div = document.getElementById('activities')
        activities_div.innerHTML = ""
        for (activity of data.tasks) {
            let form = document.createElement("form")
            form.action = "/api/start/some-activity"
            form.method = "get"
            let button = document.createElement("button")
            button.textContent = activity.name + " - " + activity.all_time_pretty
            if (activity.status == "Idle") {
                button.setAttribute("stopped", "")
            } else {
                button.setAttribute("active", "")
            }
            form.appendChild(button)
            activities_div.appendChild(form)
        }
    }
    setInterval(() => {
        fetch('api/times')
        .then(response => response.json())
        .then(data => builder(data));
    }, 2000);

</script>
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
