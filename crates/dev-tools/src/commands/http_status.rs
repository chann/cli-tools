use anyhow::Result;
use std::collections::HashMap;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub fn lookup(code: u16) -> Result<()> {
    let statuses = get_statuses();
    
    if let Some((name, description)) = statuses.get(&code) {
        println!("{}", Theme::header(format!("HTTP Status: {}", code)));
        
        let mut table = TableFormatter::create_table();
        table.add_row(vec![TableFormatter::header_cell("Field"), TableFormatter::header_cell("Value")]);
        table.add_row(vec![TableFormatter::value_cell("Code"), TableFormatter::highlight_cell(code.to_string())]);
        table.add_row(vec![TableFormatter::value_cell("Name"), TableFormatter::highlight_cell(name.to_string())]);
        table.add_row(vec![TableFormatter::value_cell("Description"), TableFormatter::value_cell(description.to_string())]);
        
        println!("{}", table);
    } else {
        anyhow::bail!("Unknown HTTP status code: {}", code);
    }
    
    Ok(())
}

pub fn list() -> Result<()> {
    let statuses = get_statuses();
    let mut codes: Vec<_> = statuses.keys().cloned().collect();
    codes.sort();
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Code"),
        TableFormatter::header_cell("Name"),
        TableFormatter::header_cell("Description"),
    ]);

    for code in codes {
        let (name, desc) = statuses.get(&code).unwrap();
        table.add_row(vec![
            TableFormatter::highlight_cell(code.to_string()),
            TableFormatter::value_cell(name.to_string()),
            TableFormatter::value_cell(desc.to_string()),
        ]);
    }
    
    println!("{}", table);
    Ok(())
}

pub fn search(query: &str) -> Result<()> {
    let statuses = get_statuses();
    let query_lower = query.to_lowercase();
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Code"),
        TableFormatter::header_cell("Name"),
        TableFormatter::header_cell("Description"),
    ]);

    let mut found = false;
    let mut codes: Vec<_> = statuses.keys().cloned().collect();
    codes.sort();

    for code in codes {
        let (name, desc) = statuses.get(&code).unwrap();
        if name.to_lowercase().contains(&query_lower) || desc.to_lowercase().contains(&query_lower) {
            table.add_row(vec![
                TableFormatter::highlight_cell(code.to_string()),
                TableFormatter::value_cell(name.to_string()),
                TableFormatter::value_cell(desc.to_string()),
            ]);
            found = true;
        }
    }

    if found {
        println!("{}", Theme::info(format!("Search results for '{}':", query)));
        println!("{}", table);
    } else {
        println!("{}", Theme::warning(format!("No matching HTTP status codes found for '{}'.", query)));
    }
    
    Ok(())
}

