use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::HashMap;

pub fn lookup(code: u16) -> Result<()> {
    let statuses = get_statuses();
    
    if let Some((name, description)) = statuses.get(&code) {
        println!("{}: {}", "Code".bold(), code.cyan());
        println!("{}: {}", "Name".bold(), name.green());
        println!("{}: {}", "Description".bold(), description);
    } else {
        anyhow::bail!("Unknown HTTP status code: {}", code);
    }
    
    Ok(())
}

pub fn list() -> Result<()> {
    let statuses = get_statuses();
    let mut codes: Vec<_> = statuses.keys().cloned().collect();
    codes.sort();
    
    for code in codes {
        let (name, _) = statuses.get(&code).unwrap();
        println!("{:>3} - {}", code.cyan(), name.green());
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

    // 3xx Redirection
    m.insert(300, ("Multiple Choices", "The request has more than one possible response."));
    m.insert(301, ("Moved Permanently", "The URL of the requested resource has been changed permanently."));
    m.insert(302, ("Found", "The URL of the requested resource has been changed temporarily."));
    m.insert(303, ("See Other", "The server sent this response to direct the client to get the requested resource at another URI with a GET request."));
    m.insert(304, ("Not Modified", "The resource has not been modified since the last request."));
    m.insert(307, ("Temporary Redirect", "The server sends this response to direct the client to get the requested resource at another URI with same method that was used in the prior request."));
    m.insert(308, ("Permanent Redirect", "The server sends this response to direct the client to get the requested resource at another URI with same method that was used in the prior request."));

    // 4xx Client Error
    m.insert(400, ("Bad Request", "The server cannot or will not process the request due to an apparent client error."));
    m.insert(418, ("I'm a teapot", "The server refuses the attempt to brew coffee with a teapot."));

    // 5xx Server Error
    m.insert(500, ("Internal Server Error", "The server has encountered a situation it doesn't know how to handle."));
    m.insert(503, ("Service Unavailable", "The server is not ready to handle the request."));

    // ... many more could be added, but these are common.
    // To be thorough, let's add a few more common ones.
    m.insert(401, ("Unauthorized", "The client must authenticate itself to get the requested response."));
    m.insert(403, ("Forbidden", "The client does not have access rights to the content."));
    m.insert(404, ("Not Found", "The server can not find the requested resource."));
    m.insert(405, ("Method Not Allowed", "The request method is known by the server but is not supported by the target resource."));
    m.insert(408, ("Request Timeout", "The server would like to shut down this unused connection."));
    m.insert(409, ("Conflict", "This response is sent when a request conflicts with the current state of the server."));
    m.insert(410, ("Gone", "This response is sent when the requested content has been permanently deleted from server."));
    m.insert(413, ("Payload Too Large", "Request entity is larger than limits defined by server."));
    m.insert(415, ("Unsupported Media Type", "The media format of the requested data is not supported by the server."));
    m.insert(422, ("Unprocessable Entity", "The request was well-formed but was unable to be followed due to semantic errors."));
    m.insert(429, ("Too Many Requests", "The user has sent too many requests in a given amount of time."));
    m.insert(501, ("Not Implemented", "The request method is not supported by the server and cannot be handled."));
    m.insert(502, ("Bad Gateway", "The server, while working as a gateway to get a response needed to handle the request, got an invalid response."));
    m.insert(504, ("Gateway Timeout", "The server, while working as a gateway, cannot get a response in time."));

    m
}
