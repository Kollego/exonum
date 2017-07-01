module.exports = function(config) {
    config.set({
        browsers: ['Firefox'],
        frameworks: ['mocha', 'chai', 'sinon'],
        files: [
            {pattern: './node_modules/jquery/dist/jquery.min.js', watched: false},
            {pattern: './node_modules/exonum-client/dist/exonum-client.min.js', watched: false},
            {pattern: './js/cryptocurrency.js', watched: false},
            {pattern: './test_data/**/*.json', watched: false, included: false, served: true, nocache: false},
            './test/cryptocurrency.js'
        ],
        proxies: {
            '/test_data/': "/base/test_data/"
        },
        reporters: ['mocha'],
        singleRun: true
    })
};
