# 🦀 CampusDual API 🦀
A (stateless) REST API which provides the link between NetWeaver-based CampusDual and a remotely reasonable frontend.
Miraculously already far more reliable and performant than the very origin it scrapes all the data from.
# CampusUnbloat
While being fairly unopinionated, this API is part of the [**CampusUnbloat**](https://github.com/greybaron/campus-unbloat) project.

## Build Dependencies
* A working Rust toolchain
* `JWT_SECRET=something AES_KEY=something_32chars cargo run`

the project includes the `GEANT OV RSA CA 4` CA certificate, which is linked into the binary. Trust but verify

## Using the API
* $api/signin has to be called with a `POST`-request and a JSON-body like
```
{
  username: "username",
  password: "password"
}
```
A `JWT` token (and some basic info) is then returned.
* Any other endpoint can be called using `GET`/`POST` and the `Authorization: "Bearer ${token}"` header (check out `routes.rs` for a list of endpoints).
* Many CampusDual calls depend on the (short-lived) cookie within this JWT. If it is expired, the CaDu call will hang indefinitely. Any session is only valid for a few hours.
* For that reason, `/check_revive_session` should be called regularly (but not every request). If the previous session was expired, a new JWT is returned.
## Data policy
No data is ever logged or stored by this API.

Session data is only stored client-side and is encrypted using an AES256 key that only the server possesses.

However since the server needs to 'see' the username and password whenever CampusDual calls are made, a bad actor could easily deploy a manipulated version that stores credentials.

Therefore, as long as the build chain is not verifiable, a modicum of cautiousness is in order.

