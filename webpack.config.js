const path = require('path');


module.exports = {
    entry: './build/scripts/infincia.js',
    output: {
        library: "Infincia",
        path: path.resolve(__dirname, 'dist'),
        publicPath: "/assets/",
        filename: 'scripts/infincia.bundle.js'
    },
    resolve: {
        alias: {
            "highcharts": path.resolve(__dirname, "node_modules/highcharts/highcharts.js")
        }
    }
};
