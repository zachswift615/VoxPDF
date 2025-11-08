// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "VoxPDF",
    platforms: [
        .iOS(.v15),
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "VoxPDF",
            targets: ["VoxPDF"]
        ),
    ],
    targets: [
        .target(
            name: "VoxPDF",
            dependencies: [],
            path: "Sources/VoxPDF"
        ),
        .testTarget(
            name: "VoxPDFTests",
            dependencies: ["VoxPDF"],
            path: "Tests/VoxPDFTests",
            resources: [
                .copy("TestPDFs")
            ]
        ),
    ]
)
