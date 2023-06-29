pub fn simply_notify(bodyname: &str) -> Result<(), notify_rust::error::Error> {
    notify_rust::Notification::new()
        .appname("Froggy")
        .body(bodyname)
        .show()
}