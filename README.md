# SDK

A tool for Xcode to show SDK paths.

## Installation

```sh
cargo install --git https://github.com/schwa/sdk-rs
```

## Usage

```sh
# List all SDKs
sdk
# Print the path of the SDK with the given name
sdk mac
# Print the path of the the System/Library/Frameworks directory of the SDK with the given name
sdk mac --frameworks
```

## Why?

So I can easily find, and search, the SDKs.

```sh
$ rg MTLMatrixLayout (sdk mac --frameworks)
/Applications/Xcode-16.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX15.sdk/System/Library/Frameworks/Metal.framework/Versions/A/Headers/MTLAccelerationStructure.h
77:typedef NS_ENUM(NSInteger, MTLMatrixLayout) {
81:    MTLMatrixLayoutColumnMajor = 0,
86:    MTLMatrixLayoutRowMajor = 1,
272: * matrix buffer. Defaults to MTLMatrixLayoutColumnMajor.
274:@property (nonatomic) MTLMatrixLayout transformationMatrixLayout API_AVAILABLE(macos(15.0), ios(18.0));
390: * matrix buffer. Defaults to MTLMatrixLayoutColumnMajor.
392:@property (nonatomic) MTLMatrixLayout transformationMatrixLayout API_AVAILABLE(macos(15.0), ios(18.0));
1049: * transformation matrix buffer. Defaults to MTLMatrixLayoutColumnMajor.
1051:@property (nonatomic) MTLMatrixLayout instanceTransformationMatrixLayout API_AVAILABLE(macos(15.0), ios(18.0));
1148: * transformation matrix buffer. Defaults to MTLMatrixLayoutColumnMajor.
1150:@property (nonatomic) MTLMatrixLayout instanceTransformationMatrixLayout API_AVAILABLE(macos(15.0), ios(18.0));
```
