// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "bluefox_lib",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "plugin",
            targets: ["plugin"]
        ),
        .library(
            name: "bluefox_lib",
            targets: ["bluefox_lib"]),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "plugin",
            resources: [
                .copy("plugin.h")
            ]      
        ),
        .target(
            name: "bluefox_lib",
            dependencies: [
                "plugin"
            ]
        ),
    ]
)
