# Server example

HTTP server to demonstrate main functionalities of `libdigidocpp`.

Endpoints:
- `POST /validate`: Validate signatures of container.
  - Example: `curl -F file=@document.asice localhost:3000/validate`
- `POST /info`: Show information about container.
  - Example: `curl -F file=@document.asice localhost:3000/info`
- `POST /store_files`: Mock extracting container's data files into storage
  - Example: `curl -F file=@document.asice localhost:3000/store_files`
- `POST /extend`: Extend signatures of container.
  - Example: `curl -F file=@document.asice localhost:3000/extend -o extended.asice`
- `POST /sign/p12`: Sign file with PKCS #12, creating a new container.
  - Example: `curl -F file=@document.txt localhost:3000/sign/p12 -o document.asice`
- `POST /sign/external`: Mock signing file with an external signer (e.g. web signature, mobile ID, e-seal). Placeholder external signer behavior is signing with a `.p12` file.
  - Example: `curl -F file=@document.txt localhost:3000/sign/external -o document.asice`

Server behavior can be controlled via env vars: 
- Defaults point at SK test environment.
- `PORT`: listening port
- `CONF`: libdigidocpp config file, by default searches system directories
- `TSL_URL`: TSL URL or file
- `TSL_CERT`: DER certificate that signed the TSL
- `TIME_STAMP_URL`: timestamp service URL
- `P12_FILE`: PKCS #12 file used for signing
- `P12_PASSWORD`: password of PKCS #12  file

Notes:
- Start with `cargo run -p digidocpp-example-server`.
- Server is single-threaded since `libdigidocpp` is not thread-safe in practice.

If libdigidocpp C++ has been installed to a custom location, use these env vars when building: 
- `DIGIDOCPP_FRAMEWORK_DIR` 
- `DIGIDOCPP_INCLUDE_DIR` and `DIGIDOCPP_LIB_DIR`
- `DIGIDOCPP_DIR` (Windows)
