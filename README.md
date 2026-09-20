# TODO

The goal is to achieve feature parity with `libdigidocpp`'s public interface as exposed through SWIG.
Below is an overview of the members of that interface. Deprecated members are excluded.

## Free functions
- [x] `appInfo()`
- [x] `userAgent()`
- [x] `version()`
- [x] `terminate()`
- [x] `initializeLib(appName, userAgent, path)`
- [x] `initializeLib(appName, userAgent)`
 
## `Conf`/`XmlConf`/`DigiDocConf`
- [x] `init()`
- [x] XmlConf Constructor `(path = "", schema = "")`
- [x] DigiDocConf Constructor `(cache = "")`
- [ ] `instance()`
- [x] `logLevel()`
- [x] `logFile()`
- [x] `xsdPath()`
- [x] `PKCS11Driver()`
- [x] `proxyHost()`
- [x] `proxyPort()`
- [x] `proxyUser()`
- [x] `proxyPass()`
- [x] `proxyForceSSL()`
- [x] `proxyTunnelSSL()`
- [x] `digestUri()`
- [x] `signatureDigestUri()`
- [x] `ocsp(issuer)`
- [x] `TSUrl()`
- [x] `verifyServiceUri()`
- [x] `TSLAllowExpired()`
- [x] `TSLAutoUpdate()`
- [x] `TSLCache()`
- [x] `TSLOnlineDigest()`
- [x] `TSLTimeOut()`
- [x] `TSLUrl()`
- [x] `setProxyHost(host)`
- [x] `setProxyPort(port)`
- [x] `setProxyUser(user)`
- [x] `setProxyPass(pass)`
- [x] `setProxyTunnelSSL(enable)`
- [x] `setTSLOnlineDigest(enable)`
- [x] `setTSLTimeOut(timeOut)`
- [x] `setTSUrl(url)`
- [x] `setVerifyServiceUri(url)`
- [x] `setLogLevel(level)`
- [x] `setLogFile(file)`
- [x] `setTSCert(cert)`
- [x] `addTSCert(cert)`
- [x] `setTSLCert(cert)`
- [x] `addTSLCert(cert)`
- [x] `setTSLUrl(url)`
- [x] `setOCSPUrls(urls)`
- [x] `setOCSPTMProfiles(profiles)`
- [x] `setVerifyServiceCert(cert)`
- [x] `addVerifyServiceCert(cert)`

## `Container`
- [x] `open(path, ContainerOpenCB *cb)`
- [x] `extendContainerValidity(doc, signer, extendedCount)`
- [x] `save(path = "")`
- [x] `mediaType()`
- [x] `addDataFile(path, mediaType)`
- [x] `addDataFile(stream, path, mediaType)`
- [x] `dataFiles()`
- [x] `removeDataFile(index)`
- [x] `addAdESSignature(bytes)`
- [x] `prepareSignature(signer)`
- [x] `signatures()`
- [x] `removeSignature(index)`
- [x] `sign(signer)`
- [ ] ~~`prepareWebSignature(cert, profile = "", roles = [], city = "", state = "", postalCode = "", country = "")`~~

## `ContainerOpenCB`
- [x] `validateOnline()`

## `DataFile`
- [x] `id()`
- [x] `fileName()`
- [x] `fileSize()`
- [x] `mediaType()`
- [x] `calcDigest(method)`
- [x] `saveAs(path)`
- [x] `saveAs(stream)`

## `Signature`
- [x] `POLv1`
- [x] `POLv2`
- [x] `id()`
- [x] `claimedSigningTime()`
- [x] `trustedSigningTime()`
- [x] `signingCertificate()`
- [x] `signatureMethod()`
- [x] `validate()`
- [x] `validate(policy)`
- [x] `dataToSign()`
- [x] `setSignatureValue(bytes)`
- [x] `policy()`
- [x] `SPUri()`
- [x] `profile()`
- [x] `city()`
- [x] `stateOrProvince()`
- [x] `postalCode()`
- [x] `countryName()`
- [x] `streetAddress()`
- [x] `signerRoles()`
- [x] `OCSPProducedAt()`
- [x] `OCSPCertificate()`
- [x] `TimeStampCertificate()`
- [x] `TimeStampTime()`
- [x] `ArchiveTimeStamps()`
- [x] `signedBy()`
- [x] `messageImprint()`
- [x] `extendSignatureProfile(signer)`
 
## `Signature::Validator`
- [ ] ~~Constructor~~
- [x] `diagnostics()`
- [x] `status()`
- [x] `Status` (enum)

## `TSAInfo`
- [x] `cert`
- [x] `time`

## `Signer`
- [x] `cert()`
- [x] `sign(method, digest)`
- [x] `method()`
- [x] `profile()`
- [x] `userAgent()`
- [x] `usingENProfile()`
- [x] `city()`
- [x] `streetAddress()`
- [x] `stateOrProvince()`
- [x] `postalCode()`
- [x] `countryName()`
- [x] `signerRoles()`
- [x] `setMethod(method)`
- [x] `setProfile(profile)`
- [x] `setUserAgent(userAgent)`
- [x] `setENProfile(enable)`
- [x] `setSignatureProductionPlace(city, state, postalCode, country)`
- [x] `setSignatureProductionPlaceV2(city, streetAddress, state, postalCode, country)`
- [x] `setSignerRoles(roles)`
- [x] `PKCS12Signer` (subclass)
- [x] `PKCS11Signer` (subclass)
- [x] `WinSigner` (subclass)
- [x] `ExternalSigner`

## Other ideas
- [ ] add cxx shared struct for vectors of non-owned data e.g. `Container::dataFiles()` and `Container::signatures()`
- [ ] use types to prevent invalid initialization and termination
- [ ] use types to prevent invalid container ops
- [ ] use a lock to ensure `libdigidocpp` can't be called from multiple threads
- [ ] custom signers