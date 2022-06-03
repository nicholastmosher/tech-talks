use std::str::FromStr;

fn main() {
    let my_url = "http://localhost:3000/some/path?query1=value1&query2=value2#fragment";
    let simple_url = SimpleUrl::from_str(my_url);
    // let simple_url: SimpleUrl = FromStr::from_str(my_url).unwrap();
    // let simple_url: SimpleUrl = my_url.parse().unwrap();
    // let simple_url = my_url.parse::<SimpleUrl>().unwrap();

    println!("{:#?}", simple_url);
}

#[derive(Debug)]
struct SimpleUrl {
    scheme: String,
    host: String,
    port: String,
    path: String,
    query: Vec<(String, String)>,
    fragment: String,
}

impl FromStr for SimpleUrl {
    type Err = String;

    // Example input string:
    //
    // http://localhost:3000/some/path?query1=value1&query2=value2#fragment
    fn from_str(s: &str) -> Result<SimpleUrl, Self::Err> {
        // ("http", "localhost:3000/some/path?query1=value1&query2=value2#fragment")
        let (scheme, rest) = s
            .split_once("://")
            .ok_or_else(|| "failed to get scheme".to_string())?;

        // ("localhost", "3000/some/path?query1=value1&query2=value2#fragment")
        let (host, rest) = rest
            .split_once(':')
            .ok_or_else(|| "failed to get hostname")?;

        // ("3000", "some/path?query1=value1&query2=value2#fragment")
        let (port, rest) = rest.split_once('/').ok_or_else(|| "failed to get port")?;

        // ("some/path", "query1=value1&query2=value2#fragment")
        let (path, rest) = rest.split_once('?').ok_or_else(|| "failed to get path")?;

        // ("query1=value1&query2=value2", "fragment")
        let (queries, fragment) = rest
            .split_once('#')
            .ok_or_else(|| "failed to split queries from fragment")?;

        // ["query1=value1", "query2=value2"]
        let query_entries: Vec<&str> = queries.split('&').collect();

        // [("query1", "value1"), ("query2", "value2")]
        let query: Vec<(String, String)> = query_entries
            .into_iter()
            .map(|entry| entry.split_once('=').unwrap())
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        Ok(SimpleUrl {
            scheme: scheme.to_string(),
            host: host.to_string(),
            port: port.to_string(),
            path: path.to_string(),
            query,
            fragment: fragment.to_string(),
        })
    }
}
