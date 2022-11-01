## Activity Tracker

Web app to track time spent on activities.

base endpoint:
    http://127.0.0.1:<PORT>/api/

default port is 8000.

api endpoints:
    start/{name}        starts tracking an activity. If it doesn't exist it
                        will be created. All other activities will be stopped,
                        only one activity can be active at a time.
    stop                stops any activity.
    times               returns the taskmanager as json.
    pretty              return the taskmanager as formatted string.
