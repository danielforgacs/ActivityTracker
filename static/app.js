const url_startActivity = 'api/start'
const url_stopActivity = 'api/stop'
const activityBtnIDprefix = 'activity-btn-'
const stoppedStatus = 'Idle'

let start_time_tag = document.getElementById('start_time')
let day_length_tag = document.getElementById('day_length')
let total_activity_time_tag = document.getElementById('total_activity_time')
let time_left_tag = document.getElementById('time_left')
let activities_div = document.getElementById('activities')

function toggle_activity(event) {
    let is_active = event.submitter.classList.contains('active')
    let name = event.submitter.getAttribute('activity_name')
    if (is_active) {
        fetch(url_stopActivity, {method: "POST", name: name})
    } else {
        if (event.target.id == 'create_form') {
            name = event.target[0].value
            event.target[0].value = ''
        }
        fetch(`${url_startActivity}`, {
            method: "POST",
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({name: name}),
        })
    }
}

function set_header_values(data) {
    start_time_tag.innerHTML = data.start_time_pretty
    total_activity_time_tag.innerHTML = data.total_activity_time
    day_length_tag.innerHTML = data.day_length
    time_left_tag.innerHTML = `<strong>${data.time_left}</strong>`
}

function update_activity_btn(button, task) {
    let new_text = `${task.name} - ${task.logged_pretty}`
    if (button.innerHTML != new_text) {
        button.innerHTML = new_text
    }
}

function set_button_status(button, task) {
    let active_task = task.status != stoppedStatus
    let active_button = button.classList.contains('active')
    if (active_button != active_task) {
        if (active_task) {
            button.classList.add('active')
        } else {
            button.classList.remove('active')
        }
    }
}

function mk_activity_button(task) {
    let acivity_btn_template = `
    <form onsubmit="event.preventDefault(); toggle_activity(event)">
        <button class="" id="${activityBtnIDprefix}${task.name}" activity_name="${task.name}">
        </button>
    </form>
    `
    let activity_form_div = document.createElement('div')
    activity_form_div.innerHTML = acivity_btn_template
    activity_form_div.id = task.name
    activities_div.appendChild(activity_form_div)
    let task_button = document.getElementById(activityBtnIDprefix+task.name)
    return task_button
}

function manage_activity_buttons(data) {
    let task_names = []
    for (task of data.activities) {
        task_names.push(task.name)
        let task_button = document.getElementById(activityBtnIDprefix+task.name)
        if (task_button === null) {
            task_button = mk_activity_button(task)
        }
        update_activity_btn(task_button, task)
        set_button_status(task_button, task)
    }
    loop1: for (activity_div of activities_div.childNodes) {
        let found = false
        for (name of task_names.reverse()) {
            if (activity_div.id == name) {
                continue loop1
            }
        }
        activities_div.removeChild(activity_div)
    }
}

function body_builder(data) {
    set_header_values(data)
    manage_activity_buttons(data)
}

let year = new Date().getUTCFullYear();
let month = new Date().getMonth();
month = `${month+1}`
month = month.padStart(2, "0")
let day = new Date().getDate();
day = `${day}`
day = day.padStart(2, "0")
let date = `${year}-${month}-${day}`

setInterval(() => {
    fetch('api/activities', {
        method: "POST",
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({date: date}),
    })
    .then(response => response.json())
    .then(data => body_builder(data));
}, 750);
