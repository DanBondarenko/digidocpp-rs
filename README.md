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
 
## `Conf`
- [x] `init()`
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
 
## `XmlConf`
- [x] Constructor `(path = "", schema = "")`
- [ ] `instance()`
- [x] `logLevel()`
- [x] `logFile()`
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
- [x] `TSLAutoUpdate()`
- [x] `TSLCache()`
- [x] `TSLOnlineDigest()`
- [x] `TSLTimeOut()`
- [x] `setProxyHost(host)`
- [x] `setProxyPort(port)`
- [x] `setProxyUser(user)`
- [x] `setProxyPass(pass)`
- [x] `setProxyTunnelSSL(enable)`
- [x] `setTSLOnlineDigest(enable)`
- [x] `setTSLTimeOut(timeOut)`
- [x] `setTSUrl(url)`
- [x] `setVerifyServiceUri(url)`

## `DigiDocConf`
- [ ] ~~Constructor `(cache = "")`~~
- [ ] `instance()`
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
- [ ] `save(path = "")`
- [x] `mediaType()`
- [ ] `addDataFile(path, mediaType)`
- [x] `dataFiles()`
- [ ] `removeDataFile(index)`
- [ ] `addAdESSignature(bytes)`
- [ ] `prepareSignature(signer)`
- [ ] `signatures()`
- [ ] `removeSignature(index)`
- [ ] `sign(signer)`
- [ ] `prepareWebSignature(cert, profile = "", roles = [], city = "", state = "", postalCode = "", country = "")`

## `ContainerOpenCB`
- [ ] `validateOnline()`

## `DataFile`
- [x] `id()`
- [x] `fileName()`
- [x] `fileSize()`
- [x] `mediaType()`
- [x] `calcDigest(method)`
- [ ] `saveAs(path)`

## `Signature`
- [ ] `POLv1`
- [ ] `POLv2`
- [ ] `id()`
- [ ] `claimedSigningTime()`
- [ ] `trustedSigningTime()`
- [ ] `signingCertificate()`
- [ ] `signatureMethod()`
- [ ] `validate()`
- [ ] `validate(policy)`
- [ ] `dataToSign()`
- [ ] `setSignatureValue(bytes)`
- [ ] `policy()`
- [ ] `SPUri()`
- [ ] `profile()`
- [ ] `city()`
- [ ] `stateOrProvince()`
- [ ] `postalCode()`
- [ ] `countryName()`
- [ ] `streetAddress()`
- [ ] `signerRoles()`
- [ ] `OCSPProducedAt()`
- [ ] `OCSPCertificate()`
- [ ] `TimeStampCertificate()`
- [ ] `TimeStampTime()`
- [ ] `ArchiveTimeStamps()`
- [ ] `signedBy()`
- [ ] `messageImprint()`
- [ ] `extendSignatureProfile(signer)`
 
## `Signature::Validator`
- [ ] Constructor
- [ ] `diagnostics()`
- [ ] `status()`
- [ ] `Status` (enum)

## `TSAInfo`
- [ ] `cert`
- [ ] `time`

## `Signer`
- [ ] `cert()`
- [ ] `sign(method, digest)`
- [ ] `method()`
- [ ] `profile()`
- [ ] `userAgent()`
- [ ] `usingENProfile()`
- [ ] `city()`
- [ ] `streetAddress()`
- [ ] `stateOrProvince()`
- [ ] `postalCode()`
- [ ] `countryName()`
- [ ] `signerRoles()`
- [ ] `setMethod(method)`
- [ ] `setProfile(profile)`
- [ ] `setUserAgent(userAgent)`
- [ ] `setENProfile(enable)`
- [ ] `setSignatureProductionPlace(city, state, postalCode, country)`
- [ ] `setSignatureProductionPlaceV2(city, streetAddress, state, postalCode, country)`
- [ ] `setSignerRoles(roles)`
- [ ] `PKCS12Signer` (subclass)
- [ ] `PKCS11Signer` (subclass)
- [ ] `WinSigner` (subclass)