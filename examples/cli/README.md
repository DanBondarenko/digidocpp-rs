CLI to demonstrate main functionalities of `libdigidocpp`. 

Commands:
- `validate <file>`: Validate signatures of container.
  - Example: `cargo run -p digidocpp-example-cli -- validate document.asice`
- `info <file>`: Show information about container.
  - Example: `cargo run -p digidocpp-example-cli -- info document.asice`
- `extract <file> --output <dir>`: Extract data files from container.
  - Example: `cargo run -p digidocpp-example-cli -- extract document.asice --output files`
- `extend <file> --output <path>`: Extend signatures of container.
  - Example: `cargo run -p digidocpp-example-cli -- extend document.asice --output extended.asice`
- `sign <file> --output <path>`: Sign file, creating a new container. Requires ID card connected to your system.
  - Example: `cargo run -p digidocpp-example-cli -- sign document.txt --output document.asice`
  - Requires PIN2, asked interactively. Can also be given via `PIN2` env var. 
  - Warning: three consecutive wrong PINs locks your ID card.

Settings:
- Defaults point at SK test environment.
- `--conf`/`CONF`: libdigidocpp config file, by default searches system directories
- `--tsl-url`/`TSL_URL`: TSL URL or file
- `--tsl-cert`/`TSL_CERT`: DER certificate that signed the TSL
- `--time-stamp-url`/`TIME_STAMP_URL`: timestamp service URL

If libdigidocpp C++ has been installed to a custom location, use these env vars when building: 
- `DIGIDOCPP_FRAMEWORK_DIR` 
- `DIGIDOCPP_INCLUDE_DIR` and `DIGIDOCPP_LIB_DIR`
- `DIGIDOCPP_DIR` (Windows)

Tests require network and use real APIs (SK test TSL and demo time-stamp service).
