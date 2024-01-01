pub fn create_caddyfile<'a>() -> &'a str {
    "
localhost

bind 0.0.0.0 [::]
file_server
    "
}
