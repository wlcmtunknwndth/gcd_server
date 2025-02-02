mod gcd;

extern crate iron;
#[macro_use] extern crate mime;
extern crate router;

use router::Router;

use iron::prelude::*;
use iron::status;


fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:4000...");
    Iron::new(router).http("localhost:4000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset = Utf8));
    response.set_mut(r#"
        <title>GCD Caclulator</title>
        <form action="/gcd" method="post">
         <input type="text" name="n"/>
         <input type="text" name="n"/>
         <button type="submit">Compute GCD</button>
        </form>
    "#);

    Ok(response)
}


extern crate urlencoded;

use std::str::FromStr;
use iron::status::Status;
use urlencoded::UrlEncodedBody;

use gcd::gcd;
fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response)
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();

    for unparsed in unparsed_numbers{
        match u64::from_str(&unparsed){
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter not a number: {:?}\n",
                            unparsed));
                return Ok(response)
            }
            Ok(n) => {numbers.push(n);}
        }
    };

    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d, *m);
    }

    response.set_mut(Status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n", numbers, d)
    );
    Ok(response)
}