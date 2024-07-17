# 🦀 CampusDual API 🦀
A (stateless) REST API which provides the link between NetWeaver-based CampusDual and a remotely reasonable frontend.
Miraculously already far more reliable and performant than the very origin it scrapes all the data from.
# CampusUnbloat
While being fairly unopinionated, this API is part of the [**CampusUnbloat**](https://github.com/greybaron/campus-unbloat) project.

## Build Dependencies
* A working Rust toolchain
* SSL development files (e.g. `libssl-dev` on Debian)
* the `GEANT OV RSA CA 4` certificate must be installed. On most Linux distributions, this certificate is not shipped

## Using the API
* $api/signin has to be called with a `POST`-request and a JSON-body like
```
{
  username: "username",
  password: "password"
}
```
A `JWT` token (and some basic info) is then returned.
* Any other endpoint can be called using a `GET` and the `Authorization: "Bearer ${token}"` header.
## Data policy
No data is ever logged or stored. However since the username and password are sent to the API
in cleartext (save for SSL) and there really aren't any guarantees wrt/ the build chain, a modicum of cautiousness is in order.
