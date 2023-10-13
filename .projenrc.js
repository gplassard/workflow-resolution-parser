// .projenrc.js
const { RustProject } = require('@gplassard/projen-extensions');
const version = require('./package.json').version;

const project = new RustProject({
   name: 'workflow-resolution-parser',
   cargo: {
      package: {
         authors: ["Gabriel Plassard <gabriel.plassard@gmail.com>"],
         version: version,
         edition: "2021",
      },
      dependencies: {
      }
   }
});
project.synth();
