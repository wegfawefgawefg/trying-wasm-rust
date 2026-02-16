const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = (_, argv) => ({
    mode: argv.mode ?? "development",
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
        clean: true,
    },
    devtool: argv.mode === "production" ? "source-map" : "eval-cheap-module-source-map",
    devServer: {
        static: {
            directory: __dirname,
        },
        hot: true,
        port: 8080,
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "index.html",
        }),
        new WasmPackPlugin({
            crateDirectory: __dirname,
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
});
