
pub fn parse_request_line(request: &str) -> Option<(&str, &str, &str)> {
    let (request_line, _) = request.split_once("\r\n")?;
    let [http_method, target, http_version] = request_line.split_whitespace().collect::<Vec<&str>>()[..] else { return None };
    Some((http_method, target, http_version))
}