use actix_web::{get, HttpResponse};

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
                    <td>start time</td>
                    <td id="start_time"></td>
                </tr>
                <tr>
                    <td>elapsed day</td>
                    <td id="elapsed_day"></td>
                </tr>
                <tr>
                    <td>total acivity time</td>
                    <td id="total_activity_time"></td>
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
        let name = event.submitter.getAttribute("activity_name")
        if (event.submitter.getAttribute("active") == "active") {
            fetch('api/stop')
        } else {
            fetch('api/start/'+name)
        }
    }

    function builder(data) {
        console.log(data)
        document.getElementById('start_time').innerHTML = data.start_time_pretty
        document.getElementById('elapsed_day').innerHTML = data.elapsed_day
        document.getElementById('total_activity_time').innerHTML = data.total_activity_time
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
    }, 500);
</script>
</html>
"#;

#[get("/")]
async fn index_view() -> HttpResponse {
    HttpResponse::Ok()
        .body(INDEX_TEMPLATE)
}
