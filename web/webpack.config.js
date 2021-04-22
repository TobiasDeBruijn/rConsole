'use strict';
const path = require('path');

module.exports = {
    devtool: "eval-cheap-source-map",
	entry: './src/ts/index.ts',
	output: {
		filename: 'dist.js',
        path: path.resolve(__dirname, 'dist'),
        libraryTarget: 'var',
        library: 'rConsole'
	},
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: 'ts-loader'
            },
            {
                test: /\.scss$/,
                use: [
                    'style-loader',
                    'css-loader',
                    'sass-loader',
                ]
            }
        ]
    },
    resolve: {
        extensions: [ '.ts', '.js', '.css', '.sass', '.scss' ],
    },
    externals: {
        // require("jquery") is external and available
        //  on the global var jQuery
        "jquery": "jQuery",
    },
    mode: 'production',
    watchOptions: {
        poll: true,
        ignored: "/node_modules/"
    }
};