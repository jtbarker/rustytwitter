extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate oauthcli;
extern crate tokio_core;
extern crate serde_json;


use futures::{future, Future, Stream};
use hyper::header;
use oauthcli::{OAuthAuthorizationHeaderBuilder, SignatureMethod};
use oauthcli::url::Url;
use std::env;


fn main() {
    let req = {
        let args: Vec<String> = env::args().collect();
        let query = &args[1];
        let twittersearchendpoint = String::from("https://api.twitter.com/1.1/search/tweets.json?q=");
        let endpointplusquery = twittersearchendpoint + &query;
        let url = Url::parse(&endpointplusquery).unwrap();


// Jon's auth, use your own in production!
        let auth_header = OAuthAuthorizationHeaderBuilder::new(
            "GET", &url, "MPkT3QnoOdtKI6ZhBs8uWnqDx", "78UfNqhcx7pd0E5eernVPaHh8gM7pF5kZ5Gsp75WTtWm9JVVZp", SignatureMethod::HmacSha1)
            .token("353982241-5vUFghPhuxLf7TbXwAqWe63Y7xoDyBx6glu0gukT", "7Ii4mFAOu5ZA5Lf6Mi2Y9H55zftomRBPP2anbRfhUtLJW")
            .finish_for_twitter();

        let mut req = hyper::Request::new(hyper::Get, url.as_str().parse().unwrap());
        req.headers_mut().set(header::Authorization(auth_header.to_string()));
        req
    };

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(1, &handle).unwrap())
        .build(&handle);

    let f = client.request(req)
        .and_then(|res| {
            let buf = match res.headers().get::<header::ContentLength>() {
                Some(&header::ContentLength(x)) => Vec::with_capacity(x as usize),
                None => Vec::new(),
            };

            res.body().fold(buf, |mut buf, chunk| {
                buf.extend(chunk);
                future::ok::<_, hyper::Error>(buf)
            })
        })
        .and_then(|buf|
            String::from_utf8(buf)
                .map_err(|e| hyper::Error::Utf8(e.utf8_error()))
        );

    let res = core.run(f).unwrap();
    println!("{}", &res);

}
