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
        <div>
            <form id="create_form" onSubmit="return create_activity(event)">
                <input type="text" name="new_name" id="new_name">
                <input type="submit" value="create / activate">
            </form>
            <form onSubmit="event.preventDefault(); fetch('api/stop')">
                <input type="submit" value="stop">
            </form>
        </div>
        <div id="activities">
        </div>
    </div>
</body>
<script>
    function create_activity(event) {
        event.preventDefault()
        let name = event.target[0].value
        fetch('api/start/'+name)
        event.target[0].value = ''
    }

    function toggle_activity(event) {
        // console.log(event)
        console.log("SUBMITTER", event.submitter)
        // console.log(event.target)
        // console.log(event.target.childNodes[0])
        // console.log(event.target.childNodes[0].activity_name)
        // console.log("pressed: ", event.target[0].activity_name)
        let name = event.submitter.getAttribute("activity_name")
        console.log("event.submitter.activity_name:", name)
        if (event.submitter.getAttribute("active") == "active") {
            fetch('api/stop')
        } else {
            fetch('api/start/'+name)
        }
    }

    function builder(data) {
        let activities_div = document.getElementById('activities')
        activities_div.innerHTML = ""
        for (activity of data.tasks) {
            let form = document.createElement("form")
            form.setAttribute('onSubmit', 'event.preventDefault(); toggle_activity(event)')
            let button = document.createElement("button")
            button.textContent = activity.name + " - " + activity.all_time_pretty
            button.setAttribute('activity_name', activity.name)
            if (activity.status == "Idle") {
                button.setAttribute("stopped", "stopped")
            } else {
                button.setAttribute("active", "active")
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
