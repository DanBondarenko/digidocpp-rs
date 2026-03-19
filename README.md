# TODO

The goal is to achieve feature parity with `libdigidocpp`'s public interface as exposed through SWIG.
Below is an overview of the members of that interface. Deprecated members are excluded.

## Free functions
- [x] `appInfo()`
- [x] `userAgent()`
- [x] `version()`
- [x] `terminate()`
- [ ] `initializeLib(appName, userAgent, path)`
- [x] `initializeLib(appName, userAgent)`
 
## `Conf`
- [ ] `init()`
- [ ] `instance()`
- [ ] `logLevel()`
- [ ] `logFile()`
- [ ] `xsdPath()`
- [ ] `PKCS11Driver()`
- [ ] `proxyHost()`
- [ ] `proxyPort()`
- [ ] `proxyUser()`
- [ ] `proxyPass()`
- [ ] `proxyForceSSL()`
- [ ] `proxyTunnelSSL()`
- [ ] `digestUri()`
- [ ] `signatureDigestUri()`
- [ ] `ocsp(issuer)`
- [ ] `TSUrl()`
- [ ] `verifyServiceUri()`
- [ ] `TSLAllowExpired()`
- [ ] `TSLAutoUpdate()`
- [ ] `TSLCache()`
- [ ] `TSLOnlineDigest()`
- [ ] `TSLTimeOut()`
- [ ] `TSLUrl()`
 
## `XmlConf`
- [ ] Constructor `(path = "", schema = "")`
- [ ] `instance()`
- [ ] `logLevel()`
- [ ] `logFile()`
- [ ] `PKCS11Driver()`
- [ ] `proxyHost()`
- [ ] `proxyPort()`
- [ ] `proxyUser()`
- [ ] `proxyPass()`
- [ ] `proxyForceSSL()`
- [ ] `proxyTunnelSSL()`
- [ ] `digestUri()`
- [ ] `signatureDigestUri()`
- [ ] `ocsp(issuer)`
- [ ] `TSUrl()`
- [ ] `verifyServiceUri()`
- [ ] `TSLAutoUpdate()`
- [ ] `TSLCache()`
- [ ] `TSLOnlineDigest()`
- [ ] `TSLTimeOut()`
- [ ] `setProxyHost(host)`
- [ ] `setProxyPort(port)`
- [ ] `setProxyUser(user)`
- [ ] `setProxyPass(pass)`
- [ ] `setProxyTunnelSSL(enable)`
- [ ] `setTSLOnlineDigest(enable)`
- [ ] `setTSLTimeOut(timeOut)`
- [ ] `setTSUrl(url)`
- [ ] `setVerifyServiceUri(url)`

## `DigiDocConf`
- [ ] Constructor `(cache = "")`
- [ ] `instance()`
- [ ] `setLogLevel(level)`
- [ ] `setLogFile(file)`
- [ ] `setTSCert(cert)`
- [ ] `addTSCert(cert)`
- [ ] `setTSLCert(cert)`
- [ ] `addTSLCert(cert)`
- [ ] `setTSLUrl(url)`
- [ ] `setOCSPUrls(urls)`
- [ ] `setOCSPTMProfiles(profiles)`
- [ ] `setVerifyServiceCert(cert)`
- [ ] `addVerifyServiceCert(cert)`

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
- [ ] `mediaType()`
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