fn get_statuses() -> HashMap<u16, (&'static str, &'static str)> {
    let mut m = HashMap::new();
    // 1xx Informational
    m.insert(100, ("Continue", "The server has received the request headers and the client should proceed to send the request body."));
    m.insert(101, ("Switching Protocols", "The requester has asked the server to switch protocols."));
    m.insert(102, ("Processing", "The server has received and is processing the request, but no response is available yet."));
    m.insert(103, ("Early Hints", "Used to return some response headers before final HTTP message."));

    // 2xx Success
    m.insert(200, ("OK", "The request has succeeded."));
    m.insert(201, ("Created", "The request has succeeded and a new resource has been created."));
    m.insert(202, ("Accepted", "The request has been accepted for processing, but the processing has not been completed."));
    m.insert(203, ("Non-Authoritative Information", "The server is a transforming proxy that received a 200 OK from its origin."));
    m.insert(204, ("No Content", "The server successfully processed the request and is not returning any content."));
    m.insert(205, ("Reset Content", "The server successfully processed the request, but is not returning any content and requires that the requester reset the document view."));
    m.insert(206, ("Partial Content", "The server is delivering only part of the resource due to a range header sent by the client."));
    m.insert(207, ("Multi-Status", "Conveys information about multiple resources, for situations where multiple status codes might be appropriate."));
    m.insert(208, ("Already Reported", "Used inside a DAV: propstat response element to avoid repeatedly enumerating the internal members of multiple bindings to the same collection."));
    m.insert(226, ("IM Used", "The server has fulfilled a GET request for the resource, and the response is a representation of the result of one or more instance-manipulations applied to the current instance."));

    // 3xx Redirection
    m.insert(300, ("Multiple Choices", "The request has more than one possible response."));
    m.insert(301, ("Moved Permanently", "The URL of the requested resource has been changed permanently."));
    m.insert(302, ("Found", "The URL of the requested resource has been changed temporarily."));
    m.insert(303, ("See Other", "The server sent this response to direct the client to get the requested resource at another URI with a GET request."));
    m.insert(304, ("Not Modified", "The resource has not been modified since the last request."));
    m.insert(305, ("Use Proxy", "The requested response must be accessed by a proxy."));
    m.insert(307, ("Temporary Redirect", "The server sends this response to direct the client to get the requested resource at another URI with same method that was used in the prior request."));
    m.insert(308, ("Permanent Redirect", "The server sends this response to direct the client to get the requested resource at another URI with same method that was used in the prior request."));

    // 4xx Client Error
    m.insert(400, ("Bad Request", "The server cannot or will not process the request due to an apparent client error."));
    m.insert(401, ("Unauthorized", "The client must authenticate itself to get the requested response."));
    m.insert(402, ("Payment Required", "Reserved for future use."));
    m.insert(403, ("Forbidden", "The client does not have access rights to the content."));
    m.insert(404, ("Not Found", "The server can not find the requested resource."));
    m.insert(405, ("Method Not Allowed", "The request method is known by the server but is not supported by the target resource."));
    m.insert(406, ("Not Acceptable", "The server cannot produce a response matching the list of acceptable values defined in the request's proactive content negotiation headers."));
    m.insert(407, ("Proxy Authentication Required", "The client must first be authenticated by a proxy."));
    m.insert(408, ("Request Timeout", "The server would like to shut down this unused connection."));
    m.insert(409, ("Conflict", "This response is sent when a request conflicts with the current state of the server."));
    m.insert(410, ("Gone", "This response is sent when the requested content has been permanently deleted from server."));
    m.insert(411, ("Length Required", "Server rejected the request because the Content-Length header field is not defined and the server requires it."));
    m.insert(412, ("Precondition Failed", "The client has put preconditions in its headers which the server does not meet."));
    m.insert(413, ("Payload Too Large", "Request entity is larger than limits defined by server."));
    m.insert(414, ("URI Too Long", "The URI requested by the client is longer than the server is willing to interpret."));
    m.insert(415, ("Unsupported Media Type", "The media format of the requested data is not supported by the server."));
    m.insert(416, ("Range Not Satisfiable", "The range specified by the Range header field in the request cannot be fulfilled."));
    m.insert(417, ("Expectation Failed", "The expectation indicated by the Expect request header field cannot be met by the server."));
    m.insert(418, ("I'm a teapot", "The server refuses the attempt to brew coffee with a teapot."));
    m.insert(421, ("Misdirected Request", "The request was directed at a server that is not able to produce a response."));
    m.insert(422, ("Unprocessable Entity", "The request was well-formed but was unable to be followed due to semantic errors."));
    m.insert(423, ("Locked", "The resource that is being accessed is locked."));
    m.insert(424, ("Failed Dependency", "The request failed due to failure of a previous request."));
    m.insert(425, ("Too Early", "Indicates that the server is unwilling to risk processing a request that might be replayed."));
    m.insert(426, ("Upgrade Required", "The server refuses to perform the request using the current protocol but might be willing to do so after the client upgrades to a different protocol."));
    m.insert(428, ("Precondition Required", "The origin server requires the request to be conditional."));
    m.insert(429, ("Too Many Requests", "The user has sent too many requests in a given amount of time."));
    m.insert(431, ("Request Header Fields Too Large", "The server is unwilling to process the request because its header fields are too large."));
    m.insert(451, ("Unavailable For Legal Reasons", "The user requests an illegal resource, such as a web page censored by a government."));

    // 5xx Server Error
    m.insert(500, ("Internal Server Error", "The server has encountered a situation it doesn't know how to handle."));
    m.insert(501, ("Not Implemented", "The request method is not supported by the server and cannot be handled."));
    m.insert(502, ("Bad Gateway", "The server, while working as a gateway to get a response needed to handle the request, got an invalid response."));
    m.insert(503, ("Service Unavailable", "The server is not ready to handle the request."));
    m.insert(504, ("Gateway Timeout", "The server, while working as a gateway, cannot get a response in time."));
    m.insert(505, ("HTTP Version Not Supported", "The HTTP version used in the request is not supported by the server."));
    m.insert(506, ("Variant Also Negotiates", "The server has an internal configuration error: the chosen variant resource is configured to engage in transparent content negotiation itself, and is therefore not a proper end point in the negotiation process."));
    m.insert(507, ("Insufficient Storage", "The server is unable to store the representation needed to complete the request."));
    m.insert(508, ("Loop Detected", "The server detected an infinite loop while processing the request."));
    m.insert(510, ("Not Extended", "Further extensions to the request are required for the server to fulfill it."));
    m.insert(511, ("Network Authentication Required", "The client needs to authenticate to gain network access."));

    m
}
