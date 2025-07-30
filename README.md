# Convolution
## Installation
### Supported Platforms
Currently, only Windows and MacOS are officially supported. 
**Linux:** Although I've not personally tested this, it's probably possible to use [yabridge](https://github.com/robbert-vdh/yabridge) to get this plugin working on Linux.
(IF you try this method, please let me know if it works or not, so that I can update these docs).
### Downloads
Go to the ["Actions" tab](https://github.com/dvub/convolution_plug/actions) of this repository, click on a build (you probably want the most recent passing build), and scroll down to the "Artifacts" section. From here, download the zip file for your platform and extract the contents.
### Building/Compiling
```shell
# compile GUI
cd gui
npm install # or pnpm, bun, etc.
npm run build
cd ..
# compile plugin
cargo xtask bundle convolution_plug --release
```
## Features
**Note**: Currently, this plugin does not bundle any IRs - you're on your own.
(If you know of free, high-quality IRs that I could bundle, let me know!) 
## Contributions
I am currently looking for people willing to contribute (appropriately sourced) IRs, as well as people who may want to design some nice presets, etc. 

Otherwise, I'd like to mainly work on this project myself. 
## Future Work
- add preset system
- use (fundsp) Net for DSP
- be able to apply envelope to IRs
- predelay
- decay / speed (something..) - maybe check convology
- reverse (for fun LOL)
- add visual EQ controls (spline type thing)
- support stereo/quad/whatever IRs as well as multiple formats - somewhat supported, but no parameters
- general optimizations
